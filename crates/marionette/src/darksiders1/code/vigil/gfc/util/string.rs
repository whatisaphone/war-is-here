use crate::utils::mem::init_with;
use darksiders1_sys::target;
use std::ffi::CStr;

struct_wrapper!(String, target::gfc__String);

impl String {
    pub fn from_cstr(string: &CStr) -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__String__String_4(this, string.as_ptr());
            })
        };
        Self { inner }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            target::gfc__String___String(&mut self.inner);
        }
    }
}
