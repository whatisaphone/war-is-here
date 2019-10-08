use crate::utils::mem::init_with;
use darksiders1_sys::target;
use std::ffi::{CStr, CString};

struct_wrapper!(HString, target::gfc__HString);
struct_wrapper_drop!(HString, target::gfc__HString___HString);

impl HString {
    pub fn from_str(s: &str) -> Self {
        let cstring = CString::new(s).unwrap();
        Self::from_cstr(&cstring)
    }

    pub fn from_cstr(src: &CStr) -> Self {
        let inner =
            unsafe { init_with(|this| target::gfc__HString__HString_3(this, src.as_ptr(), true)) };
        Self { inner }
    }

    pub fn from_hash(hash: u64) -> Self {
        let inner = unsafe { init_with(|this| target::gfc__HString__HString_4(this, hash)) };
        Self { inner }
    }

    pub fn hash(&self) -> u64 {
        self.inner.mHash
    }

    pub fn c_str(&self) -> &CStr {
        unsafe {
            let p = target::gfc__HString__c_str(&self.inner);
            CStr::from_ptr(p)
        }
    }
}

impl Clone for HString {
    fn clone(&self) -> Self {
        Self::from_hash(self.hash())
    }
}

impl PartialEq for HString {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Eq for HString {}
