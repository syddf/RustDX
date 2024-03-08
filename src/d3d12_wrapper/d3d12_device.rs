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
use crate::d3d12_sync::*;

use widestring::WideCStr;
use winapi::shared::winerror;

pub trait D3D12DeviceInterface
{
    fn check_feature_support<T>(
        &self,
        feature: Feature,
        feature_support_data: &mut T,
    ) -> DxResult<()>;

    fn create_fence(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> DxResult<Fence>;

    fn get_descriptor_handle_increment_size(
        &self,
        heap_type: DescriptorHeapType,
    ) -> ByteCount;

    fn create_graphics_pipeline_state(
        &self,
        pso_desc: &GraphicsPipelineStateDesc,
    ) -> DxResult<PipelineState>;

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

    fn create_staging_buffer(
        &self, 
        size: ByteCount
    ) -> DxResult<Resource>;

    fn create_default_buffer(
        &self, 
        size: ByteCount
    ) -> DxResult<Resource>;

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

impl Device
{
    pub fn new(adapter: &Adapter) -> DxResult<Self> {
        let mut hw_device: *mut ID3D12Device2 = std::ptr::null_mut();
        unsafe {
            dx_try!(D3D12CreateDevice(
                cast_to_iunknown!(adapter.this),
                D3D_FEATURE_LEVEL_D3D_FEATURE_LEVEL_12_0,
                &IID_ID3D12Device2,
                cast_to_ppv(&mut hw_device),
            ));
        }

        Ok(Device { this: hw_device })
    }
}

unsafe impl Sync for Device {}
unsafe impl Send for Device {}

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

    fn create_fence(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> DxResult<Fence> {
        let mut hw_fence: *mut ID3D12Fence = std::ptr::null_mut();

        unsafe {
            dx_try!(
                self.this,
                CreateFence,
                initial_value,
                flags.bits(),
                &IID_ID3D12Fence,
                cast_to_ppv(&mut hw_fence)
            )
        }

        Ok(Fence { this: hw_fence })
    }

    fn get_descriptor_handle_increment_size(
        &self,
        heap_type: DescriptorHeapType,
    ) -> ByteCount {
        ByteCount::from(unsafe {
            dx_call!(
                self.this,
                GetDescriptorHandleIncrementSize,
                heap_type as i32
            )
        })
    }

    fn create_graphics_pipeline_state(
        &self,
        pso_desc: &GraphicsPipelineStateDesc,
    ) -> DxResult<PipelineState> {
        let mut hw_pipeline_state: *mut ID3D12PipelineState =
            std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                CreateGraphicsPipelineState,
                &pso_desc.0,
                &IID_ID3D12PipelineState,
                cast_to_ppv(&mut hw_pipeline_state)
            );
        }
        Ok(PipelineState {
            this: hw_pipeline_state,
        })
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

    fn create_staging_buffer(&self, size: ByteCount) -> DxResult<Resource>
    {
        let mut heap_props = HeapProperties::default();
        heap_props.0.Type = HeapType::Upload as i32;

        let mut resource_desc = ResourceDesc::default();
        resource_desc.0.Dimension = ResourceDimension::Buffer as i32;
        resource_desc.0.Width = size.0;
        resource_desc.0.Layout = TextureLayout::RowMajor as i32;

        self.create_committed_resource(
            &heap_props,
            HeapFlags::None,
            &resource_desc,
            ResourceStates::GenericRead,
            None,
        )
    }

    fn create_default_buffer(&self, size: ByteCount) -> DxResult<Resource>
    {
        let mut heap_props = HeapProperties::default();
        heap_props.0.Type = HeapType::Default as i32;

        let mut resource_desc = ResourceDesc::default();
        resource_desc.0.Dimension = ResourceDimension::Buffer as i32;
        resource_desc.0.Width = size.0;
        resource_desc.0.Layout = TextureLayout::RowMajor as i32;

        self.create_committed_resource(
            &heap_props,
            HeapFlags::None,
            &resource_desc,
            ResourceStates::Common,
            None,
        )
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

#[derive(Debug)]
#[repr(transparent)]
pub struct Factory {
    pub this: *mut IDXGIFactory6,
}
impl_com_object_refcount_unnamed!(Factory);
impl_com_object_clone_drop!(Factory);

impl Factory {
    pub fn new(flags: CreateFactoryFlags) -> DxResult<Self> {
        let mut factory: *mut IDXGIFactory6 = std::ptr::null_mut();
        unsafe {
            dx_try!(CreateDXGIFactory2(
                flags.bits(),
                &IID_IDXGIFactory6,
                cast_to_ppv(&mut factory),
            ));
        }
        Ok(Factory { this: factory })
    }

    pub fn enum_adapters(&self) -> DxResult<Vec<Adapter>> {
        let mut result: Vec<Adapter> = vec![];

        unsafe {
            let mut adapter_index = 0;
            loop {
                let mut temp_adapter: *mut IDXGIAdapter1 = std::ptr::null_mut();

                let ret_code = dx_call!(
                    self.this,
                    EnumAdapters1,
                    adapter_index,
                    &mut temp_adapter
                );
                if ret_code == winerror::DXGI_ERROR_NOT_FOUND {
                    break;
                } else if ret_code != winerror::S_OK {
                    return Err(DxError::new("EnumAdapters1", ret_code));
                }

                let mut real_adapter: *mut IDXGIAdapter3 = std::ptr::null_mut();
                dx_try!(
                    temp_adapter,
                    QueryInterface,
                    &IID_IDXGIAdapter3,
                    cast_to_ppv(&mut real_adapter)
                );

                // Apparently QueryInterface increases ref count?
                dx_call!(temp_adapter, Release,);

                result.push(Adapter { this: real_adapter });
                adapter_index += 1;
            }
        }
        Ok(result)
    }

    pub fn enum_adapters_by_gpu_preference(
        &self,
        preference: GpuPreference,
    ) -> DxResult<Vec<Adapter>> {
        let mut result: Vec<Adapter> = vec![];

        unsafe {
            let mut adapter_index = 0;
            loop {
                let mut adapter: *mut IDXGIAdapter3 = std::ptr::null_mut();

                let ret_code = dx_call!(
                    self.this,
                    EnumAdapterByGpuPreference,
                    adapter_index,
                    preference as i32,
                    &IID_IDXGIAdapter3,
                    cast_to_ppv(&mut adapter)
                );
                if ret_code == winerror::DXGI_ERROR_NOT_FOUND {
                    break;
                } else if ret_code != winerror::S_OK {
                    return Err(DxError::new(
                        "EnumAdapterByGpuPreference",
                        ret_code,
                    ));
                }

                result.push(Adapter { this: adapter });
                adapter_index += 1;
            }
        }
        Ok(result)
    }

    pub fn enum_warp_adapter(&self) -> DxResult<Adapter> {
        let mut hw_adapter: *mut IDXGIAdapter3 = std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                EnumWarpAdapter,
                &IID_IDXGIAdapter3,
                cast_to_ppv(&mut hw_adapter)
            );
        }

        Ok(Adapter { this: hw_adapter })
    }

    /// # Safety
    ///
    /// window_handle must be valid
    pub unsafe fn create_swapchain(
        &self,
        command_queue: &CommandQueue,
        window_handle: HWND,
        desc: &SwapChainDesc,
    ) -> DxResult<Swapchain> {
        let mut temp_hw_swapchain: *mut IDXGISwapChain1 = std::ptr::null_mut();

        dx_try!(
            self.this,
            CreateSwapChainForHwnd,
            cast_to_iunknown!(command_queue.this),
            window_handle,
            &desc.0,
            std::ptr::null(),
            std::ptr::null_mut(),
            &mut temp_hw_swapchain
        );

        let mut hw_swapchain: *mut IDXGISwapChain4 = std::ptr::null_mut();
        dx_try!(
            temp_hw_swapchain,
            QueryInterface,
            &IID_IDXGISwapChain4,
            cast_to_ppv(&mut hw_swapchain)
        );

        Ok(Swapchain { this: hw_swapchain })
    }

    pub fn make_window_association(
        &self,
        hwnd: *mut std::ffi::c_void,
        flags: MakeWindowAssociationFlags,
    ) -> DxResult<()> {
        unsafe {
            dx_try!(
                self.this,
                MakeWindowAssociation,
                hwnd as HWND,
                flags.bits()
            )
        }

        Ok(())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Adapter {
    pub this: *mut IDXGIAdapter3,
}
impl_com_object_refcount_unnamed!(Adapter);
impl_com_object_clone_drop!(Adapter);

impl Adapter {
    pub fn get_desc(&self) -> DxResult<AdapterDesc> {
        let mut hw_adapter_desc = AdapterDesc::default();
        unsafe {
            dx_try!(self.this, GetDesc1, &mut hw_adapter_desc.0);
        }
        Ok(hw_adapter_desc)
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct AdapterDesc(pub(crate) DXGI_ADAPTER_DESC1);

impl AdapterDesc {
    pub fn is_software(&self) -> bool {
        self.0.Flags & DXGI_ADAPTER_FLAG_DXGI_ADAPTER_FLAG_SOFTWARE as u32 != 0
    }

    // ToDo: clean up?
    pub fn description(&self) -> Option<String> {
        WideCStr::from_slice_with_nul(&self.0.Description)
            .map(|wide_cstr| wide_cstr.to_string_lossy())
            .ok()
    }
}

impl Default for AdapterDesc {
    fn default() -> Self {
        AdapterDesc(DXGI_ADAPTER_DESC1 {
            Description: [0; 128],
            VendorId: 0,
            DeviceId: 0,
            SubSysId: 0,
            Revision: 0,
            DedicatedVideoMemory: 0,
            DedicatedSystemMemory: 0,
            SharedSystemMemory: 0,
            AdapterLuid: LUID {
                LowPart: 0,
                HighPart: 0,
            },
            Flags: 0,
        })
    }
}

impl std::fmt::Display for AdapterDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            concat!(
                "Description: {}, VendorId: {:x}, DeviceId: {:x}, ",
                "SubSysId: {:x}, Revision: {:x}, DedicatedVideoMemory: {}, ",
                "DedicatedSystemMemory: {}, SharedSystemMemory: {}, ",
                "AdapterLuid.LowPart: {:x}, AdapterLuid.HighPart: {:x}, Flags: {:x}"
            ),
            WideCStr::from_slice_with_nul(&self.0.Description)
                .expect("Adapter desc is not valid utf-16")
                .to_string_lossy(),
            self.0.VendorId,
            self.0.DeviceId,
            self.0.SubSysId,
            self.0.Revision,
            self.0.DedicatedVideoMemory,
            self.0.DedicatedSystemMemory,
            self.0.SharedSystemMemory,
            self.0.AdapterLuid.LowPart,
            self.0.AdapterLuid.HighPart,
            self.0.Flags
        )
    }
}

impl std::fmt::Debug for AdapterDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub struct SwapChainDesc(pub DXGI_SWAP_CHAIN_DESC1);

impl Default for SwapChainDesc {
    fn default() -> Self {
        SwapChainDesc(DXGI_SWAP_CHAIN_DESC1 {
            Width: 0,
            Height: 0,
            Format: DXGI_FORMAT_DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: 0,
            SampleDesc: SampleDesc::default().0,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 2,
            Scaling: DXGI_SCALING_DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: DXGI_ALPHA_MODE_DXGI_ALPHA_MODE_UNSPECIFIED,
            Flags: DXGI_SWAP_CHAIN_FLAG_DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING
                as u32,
        })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Swapchain {
    pub this: *mut IDXGISwapChain4,
}
impl_com_object_refcount_unnamed!(Swapchain);
impl_com_object_clone_drop!(Swapchain);

impl Swapchain {
    pub fn get_buffer(&self, index: u32) -> DxResult<Resource> {
        let mut buffer: *mut ID3D12Resource = std::ptr::null_mut();
        unsafe {
            dx_try!(
                self.this,
                GetBuffer,
                index,
                &IID_ID3D12Resource,
                cast_to_ppv(&mut buffer)
            )
        }

        Ok(Resource { this: buffer })
    }

    pub fn get_frame_latency_waitable_object(&self) -> Win32Event {
        Win32Event {
            handle: unsafe {
                dx_call!(self.this, GetFrameLatencyWaitableObject,)
            },
        }
    }

    pub fn get_current_back_buffer_index(&self) -> u32 {
        unsafe { dx_call!(self.this, GetCurrentBackBufferIndex,) }
    }

    pub fn present(
        &self,
        sync_interval: u32,
        flags: PresentFlags,
    ) -> DxResult<()> {
        unsafe { dx_try!(self.this, Present, sync_interval, flags.bits()) };
        Ok(())
    }
}

unsafe impl Sync for Swapchain {}
unsafe impl Send for Swapchain {}
