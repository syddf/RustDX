#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_device::*;
use crate::d3d12_pso::*;
use crate::d3d12_texture::*;

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct IndexBufferView(pub D3D12_INDEX_BUFFER_VIEW);

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct VertexBufferView(pub D3D12_VERTEX_BUFFER_VIEW);
