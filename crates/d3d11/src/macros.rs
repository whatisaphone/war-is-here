macro_rules! wrap_iunknown {
    ($vis:vis $name:ident, $inner:ty) => {
        $vis struct $name($inner);

        impl $name {
            pub unsafe fn from_raw(raw: $inner) -> Self {
                Self(raw)
            }

            pub unsafe fn from_ptr(raw: $inner) -> Self {
                (*raw).AddRef();
                Self::from_raw(raw)
            }

            #[must_use]
            pub fn raw(&self) -> $inner {
                self.0
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    (*self.0).Release();
                }
            }
        }
    };
}

macro_rules! iunknown_cast {
    ($name:ident:: $meth:ident() -> $type:ty) => {
        impl $name {
            #[must_use]
            pub fn $meth(&self) -> &$type {
                unsafe { &*(self as *const _ as *const $type) }
            }
        }
    };
}
