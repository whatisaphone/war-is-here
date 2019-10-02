use crate::darksiders1::gfc;
use darksiders1_sys::target;
use std::{mem, ops::Deref};

pub struct AutoRefObject {
    inner: target::gfc__AutoRef_gfc__IRefObject_,
}

impl AutoRefObject {
    pub unsafe fn from_raw(inner: target::gfc__AutoRef_gfc__Object_) -> Self {
        let inner = mem::transmute::<
            target::gfc__AutoRef_gfc__Object_,
            target::gfc__AutoRef_gfc__IRefObject_,
        >(inner);
        Self { inner }
    }
}

impl Deref for AutoRefObject {
    type Target = gfc::Object;

    fn deref(&self) -> &Self::Target {
        unsafe { gfc::Object::from_ptr(self.inner.p as *mut _) }
    }
}

impl Drop for AutoRefObject {
    fn drop(&mut self) {
        unsafe {
            target::gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_(&mut self.inner);
        }
    }
}
