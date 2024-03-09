use crate::raw_bindings::d3d12::*;
use crate::d3d12_common::*;
use crate::d3d12_enum::*;
use crate::d3d12_device::*;
use std::{slice, str};
use log::warn;

#[derive(Debug)]
#[repr(transparent)]
pub struct Debug {
    pub this: *mut ID3D12Debug5,
}
impl Debug {
    pub fn new() -> DxResult<Self> {
        let mut debug_interface: *mut ID3D12Debug5 = std::ptr::null_mut();
        unsafe {
            dx_try!(D3D12GetDebugInterface(
                &IID_ID3D12Debug5,
                cast_to_ppv(&mut debug_interface),
            ));

            Ok(Debug {
                this: debug_interface,
            })
        }
    }

    pub fn enable_debug_layer(&self) {
        unsafe { dx_call!(self.this, EnableDebugLayer,) }
    }

    pub fn enable_gpu_based_validation(&self) {
        unsafe { dx_call!(self.this, SetEnableGPUBasedValidation, 1) }
    }

    pub fn enable_object_auto_name(&self) {
        unsafe { dx_call!(self.this, SetEnableAutoName, 1) }
    }
}

#[cfg(feature = "debug_callback")]
#[derive(Debug)]
#[repr(transparent)]
pub struct InfoQueue {
    pub this: *mut ID3D12InfoQueue1,
}

#[cfg(not(feature = "debug_callback"))]
#[derive(Debug)]
#[repr(transparent)]
pub struct InfoQueue {
    pub this: *mut ID3D12InfoQueue,
}


impl InfoQueue {
    pub fn new(
        device: &Device,
        break_flags: Option<&[MessageSeverity]>,
    ) -> DxResult<Self> {
        #[cfg(feature = "debug_callback")]
        {
            let mut info_queue: *mut ID3D12InfoQueue1 = std::ptr::null_mut();
            unsafe {
                dx_try!(
                    device.this,
                    QueryInterface,
                    &IID_ID3D12InfoQueue1,
                    cast_to_ppv(&mut info_queue)
                );
                // ToDo: do we need it? It leads to refcount-related exceptions
                // under certain circumstances (see commit a738100)
                // device.release();

                if let Some(break_flags) = break_flags {
                    for flag in break_flags {
                        dx_try!(
                            info_queue,
                            SetBreakOnSeverity,
                            *flag as i32,
                            1
                        );
                    }
                }
            }

            Ok(InfoQueue { this: info_queue })
        }
        #[cfg(not(feature = "debug_callback"))]
        {
            let mut info_queue: *mut ID3D12InfoQueue = std::ptr::null_mut();
            unsafe {
                dx_try!(
                    device.this,
                    QueryInterface,
                    &IID_ID3D12InfoQueue,
                    cast_to_ppv(&mut info_queue)
                );
                // ToDo: do we need it? It leads to refcount-related exceptions
                // under certain circumstances (see commit a738100)
                // device.release();

                if let Some(break_flags) = break_flags {
                    for flag in break_flags {
                        dx_try!(
                            info_queue,
                            SetBreakOnSeverity,
                            *flag as i32,
                            1
                        );
                    }
                }
            }

            Ok(InfoQueue { this: info_queue })
        }
    }

    pub fn add_storage_filter_entries(
        &self,
        filter: &mut InfoQueueFilter,
    ) -> DxResult<()> {
        unsafe {
            dx_try!(
                self.this,
                AddStorageFilterEntries,
                filter as *mut _ as *mut D3D12_INFO_QUEUE_FILTER
            );
        }

        Ok(())
    }

    pub fn get_messages(&self) -> DxResult<Vec<String>> {
        let mut messages: Vec<String> = Vec::new();
        unsafe {
            let message_count = dx_call!(self.this, GetNumStoredMessages,);

            for message_index in 0..message_count {
                let mut message_size: SIZE_T = 0;
                dx_try!(
                    self.this,
                    GetMessageA,
                    message_index,
                    std::ptr::null_mut(),
                    &mut message_size
                );

                let allocation_layout = std::alloc::Layout::from_size_align(
                    message_size as usize,
                    8,
                )
                .expect("Wrong allocation layout");
                let message_struct =
                    std::alloc::alloc(allocation_layout) as *mut D3D12_MESSAGE;
                dx_try!(
                    self.this,
                    GetMessageA,
                    message_index,
                    message_struct,
                    &mut message_size
                );

                let message_string =
                    str::from_utf8_unchecked(slice::from_raw_parts(
                        (*message_struct).pDescription as *const u8,
                        (*message_struct).DescriptionByteLength as usize,
                    ));
                messages.push(message_string.to_string());
                std::alloc::dealloc(
                    message_struct as *mut u8,
                    allocation_layout,
                )
            }
            dx_call!(self.this, ClearStoredMessages,);
        }
        Ok(messages)
    }

    pub fn print_messages(&self) -> DxResult<()> {
        let messages = self.get_messages()?;
        for message in messages {
            warn!("{}", message);
        }

        Ok(())
    }

    #[cfg(feature = "debug_callback")]
    pub fn register_callback(
        &self,
        callback: unsafe extern "C" fn(
            i32,
            i32,
            i32,
            *const c_char,
            *mut c_void,
        ) -> (),
        filter_flags: MessageCallbackFlags,
        // ToDo: context and cookie
    ) -> DxResult<()> {
        unsafe {
            let mut cookie = 0u32;
            dx_try!(
                self.this,
                RegisterMessageCallback,
                Some(callback),
                filter_flags as i32,
                std::ptr::null_mut(),
                &mut cookie
            );
        }

        Ok(())
    }
}

#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct InfoQueueFilter(pub D3D12_INFO_QUEUE_FILTER);
