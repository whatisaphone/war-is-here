use imgui::ImStr;
use std::{ffi::CStr, str::Utf8Error};

pub trait ImStrExt {
    fn try_from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error>;
}

impl ImStrExt for ImStr {
    fn try_from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error> {
        // Use `to_str` to validate UTF-8.
        cstr.to_str()
            .map(|_| unsafe { Self::from_cstr_unchecked(cstr) })
    }
}
