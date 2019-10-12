use crate::darksiders1::{Heap, Lift, Lower};
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
        if self.reference_count().fetch_sub(1, Ordering::SeqCst) == 0 {
            self.inner.__vecDelDtor(1);
        }
    }

    unsafe fn reference_count(&self) -> &AtomicI32 {
        &*(&self.inner.ReferenceCount as *const i32 as *const AtomicI32)
    }
}

pub struct AutoRef<T: AsRef<IRefObject>>(*mut T);

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
impl<T: AsRef<IRefObject>> AutoRef<T> {
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

    unsafe fn map<U: AsRef<IRefObject>>(
        this: Self,
        f: impl FnOnce(*mut T) -> *mut U,
    ) -> AutoRef<U> {
        AutoRef::from_raw(f(Self::into_raw(this)))
    }

    pub fn cast<U: AsRef<IRefObject>>(this: Self) -> AutoRef<U>
    where
        T: AsRef<U>,
    {
        unsafe { Self::map(this, |p| AsRef::<U>::as_ref(&*p) as *const U as *mut U) }
    }
}

impl<T: AsRef<IRefObject>> Drop for AutoRef<T> {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().release_ref();
        }
    }
}

impl<T: AsRef<IRefObject>> Deref for AutoRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T: AsRef<IRefObject>> Clone for AutoRef<T> {
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
    type Target;

    unsafe fn from_ptr(p: *mut Self::Target) -> Self;
    fn from_raw(p: *mut Self::Target) -> Self;
    fn into_raw(self) -> *mut Self::Target;
    fn ptr(&self) -> *mut Self::Target;
}

pub trait LoweredAutoRefTarget: Sized
where
    Self::Struct: LoweredAutoRef<Target = Self>,
{
    type Struct;
}

macro_rules! lowered_autoref {
    ($autoref:ty, $target:path $(,)?) => {
        lowered_autoref!(@base => $autoref, $target);
    };
    ($autoref:ty, $target:path, lift = true $(,)?) => {
        lowered_autoref!($autoref, $target);
        lowered_autoref!(@lift => $autoref, $target);
    };
    (@base => $autoref:ty, $target:path) => {
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
    (@lift => $autoref:ty, $target:path) => {
        unsafe impl Lift for $autoref {
            type Target = AutoRef<<$target as Lift>::Target>;

            fn lift(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }

        impl Lower for AutoRef<<$target as Lift>::Target> {
            type Target = $autoref;

            fn lower(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }
    }
}

lowered_autoref!(
    target::gfc__AutoRef_gfc__InputStream_,
    target::gfc__InputStream,
    lift = true,
);
lowered_autoref!(target::gfc__AutoRef_gfc__MBSubMesh_, target::gfc__MBSubMesh);
lowered_autoref!(
    target::gfc__AutoRef_gfc__MeshBuilder_,
    target::gfc__MeshBuilder,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__Object_,
    target::gfc__Object,
    lift = true,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__Object3D_,
    target::gfc__Object3D,
    lift = true,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__OutputStream_,
    target::gfc__OutputStream,
    lift = true,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__RegionLayer_,
    target::gfc__RegionLayer,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__RegionLayerData_,
    target::gfc__RegionLayerData,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__Skeleton3D_,
    target::gfc__Skeleton3D,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__StaticMesh_,
    target::gfc__StaticMesh,
    lift = true,
);
lowered_autoref!(target::gfc__AutoRef_gfc__Visual_, target::gfc__Visual);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldGroup_,
    target::gfc__WorldGroup,
    lift = true,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldObject_,
    target::gfc__WorldObject,
    lift = true,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldRegion_,
    target::gfc__WorldRegion,
);
lowered_autoref!(
    target::gfc__AutoRef_gfc__WorldRegionData_,
    target::gfc__WorldRegionData,
);
