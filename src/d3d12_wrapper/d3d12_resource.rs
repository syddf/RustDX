#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ marker::PhantomData };
use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_device::*;
use crate::d3d12_pso::*;
use crate::d3d12_texture::*;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct Resource {
    pub this: *mut ID3D12Resource,
}
impl_com_object_clone_drop!(Resource);
impl_com_object_refcount_named!(Resource);
impl_com_object_set_get_name!(Resource);

unsafe impl Send for Resource {}
impl Default for Resource {
    fn default() -> Self {
        Resource {
            this: std::ptr::null_mut()
        }
    }
}

impl Resource {
    pub fn get_desc(&self) -> ResourceDesc {
        unsafe {
            let mut hw_desc: D3D12_RESOURCE_DESC = std::mem::zeroed();
            dx_call!(self.this, GetDesc, &mut hw_desc);
            ResourceDesc(hw_desc)
        }
    }

    pub fn get_device(&self) -> DxResult<Device> {
        let mut hw_device: *mut ID3D12Device2 = std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                GetDevice,
                &IID_ID3D12Device2,
                cast_to_ppv(&mut hw_device)
            );
        }
        Ok(Device { this: hw_device })
    }

    pub fn get_gpu_virtual_address(&self) -> GpuVirtualAddress {
        unsafe { GpuVirtualAddress(dx_call!(self.this, GetGPUVirtualAddress,)) }
    }

    // from d3dx12.h
    pub fn get_required_intermediate_size(
        &self,
        first_subresouce: u32,
        num_subresources: u32,
    ) -> DxResult<ByteCount> {
        let resource_desc = self.get_desc();

        let device = self.get_device()?;
        let (_, _, _, total_size) = device.get_copyable_footprints(
            &resource_desc,
            first_subresouce,
            num_subresources,
            ByteCount(0),
        );
        device.release();

        Ok(total_size)
    }

    pub fn map(
        &self,
        subresource: u32,
        range: Option<&Range>,
    ) -> DxResult<*mut u8> {
        let mut data: *mut u8 = std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                Map,
                subresource,
                match range {
                    Some(rng) => &rng.0,
                    None => std::ptr::null(),
                },
                cast_to_ppv(&mut data)
            )
        };
        Ok(data)
    }

    pub fn unmap(&self, subresource: UINT, range: Option<&Range>) {
        unsafe {
            dx_call!(
                self.this,
                Unmap,
                subresource,
                match range {
                    Some(rng) => &rng.0,
                    None => std::ptr::null(),
                }
            )
        }
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub struct GpuVirtualAddress(pub D3D12_GPU_VIRTUAL_ADDRESS);

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct Range(pub D3D12_RANGE);

#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
pub struct ResourceDesc(pub D3D12_RESOURCE_DESC);

impl Default for ResourceDesc {
    fn default() -> Self {
        ResourceDesc(D3D12_RESOURCE_DESC {
            Dimension: ResourceDimension::Unknown as i32,
            Alignment: D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64,
            Width: 0,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Format::Unknown as i32,
            SampleDesc: SampleDesc::default().0,
            Layout: TextureLayout::Unknown as i32,
            Flags: ResourceFlags::None.bits(),
        })
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Rect(pub D3D12_RECT);

impl Default for Rect {
    fn default() -> Self {
        Rect(D3D12_RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        })
    }
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct Box(pub D3D12_BOX);

impl Default for Box {
    fn default() -> Self {
        Self(D3D12_BOX {
            left: 0,
            top: 0,
            front: 0,
            right: 0,
            bottom: 1,
            back: 1,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct CpuDescriptorHandle {
    pub hw_handle: D3D12_CPU_DESCRIPTOR_HANDLE,
}

impl CpuDescriptorHandle {
    #[must_use]
    pub fn advance(self, distance: u32, handle_size: ByteCount) -> Self {
        CpuDescriptorHandle {
            hw_handle: D3D12_CPU_DESCRIPTOR_HANDLE {
                ptr: self.hw_handle.ptr
                    + (distance * handle_size.0 as u32) as u64,
            },
        }
    }

    #[must_use]
    pub fn get_heap_index(
        &self,
        heap_start: CpuDescriptorHandle,
        handle_size: ByteCount,
    ) -> u32 {
        ((self.hw_handle.ptr - heap_start.hw_handle.ptr) / handle_size.0) as u32
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct GpuDescriptorHandle {
    pub hw_handle: D3D12_GPU_DESCRIPTOR_HANDLE,
}

impl GpuDescriptorHandle {
    pub fn advance(self, distance: u32, handle_size: ByteCount) -> Self {
        GpuDescriptorHandle {
            hw_handle: D3D12_GPU_DESCRIPTOR_HANDLE {
                ptr: self.hw_handle.ptr
                    + (distance * handle_size.0 as u32) as u64,
            },
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct DescriptorHeap {
    pub this: *mut ID3D12DescriptorHeap,
}

impl_com_object_set_get_name!(DescriptorHeap);
impl_com_object_refcount_unnamed!(DescriptorHeap);
impl_com_object_clone_drop!(DescriptorHeap);

unsafe impl Send for DescriptorHeap {}

impl DescriptorHeap {
    pub fn get_cpu_descriptor_handle_for_heap_start(
        &self,
    ) -> CpuDescriptorHandle {
        let mut hw_handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: 0 };
        unsafe {
            dx_call!(
                self.this,
                GetCPUDescriptorHandleForHeapStart,
                &mut hw_handle
            );
        }
        CpuDescriptorHandle { hw_handle }
    }

    pub fn get_gpu_descriptor_handle_for_heap_start(
        &self,
    ) -> GpuDescriptorHandle {
        let mut hw_handle = D3D12_GPU_DESCRIPTOR_HANDLE { ptr: 0 };
        unsafe {
            dx_call!(
                self.this,
                GetGPUDescriptorHandleForHeapStart,
                &mut hw_handle
            );
        }
        GpuDescriptorHandle { hw_handle }
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct SubresourceData<'a>(
    pub D3D12_SUBRESOURCE_DATA,
    PhantomData<&'a [()]>,
);

pub unsafe fn memcpy_subresource(
    dest: &D3D12_MEMCPY_DEST,
    src: &D3D12_SUBRESOURCE_DATA,
    row_sizes_in_bytes: ByteCount,
    num_rows: u32,
    num_slices: u32,
) {
    for z in 0..num_slices {
        let dest_slice =
            dest.pData.offset((dest.SlicePitch * z as u64) as isize);
        let src_slice = src.pData.offset((src.SlicePitch * z as i64) as isize);

        for y in 0..num_rows {
            std::ptr::copy_nonoverlapping(
                src_slice.offset((src.RowPitch * y as i64) as isize),
                dest_slice.offset((dest.RowPitch * y as u64) as isize),
                row_sizes_in_bytes.0 as usize,
            );
        }
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct PlacedSubresourceFootprint(
    pub D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
);

impl Default for PlacedSubresourceFootprint {
    fn default() -> Self {
        Self(D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
            Offset: 0,
            Footprint: SubresourceFootprint::default().0,
        })
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct ClearValue(pub D3D12_CLEAR_VALUE);

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct HeapProperties(pub D3D12_HEAP_PROPERTIES);

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct HeapDesc(pub D3D12_HEAP_DESC);

#[derive(Debug)]
#[repr(transparent)]
pub struct Heap {
    pub this: *mut ID3D12Heap,
}
impl_com_object_set_get_name!(Heap);
impl_com_object_refcount_named!(Heap);
impl_com_object_clone_drop!(Heap);

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ResourceAllocationInfo(pub D3D12_RESOURCE_ALLOCATION_INFO);

#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct ShaderResourceViewDesc(pub D3D12_SHADER_RESOURCE_VIEW_DESC);

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug)]
pub struct UnorderedAccessViewDesc(pub D3D12_UNORDERED_ACCESS_VIEW_DESC);

#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct DepthStencilViewDesc(pub D3D12_DEPTH_STENCIL_VIEW_DESC);

#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
pub struct DescriptorHeapDesc(pub D3D12_DESCRIPTOR_HEAP_DESC);

impl Default for DescriptorHeapDesc {
    fn default() -> Self {
        Self(D3D12_DESCRIPTOR_HEAP_DESC {
            Type: DescriptorHeapType::CbvSrvUav as i32,
            NumDescriptors: 0,
            Flags: DescriptorHeapFlags::None.bits(),
            NodeMask: 0,
        })
    }
}