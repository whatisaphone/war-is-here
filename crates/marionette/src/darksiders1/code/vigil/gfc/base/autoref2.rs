use crate::darksiders1::{code::vigil::gfc::base::object::Lower, Heap};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::{
    mem,
    ops::Deref,
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

#[allow(clippy::use_self)]
impl<T: AsRef<IRefObject> + ?Sized> AutoRef2<T> {
    pub fn new(p: Heap<T>) -> Self {
        let p = Heap::into_raw(p);
        unsafe {
            (*p).as_ref().add_ref();
        }
        Self(p)
    }

    pub unsafe fn from_raw(p: *mut T) -> Self {
        Self(p)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw(this: Self) -> *mut T {
        let raw = this.0;
        mem::forget(this);
        raw
    }

    unsafe fn map<U: AsRef<IRefObject> + ?Sized>(
        this: Self,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> AutoRef2<U> {
        AutoRef2::from_raw(f(Self::into_raw(this)))
    }

    pub fn cast<U: AsRef<IRefObject> + ?Sized>(this: Self) -> AutoRef2<U>
    where
        T: AsRef<U>,
    {
        unsafe { Self::map(this, |p| AsRef::<U>::as_ref(&*p) as *const U as *mut U) }
    }

    pub fn lower(this: Self) -> <T::Target as AutoRefWrap>::Struct
    where
        T: Lower,
        T::Target: AutoRefWrap,
    {
        let p = Self::into_raw(this);
        let p = Lower::lower(p);
        AutoRefWrap::from_raw(p)
    }

    pub unsafe fn lift(autoref: <T::Target as AutoRefWrap>::Struct) -> Self
    where
        T: Lower,
        T::Target: AutoRefWrap,
    {
        let p = AutoRefWrap::into_raw(autoref);
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

impl<T: AsRef<IRefObject> + ?Sized> Deref for AutoRef2<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

/// The PDB doesn't use the right type for `AutoRef<T>`'s field. For example,
/// `AutoRef<WorldObject>` has a field of type `*mut gfc__IRefObject`, whereas
/// it really _should_ be typed as `*mut gfc__WorldObject`.
///
/// This trait encodes into the type system the correct field type for each
/// `AutoRef` instantiation.
pub unsafe trait AutoRefWrap {
    type Struct;

    unsafe fn wrap(p: *mut Self) -> Self::Struct;
    fn from_raw(p: *mut Self) -> Self::Struct;
    fn borrow(this: *const Self::Struct) -> *mut Self;
    fn into_raw(this: Self::Struct) -> *mut Self;
}

pub unsafe trait AutoRefUnwrap: Sized
where
    Self::Target: AutoRefWrap<Struct = Self>,
{
    type Target;

    fn into_raw(this: Self) -> *mut Self::Target {
        Self::Target::into_raw(this)
    }

    fn borrow(&self) -> *mut Self::Target {
        Self::Target::borrow(self)
    }
}

macro_rules! impl_autoref_wrap {
    ($inner:ty, $autoref:path $(,)?) => {
        unsafe impl AutoRefWrap for $inner {
            type Struct = $autoref;

            unsafe fn wrap(p: *mut Self) -> $autoref {
                IRefObject::from_ptr(p.static_cast()).add_ref();
                Self::from_raw(p)
            }

            fn from_raw(p: *mut Self) -> $autoref {
                $autoref { p: p.static_cast() }
            }

            fn borrow(this: *const $autoref) -> *mut Self {
                unsafe { (*this).p }.cast()
            }

            fn into_raw(this: $autoref) -> *mut Self {
                this.p.cast()
            }
        }

        unsafe impl AutoRefUnwrap for $autoref {
            type Target = $inner;
        }
    };
}

impl_autoref_wrap!(
    target::gfc__InputStream,
    target::gfc__AutoRef_gfc__InputStream_,
);
impl_autoref_wrap!(target::gfc__MBSubMesh, target::gfc__AutoRef_gfc__MBSubMesh_);
impl_autoref_wrap!(
    target::gfc__MeshBuilder,
    target::gfc__AutoRef_gfc__MeshBuilder_,
);
impl_autoref_wrap!(target::gfc__Object, target::gfc__AutoRef_gfc__Object_);
impl_autoref_wrap!(target::gfc__Object3D, target::gfc__AutoRef_gfc__Object3D_);
impl_autoref_wrap!(
    target::gfc__OutputStream,
    target::gfc__AutoRef_gfc__OutputStream_,
);
impl_autoref_wrap!(
    target::gfc__RegionLayer,
    target::gfc__AutoRef_gfc__RegionLayer_,
);
impl_autoref_wrap!(
    target::gfc__Skeleton3D,
    target::gfc__AutoRef_gfc__Skeleton3D_,
);
impl_autoref_wrap!(
    target::gfc__StaticMesh,
    target::gfc__AutoRef_gfc__StaticMesh_,
);
impl_autoref_wrap!(target::gfc__Visual, target::gfc__AutoRef_gfc__Visual_);
impl_autoref_wrap!(
    target::gfc__WorldGroup,
    target::gfc__AutoRef_gfc__WorldGroup_,
);
impl_autoref_wrap!(
    target::gfc__WorldObject,
    target::gfc__AutoRef_gfc__WorldObject_,
);
impl_autoref_wrap!(
    target::gfc__WorldRegion,
    target::gfc__AutoRef_gfc__WorldRegion_,
);
