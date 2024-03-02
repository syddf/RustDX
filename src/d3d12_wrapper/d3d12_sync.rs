#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct ResourceBarrier(pub D3D12_RESOURCE_BARRIER);

impl ResourceBarrier {
    pub fn barrier_type(&self) -> ResourceBarrierType {
        unsafe { std::mem::transmute(self.0.Type) }
    }
    pub fn flags(&self) -> ResourceBarrierFlags {
        unsafe { ResourceBarrierFlags::from_bits_unchecked(self.0.Flags) }
    }

    // ToDo: rename it??
    pub fn new_transition(desc: &ResourceTransitionBarrier) -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: ResourceBarrierType::Transition as i32,
            Flags: ResourceBarrierFlags::None.bits(),
            __bindgen_anon_1: D3D12_RESOURCE_BARRIER__bindgen_ty_1 {
                Transition: desc.0,
            },
        })
    }

    pub fn transition(&self) -> Option<ResourceTransitionBarrier> {
        unsafe {
            match self.barrier_type() {
                ResourceBarrierType::Transition => {
                    Some(ResourceTransitionBarrier(
                        self.0.__bindgen_anon_1.Transition,
                    ))
                }
                _ => None,
            }
        }
    }

    pub fn new_aliasing(desc: &ResourceAliasingBarrier) -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: ResourceBarrierType::Aliasing as i32,
            Flags: ResourceBarrierFlags::None.bits(),
            __bindgen_anon_1: D3D12_RESOURCE_BARRIER__bindgen_ty_1 {
                Aliasing: desc.0,
            },
        })
    }

    pub fn aliasing(&self) -> Option<ResourceAliasingBarrier> {
        unsafe {
            match self.barrier_type() {
                ResourceBarrierType::Aliasing => Some(ResourceAliasingBarrier(
                    self.0.__bindgen_anon_1.Aliasing,
                )),
                _ => None,
            }
        }
    }

    pub fn new_uav(desc: &ResourceUavBarrier) -> Self {
        Self(D3D12_RESOURCE_BARRIER {
            Type: ResourceBarrierType::Uav as i32,
            Flags: ResourceBarrierFlags::None.bits(),
            __bindgen_anon_1: D3D12_RESOURCE_BARRIER__bindgen_ty_1 {
                UAV: desc.0,
            },
        })
    }

    pub fn uav(&self) -> Option<ResourceUavBarrier> {
        unsafe {
            match self.barrier_type() {
                ResourceBarrierType::Uav => {
                    Some(ResourceUavBarrier(self.0.__bindgen_anon_1.UAV))
                }
                _ => None,
            }
        }
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

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ResourceTransitionBarrier(
    pub D3D12_RESOURCE_TRANSITION_BARRIER,
);

/// Wrapper around D3D12_RESOURCE_ALIASING_BARRIER structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ResourceAliasingBarrier(pub D3D12_RESOURCE_ALIASING_BARRIER);

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ResourceUavBarrier(pub D3D12_RESOURCE_UAV_BARRIER);
