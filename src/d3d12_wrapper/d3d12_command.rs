#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_device::*;
use crate::d3d12_pso::*;
use crate::d3d12_texture::*;
use crate::d3d12_resource::*;
use crate::d3d12_sync::*;
use crate::d3d12_buffer::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct CommandAllocator {
    pub this: *mut ID3D12CommandAllocator,
}
impl_com_object_set_get_name!(CommandAllocator);
impl_com_object_refcount_named!(CommandAllocator);
impl_com_object_clone_drop!(CommandAllocator);

impl CommandAllocator {
    pub fn reset(&self) -> DxResult<()> {
        unsafe { dx_try!(self.this, Reset,) };
        Ok(())
    }
}
unsafe impl Send for CommandAllocator {}

#[derive(Debug)]
#[repr(transparent)]
pub struct QueryHeap {
    pub this: *mut ID3D12QueryHeap,
}
impl_com_object_set_get_name!(QueryHeap);
impl_com_object_refcount_named!(QueryHeap);
impl_com_object_clone_drop!(QueryHeap);


#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[repr(transparent)]
pub struct CommandList {
    pub this: *mut ID3D12GraphicsCommandList6,
}
impl_com_object_set_get_name!(CommandList);
impl_com_object_refcount_named!(CommandList);
impl_com_object_clone_drop!(CommandList);

unsafe impl Send for CommandList {}

impl CommandList {
    pub fn add_resource_barrier(
        &self,
        resource: &Resource,
        from: ResourceStates,
        to: ResourceStates,
    ) {
        let mut resource_barrier = ResourceTransitionBarrier::default();
        resource_barrier.0.pResource = resource.this;
        resource_barrier.0.StateBefore = from.bits() as i32; 
        resource_barrier.0.StateAfter = to.bits() as i32; 
        let barriers = [ResourceBarrier::new_transition(
            &resource_barrier
        )];
        self.resource_barrier(&barriers);
    }

    pub fn begin_query(
        &self,
        query_heap: &QueryHeap,
        query_type: QueryType,
        index: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                BeginQuery,
                query_heap.this,
                query_type as i32,
                index
            );
        }
    }

    pub fn clear_depth_stencil_view(
        &self,
        descriptor: CpuDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: &[Rect],
    ) {
        unsafe {
            dx_call!(
                self.this,
                ClearDepthStencilView,
                descriptor.hw_handle,
                clear_flags.bits(),
                depth,
                stencil,
                rects.len() as u32,
                rects.as_ptr() as *const D3D12_RECT
            )
        }
    }

    pub fn clear_render_target_view(
        &self,
        descriptor: CpuDescriptorHandle,
        color: [f32; 4],
        rects: &[Rect],
    ) {
        unsafe {
            dx_call!(
                self.this,
                ClearRenderTargetView,
                descriptor.hw_handle,
                color.as_ptr(),
                rects.len() as u32,
                rects.as_ptr() as *const D3D12_RECT
            )
        }
    }

    pub fn close(&self) -> DxResult<()> {
        unsafe { dx_try!(self.this, Close,) };
        Ok(())
    }

    pub fn copy_buffer_region(
        &self,
        dest: &Resource,
        dest_offset: ByteCount,
        source: &Resource,
        source_offset: ByteCount,
        span: ByteCount,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CopyBufferRegion,
                dest.this,
                dest_offset.0,
                source.this,
                source_offset.0,
                span.0 as u64
            );
        }
    }

    pub fn copy_resource(&self, dest: &Resource, source: &Resource) {
        unsafe { dx_call!(self.this, CopyResource, dest.this, source.this) }
    }

    pub fn copy_texture_region(
        &self,
        dest_location: TextureCopyLocation,
        dest_x: u32,
        dest_y: u32,
        dest_z: u32,
        source_location: TextureCopyLocation,
        source_box: Option<&Box>,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CopyTextureRegion,
                &dest_location.0,
                dest_x,
                dest_y,
                dest_z,
                &source_location.0,
                match source_box {
                    Some(b) => &b.0,
                    None => std::ptr::null_mut(),
                }
            )
        }
    }

    pub fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                Dispatch,
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z
            )
        }
    }

    pub fn dispatch_mesh(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                DispatchMesh,
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z
            )
        }
    }

    pub fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                DrawIndexedInstanced,
                index_count_per_instance,
                instance_count,
                start_index_location,
                base_vertex_location,
                start_instance_location
            )
        }
    }

    pub fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                DrawInstanced,
                vertex_count_per_instance,
                instance_count,
                start_vertex_location,
                start_instance_location
            )
        }
    }

    pub fn end_query(
        &self,
        query_heap: &QueryHeap,
        query_type: QueryType,
        index: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                EndQuery,
                query_heap.this,
                query_type as i32,
                index
            );
        }
    }

    pub fn execute_bundle(&self, command_list: &CommandList) {
        unsafe {
            dx_call!(
                self.this,
                ExecuteBundle,
                // ToDo: is it 100% safe?
                command_list.this as *mut ID3D12GraphicsCommandList
            );
        }
    }

    pub fn reset(
        &self,
        command_allocator: &CommandAllocator,
        pipeline_state: Option<&PipelineState>,
    ) -> DxResult<()> {
        unsafe {
            dx_try!(
                self.this,
                Reset,
                command_allocator.this,
                match pipeline_state {
                    Some(pso) => pso.this,
                    None => std::ptr::null_mut(),
                }
            )
        };
        Ok(())
    }

    pub fn resolve_query_data(
        &self,
        query_heap: &QueryHeap,
        query_type: QueryType,
        start_index: u32,
        num_queries: u32,
        destination_buffer: &Resource,
        aligned_destination_buffer_offset: ByteCount,
    ) {
        unsafe {
            dx_call!(
                self.this,
                ResolveQueryData,
                query_heap.this,
                query_type as i32,
                start_index,
                num_queries,
                destination_buffer.this,
                aligned_destination_buffer_offset.0
            );
        }
    }

    pub fn resource_barrier(&self, barriers: &[ResourceBarrier]) {
        unsafe {
            dx_call!(
                self.this,
                ResourceBarrier,
                barriers.len() as std::os::raw::c_uint,
                barriers.as_ptr() as *const D3D12_RESOURCE_BARRIER
            );
        }
    }

    pub fn set_blend_factor(&self, blend_factor: [f32; 4]) {
        unsafe { dx_call!(self.this, OMSetBlendFactor, blend_factor.as_ptr()) }
    }

    pub fn set_compute_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRoot32BitConstant,
                root_parameter_index,
                src_data,
                dest_offset
            )
        }
    }

    // ToDo: 32_bit
    pub fn set_compute_root_32bit_constants(
        &self,
        root_parameter_index: u32,
        src_data: &[u32],
        dest_offset: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRoot32BitConstants,
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const std::ffi::c_void,
                dest_offset
            )
        }
    }

    pub fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRootConstantBufferView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_compute_root_descriptor_table(
        &self,
        parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRootDescriptorTable,
                parameter_index,
                base_descriptor.hw_handle
            )
        }
    }

    pub fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRootShaderResourceView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_compute_root_signature(&self, root_signature: &RootSignature) {
        unsafe {
            dx_call!(self.this, SetComputeRootSignature, root_signature.this)
        }
    }

    pub fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetComputeRootUnorderedAccessView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_descriptor_heaps(&self, heaps: &[DescriptorHeap]) {
        unsafe {
            dx_call!(
                self.this,
                SetDescriptorHeaps,
                heaps.len() as std::os::raw::c_uint,
                heaps.as_ptr() as *const *mut ID3D12DescriptorHeap
            )
        }
    }

    pub fn set_graphics_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRoot32BitConstant,
                root_parameter_index,
                src_data,
                dest_offset
            )
        }
    }

    pub fn set_graphics_root_32bit_constants(
        &self,
        root_parameter_index: u32,
        src_data: &[u32],
        dest_offset: u32,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRoot32BitConstants,
                root_parameter_index,
                src_data.len() as u32,
                src_data.as_ptr() as *const std::ffi::c_void,
                dest_offset
            )
        }
    }

    pub fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRootConstantBufferView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_graphics_root_descriptor_table(
        &self,
        parameter_index: u32,
        base_descriptor: GpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRootDescriptorTable,
                parameter_index,
                base_descriptor.hw_handle
            )
        }
    }

    pub fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRootShaderResourceView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_graphics_root_signature(&self, root_signature: &RootSignature) {
        unsafe {
            dx_call!(self.this, SetGraphicsRootSignature, root_signature.this)
        }
    }

    pub fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GpuVirtualAddress,
    ) {
        unsafe {
            dx_call!(
                self.this,
                SetGraphicsRootUnorderedAccessView,
                root_parameter_index,
                buffer_location.0
            )
        }
    }

    pub fn set_index_buffer(&self, view: &IndexBufferView) {
        unsafe { dx_call!(self.this, IASetIndexBuffer, &view.0) }
    }

    pub fn set_pipeline_state(&self, pipeline_state: &PipelineState) {
        unsafe { dx_call!(self.this, SetPipelineState, pipeline_state.this) }
    }

    pub fn set_primitive_topology(&self, topology: PrimitiveTopology) {
        unsafe { dx_call!(self.this, IASetPrimitiveTopology, topology as i32) }
    }

    pub fn set_render_targets(
        &self,
        descriptors: &[CpuDescriptorHandle],
        single_handle_to_descriptor_range: bool,
        depth_stencil: Option<CpuDescriptorHandle>,
    ) {
        unsafe {
            dx_call!(
                self.this,
                OMSetRenderTargets,
                descriptors.len() as std::os::raw::c_uint,
                descriptors.as_ptr() as *mut D3D12_CPU_DESCRIPTOR_HANDLE,
                match single_handle_to_descriptor_range {
                    true => 1,
                    false => 0,
                },
                match depth_stencil {
                    Some(ref depth_desc) => &depth_desc.hw_handle,
                    None => std::ptr::null_mut(),
                }
            )
        }
    }

    pub fn set_scissor_rects(&self, scissors: &[Rect]) {
        unsafe {
            dx_call!(
                self.this,
                RSSetScissorRects,
                scissors.len() as std::os::raw::c_uint,
                scissors.as_ptr() as *const D3D12_RECT
            );
        }
    }

    pub fn set_vertex_buffers(
        &self,
        start_slot: u32,
        views: &[VertexBufferView],
    ) {
        unsafe {
            dx_call!(
                self.this,
                IASetVertexBuffers,
                start_slot,
                views.len() as UINT,
                views.as_ptr() as *const D3D12_VERTEX_BUFFER_VIEW
            )
        }
    }

    pub fn set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            dx_call!(
                self.this,
                RSSetViewports,
                viewports.len() as std::os::raw::c_uint,
                viewports.as_ptr() as *const D3D12_VIEWPORT
            );
        }
    }

    // d3dx12.h helper
    #[allow(clippy::too_many_arguments)]
    pub fn update_subresources(
        &self,
        destination_resource: &Resource,
        intermediate_resource: &Resource,
        first_subresouce: u32,
        num_subresources: u32,
        required_size: ByteCount,
        layouts: &[PlacedSubresourceFootprint],
        num_rows: &[u32],
        row_sizes_in_bytes: &[ByteCount],
        source_data: &[SubresourceData],
    ) -> DxResult<ByteCount> {
        // ToDo: implement validation as in the original function

        let data = intermediate_resource.map(0, None)?;

        unsafe {
            for i in 0..num_subresources as usize {
                let dest_data = D3D12_MEMCPY_DEST {
                    pData: data.offset(layouts[i].0.Offset as isize)
                        as *mut std::ffi::c_void,
                    RowPitch: layouts[i].0.Footprint.RowPitch as u64,
                    SlicePitch: (layouts[i].0.Footprint.RowPitch as u64)
                        * num_rows[i] as u64,
                };

                memcpy_subresource(
                    &dest_data,
                    &source_data[i].0,
                    row_sizes_in_bytes[i],
                    num_rows[i],
                    layouts[i].0.Footprint.Depth,
                );
            }
        }
        intermediate_resource.unmap(0, None);

        let destination_desc = destination_resource.get_desc();
        if destination_desc.0.Dimension == ResourceDimension::Buffer as i32 {
            self.copy_buffer_region(
                destination_resource,
                ByteCount(0),
                intermediate_resource,
                ByteCount(layouts[0].0.Offset),
                ByteCount(layouts[0].0.Footprint.Width as u64),
            );
        } else {
            for i in 0..num_subresources as usize {
                let dest_location = TextureCopyLocation::new_subresource_index(
                    destination_resource,
                    i as u32 + first_subresouce,
                );
                let source_location = TextureCopyLocation::new_placed_footprint(
                    intermediate_resource,
                    layouts[i],
                );

                self.copy_texture_region(
                    dest_location,
                    0,
                    0,
                    0,
                    source_location,
                    None,
                );
            }
        }

        Ok(required_size)
    }

    // The stack-allocating version cannot be implemented without changing
    // function signature since it would require function output parameters
    pub fn update_subresources_heap_alloc(
        &self,
        destination_resource: &Resource,
        intermediate_resource: &Resource,
        intermediate_offset: ByteCount,
        first_subresouce: u32,
        num_subresources: u32,
        source_data: &[SubresourceData],
    ) -> DxResult<ByteCount> {
        let allocation_size = ByteCount::from(
            std::mem::size_of::<PlacedSubresourceFootprint>()
                + std::mem::size_of::<u32>()
                + std::mem::size_of::<u64>(),
        ) * num_subresources;

        let destination_desc = destination_resource.get_desc();
        let device = destination_resource.get_device()?;
        let (layouts, num_rows, row_sizes_in_bytes, required_size) = device
            .get_copyable_footprints(
                &destination_desc,
                first_subresouce,
                num_subresources,
                intermediate_offset,
            );
        self.update_subresources(
            destination_resource,
            intermediate_resource,
            first_subresouce,
            num_subresources,
            required_size,
            &layouts,
            &num_rows,
            &row_sizes_in_bytes,
            source_data,
        )
    }
}

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct CommandQueueDesc(pub D3D12_COMMAND_QUEUE_DESC);

#[derive(Debug)]
#[repr(transparent)]
pub struct CommandQueue {
    pub this: *mut ID3D12CommandQueue,
}
impl_com_object_refcount_unnamed!(CommandQueue);
impl_com_object_clone_drop!(CommandQueue);

unsafe impl Send for CommandQueue {}

impl CommandQueue {
    pub fn execute_command_lists(&self, command_lists: &[CommandList]) {
        unsafe {
            dx_call!(
                self.this,
                ExecuteCommandLists,
                command_lists.len() as std::os::raw::c_uint,
                command_lists.as_ptr() as *const *mut ID3D12CommandList
            );
        }
    }

    pub fn get_timestamp_frequency(&self) -> DxResult<u64> {
        let mut frequency = 0u64;
        unsafe {
            dx_try!(self.this, GetTimestampFrequency, &mut frequency);

            Ok(frequency)
        }
    }

    pub fn signal(&self, fence: &Fence, value: u64) -> DxResult<()> {
        unsafe { dx_try!(self.this, Signal, fence.this, value) };
        Ok(())
    }

    pub fn wait(&self, fence: &Fence, value: u64) -> DxResult<()> {
        unsafe { dx_try!(self.this, Wait, fence.this, value) };
        Ok(())
    }
}
