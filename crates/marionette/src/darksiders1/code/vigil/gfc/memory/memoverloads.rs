use crate::darksiders1::gfc;
use std::{
    convert::TryFrom,
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

#[repr(transparent)]
pub struct Heap<T: ?Sized>(*mut T);

impl<T> Heap<T> {
    pub fn new(x: T) -> Self {
        let size = u32::try_from(mem::size_of::<T>()).unwrap();
        let align = u32::try_from(mem::align_of::<T>()).unwrap();
        unsafe {
            let p = gfc::mem_alloc(size, align).cast();
            ptr::write(p, x);
            Self(p)
        }
    }
}

impl<T: ?Sized> Heap<T> {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw(this: Self) -> *mut T {
        let raw = this.0;
        mem::forget(this);
        raw
    }
}

impl<T: ?Sized> Drop for Heap<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(&mut **self);
            gfc::mem_free(&mut **self);
        }
    }
}

impl<T: ?Sized> Deref for Heap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap() }
    }
}

impl<T: ?Sized> DerefMut for Heap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut().unwrap() }
    }
}
