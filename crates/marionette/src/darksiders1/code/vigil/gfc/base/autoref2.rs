use crate::darksiders1::{gfc, Heap};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::{
    mem,
    sync::atomic::{AtomicI32, Ordering},
};

struct_wrapper!(IRefObject, target::gfc__IRefObject);

impl IRefObject {
    unsafe fn add_ref(&self) {
        self.reference_count().fetch_add(1, Ordering::SeqCst);
    }

    unsafe fn release_ref(&self) {
        if self.reference_count().fetch_add(-1, Ordering::SeqCst) == 0 {
            let mut_self = &self.inner as *const _ as *mut _;
            ((*self.inner.__vfptr).__vecDelDtor)(mut_self, 1);
        }
    }

    unsafe fn reference_count(&self) -> &AtomicI32 {
        &*(&self.inner.ReferenceCount as *const i32 as *const AtomicI32)
    }
}

pub struct AutoRef2<T: AsRef<IRefObject> + ?Sized>(*mut T);

impl<T: AsRef<IRefObject> + ?Sized> AutoRef2<T> {
    pub fn new(p: Heap<T>) -> Self {
        let p = Heap::into_raw(p);
        unsafe {
            (*p).as_ref().add_ref();
        }
        Self(p)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw(this: Self) -> *mut T {
        let raw = this.0;
        mem::forget(this);
        raw
    }

    pub fn lower(this: Self) -> <T as AutoRefLower>::TargetAutoRef
    where
        T: AutoRefLower,
    {
        AutoRefLower::lower(this)
    }
}

impl<T: AsRef<IRefObject> + ?Sized> Drop for AutoRef2<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.0).as_ref().release_ref();
        }
    }
}

pub trait AutoRefLower: AsRef<IRefObject> + Lower {
    type TargetAutoRef;

    fn lower(this: AutoRef2<Self>) -> Self::TargetAutoRef;
}

impl AutoRefLower for gfc::Object3D {
    type TargetAutoRef = target::gfc__AutoRef_gfc__Object3D_;

    fn lower(this: AutoRef2<Self>) -> Self::TargetAutoRef {
        let p = AutoRef2::into_raw(this);
        target::gfc__AutoRef_gfc__Object3D_ {
            p: Lower::lower(p).static_cast(),
        }
    }
}

pub trait Lower {
    type Target;

    fn lower(this: *mut Self) -> *mut Self::Target;
}

impl Lower for gfc::Object3D {
    type Target = target::gfc__Object3D;

    fn lower(this: *mut Self) -> *mut Self::Target {
        unsafe { (*this).as_ptr() }
    }
}
