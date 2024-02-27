#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;

pub trait D3D12DeviceInterface
{

}

#[derive(Debug)]
#[repr(transparent)]
pub struct Device {
    pub this: *mut ID3D12Device2,
}
impl_com_object_refcount_unnamed!(Device);
impl_com_object_clone_drop!(Device);