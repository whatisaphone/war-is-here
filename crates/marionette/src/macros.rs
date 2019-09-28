#[allow(unused_macros)]
macro_rules! hstring {
    ($str:literal) => {{
        let string = concat!($str, "\0");
        let mut hstring = mem::MaybeUninit::uninit();
        target::gfc__HString__HString_3(hstring.as_mut_ptr(), string.as_ptr() as *const _, false);
        hstring.assume_init()
    }};
}
