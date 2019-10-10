use crate::darksiders1::{code::vigil::gfc::base::object::Lower, Heap};
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

    pub fn lower(this: Self) -> <T::Target as AutoRefWrap>::Struct
    where
        T: Lower,
        T::Target: AutoRefWrap,
    {
        let p = Self::into_raw(this);
        let p = Lower::lower(p);
        AutoRefWrap::wrap(p)
    }

    pub fn lift(autoref: <T::Target as AutoRefWrap>::Struct) -> Self
    where
        T: Lower,
        T::Target: AutoRefWrap,
    {
        let p = AutoRefWrap::unwrap(autoref);
        let p = Lower::lift(p);
        Self::from_raw(p)
    }
}

impl<T: AsRef<IRefObject> + ?Sized> Drop for AutoRef2<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.0).as_ref().release_ref();
        }
    }
}

/// The PDB doesn't use the right type for `AutoRef<T>`'s field. For example,
/// `AutoRef<WorldObject>` has a field of type `*mut gfc__IRefObject`, whereas
/// it really _should_ be typed as `*mut gfc__WorldObject`.
///
/// This trait encodes into the type system the correct field type for each
/// `AutoRef` instantiation.
pub trait AutoRefWrap {
    type Struct;

    fn wrap(p: *mut Self) -> Self::Struct;
    fn unwrap(this: Self::Struct) -> *mut Self;
}

macro_rules! impl_autoref_wrap {
    ($inner:ty, $autoref:path $(,)?) => {
        impl AutoRefWrap for $inner {
            type Struct = $autoref;

            fn wrap(p: *mut Self) -> $autoref {
                $autoref { p: p.static_cast() }
            }

            fn unwrap(this: $autoref) -> *mut Self {
                this.p.cast()
            }
        }
    };
}

impl_autoref_wrap!(target::gfc__Object3D, target::gfc__AutoRef_gfc__Object3D_);
impl_autoref_wrap!(
    target::gfc__StaticMesh,
    target::gfc__AutoRef_gfc__StaticMesh_,
);
