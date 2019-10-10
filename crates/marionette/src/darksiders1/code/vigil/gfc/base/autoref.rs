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

pub struct AutoRef<T: AsRef<IRefObject> + ?Sized>(*mut T);

impl<T: AsRef<IRefObject>> AutoRef<T> {
    pub fn new(x: T) -> Self {
        let p = Heap::into_raw(Heap::new(x));
        unsafe {
            (*p).as_ref().add_ref();
        }
        Self(p)
    }
}

#[allow(clippy::use_self)]
impl<T: AsRef<IRefObject> + ?Sized> AutoRef<T> {
    pub unsafe fn from_ptr(p: *const T) -> Self {
        (*p).as_ref().add_ref();
        Self::from_raw(p)
    }

    pub unsafe fn from_raw(p: *const T) -> Self {
        Self(p as *mut T)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw(this: Self) -> *mut T {
        let raw = this.0;
        mem::forget(this);
        raw
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.as_ref().inner.ReferenceCount == 1 {
            Some(unsafe { &mut *this.0 })
        } else {
            None
        }
    }

    unsafe fn map<U: AsRef<IRefObject> + ?Sized>(
        this: Self,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> AutoRef<U> {
        AutoRef::from_raw(f(Self::into_raw(this)))
    }

    pub fn cast<U: AsRef<IRefObject> + ?Sized>(this: Self) -> AutoRef<U>
    where
        T: AsRef<U>,
    {
        unsafe { Self::map(this, |p| AsRef::<U>::as_ref(&*p) as *const U as *mut U) }
    }

    pub fn lower(this: Self) -> <T::Target as LoweredAutoRefTarget>::Struct
    where
        T: Lower,
        T::Target: LoweredAutoRefTarget,
    {
        let p = Self::into_raw(this);
        let p = Lower::lower(p);
        <T::Target as LoweredAutoRefTarget>::Struct::from_raw(p)
    }

    pub unsafe fn lift(autoref: <T::Target as LoweredAutoRefTarget>::Struct) -> Self
    where
        T: Lower,
        T::Target: LoweredAutoRefTarget,
    {
        let p = autoref.into_raw();
        let p = Lower::lift(p);
        Self::from_raw(p)
    }
}

impl<T: AsRef<IRefObject> + ?Sized> Drop for AutoRef<T> {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().release_ref();
        }
    }
}

impl<T: AsRef<IRefObject> + ?Sized> Deref for AutoRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T: AsRef<IRefObject> + ?Sized> Clone for AutoRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.as_ref().add_ref();
        }
        Self(self.0)
    }
}

/// The PDB structs don't use the right type for `AutoRef<T>`'s field. For
/// example, `AutoRef<WorldObject>` has a field of type `*mut gfc__IRefObject`,
/// whereas it really _should_ be typed as `*mut gfc__WorldObject`.
///
/// This trait encodes into the type system the correct field type for each
/// `AutoRef` instantiation in the PDB.
pub trait LoweredAutoRef: Sized
where
    Self::Target: LoweredAutoRefTarget<Struct = Self>,
{
    type Target: ?Sized;

    unsafe fn from_ptr(p: *mut Self::Target) -> Self;
    fn from_raw(p: *mut Self::Target) -> Self;
    fn into_raw(self) -> *mut Self::Target;
    fn ptr(&self) -> *mut Self::Target;
}

pub trait LoweredAutoRefTarget
where
    Self::Struct: LoweredAutoRef<Target = Self>,
{
    type Struct;
}

macro_rules! lowered_autoref {
    ($autoref:ty, $target:path $(,)?) => {
        impl LoweredAutoRef for $autoref {
            type Target = $target;

            unsafe fn from_ptr(p: *mut $target) -> Self {
                IRefObject::from_ptr(p.static_cast()).add_ref();
                Self::from_raw(p)
            }

            fn from_raw(p: *mut $target) -> Self {
                Self { p: p.cast() }
            }

            fn into_raw(self) -> *mut $target {
                self.ptr()
            }

            fn ptr(&self) -> *mut $target {
                self.p.cast()
            }
        }

        impl LoweredAutoRefTarget for $target {
            type Struct = $autoref;
        }
    };
}

lowered_autoref!(
    target::gfc__AutoRef_gfc__InputStream_,
    target::gfc__InputStream,
);
lowered_autoref!(target::gfc__AutoRef_gfc__MBSubMesh_, target::gfc__MBSubMesh);
lowered_autoref!(
    target::gfc__AutoRef_gfc__MeshBuilder_,
    target::gfc__MeshBuilder,
);
lowered_autoref!(target::gfc__AutoRef_gfc__Object_, target::gfc__Object);
lowered_autoref!(target::gfc__AutoRef_gfc__Object3D_, target::gfc__Object3D);
lowered_autoref!(
    target::gfc__AutoRef_gfc__OutputStream_,
    target::gfc__OutputStream,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__RegionLayer_,
    target::gfc__RegionLayer,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__Skeleton3D_,
    target::gfc__Skeleton3D,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__StaticMesh_,
    target::gfc__StaticMesh,
);
lowered_autoref!(target::gfc__AutoRef_gfc__Visual_, target::gfc__Visual);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldGroup_,
    target::gfc__WorldGroup,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldObject_,
    target::gfc__WorldObject,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldRegion_,
    target::gfc__WorldRegion,
);
