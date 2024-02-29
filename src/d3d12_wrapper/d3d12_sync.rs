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

#[derive(Debug)]
#[repr(transparent)]
pub struct Fence {
    pub this: *mut ID3D12Fence,
}

impl_com_object_set_get_name!(Fence);
impl_com_object_refcount_named!(Fence);
impl_com_object_clone_drop!(Fence);

// ToDo: make sure ID3D12Fence is thread-safe
unsafe impl Send for Fence {}

impl Fence {
    pub fn get_completed_value(&self) -> u64 {
        unsafe { dx_call!(self.this, GetCompletedValue,) }
    }

    pub fn set_event_on_completion(
        &self,
        value: u64,
        event: &Win32Event,
    ) -> DxResult<()> {
        unsafe {
            dx_try!(self.this, SetEventOnCompletion, value, event.handle);
        }
        Ok(())
    }

    pub fn signal(&self, value: u64) -> DxResult<()> {
        unsafe { dx_try!(self.this, Signal, value) }
        Ok(())
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Event {
    pub handle: HANDLE,
}

unsafe impl Send for Win32Event {}

impl Default for Win32Event {
    fn default() -> Self {
        unsafe {
            Win32Event {
                handle: CreateEventW(
                    std::ptr::null_mut(),
                    0,
                    0,
                    std::ptr::null(),
                ),
            }
        }
    }
}

impl Win32Event {
    pub fn wait(&self, milliseconds: Option<u32>) {
        unsafe {
            WaitForSingleObject(
                self.handle,
                milliseconds.unwrap_or(0xFFFFFFFF),
            );
        }
    }

    pub fn close(&self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
