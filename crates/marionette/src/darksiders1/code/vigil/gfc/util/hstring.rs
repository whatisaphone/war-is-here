use darksiders1_sys::target;
use std::{ffi::CString, mem};

#[repr(transparent)]
pub struct HString {
    inner: target::gfc__HString,
}

impl HString {
    pub fn from_str(s: &str) -> Self {
        let cstring = CString::new(s).unwrap();

        let inner = unsafe {
            let mut inner = mem::MaybeUninit::uninit();
            target::gfc__HString__HString_3(inner.as_mut_ptr(), cstring.as_ptr(), true);
            inner.assume_init()
        };
        Self { inner }
    }

    pub fn as_ref(&self) -> &target::gfc__HString {
        &self.inner
    }
}
