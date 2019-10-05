use crate::utils::mem::init_with;
use darksiders1_sys::target;
use std::{marker::PhantomData, ops::Deref};

// These binding names are a little bit ridiculous.
macro_rules! constructor {
    () => {
        target::gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject_
    };
}
macro_rules! destructor {
    () => {
        target::gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_
    };
}

pub struct AutoRef<T> {
    inner: target::gfc__AutoRef_gfc__IRefObject_,
    phantom: PhantomData<T>,
}

impl<T> AutoRef<T> {
    pub unsafe fn from_ptr(p: *mut target::gfc__IRefObject) -> Self {
        Self {
            inner: init_with(|this| constructor!()(this, p)),
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for AutoRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner.p as *mut T) }
    }
}

impl<T> Drop for AutoRef<T> {
    fn drop(&mut self) {
        unsafe {
            destructor!()(&mut self.inner);
        }
    }
}
