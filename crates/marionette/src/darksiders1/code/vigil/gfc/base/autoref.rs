use crate::utils::mem::init_with;
use darksiders1_sys::target;
use std::{
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
};

struct_wrapper!(IRefObject, target::gfc__IRefObject);

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

    #[allow(clippy::wrong_self_convention)]
    pub fn into_ptr(this: Self) -> *mut target::gfc__IRefObject {
        let p = this.inner.p;
        mem::forget(this);
        p
    }
}

impl<T> Drop for AutoRef<T> {
    fn drop(&mut self) {
        unsafe {
            destructor!()(&mut self.inner);
        }
    }
}

impl<T> Deref for AutoRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner.p as *mut T) }
    }
}

impl<T> DerefMut for AutoRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.inner.p as *mut T) }
    }
}
