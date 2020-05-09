macro_rules! struct_wrapper {
    ($(#[$($attrs:meta),*])* $name:ident, $inner:ty $(,)?) => {
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

        impl_lift_lower!($name, $inner);
    };
}

macro_rules! impl_lift_lower {
    ($lifted:ty, $lowered:ty) => {
        #[allow(clippy::use_self)]
        unsafe impl $crate::darksiders1::Lift for $lowered {
            type Target = $lifted;

            #[allow(clippy::useless_transmute)]
            fn lift(self) -> Self::Target {
                unsafe { ::std::mem::transmute(self) }
            }
        }

        #[allow(clippy::use_self)]
        impl $crate::darksiders1::Lower for $lifted {
            type Target = $lowered;

            #[allow(clippy::useless_transmute)]
            fn lower(self) -> Self::Target {
                unsafe { ::std::mem::transmute(self) }
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
    ($sub:ty, $super:ty) => {
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
        unsafe impl $crate::darksiders1::gfc::Reflect for $type {
            fn class() -> &'static $crate::darksiders1::gfc::Class {
                unsafe { $crate::darksiders1::gfc::Class::from_ptr(*$class) }
            }
        }
    };
}

macro_rules! cstr {
    ($str:literal) => {{
        let zstr = concat!($str, "\0");
        ::std::ffi::CStr::from_bytes_with_nul(zstr.as_bytes()).unwrap()
    }};
}

macro_rules! hstring {
    ($str:literal) => {
        $crate::darksiders1::gfc::HString::from_cstr(cstr!($str))
    };
}

// Yes, I just wrote 80 lines of code to save myself 1 line of code. Welcome to
// the life of a software engineer.
/// Automatically pops imgui tokens when they go out of scope.
macro_rules! imgui_token_guard {
    // @ui is empty. Capture the first token as @ui.
    (
        @input = {$head:ident $($tail:tt)*},
        @ui = {},
        @current = [],
        @output = [],
    ) => {
        imgui_token_guard!(
            @input = {$head $($tail)*},
            @ui = {$head},
            @current = [],
            @output = [],
        )
    };
    // If @input starts with a semicolon, complete the statement by moving @current to @output.
    (
        @input = {; $($tail:tt)*},
        @ui = {$ui:ident},
        @current = [$($current:tt)*],
        @output = [$({$($output:tt)*})*],
    ) => {
        imgui_token_guard!(
            @input = {$($tail)*},
            @ui = {$ui},
            @current = [],
            @output = [$({$($output)*})* {$($current)*}],
        )
    };
    // Any other token, keep appending to @current.
    (
        @input = {$head:tt $($tail:tt)*},
        @ui = {$ui:ident},
        @current = [$($current:tt)*],
        @output = [$({$($output:tt)*})*],
    ) => {
        imgui_token_guard!(
            @input = {$($tail)*},
            @ui = {$ui},
            @current = [$($current)* $head],
            @output = [$({$($output)*})*],
        )
    };
    // @input is empty, but @current is not. Append @current to @output.
    (
        @input = {},
        @ui = {$ui:ident},
        @current = [$($current:tt)+],
        @output = [$({$($output:tt)*})*],
    ) => {
        imgui_token_guard!(
            @input = {},
            @ui = {$ui},
            @current = [],
            @output = [$({$($output)*})* {$($current)+}],
        )
    };
    // Emit the final output.
    (
        @input = {},
        @ui = {$ui:ident},
        @current = [],
        @output = [$({$($output:tt)*})*],
    ) => {
        $(
            let _guard = scopeguard::guard($($output)*, |token| {
                token.pop($ui);
            });
        )*
    };
    // Public interface.
    ($($input:tt)*) => {
        imgui_token_guard!(
            @input = {$($input)*},
            @ui = {},
            @current = [],
            @output = [],
        )
    };
}
