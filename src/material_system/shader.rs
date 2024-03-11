use std::{collections::BTreeMap, fs, io::{Error, Read, Write}};

use hassle_rs::*;
use rspirv_reflect::*;
use log::{debug, error, trace, warn};
use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use serde::{Deserialize, Serialize, Deserializer, de::Visitor, de::MapAccess};
use std::fmt;
use std::path::{Path, PathBuf};

fn add_prefix_to_file_name(path: &str, prefix: &str) -> Option<PathBuf> {
    let mut path_buf = PathBuf::from(path);
    if let Some(file_name) = path_buf.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            let new_file_name = format!("{}{}", prefix, file_name_str);
            path_buf.set_file_name(new_file_name);
            return Some(path_buf);
        }
    }
    None
}

const TEXTURE_REGISTER_OFFSET : usize = 0;
const SAMPLER_REGISTER_OFFSET : usize = 20;
const CBUFFER_REGISTER_OFFSET : usize = 40;
const UAV_REGISTER_OFFSET : usize = 60;
const SHADER_ROOT_DIR : &str = r"assets\shaders\";
const SHADER_OUT_ROOT_DIR : &str = r"assets\shaders\out\";

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::d3d12_pso::InputElementDesc;

pub fn compile_shader(
    name : &str,
    source:&str,
    entry_point : &str,
    shader_model : &str,
    is_spirv : bool,
    in_macros: &Vec<&str>
) -> Result<Vec<u8>, String>
{
    let texture_register_offset_str = TEXTURE_REGISTER_OFFSET.to_string();
    let sampler_register_offset_str = SAMPLER_REGISTER_OFFSET.to_string();
    let cbuffer_register_offset_str = CBUFFER_REGISTER_OFFSET.to_string();
    let uav_register_offset_str = UAV_REGISTER_OFFSET.to_string();

    let mut compile_args = vec!["/Zi", "/Od"];
    if is_spirv == true
    {
        compile_args.push("-spirv");

        compile_args.push("-fvk-t-shift");
        compile_args.push(&texture_register_offset_str);
        compile_args.push("0");

        compile_args.push("-fvk-s-shift");
        compile_args.push(&sampler_register_offset_str);
        compile_args.push("0");

        compile_args.push("-fvk-b-shift");
        compile_args.push(&cbuffer_register_offset_str);
        compile_args.push("0");

        compile_args.push("-fvk-u-shift");
        compile_args.push(&uav_register_offset_str);
        compile_args.push("0");
    }
    for macro_str in in_macros
    {
        compile_args.push(macro_str);
    }

    let result = hassle_rs::utils::compile_hlsl(
        name,
        source,
        entry_point,
        shader_model,
        &compile_args,
        &[],
    );

    match result{
        Ok(bytecode) =>
        {
            debug!("Shader {} compiled successfully", name);
            Ok(bytecode)
        }
        Err(error) =>
        {
            error!("Cannot compile shader: {}", &error);
            Err(error)
        }
    }
}

pub fn get_shader_reflection(
    name : &str,
    source:&str,
    entry_point : &str,
    shader_model : &str,
    in_macros: &Vec<&str>
) -> Option<Reflection>
{
    let compile_result = compile_shader(name, source, entry_point, shader_model, true, in_macros);
    let reflection_module=
    match rspirv_reflect::Reflection::new_from_spirv(compile_result.unwrap().as_ref())
    {
        Ok(refl) => Some(refl),
        Err(refl_err) =>
        {
            error!("Failed To Reflect {}.", &refl_err);
            None
        }
    };
    reflection_module
}

const FIELDS: &[&str] = &["descriptor_type", "binding_count", "name"];
#[derive(Serialize)]
struct DescriptorInfoWrapper
{
    descriptor_type : u32,
    binding_count: usize,
    name: String,
}

struct DescriptorInfoWrapperVisitor;
impl<'de> Visitor<'de> for DescriptorInfoWrapperVisitor {
    type Value = DescriptorInfoWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a struct DescriptorInfoWrapper with fields descriptor_type, binding_count and name.")
    }

    // `visit_map` 用于从序列化的 Map 数据中构造 Point
    fn visit_map<V>(self, mut map: V) -> Result<DescriptorInfoWrapper, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut descriptor_type = None;
        let mut binding_count = None;
        let mut name = None;
        while let Some(key) = map.next_key()? {
            match key {
                "descriptor_type" => {
                    if descriptor_type.is_some() {
                        return Err(serde::de::Error::duplicate_field("descriptor_type"));
                    }
                    descriptor_type = Some(map.next_value()?);
                }
                "binding_count" => {
                    if binding_count.is_some() {
                        return Err(serde::de::Error::duplicate_field("binding_count"));
                    }
                    let mut real_binding = Some(map.next_value()?).ok_or_else(|| serde::de::Error::missing_field("binding_count"))?;
                    if real_binding >= SAMPLER_REGISTER_OFFSET && real_binding < CBUFFER_REGISTER_OFFSET
                    {
                        real_binding = real_binding - SAMPLER_REGISTER_OFFSET;
                    }
                    else if real_binding >= CBUFFER_REGISTER_OFFSET && real_binding < UAV_REGISTER_OFFSET
                    {
                        real_binding = real_binding - CBUFFER_REGISTER_OFFSET;
                    }
                    else if real_binding >= UAV_REGISTER_OFFSET
                    {
                        real_binding = real_binding - UAV_REGISTER_OFFSET;
                    }
    
                    binding_count = Some(real_binding);
                }
                "name" => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }

                _ => {
                    return Err(serde::de::Error::unknown_field(key, FIELDS));
                }
            }
        }
        let descriptor_type = descriptor_type.ok_or_else(|| serde::de::Error::missing_field("descriptor_type"))?;
        let binding_count = binding_count.ok_or_else(|| serde::de::Error::missing_field("binding_count"))?;
        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        Ok(DescriptorInfoWrapper { descriptor_type, binding_count, name })
    }
}

impl<'a> Deserialize<'a> for DescriptorInfoWrapper
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'a>
    {       
        deserializer.deserialize_struct("DescriptorInfoWrapper", FIELDS, DescriptorInfoWrapperVisitor)
    }
}

fn create_folder(input_file_path : String)
{
    let file_folder = input_file_path.clone() + "\\..";
    if !std::path::Path::new(&file_folder).exists() {
        create_folder(file_folder.clone());
    };
    if !std::path::Path::new(&input_file_path).exists() {
        std::fs::create_dir(&input_file_path).unwrap();
    }
}

type DescriptorSetMap = BTreeMap<u32, BTreeMap<u32, DescriptorInfoWrapper>>;
type DescriptorInfoWrapperMap = BTreeMap<u32, DescriptorInfoWrapper>;

#[derive(Default)]
pub struct VertexFactoryInfo
{
    name : &'static str,
    macros : Vec<&'static str>
}

#[derive(Default)]
pub struct ShaderManager
{
    shader_code_map: BTreeMap<String, Vec<u8>>,
    shader_reflection_descriptor_map: BTreeMap<String, DescriptorSetMap>,
    vertex_factory_infos: Vec<VertexFactoryInfo>
}

impl ShaderManager
{
    pub fn add_vertex_factory(&mut self, name: &'static str, macros: Vec<&'static str>)
    {
        let new_vertex_factory = VertexFactoryInfo{name: name, macros: macros};
        self.vertex_factory_infos.push(new_vertex_factory);
    }

    pub fn get_vf_macros(&self, vf_name: &str) -> Option<&Vec<&'static str>>
    {
        for i in 0..self.vertex_factory_infos.len()
        {
            if self.vertex_factory_infos[i].name == vf_name
            {
                return Some(&self.vertex_factory_infos[i].macros)
            }
        }
        None
    }

    fn load_shader_out(&mut self, entry: &DirEntry)
    {
        let path_name = entry.path().to_str().unwrap();
        let file_name = entry.file_name().to_str().unwrap();
        if file_name.ends_with(".binaray")
        {
            let data = fs::read(path_name).expect("open file failed.");
            self.shader_code_map.insert(file_name.replace(".binaray", ""), data);
        }
        else if file_name.ends_with(".reflect")
        {
            let data = fs::read(path_name).expect("open file failed.");
            if let Ok(content)=String::from_utf8(data)
            {
                let descriptor_set_map = serde_json::from_str::<DescriptorSetMap>(&content).unwrap();
                self.shader_reflection_descriptor_map
                    .insert(
                        file_name.replace(".binaray", ""),
                        descriptor_set_map);
            }
        }
    }

    fn cache_compiled_result_to_file(
        &self,
        entry:&DirEntry,
        compiled_code:Result<Vec<u8>,String>,
        reflection: Option<Reflection>,
        file_prefix:&str)
    {
        let real_new_path = add_prefix_to_file_name(entry.path().to_str().unwrap(), file_prefix).unwrap();
        
        let code_file_path = real_new_path.to_str().unwrap().replace(".hlsl", ".binaray").replace("assets\\shaders\\", "assets\\shaders\\out\\");
        let code_file_folder = code_file_path.clone() + "\\..";
        create_folder(code_file_folder);
        let mut code_file = std::fs::File::create(code_file_path).expect("create file failed.");
        code_file.write_all(&compiled_code.unwrap()).expect("write shader code file failed.");
        let mut descriptor_map = DescriptorSetMap::new();
        for (key, value) in &reflection.unwrap().get_descriptor_sets().unwrap()
        {
            let mut descriptor_map_in_space = DescriptorInfoWrapperMap::new();
   
            for (binding, descriptor_info) in value
            {
                let mut real_binding = *binding as usize;
                let binding_count_val = match descriptor_info.binding_count
                {
                    BindingCount::Unbounded => 0,
                    BindingCount::One => 1,
                    BindingCount::StaticSized(size) => size
                };
                let new_wrapper = DescriptorInfoWrapper
                {
                    descriptor_type : descriptor_info.ty.0,
                    binding_count : binding_count_val,
                    name : descriptor_info.name.clone()
                };
                descriptor_map_in_space.insert(real_binding as u32, new_wrapper);
            }
   
            descriptor_map.insert(*key, descriptor_map_in_space);
        }
        let descriptor_str = serde_json::to_string(&descriptor_map);
        let reflection_file_path = real_new_path.to_str().unwrap().replace(".hlsl", ".reflect").replace("assets\\shaders\\", "assets\\shaders\\out\\");
        let reflection_file_folder = reflection_file_path.clone() + "\\..";
        create_folder(reflection_file_folder);
        let mut reflection_file = std::fs::File::create(reflection_file_path).expect("create file failed.");
        reflection_file.write(&mut format!("{}", descriptor_str.unwrap()).as_bytes()).expect("write reflection failed.");
    }
   
    fn update_hlsl_shader_file(&self, entry: &DirEntry)
    {
        if entry.file_name().to_str().unwrap().ends_with("VS.hlsl") || entry.file_name().to_str().unwrap().ends_with("PS.hlsl")
        {
            let path_name = entry.path().to_str().unwrap();
            let data = fs::read_to_string(path_name).expect("Can't Open File.");
            let ps_entry_point = "PSMain";
            let vs_entry_point: &str = "VSMain";
            let mut macros = vec![];
            if entry.file_name().to_str().unwrap().ends_with("VS.hlsl")
            {
                for vf_entry in &self.vertex_factory_infos
                {
                    for open_define in &vf_entry.macros
                    {
                        macros.push("-D");
                        macros.push(open_define);
                    }
                    let compiled_code = compile_shader(path_name, &data, vs_entry_point, "vs_6_0", false, &macros);
                    let shader_reflection = get_shader_reflection(path_name, &data, vs_entry_point, "vs_6_0", &macros);
                    self.cache_compiled_result_to_file(entry, compiled_code, shader_reflection, vf_entry.name);    
                }
            }
            else if entry.file_name().to_str().unwrap().ends_with("PS.hlsl")
            {
                let compiled_code = compile_shader(path_name, &data, ps_entry_point, "ps_6_0", false, &macros);
                let shader_reflection = get_shader_reflection(path_name, &data, ps_entry_point, "ps_6_0", &macros);
                self.cache_compiled_result_to_file(entry, compiled_code, shader_reflection, "");
            }
        }
    }

    pub fn update_all_shader(&self)
    {
        WalkDir::new(SHADER_ROOT_DIR)
            .into_iter()
            .filter_map(|v| v.ok())
            .for_each(|x| self.update_hlsl_shader_file(&x));
    }
   
    pub fn load_all_shader(&mut self)
    {
        WalkDir::new(SHADER_OUT_ROOT_DIR)
            .into_iter()
            .filter_map(|v| v.ok())
            .for_each(|x| self.load_shader_out(&x))
    }
}

lazy_static! 
{
    pub static ref G_SHADER_MANAGER: Mutex<ShaderManager> = Mutex::new(ShaderManager::default());
}