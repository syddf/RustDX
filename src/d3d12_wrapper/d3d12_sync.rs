#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct ResourceBarrier(pub(crate) D3D12_RESOURCE_BARRIER);

impl ResourceBarrier {
    pub fn barrier_type(&self) -> ResourceBarrierType {
        unsafe { std::mem::transmute(self.0.Type) }
    }

}