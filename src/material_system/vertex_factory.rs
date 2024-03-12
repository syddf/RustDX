#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

use crate::d3d12_enum::Format;
use crate::mesh::get_vertex_attribute_offset;
use crate::mesh::MeshChannelData;
use crate::mesh::MeshDataChannel;
use crate::shader::*;
use crate::d3d12_pso::*;
use crate::D3D12_INPUT_ELEMENT_DESC;
use crate::D3D12_INPUT_LAYOUT_DESC;

macro_rules! impl_vertex_factory {
    ($vf_name:ident) => {{
        $vf_name::add_to_shader_manager()
    }}
}

macro_rules! add_vertex_factory {
    ($name:expr, $($macro_name:expr),+) => {
        fn add_to_shader_manager()
        {
            let macros = vec![$($macro_name),+];
            let mut shader_manager = G_SHADER_MANAGER.lock().unwrap();
            shader_manager.add_vertex_factory(concat!($name, "_"), macros);
        }
    };
}

pub trait VertexFactory
{
    fn add_to_shader_manager();
}

pub struct CommonVertexFactory
{
    
}

impl CommonVertexFactory
{
    fn new() -> Self
    {
        CommonVertexFactory{}
    }
}

impl VertexFactory for CommonVertexFactory
{
    add_vertex_factory!("CommonVertexFactory", "VERTEX_FACTORY_USE_POSITION", "VERTEX_FACTORY_USE_COLOR");
}

pub fn get_vertex_input_layout<'a>(vf_name: &str) -> Vec<InputElementDesc<'a>>
{
    let mut element_vec = vec![]; 
    let shader_manager = G_SHADER_MANAGER.lock().unwrap();
    let macros_res = shader_manager.get_vf_macros(vf_name);
    if macros_res.is_none()
    {
        return element_vec;
    }
    let macros = macros_res.unwrap();
    if macros.contains(&"VERTEX_FACTORY_USE_POSITION")
    {
        let mut position_element_desc = InputElementDesc::default();
        position_element_desc.0.SemanticName = CString::new("Position").unwrap().into_raw() as *const i8;
        position_element_desc.0.Format = Format::R32G32B32Float as i32;
        position_element_desc.0.InputSlot = 0;
        position_element_desc.0.AlignedByteOffset = get_vertex_attribute_offset(MeshDataChannel::Position as u32);
        element_vec.push(position_element_desc);
    }
    if macros.contains(&"VERTEX_FACTORY_USE_COLOR")
    {
        let mut color_element_desc = InputElementDesc::default();
        color_element_desc.0.SemanticName = CString::new("Color").unwrap().into_raw() as *const i8;
        color_element_desc.0.Format = Format::R32G32B32A32Float as i32;
        color_element_desc.0.InputSlot = 0;
        color_element_desc.0.AlignedByteOffset = get_vertex_attribute_offset(MeshDataChannel::Color as u32);
        element_vec.push(color_element_desc);
    }
    element_vec        
}
pub struct VertexFactoryInitializer;
impl VertexFactoryInitializer
{
    pub fn Init()
    {
        impl_vertex_factory!(CommonVertexFactory);
    }
}