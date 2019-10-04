macro_rules! hstring {
    ($str:literal) => {{
        use std::{ffi::CStr, os::raw::c_char};
        use $crate::darksiders1::gfc::HString;

        let zstr = concat!($str, "\0");
        let cstr = CStr::from_ptr(zstr.as_ptr() as *const c_char);
        HString::from_cstr(cstr)
    }};
}

macro_rules! struct_wrapper {
    ($name:ident, $inner:ty) => {
        #[repr(transparent)]
        pub struct $name {
            inner: $inner,
        }

        #[allow(dead_code)]
        impl $name {
            pub unsafe fn from_ptr<'a>(inner: *const $inner) -> &'a Self {
                &*(inner as *const Self)
            }

            pub fn as_ptr(&self) -> *mut $inner {
                &self.inner as *const $inner as *mut $inner
            }
        }
    };
}

macro_rules! autoref_transmute {
    ($autoref:expr) => {
        target::gfc__AutoRef_gfc__IRefObject_ { p: $autoref.p }
    };
}
