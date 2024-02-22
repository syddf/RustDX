use std::{collections::BTreeMap, fs, io::{Error, Write}};

use hassle_rs::*;
use rspirv_reflect::*;
use log::{debug, error, trace, warn};
use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use serde::{Deserialize, Serialize};

const TEXTURE_REGISTER_OFFSET : usize = 0;
const SAMPLER_REGISTER_OFFSET : usize = 20;
const CBUFFER_REGISTER_OFFSET : usize = 40;
const UAV_REGISTER_OFFSET : usize = 60;
const SHADER_ROOT_DIR : &str = r"G:\Rust\RustDX\RustDX\assets\shaders\";

pub fn compile_shader(
    name : &str,
    source : &str, 
    entry_point : &str,
    shader_model : &str,
    is_spirv : bool
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
    source : &str, 
    entry_point : &str,
    shader_model : &str
) -> Option<Reflection>
{
    let compile_result = compile_shader(name, source, entry_point, shader_model, true);
    let reflection_module = 
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

#[derive(Serialize, Deserialize)]
struct DescriptorInfoWrapper
{
    descriptor_type : u32,
    binding_count: usize,
    name: String,
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
fn cache_compiled_result_to_file(
    entry: &DirEntry, 
    compiled_code: Result<Vec<u8>, String>, 
    reflection: Option<Reflection>)
{
    let code_file_path = entry.path().to_str().unwrap().replace(".hlsl", ".binaray").replace("assets\\shaders\\", "assets\\shaders\\out\\");
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
            descriptor_map_in_space.insert(*binding, new_wrapper);
        }

        descriptor_map.insert(*key, descriptor_map_in_space);
    }
    let descriptor_str = serde_json::to_string(&descriptor_map);
    let reflection_file_path = entry.path().to_str().unwrap().replace(".hlsl", ".reflect").replace("assets\\shaders\\", "assets\\shaders\\out\\");
    let reflection_file_folder = reflection_file_path.clone() + "\\..";
    create_folder(reflection_file_folder);
    let mut reflection_file = std::fs::File::create(reflection_file_path).expect("create file failed.");
    reflection_file.write(&mut format!("{}", descriptor_str.unwrap()).as_bytes()).expect("write reflection failed.");
}

fn update_hlsl_shader_file(entry: &DirEntry)
{
    if entry.file_name().to_str().unwrap().ends_with("VS.hlsl") || entry.file_name().to_str().unwrap().ends_with("PS.hlsl")
    {
        let path_name = entry.path().to_str().unwrap();
        let data = fs::read_to_string(path_name).expect("Can't Open File.");
        let psEntryPoint = "PSMain";
        let vsEntryPoint: &str = "VSMain";

        if entry.file_name().to_str().unwrap().ends_with("VS.hlsl")
        {
            let compiled_code = compile_shader(path_name, &data, vsEntryPoint, "vs_6_0", false);
            let shader_reflection = get_shader_reflection(path_name, &data, vsEntryPoint, "vs_6_0");
            cache_compiled_result_to_file(entry, compiled_code, shader_reflection);
        }
        else if entry.file_name().to_str().unwrap().ends_with("PS.hlsl")
        {
            let compiled_code = compile_shader(path_name, &data, psEntryPoint, "ps_6_0", false);
            let shader_reflection = get_shader_reflection(path_name, &data, psEntryPoint, "ps_6_0");
            cache_compiled_result_to_file(entry, compiled_code, shader_reflection);
        }
    }
}

pub fn update_all_shader()
{
    WalkDir::new(SHADER_ROOT_DIR)
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|x| update_hlsl_shader_file(&x));
}