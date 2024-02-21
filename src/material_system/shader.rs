use std::io::Error;

use hassle_rs::*;
use rspirv_reflect::*;
use log::{debug, error, trace, warn};

const TEXTURE_REGISTER_OFFSET : usize = 0;
const SAMPLER_REGISTER_OFFSET : usize = 20;
const CBUFFER_REGISTER_OFFSET : usize = 40;
const UAV_REGISTER_OFFSET : usize = 60;


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

