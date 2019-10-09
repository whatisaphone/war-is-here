macro_rules! struct_wrapper {
    ($(#[$($attrs:meta),*])* $name:ident, $inner:ty) => {
        #[repr(transparent)]
        $(#[$($attrs)*])*
        pub struct $name {
            inner: $inner,
        }

        impl $name {
            // This is only unsafe for some types, but I'm marking it unsafe always so
            // don't have to parameterize this macro.
            pub unsafe fn from_raw(inner: $inner) -> Self {
                Self { inner }
            }

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

macro_rules! struct_wrapper_drop {
    ($name:ty, $destructor:path $(,)?) => {
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $destructor(self.as_ptr());
                }
            }
        }
    };
}

macro_rules! struct_wrapper_super {
    ($sub:ty, $super:ty, $cast_method:ident $(,)?) => {
        impl std::ops::Deref for $sub {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                use pdbindgen_runtime::StaticCast;
                unsafe { <$super>::from_ptr(self.as_ptr().static_cast()) }
            }
        }

        impl std::ops::DerefMut for $sub {
            fn deref_mut(&mut self) -> &mut Self::Target {
                use pdbindgen_runtime::StaticCast;
                unsafe { <$super>::from_ptr_mut(self.as_ptr().static_cast()) }
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

macro_rules! cstr {
    ($str:literal) => {{
        use std::{ffi::CStr, os::raw::c_char};

        let zstr = concat!($str, "\0");
        #[allow(unused_unsafe)]
        unsafe { CStr::from_ptr(zstr.as_ptr() as *const c_char) }
    }};
}

macro_rules! hstring {
    ($str:literal) => {{
        use $crate::darksiders1::gfc;

        let cstr = cstr!($str);
        gfc::HString::from_cstr(cstr)
    }};
}

macro_rules! autoref_cast {
    ($autoref:expr, $type:path) => {{
        $type {
            p: gfc::AutoRef::into_ptr($autoref),
        }
    }};
}
