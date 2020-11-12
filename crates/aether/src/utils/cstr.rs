use std::{ffi::CStr, os::raw::c_char};

pub trait CStrExt {
    fn to_slice_with_nul(&self) -> &[c_char];
}

impl CStrExt for CStr {
    fn to_slice_with_nul(&self) -> &[i8] {
        unsafe { &*(self.to_bytes_with_nul() as *const [u8] as *const [c_char]) }
    }
}
