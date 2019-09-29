macro_rules! hstring {
    ($str:literal) => {{
        use std::os::raw::c_char;
        use $crate::utils::mem::init_with;

        let zstr = concat!($str, "\0");
        let cstr = zstr.as_ptr() as *const c_char;
        init_with(|this| target::gfc__HString__HString_3(this, cstr, false))
    }};
}
