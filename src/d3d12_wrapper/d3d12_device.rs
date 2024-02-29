#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;
use crate::d3d12_command::*;
use crate::d3d12_pso::*;
use crate::d3d12_texture::*;

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
    fn create_command_allocator(
        &self,
        command_list_type: CommandListType,
    ) -> DxResult<CommandAllocator>;

    fn create_command_list(
        &self,
        command_list_type: CommandListType,
        command_allocator: &CommandAllocator,
        initial_state: Option<&PipelineState>,
    ) -> DxResult<CommandList>;

    fn create_command_queue(
        &self,
        desc: &CommandQueueDesc,
    ) -> DxResult<CommandQueue>;

    fn create_committed_resource(
        &self,
        heap_props: &HeapProperties,
        heap_flags: HeapFlags,
        resource_desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> DxResult<Resource>;

    fn create_heap(&self, heap_desc: HeapDesc) -> DxResult<Heap>;

    fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        resource_descs: &[ResourceDesc],
    ) -> ResourceAllocationInfo;

    fn create_placed_resource(
        &self,
        heap: &Heap,
        heap_offset: ByteCount,
        resource_desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> DxResult<Resource>;

    fn create_render_target_view(
        &self,
        resource: &Resource,
        dest_descriptor: CpuDescriptorHandle,
    );

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
        dest_descriptor: CpuDescriptorHandle,
    );

    fn create_shader_resource_view(
        &self,
        resource: &Resource,
        desc: Option<&ShaderResourceViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    fn create_unordered_access_view(
        &self,
        resource: &Resource,
        counter_resource: Option<&Resource>,
        desc: Option<&UnorderedAccessViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    fn create_depth_stencil_view(
        &self,
        resource: &Resource,
        desc: &DepthStencilViewDesc,
        dest_descriptor: CpuDescriptorHandle,
    );

    fn create_root_signature(
        &self,
        node_mask: UINT,
        bytecode: &ShaderBytecode,
    ) -> DxResult<RootSignature>;

    fn create_descriptor_heap(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> DxResult<DescriptorHeap>;

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

    fn create_command_allocator(
        &self,
        command_list_type: CommandListType,
    ) -> DxResult<CommandAllocator> {
        let mut hw_command_allocator: *mut ID3D12CommandAllocator =
            std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreateCommandAllocator,
                command_list_type as i32,
                &IID_ID3D12CommandAllocator,
                cast_to_ppv(&mut hw_command_allocator)
            )
        }

        Ok(CommandAllocator {
            this: hw_command_allocator,
        })
    }

    fn create_command_list(
        &self,
        command_list_type: CommandListType,
        command_allocator: &CommandAllocator,
        initial_state: Option<&PipelineState>,
    ) -> DxResult<CommandList> {
        let mut hw_command_list: *mut ID3D12GraphicsCommandList6 =
            std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreateCommandList,
                0,
                command_list_type as i32,
                command_allocator.this,
                match initial_state {
                    Some(state) => state.this,
                    None => std::ptr::null_mut(),
                },
                &IID_ID3D12CommandList,
                cast_to_ppv(&mut hw_command_list)
            )
        }

        Ok(CommandList {
            this: hw_command_list,
        })
    }

    fn create_command_queue(
        &self,
        desc: &CommandQueueDesc,
    ) -> DxResult<CommandQueue> {
        let mut hw_queue: *mut ID3D12CommandQueue = std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                CreateCommandQueue,
                &desc.0,
                &IID_ID3D12CommandQueue,
                cast_to_ppv(&mut hw_queue)
            );
        }

        Ok(CommandQueue { this: hw_queue })
    }

    fn create_committed_resource(
        &self,
        heap_props: &HeapProperties,
        heap_flags: HeapFlags,
        resource_desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> DxResult<Resource> {
        let mut hw_resource: *mut ID3D12Resource = std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreateCommittedResource,
                &heap_props.0,
                heap_flags.bits(),
                &resource_desc.0,
                initial_state.bits(),
                match optimized_clear_value {
                    Some(clear_value) => {
                        &clear_value.0
                    }
                    None => std::ptr::null(),
                },
                &IID_ID3D12Resource,
                cast_to_ppv(&mut hw_resource)
            )
        }

        Ok(Resource { this: hw_resource })
    }

    fn create_heap(&self, heap_desc: HeapDesc) -> DxResult<Heap> {
        let mut hw_heap: *mut ID3D12Heap = std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreateHeap,
                &heap_desc.0,
                &IID_ID3D12Heap,
                cast_to_ppv(&mut hw_heap)
            )
        }

        Ok(Heap { this: hw_heap })
    }

    fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        resource_descs: &[ResourceDesc],
    ) -> ResourceAllocationInfo {
        let mut hw_allocation_info = D3D12_RESOURCE_ALLOCATION_INFO::default();
        unsafe {
            dx_call!(
                self.this,
                GetResourceAllocationInfo,
                &mut hw_allocation_info,
                visible_mask,
                resource_descs.len() as u32,
                resource_descs.as_ptr() as *const D3D12_RESOURCE_DESC
            );
        }

        ResourceAllocationInfo(hw_allocation_info)
    }

    fn create_placed_resource(
        &self,
        heap: &Heap,
        heap_offset: ByteCount,
        resource_desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> DxResult<Resource> {
        let mut hw_resource: *mut ID3D12Resource = std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreatePlacedResource,
                heap.this,
                heap_offset.0,
                &resource_desc.0,
                initial_state.bits(),
                match optimized_clear_value {
                    Some(clear_value) => {
                        &clear_value.0
                    }
                    None => std::ptr::null(),
                },
                &IID_ID3D12Resource,
                cast_to_ppv(&mut hw_resource)
            )
        }

        Ok(Resource { this: hw_resource })
    }

    fn create_render_target_view(
        &self,
        resource: &Resource,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CreateRenderTargetView,
                resource.this,
                std::ptr::null(),
                dest_descriptor.hw_handle
            )
        }
    }

    fn create_shader_resource_view(
        &self,
        resource: &Resource,
        desc: Option<&ShaderResourceViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CreateShaderResourceView,
                resource.this,
                match desc {
                    Some(d) => &d.0,
                    None => std::ptr::null(),
                },
                dest_descriptor.hw_handle
            )
        }
    }

    fn create_unordered_access_view(
        &self,
        resource: &Resource,
        counter_resource: Option<&Resource>,
        desc: Option<&UnorderedAccessViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CreateUnorderedAccessView,
                resource.this,
                match counter_resource {
                    Some(res) => res.this,
                    None => std::ptr::null_mut(),
                },
                match desc {
                    Some(d) => &d.0,
                    None => std::ptr::null(),
                },
                dest_descriptor.hw_handle
            )
        }
    }

    fn create_depth_stencil_view(
        &self,
        resource: &Resource,
        desc: &DepthStencilViewDesc,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CreateDepthStencilView,
                resource.this,
                &desc.0,
                dest_descriptor.hw_handle
            )
        }
    }


    fn create_sampler(
        &self,
        desc: &SamplerDesc,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            dx_call!(
                self.this,
                CreateSampler,
                &desc.0 as *const D3D12_SAMPLER_DESC,
                dest_descriptor.hw_handle
            )
        }
    }

    fn create_root_signature(
        &self,
        node_mask: UINT,
        bytecode: &ShaderBytecode,
    ) -> DxResult<RootSignature> {
        let mut hw_root_signature: *mut ID3D12RootSignature =
            std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                CreateRootSignature,
                node_mask,
                bytecode.0.pShaderBytecode,
                bytecode.0.BytecodeLength,
                &IID_ID3D12RootSignature,
                cast_to_ppv(&mut hw_root_signature)
            );
        }
        Ok(RootSignature {
            this: hw_root_signature,
        })
    }

    fn create_descriptor_heap(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> DxResult<DescriptorHeap> {
        let mut hw_descriptor_heap: *mut ID3D12DescriptorHeap =
            std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                CreateDescriptorHeap,
                &desc.0,
                &IID_ID3D12DescriptorHeap,
                cast_to_ppv(&mut hw_descriptor_heap)
            );
        }
        Ok(DescriptorHeap {
            this: hw_descriptor_heap,
        })
    }

}