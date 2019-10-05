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

macro_rules! inherits {
    ($sub:ty, $super:ty, $cast_method:ident) => {
        use std::ops::Deref;

        impl Deref for $sub {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                unsafe { <$super>::from_ptr((*self.as_ptr()).$cast_method()) }
            }
        }
    };
}

macro_rules! impl_reflection {
    ($type:ty, $class:expr) => {
        use $crate::darksiders1::{code::vigil::gfc::base::object::Reflect, gfc};

        unsafe impl Reflect for $type {
            fn class() -> &'static gfc::Class {
                unsafe { gfc::Class::from_ptr(*$class) }
            }
        }
    };
}

macro_rules! autoref_transmute {
    ($autoref:expr) => {
        target::gfc__AutoRef_gfc__IRefObject_ { p: $autoref.p }
    };
}
