macro_rules! hstring {
    ($str:literal) => {{
        use std::{ffi::CStr, os::raw::c_char};
        use $crate::darksiders1::gfc::HString;

        let zstr = concat!($str, "\0");
        let cstr = CStr::from_ptr(zstr.as_ptr() as *const c_char);
        HString::from_cstr(cstr)
    }};
}
