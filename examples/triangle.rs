// ToDo: remove these when finished
//#![allow(unused_variables)]
#![allow(dead_code)]

use log::{debug, error, trace, warn};
use memoffset::offset_of;

use RustDX::static_mesh::StaticMesh;
use RustDX::vertex_factory::get_vertex_input_layout;
use RustDX::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;
use crate::d3d12_command::*;
use crate::d3d12_pso::*;
use crate::d3d12_texture::*;
use crate::d3d12_debug::*;
use crate::d3d12_sync::*;
use crate::d3d12_device::*;
use crate::d3d12_buffer::*;
use crate::d3d12_window::*;
use crate::shader::*;
use std::ffi::{CStr, CString, NulError};

#[no_mangle]
pub static D3D12SDKVersion: u32 = 606;

#[no_mangle]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

use std::rc::Rc;
use widestring::WideCStr;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

struct ScopedDebugMessagePrinter {
    info_queue: Rc<InfoQueue>,
}

impl ScopedDebugMessagePrinter {
    fn new(info_queue: Rc<InfoQueue>) -> Self {
        ScopedDebugMessagePrinter { info_queue }
    }
}

impl Drop for ScopedDebugMessagePrinter {
    fn drop(&mut self) {
        self.info_queue
            .print_messages()
            .expect("Cannot print info queue messages");
    }
}

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    fn make_desc() -> Vec<InputElementDesc<'static>> {
        let mut position_element_desc = InputElementDesc::default();
        position_element_desc.0.SemanticName = CString::new("Position").unwrap().into_raw() as *const i8;
        position_element_desc.0.Format = Format::R32G32B32Float as i32;
        position_element_desc.0.InputSlot = 0;
        position_element_desc.0.AlignedByteOffset = 0 as u32;

        let mut color_element_desc = InputElementDesc::default();
        color_element_desc.0.SemanticName = CString::new("Color").unwrap().into_raw() as *const i8;
        color_element_desc.0.Format = Format::R32G32B32A32Float as i32;
        color_element_desc.0.InputSlot = 0;
        color_element_desc.0.AlignedByteOffset = 44 as u32;

        vec![
            position_element_desc,
            color_element_desc
        ]
    }
}

struct HelloTriangleSample<> {
    root_signature: Option<RootSignature>,
    pipeline_state: Option<PipelineState>,
    static_meshes: Vec<StaticMesh>,
    current_frame: u64,
    current_fence_value: u64,
    rtv_descriptor_size: ByteCount,
    rtv_heap: DescriptorHeap,
    command_list: CommandList,
    command_allocator: CommandAllocator,
    fence: Fence,
    info_queue: Rc<InfoQueue>,
}

impl HelloTriangleSample<> {
    pub fn new(hwnd: *mut std::ffi::c_void) -> Result<Self, HRESULT> {
        d3d12_window::InitD3D12Device(hwnd);

        let device = d3d12_window::G_D3D12_DEVICE.lock().unwrap();
        let info_queue = Rc::new(
            InfoQueue::new(&device, None)
                .expect("Cannot create debug info queue"),
        );
    
        let _debug_printer =
            ScopedDebugMessagePrinter::new(Rc::clone(&info_queue));

        let fence = device
            .create_fence(0, FenceFlags::None)
            .expect("Cannot create fence");

        let rtv_descriptor_size = device
            .get_descriptor_handle_increment_size(DescriptorHeapType::Rtv);

        let command_allocator = device
            .create_command_allocator(CommandListType::Direct)
            .expect("Cannot create command allocator");
    
        let command_list = device
            .create_command_list(
                CommandListType::Direct,
                &command_allocator,
                None,
            )
            .expect("Cannot create command list");
        command_list.close().expect("Cannot close command list");
    
    

        let mut rtv_heap_desc = DescriptorHeapDesc::default();
        rtv_heap_desc.0.Type = DescriptorHeapType::Rtv as i32;
        rtv_heap_desc.0.NumDescriptors = FRAMES_IN_FLIGHT;
        let rtv_heap = device
            .create_descriptor_heap(
                &rtv_heap_desc
            )
            .expect("Cannot create RTV heap");

        let mut triangle = static_mesh::StaticMesh::new("triangle");
        triangle.add_channel_data(mesh::MeshDataChannel::Position, vec![-1., -1., 0.]);
        triangle.add_channel_data(mesh::MeshDataChannel::Position, vec![0., 1., 0.]);
        triangle.add_channel_data(mesh::MeshDataChannel::Position, vec![1., -1., 0.]);
        triangle.add_channel_data(mesh::MeshDataChannel::Color, vec![1., 0., 0., 1.]);
        triangle.add_channel_data(mesh::MeshDataChannel::Color, vec![0., 1., 0., 1.]);
        triangle.add_channel_data(mesh::MeshDataChannel::Color, vec![1., 0., 1., 1.]);
        triangle.set_index_buffer(vec![0, 1, 2]);
        triangle.generate_gpu_resource(&device);
    
        let meshes = vec![triangle];
        let mut renderer = HelloTriangleSample {
            root_signature: None,
            pipeline_state: None,
            static_meshes: meshes,
            current_frame: 0,
            current_fence_value: 0,
            info_queue: info_queue,
            fence: fence,
            rtv_descriptor_size,
            command_allocator: command_allocator,
            command_list: command_list,
            rtv_heap: rtv_heap,
        };

        renderer.create_render_target_views(&device);

        let raw_vertex_shader_bytecode = HelloTriangleSample::compile_shader(
            "VertexShader",
            r#"
struct VertexIn
{
    float3 pos: Position;
    float4 color: Color;
};

struct VertexOut
{
    float4 pos: SV_POSITION;
    float4 color: Color;
};

[RootSignature("RootFlags(ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT)")]
VertexOut VS(VertexIn input)
{
    VertexOut result = (VertexOut)0;
    result.pos = float4(input.pos, 1.);
    result.color = input.color;

    return result;
}
"#,
            "VS",
            "vs_6_0",
        )
        .expect("Cannot compile vertex shader");
        let vertex_bytecode = ShaderBytecode::new(&raw_vertex_shader_bytecode);

        let raw_pixel_shader_bytecode = HelloTriangleSample::compile_shader(
            "PixelShader",
            r#"
struct VertexOut
{
    float4 pos: SV_Position;
    float4 color: Color;
};

[RootSignature("RootFlags(ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT)")]
float4 PS(VertexOut input) : SV_Target
{
    return input.color;
}
"#,
            "PS",
            "ps_6_0",
        )
        .expect("Cannot compile pixel shader");
        let pixel_bytecode = ShaderBytecode::new(&raw_pixel_shader_bytecode);

        let root_signature = device
            .create_root_signature(0, &pixel_bytecode)
            .expect("Cannot create root signature");

        debug!("Created root signature");

        renderer.root_signature = Some(root_signature);

        let vertex_desc = get_vertex_input_layout("CommonVertexFactory_");
        let mut input_layout = InputLayoutDesc::default();
        input_layout.0.pInputElementDescs = vertex_desc.as_ptr() as *const D3D12_INPUT_ELEMENT_DESC;
        input_layout.0.NumElements = vertex_desc.len() as u32;
        debug!("Created input layout");

        let mut pso_desc = GraphicsPipelineStateDesc::default();
        pso_desc.0.VS = vertex_bytecode.0;
        pso_desc.0.PS = pixel_bytecode.0;
        pso_desc.0.BlendState = BlendDesc::default().0;
        pso_desc.0.RasterizerState = RasterizerDesc::default().0;
        let mut depth_stencil_state = DepthStencilDesc::default();
        depth_stencil_state.0.DepthEnable = false as i32;
        pso_desc.0.DepthStencilState = depth_stencil_state.0;
        pso_desc.0.InputLayout = input_layout.0;
        pso_desc.0.PrimitiveTopologyType = PrimitiveTopologyType::Triangle as i32;
        pso_desc.0.RTVFormats[0] = Format::R8G8B8A8Unorm as i32;
        pso_desc.0.DSVFormat = Format::D24UnormS8Uint as i32;

        let pso = device
            .create_graphics_pipeline_state(&pso_desc)
            .expect("Cannot create PSO");

        debug!("Created PSO");

        renderer.pipeline_state = Some(pso);

        Ok(renderer)
    }

    pub fn draw(&mut self) {
        let _debug_printer =
            ScopedDebugMessagePrinter::new(Rc::clone(&self.info_queue));

        self.command_allocator
            .reset()
            .expect("Cannot reset command allocator");
        self.command_list
            .reset(&self.command_allocator, None)
            .expect("Cannot reset command list");

        let swapchain = G_SWAP_CHAIN.lock().unwrap();
        let current_buffer_index =
            swapchain.get_current_back_buffer_index();
        let current_buffer = swapchain
            .get_buffer(u32::from(current_buffer_index))
            .expect("Cannot get current swapchain buffer");

        let rtv_handle = self
            .rtv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .advance(current_buffer_index, self.rtv_descriptor_size);

        HelloTriangleSample::add_transition(
            &self.command_list,
            &current_buffer,
            ResourceStates::Common,
            ResourceStates::RenderTarget,
        );

        let mut viewport_desc = Viewport::default();
        viewport_desc.0.Width = WINDOW_WIDTH as f32;
        viewport_desc.0.Height = WINDOW_HEIGHT as f32;

        self.command_list.set_pipeline_state(
            self.pipeline_state
                .as_ref()
                .expect("No pipeline state found"),
        );
        self.command_list.set_graphics_root_signature(
            self.root_signature
                .as_ref()
                .expect("No root signature to set"),
        );
        self.command_list.set_viewports(&[viewport_desc]);

        let mut scissor_desc = Rect::default();
        scissor_desc.0.right = WINDOW_WIDTH as i32;
        scissor_desc.0.bottom = WINDOW_HEIGHT as i32;

        self.command_list.set_scissor_rects(&[scissor_desc]);
        self.command_list.clear_render_target_view(
            rtv_handle,
            [0., 0.1, 0.8, 1.],
            &[],
        );
        self.command_list
            .set_render_targets(&mut [rtv_handle], false, None);

        let (_,_,vertex_view, index_view) = self.static_meshes[0].get_gpu_resource();
        self.command_list
            .set_vertex_buffers(0, &[vertex_view.clone()]);

        self.command_list
            .set_index_buffer(&index_view.clone());
        self.command_list
            .set_primitive_topology(PrimitiveTopology::TriangleList);
        self.command_list.draw_indexed_instanced(3, 1, 0, 0, 0);

        HelloTriangleSample::add_transition(
            &self.command_list,
            &current_buffer,
            ResourceStates::RenderTarget,
            ResourceStates::Common,
        );

        self.command_list
            .close()
            .expect("Cannot close command list");

        let command_queue = G_DIRECT_COMMAND_QUEUE.lock().unwrap();
        command_queue
            .execute_command_lists(std::slice::from_ref(&self.command_list));

        swapchain
            .present(0, PresentFlags::None)
            .expect("Cannot present frame");

        self.flush_command_queue(&command_queue);

        self.current_frame += 1;
    }
}

// Private methods

impl HelloTriangleSample<> {
    fn choose_adapter(factory: &mut Factory) -> Adapter {
        let mut adapters =
            factory.enum_adapters().expect("Cannot enumerate adapters");
        debug!("Found adapters:");
        for adapter in &adapters {
            let desc_struct =
                adapter.get_desc().expect("Cannot get adapter desc");
            // ToDo: move this inside DxgiAdapterDesc?

            debug!(
                "\t{}",
                &desc_struct.description().expect("cannot get adapter desc")
            );
        }

        adapters.remove(0)
    }

    fn create_render_target_views(&self, device:&Device) {
        let swapchain = G_SWAP_CHAIN.lock().unwrap();
        
        for buffer_index in 0..(FRAMES_IN_FLIGHT as u32) {
            let rtv_handle = self
                .rtv_heap
                .get_cpu_descriptor_handle_for_heap_start()
                .advance(buffer_index, self.rtv_descriptor_size);
            let buffer = swapchain
                .get_buffer(buffer_index)
                .expect("Cannot obtain swapchain buffer");
            device.create_render_target_view(&buffer, rtv_handle);
        }
    }

    fn create_buffer(
        &self,
        device: &Device,
        size: ByteCount,
        heap_type: HeapType,
        initial_state: ResourceStates,
    ) -> DxResult<Resource> {
        let mut heap_props = HeapProperties::default();
        heap_props.0.Type = heap_type as i32;

        let mut resource_desc = ResourceDesc::default();
        resource_desc.0.Dimension = ResourceDimension::Buffer as i32;
        resource_desc.0.Width = size.0;
        resource_desc.0.Layout = TextureLayout::RowMajor as i32;

        device.create_committed_resource(
            &heap_props,
            HeapFlags::None,
            &resource_desc,
            initial_state,
            None,
        )
    }

    fn add_transition(
        command_list: &CommandList,
        resource: &Resource,
        from: ResourceStates,
        to: ResourceStates,
    ) {
        let mut resource_barrier = ResourceTransitionBarrier::default();
        resource_barrier.0.pResource = resource.this;
        resource_barrier.0.StateBefore = from.bits() as i32; 
        resource_barrier.0.StateAfter = to.bits() as i32; 

        command_list.resource_barrier(&[ResourceBarrier::new_transition(
            &resource_barrier
        )
            ]);
    }

    fn flush_command_queue(&mut self, command_queue: &CommandQueue) {
        self.current_fence_value += 1;
        command_queue
            .signal(&self.fence, self.current_fence_value)
            .expect("Cannot signal fence from command queue");
        if self.fence.get_completed_value() < self.current_fence_value {
            let event_handle = Win32Event::default();
            self.fence
                .set_event_on_completion(
                    self.current_fence_value,
                    &event_handle,
                )
                .expect("Cannot set fence completion event");
            event_handle.wait(None);
            event_handle.close();
        }
    }

    fn compile_shader(
        name: &str,
        source: &str,
        entry_point: &str,
        shader_model: &str,
    ) -> Result<Vec<u8>, String> {
        let result = hassle_rs::utils::compile_hlsl(
            name,
            source,
            entry_point,
            shader_model,
            &["/Zi", "/Od"],
            &[],
        );
        match result {
            Ok(bytecode) => {
                debug!("Shader {} compiled successfully", name);
                Ok(bytecode)
            }
            Err(error) => {
                error!("Cannot compile shader: {}", &error);
                Err(error)
            }
        }
    }
}

impl Drop for HelloTriangleSample<> {
    fn drop(&mut self) {
        self.info_queue
            .print_messages()
            .expect("Cannot print info queue messages");
        debug!("Renderer destroyed");
    }
}

fn main() {
    vertex_factory::VertexFactoryInitializer::Init();
    {
        let shader_manager = G_SHADER_MANAGER.lock().unwrap();
        shader_manager.update_all_shader();
    }
        
    let command_args = clap::App::new("Hobbiton")
        .arg(
            clap::Arg::with_name("frame_count")
                .short("f")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Run <frame_count> frames and exit"),
        )
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Verbosity level"),
        )
        .get_matches();

    let frame_count = command_args
        .value_of("frame_count")
        .unwrap_or(&std::u64::MAX.to_string())
        .parse::<u64>()
        .expect("Cannot parse frame count");
    let log_level: log::Level;
    match command_args.occurrences_of("v") {
        0 => log_level = log::Level::Info,
        1 => log_level = log::Level::Debug,
        2 | _ => log_level = log::Level::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Cannot create window");
    window.set_inner_size(winit::dpi::LogicalSize::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    ));
    let mut sample = HelloTriangleSample::new(window.hwnd())
        .expect("Cannot create renderer");

    let mut current_frame: u64 = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.
                if current_frame > frame_count {
                    *control_flow = ControlFlow::Exit;
                }
                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.

                sample.draw();
                current_frame += 1;
            }
            _ => (),
        }
    });
}
