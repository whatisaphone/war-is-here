use crate::darksiders1::{code::vigil::gfc::base::object::Lower, gfc, Heap};
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

    pub fn from_raw(p: *mut T) -> Self {
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

    pub fn lift(this: <T as AutoRefLower>::TargetAutoRef) -> Self
    where
        T: AutoRefLower,
    {
        AutoRefLower::lift(this)
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
    fn lift(autoref: Self::TargetAutoRef) -> AutoRef2<Self>;
}

macro_rules! impl_autoref_lower {
    ($type:ty, $target:path) => {
        impl AutoRefLower for $type {
            type TargetAutoRef = $target;

            fn lower(this: AutoRef2<Self>) -> $target {
                let p = AutoRef2::into_raw(this);
                $target {
                    p: Lower::lower(p).static_cast(),
                }
            }

            fn lift(autoref: $target) -> AutoRef2<Self> {
                let p = Lower::lift(autoref.p.cast());
                AutoRef2::from_raw(p)
            }
        }
    };
}

impl_autoref_lower!(gfc::Object3D, target::gfc__AutoRef_gfc__Object3D_);
impl_autoref_lower!(gfc::StaticMesh, target::gfc__AutoRef_gfc__StaticMesh_);
