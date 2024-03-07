#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
use crate::shader::*;
use crate::HWND;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

use std::rc::Rc;
use widestring::WideCStr;

use lazy_static::lazy_static;
use std::sync::Mutex;

use log::{debug, error, trace, warn};

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

pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const FRAMES_IN_FLIGHT: u32 = 3;

pub fn InitD3D12Device(hwnd: *mut std::ffi::c_void)
{
    let debug_layer = Debug::new().expect("Cannot create debug layer");
    debug_layer.enable_debug_layer();
    debug_layer.enable_gpu_based_validation();
    debug_layer.enable_object_auto_name();

    let mut factory = Factory::new(CreateFactoryFlags::Debug)
        .expect("Cannot create factory");
    let adapter = choose_adapter(&mut factory);

    let device = Device::new(&adapter).expect("Cannot create device");
    let mut d3d12_device = G_D3D12_DEVICE.lock().unwrap();
    d3d12_device.this = device.this;

    let command_queue = d3d12_device
    .create_command_queue(&CommandQueueDesc::default())
    .expect("Cannot create command queue");

    let mut swapchain_desc = SwapChainDesc::default();
    swapchain_desc.0.Width = WINDOW_WIDTH;
    swapchain_desc.0.Height = WINDOW_HEIGHT;
    swapchain_desc.0.BufferCount = FRAMES_IN_FLIGHT;

    println!("swapchain_desc: {:?}", &swapchain_desc);

    let swapchain = unsafe {
        factory
            .create_swapchain(&command_queue, hwnd as HWND, &swapchain_desc)
            .expect("Cannot create swapchain")
    };
    let mut g_swap_chain = G_SWAP_CHAIN.lock().unwrap();
    g_swap_chain.this = swapchain.this;

    let mut g_direct_command_queue = G_DIRECT_COMMAND_QUEUE.lock().unwrap();
    g_direct_command_queue.this = command_queue.this;


    let mut copy_command_queue_desc = CommandQueueDesc::default();
    copy_command_queue_desc.0.Type = crate::D3D12_COMMAND_LIST_TYPE_D3D12_COMMAND_LIST_TYPE_COPY as i32;
    let copy_command_queue = d3d12_device
        .create_command_queue(&copy_command_queue_desc)
        .expect("Cannot create command queue");

    let mut g_copy_command_queue = G_COPY_COMMAND_QUEUE.lock().unwrap();
    g_copy_command_queue.this = copy_command_queue.this;
}

lazy_static! 
{
    pub static ref G_D3D12_DEVICE: Mutex<Device> = Mutex::new(Device { this:std::ptr::null_mut() });
    pub static ref G_SWAP_CHAIN: Mutex<Swapchain> = Mutex::new(Swapchain { this:std::ptr::null_mut() });
    pub static ref G_DIRECT_COMMAND_QUEUE: Mutex<CommandQueue> = Mutex::new(CommandQueue { this:std::ptr::null_mut() });
    pub static ref G_COPY_COMMAND_QUEUE: Mutex<CommandQueue> = Mutex::new(CommandQueue { this:std::ptr::null_mut() });
    
}