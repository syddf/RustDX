#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;

pub trait D3D12DeviceInterface
{
    fn check_feature_support<T>(
        &self,
        feature: Feature,
        feature_support_data: &mut T,
    ) -> DxResult<()>;

    fn get_copyable_footprints(
        &self,
        resource_desc: &ResourceDesc,
        first_subresouce: u32,
        num_subresources: u32,
        base_offset: ByteCount,
    ) -> (
        Vec<PlacedSubresourceFootprint>,
        Vec<u32>,
        Vec<ByteCount>,
        ByteCount,
    );
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Device {
    pub this: *mut ID3D12Device2,
}
impl_com_object_refcount_unnamed!(Device);
impl_com_object_clone_drop!(Device);

impl D3D12DeviceInterface for Device
{
    fn check_feature_support<T>(
        &self,
        feature: Feature,
        feature_support_data: &mut T,
    ) -> DxResult<()>
    {
        unsafe {
            let data = feature_support_data as *mut _ as *mut std::ffi::c_void;
            let data_size = std::mem::size_of::<T>() as u32;

            dx_try!(
                self.this,
                CheckFeatureSupport,
                feature as i32,
                data,
                data_size
            );
        }

        Ok(())
    }

    fn get_copyable_footprints(
        &self,
        resource_desc: &ResourceDesc,
        first_subresouce: u32,
        num_subresources: u32,
        base_offset: ByteCount,
    ) -> (
        Vec<PlacedSubresourceFootprint>,
        Vec<u32>,
        Vec<ByteCount>,
        ByteCount,
    )
    {
        let mut placed_subresource_footprints: Vec<PlacedSubresourceFootprint> =
        vec![
            PlacedSubresourceFootprint::default();
            num_subresources as usize
        ];

        let mut num_rows: Vec<u32> = vec![0; num_subresources as usize];

        let mut row_sizes: Vec<ByteCount> =
            vec![ByteCount(0); num_subresources as usize];

        let mut total_bytes = 0u64;

        unsafe {
            dx_call!(
                self.this,
                GetCopyableFootprints,
                &resource_desc.0 as *const D3D12_RESOURCE_DESC,
                first_subresouce,
                num_subresources,
                base_offset.0,
                placed_subresource_footprints.as_mut_ptr()
                    as *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
                num_rows.as_mut_ptr(),
                row_sizes.as_mut_ptr() as *mut u64,
                &mut total_bytes
            )
        }

        (
            placed_subresource_footprints,
            num_rows,
            row_sizes,
            ByteCount(total_bytes),
        )
    }
}