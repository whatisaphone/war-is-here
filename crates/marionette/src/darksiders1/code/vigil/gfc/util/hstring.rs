use crate::utils::mem::init_with;
use darksiders1_sys::target;
use std::ffi::{CStr, CString};

#[repr(transparent)]
pub struct HString {
    inner: target::gfc__HString,
}

impl HString {
    pub fn from_str(s: &str) -> Self {
        let cstring = CString::new(s).unwrap();
        Self::from_cstr(&cstring)
    }

    pub fn from_cstr(s: &CStr) -> Self {
        let inner =
            unsafe { init_with(|this| target::gfc__HString__HString_3(this, s.as_ptr(), true)) };
        Self { inner }
    }

    pub unsafe fn from_ptr<'a>(inner: *const target::gfc__HString) -> &'a Self {
        &*(inner as *const _)
    }

    pub fn as_ptr(&self) -> *const target::gfc__HString {
        &self.inner
    }

    pub fn c_str(&self) -> &CStr {
        unsafe {
            let p = target::gfc__HString__c_str(&self.inner);
            CStr::from_ptr(p)
        }
    }
}

impl Drop for HString {
    fn drop(&mut self) {
        unsafe { target::gfc__HString___HString(&mut self.inner) }
    }
}
