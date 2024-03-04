#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::shader::*;

macro_rules! impl_vertex_factory {
    ($vf_name:ident) => {{
        $vf_name::add_to_shader_manager()
    }}
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
    fn add_to_shader_manager()
    {
        let macros = vec!["VERTEX_FACTORY_USE_POSITION", "VERTEX_FACTORY_USE_COLOR"];
        let mut shader_manager = G_SHADER_MANAGER.lock().unwrap();
        shader_manager.add_vertex_factory("CommonVertexFactory_", macros);        
    }
}

pub struct VertexFactoryInitializer;
impl VertexFactoryInitializer
{
    pub fn Init()
    {
        impl_vertex_factory!(CommonVertexFactory);
    }
}