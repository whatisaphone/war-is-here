macro_rules! hstring {
    ($str:literal) => {{
        use std::{ffi::CStr, os::raw::c_char};
        use $crate::darksiders1::gfc::HString;

        let zstr = concat!($str, "\0");
        #[allow(unused_unsafe)]
        let cstr = unsafe { CStr::from_ptr(zstr.as_ptr() as *const c_char) };
        HString::from_cstr(cstr)
    }};
}

macro_rules! struct_wrapper {
    ($(#[$($attrs:meta),*])* $name:ident, $inner:ty) => {
        #[repr(transparent)]
        $(#[$($attrs)*])*
        pub struct $name {
            inner: $inner,
        }

        impl $name {
            pub unsafe fn from_ptr<'a>(inner: *const $inner) -> &'a Self {
                &*(inner as *const Self)
            }

            pub unsafe fn from_ptr_mut<'a>(inner: *mut $inner) -> &'a mut Self {
                &mut *(inner as *mut Self)
            }

            pub fn as_ptr(&self) -> *mut $inner {
                &self.inner as *const $inner as *mut $inner
            }

            pub fn into_raw(self) -> $inner {
                let result = unsafe { ::std::ptr::read(&self.inner) };
                ::std::mem::forget(self);
                result
            }
        }
    };
}

macro_rules! inherits {
    ($sub:ty, $super:ty, $cast_method:ident $(,)?) => {
        use std::ops::{Deref, DerefMut};

        impl Deref for $sub {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                unsafe { <$super>::from_ptr((*self.as_ptr()).$cast_method()) }
            }
        }

        impl DerefMut for $sub {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { <$super>::from_ptr_mut((*self.as_ptr()).$cast_method()) }
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

macro_rules! autoref_cast {
    ($autoref:expr, $type:path) => {{
        use std::mem;

        let value = $autoref;
        let result = {
            $type {
                p: value.as_ptr() as *mut target::gfc__IRefObject,
            }
        };
        mem::forget(value);
        result
    }};
}
