#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use winapi::shared::winerror;

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
        $ret_code >= winapi::shared::winerror::S_OK
    };
}

macro_rules! fail {
    ($ret_code:expr) => {
        $ret_code < winapi::shared::winerror::S_OK
    };
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


// ToDo: get rid of it in favor of usize??
/// A newtype around [u64] made to distinguish between element counts and byte sizes in APIs
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ByteCount(pub u64);

// ByteCount + ByteCount = ByteCount
impl std::ops::Add<ByteCount> for ByteCount {
    type Output = Self;

    fn add(self, rhs: ByteCount) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<ByteCount> for ByteCount {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}

macro_rules! impl_from {
    ($struct_type:ty, $integer_type:ty) => {
        impl From<$integer_type> for $struct_type {
            fn from(value: $integer_type) -> Self {
                Self(value as u64)
            }
        }
    };
}


macro_rules! impl_mul_div {
    ($struct_type:tt, $integer_type:ty) => {
        impl std::ops::Mul<$integer_type> for $struct_type {
            type Output = Self;

            fn mul(self, rhs: $integer_type) -> Self {
                Self(self.0 * rhs as u64)
            }
        }

        impl std::ops::Mul<$struct_type> for $integer_type {
            type Output = $struct_type;

            fn mul(self, rhs: $struct_type) -> Self::Output {
                $struct_type(self as u64 * rhs.0)
            }
        }

        impl std::ops::Div<$integer_type> for $struct_type {
            type Output = Self;

            fn div(self, rhs: $integer_type) -> Self {
                Self(self.0 / rhs as u64)
            }
        }

        impl std::ops::Div<$struct_type> for $integer_type {
            type Output = $struct_type;

            fn div(self, rhs: $struct_type) -> Self::Output {
                $struct_type(self as u64 / rhs.0)
            }
        }
    };
}

impl_mul_div!(ByteCount, u8);
impl_mul_div!(ByteCount, i8);
impl_mul_div!(ByteCount, u16);
impl_mul_div!(ByteCount, i16);
impl_mul_div!(ByteCount, u32);
impl_mul_div!(ByteCount, i32);
impl_mul_div!(ByteCount, u64);
impl_mul_div!(ByteCount, i64);
impl_mul_div!(ByteCount, usize);
impl_mul_div!(ByteCount, isize);

// // Bytes * Elements = Bytes
// impl std::ops::Mul<Elements> for Bytes {
//     type Output = Self;

//     fn mul(self, rhs: Elements) -> Self::Output {
//         Self(self.0 * rhs.0)
//     }
// }

impl Into<usize> for ByteCount {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl_from!(ByteCount, u8);
impl_from!(ByteCount, i8);
impl_from!(ByteCount, u16);
impl_from!(ByteCount, i16);
impl_from!(ByteCount, u32);
impl_from!(ByteCount, i32);
impl_from!(ByteCount, u64);
impl_from!(ByteCount, i64);
impl_from!(ByteCount, usize);
impl_from!(ByteCount, isize);



/// Wrapper around ID3DBlob interface
#[derive(Debug)]
#[repr(transparent)]
pub struct Blob {
    pub this: *mut ID3DBlob,
}

impl Blob {
    pub fn get_buffer(&self) -> &[u8] {
        unsafe {
            let buffer_pointer: *mut u8 =
                dx_call!(self.this, GetBufferPointer,) as *mut u8;
            let buffer_size: ByteCount =
                ByteCount(dx_call!(self.this, GetBufferSize,));
            std::slice::from_raw_parts(buffer_pointer, buffer_size.0 as usize)
        }
    }
}

macro_rules! cast_to_iunknown {
    ($pointer:expr) => {{
        let mut result: *mut IUnknown = std::ptr::null_mut();
        dx_try!(
            $pointer,
            QueryInterface,
            &IID_IUnknown,
            cast_to_ppv(&mut result)
        );

        dx_call!($pointer, Release,);
        result
    }};
}