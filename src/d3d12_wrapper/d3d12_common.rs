#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;

macro_rules! dx_call {
    ($object_ptr:expr, $method_name:ident, $($args:expr),*) => {{
        let vtbl = (*$object_ptr).lpVtbl;
        let raw_func = (*vtbl).$method_name.unwrap();
        raw_func($object_ptr, $($args),*)
    }};
    ($fn_name:ident $args:tt) => {$fn_name $args;}
}

macro_rules! dx_try {
    ($object_ptr:expr, $method_name:ident, $($args:expr),*) => {{
        let vtbl = (*$object_ptr).lpVtbl;
        let raw_func = (*vtbl).$method_name.unwrap();
        let ret_code =  raw_func($object_ptr, $($args),*);
        if fail!(ret_code) {
            return Err(DxError::new(
                stringify!($method_name),
                ret_code,
            ));
        }
    }};
    ($fn_name:ident $args:tt) => {{
        let ret_code = $fn_name $args;
        if fail!(ret_code) {
            return Err(DxError::new(
                stringify!($fn_name),
                ret_code,
            ));
        }
    }}
}

const MAX_FUNC_NAME_LEN: usize = 64;
const MAX_ERROR_MSG_LEN: usize = 512;
pub struct DxError([u8; MAX_FUNC_NAME_LEN], HRESULT);

impl DxError {
    pub fn new(func_name: &str, err_code: HRESULT) -> Self {
        use std::io::Write;
        let mut func_name_owned = [0; MAX_FUNC_NAME_LEN];
        write!(&mut func_name_owned[..], "{}", func_name,)
            .expect("Ironically, DxError creation has failed");
        Self(func_name_owned, err_code)
    }

    fn write_as_str(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        unsafe {
            use winapi::um::winbase::{
                FormatMessageA, FORMAT_MESSAGE_FROM_SYSTEM,
                FORMAT_MESSAGE_IGNORE_INSERTS,
            };
            let mut error_message = [0; MAX_ERROR_MSG_LEN];
            let _char_count = FormatMessageA(
                FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null(),
                self.1 as u32,
                0,
                &mut error_message as *mut _ as *mut i8,
                MAX_ERROR_MSG_LEN as u32,
                std::ptr::null_mut(),
            );

            // FormatMessage shoves in new line symbols for some reason
            for char in &mut error_message {
                if *char == 0xA || *char == 0xD {
                    *char = 0x20;
                }
            }

            write!(
                f,
                "{} failed: [{:#010x}] {}",
                std::str::from_utf8(&self.0)
                    .expect("Cannot format error message: function name is not valid utf-8"),
                self.1,
                std::str::from_utf8(&error_message)
                    .expect("Cannot format error message: error description is not valid utf-8"),
            )
        }
    }
}

impl std::error::Error for DxError {}

impl std::fmt::Display for DxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_as_str(f)
    }
}

impl std::fmt::Debug for DxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_as_str(f)
    }
}

pub type DxResult<T> = Result<T, DxError>;

macro_rules! success {
    ($ret_code:expr) => {
        $ret_code >= winerror::S_OK
    };
}

macro_rules! fail {
    ($ret_code:expr) => {
        $ret_code < winerror::S_OK
    };
}

macro_rules! impl_com_object_clone_drop{
    ($struct_type:ty
        $(, $extra_member:ident)*
    ) => {
        impl Clone for $struct_type {
            fn clone(&self) -> Self {
                self.add_ref();
                Self {
                    this: self.this,
                    $(
                        $extra_member: self.$extra_member,
                    )*
                }
            }
        }

        impl Drop for $struct_type {
            fn drop(&mut self) {
                self.release();
            }
        }
    };
}

macro_rules! impl_com_object_refcount_unnamed {
    ($struct_type:ty
        $(, $extra_member:ident)*
    ) => {
        impl $struct_type {
            pub fn add_ref(&self) -> u64 {
                unsafe {
                    let live_ref_count: ULONG = dx_call!(self.this, AddRef,);

                    #[cfg(feature = "log_ref_counting")]
                    trace!(
                        "Increased refcount for {}, live reference count: {}",
                        stringify!($struct_type),
                        live_ref_count
                    );

                    live_ref_count as u64
                }
            }

            pub fn release(&self) -> u64 {
                unsafe {
                    let live_ref_count: ULONG = dx_call!(self.this, Release,);

                    #[cfg(feature = "log_ref_counting")]
                    trace!(
                        "Released {}, live reference count: {}",
                        stringify!($struct_type),
                        live_ref_count
                    );

                    live_ref_count as u64
                }
            }
        }
    };
}

macro_rules! impl_com_object_refcount_named {
    ($struct_type:ty
        $(, $extra_member:ident)*
        ) => {
        impl $struct_type {
            pub fn add_ref(&self) -> u64 {
                unsafe {
                    let live_ref_count: ULONG = dx_call!(self.this, AddRef,);
                    #[cfg(feature = "log_ref_counting")]
                    {
                        let name =   self.get_name();
                        trace!(
                                "Increased refcount for {} '{}', live reference count: {}",
                                stringify!($struct_type),
                                match name.as_ref() {
                                    Ok(name) => name,
                                    Err(_) => "unnamed object"
                                },
                            live_ref_count
                        )
                    }
                    live_ref_count as u64
                }
            }

            pub fn release(&self) -> u64 {
                unsafe {
                    #[cfg(feature = "log_ref_counting")]
                    let name = self.get_name();
                    let live_ref_count: ULONG = dx_call!(self.this, Release,);
                    #[cfg(feature = "log_ref_counting")]
                    {
                        trace!(
                            "Released {} '{}', live reference count: {}",
                            stringify!($struct_type),
                            match name.as_ref() {
                                Ok(name) => name,
                                Err(_) => "unnamed object",
                            },
                            live_ref_count
                        );
                    }
                    live_ref_count as u64
                }
            }
        }
    }
}

macro_rules! impl_com_object_set_get_name {
    ($struct_type:ty
        $(, $extra_member:ident)*
    ) => {
        impl $struct_type {
            pub fn set_name(&self, name: &str) -> DxResult<()> {
                let name_wstr = widestring::U16CString::from_str(name)
                    .expect("Cannot convert object name to utf-16");
                unsafe {
                    dx_try!(self.this, SetName, name_wstr.as_ptr());
                }
                Ok(())
            }

            pub fn get_name(&self) -> DxResult<String> {
                let mut buffer_size = 128u32;
                let buffer = vec![0; buffer_size as usize];
                unsafe {
                    dx_try!(
                        self.this,
                        GetPrivateData,
                        &WKPDID_D3DDebugObjectNameW,
                        &mut buffer_size,
                        buffer.as_ptr() as *mut std::ffi::c_void
                    );
                }

                widestring::U16CString::from_vec_with_nul(buffer).map_or_else(
                    |_| Err(DxError::new("U16CString::from_vec_with_nul", -1)),
                    |name_wstr| {
                        name_wstr
                            .to_string()
                            .and_then(|name_string| Ok(name_string))
                            .or_else(|_| {
                                Err(DxError::new("U16CString::to_string", -1))
                            })
                    },
                )
            }
        }
    };
}

