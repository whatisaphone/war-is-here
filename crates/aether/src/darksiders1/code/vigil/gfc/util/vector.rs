use crate::darksiders1::{gfc, Lift, Lift1, Lower};
use darksiders1_sys::target;
use std::{
    convert::{TryFrom, TryInto},
    mem,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr,
    slice,
};

pub struct Vector<T> {
    data: *mut T,
    size: i32,
    capacity_and_flags: i32,
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            size: 0,
            capacity_and_flags: Self::OWNED_MASK,
        }
    }
}

impl<T> Vector<T> {
    const CAPACITY_MASK: i32 = 0x7fff_ffff;
    #[allow(overflowing_literals)]
    const OWNED_MASK: i32 = 0x8000_0000;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn add(&mut self, object: T) {
        self.reserve(self.size + 1);
        unsafe {
            let end = self.data.add(usize::try_from(self.size).unwrap());
            ptr::write(end, object);
        }
        self.size += 1;
    }

    pub fn reserve(&mut self, min_capacity: i32) {
        if self.capacity() >= min_capacity {
            return;
        }

        let new_capacity = min_capacity.max(self.capacity() * 2);

        let new_data = gfc::mem_alloc(
            u32::try_from(mem::size_of::<T>()).unwrap() * u32::try_from(new_capacity).unwrap(),
            u32::try_from(mem::align_of::<T>()).unwrap(),
        )
        .cast::<T>();

        unsafe {
            ptr::copy_nonoverlapping(self.data, new_data, usize::try_from(self.size).unwrap());
            if self.is_memory_owned() {
                gfc::mem_free(self.data);
            }
        }

        self.data = new_data;
        self.capacity_and_flags = new_capacity | Self::OWNED_MASK;
    }

    pub fn capacity(&self) -> i32 {
        self.capacity_and_flags & Self::CAPACITY_MASK
    }

    pub fn is_memory_owned(&self) -> bool {
        (self.capacity_and_flags & Self::OWNED_MASK) != 0
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.is_memory_owned() {
            // This should actually run the destructor on each element before freeing the
            // memory, but I'm not too worried about it.
            unsafe {
                gfc::mem_free(self.data);
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[allow(clippy::must_use_candidate)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vector<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data, self.size.try_into().unwrap()) }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.data, self.size.try_into().unwrap()) }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size.try_into().unwrap());
        unsafe { &*self.data.add(index) }
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size.try_into().unwrap());
        unsafe { &mut *self.data.add(index) }
    }
}

macro_rules! lowered_vector {
    ($vector:ty, $element:ty $(,)?) => {
        lowered_vector!(@base => $vector, $element);
    };
    ($vector:ty, $element:ty, lift = true $(,)?) => {
        lowered_vector!($vector, $element);
        lowered_vector!(@lift => $vector, $element);
    };
    (@base => $vector:ty, $element:ty) => {
        unsafe impl Lift1 for $vector {
            type Target = Vector<$element>;

            fn lift1(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }
    };
    (@lift => $vector:ty, $element:ty) => {
        unsafe impl Lift for $vector {
            type Target = Vector<<$element as Lift>::Target>;

            fn lift(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }

        impl Lower for Vector<<$element as Lift>::Target> {
            type Target = $vector;

            fn lower(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }
    };
}

lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__MBSubMesh__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__MBSubMesh_,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__Object_,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__RegionLayerData__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__RegionLayerData_,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__Visual__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__Visual_,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__WorldObject_,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__WorldRegionData__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__WorldRegionData_,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_,
    *mut target::gfc__RigidBody,
);
lowered_vector!(
    target::gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector3_float_gfc__FloatMath_,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector4_float_gfc__FloatMath_,
    lift = true,
);
lowered_vector!(target::gfc__Vector_int_0_gfc__CAllocator_, i32, lift = true);
lowered_vector!(
    target::gfc__Vector_unsigned_char_0_gfc__CAllocator_,
    u8,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_unsigned_long_0_gfc__CAllocator_,
    u32,
    lift = true,
);
lowered_vector!(
    target::gfc__Vector_unsigned_short_0_gfc__CAllocator_,
    u16,
    lift = true,
);

/// This is an adapter that takes a `&[T]` and exposes it as a `&Vector<T>`,
/// without copying the contents.
///
/// The resulting `Vector<T>` is read-only.
pub struct Vector__SliceAdapter<T> {
    vector: Vector<T>,
}

impl<T> Vector__SliceAdapter<T> {
    pub fn new(xs: &[T]) -> Self {
        Self {
            vector: Vector {
                data: xs.as_ptr() as *mut _,
                size: xs.len().try_into().unwrap(),
                capacity_and_flags: 0,
            },
        }
    }

    pub fn as_vector(&self) -> &Vector<T> {
        &self.vector
    }
}
