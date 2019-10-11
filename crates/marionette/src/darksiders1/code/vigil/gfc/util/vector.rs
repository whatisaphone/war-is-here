use crate::darksiders1::{gfc, Lift, Lift1, Lift2, Lower};
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

    pub unsafe fn from_ptr<'a, V>(p: *const V) -> &'a Self
    where
        V: LoweredVector<Element = T>,
        T: LoweredVectorElement<Vec = V>,
    {
        &*(p as *const Self)
    }

    pub unsafe fn from_ptr_mut<'a, V>(p: *mut V) -> &'a mut Self
    where
        V: LoweredVector<Element = T>,
        T: LoweredVectorElement<Vec = V>,
    {
        &mut *(p as *mut Self)
    }

    pub unsafe fn lift<'a, V>(p: *const V) -> &'a Self
    where
        V: LoweredVector,
        V::Element: Lift<Target = T>,
        T: Lower<Target = <V as LoweredVector>::Element>,
    {
        &*(p as *mut Self)
    }

    pub unsafe fn lift_mut<'a, V>(p: *mut V) -> &'a mut Self
    where
        V: LoweredVector,
        V::Element: Lift<Target = T>,
        T: Lower<Target = <V as LoweredVector>::Element>,
    {
        &mut *(p as *mut Self)
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

pub unsafe trait LoweredVector: Sized
where
    Self::Element: LoweredVectorElement<Vec = Self>,
{
    type Element;
}

pub unsafe trait LoweredVectorElement: Sized
where
    Self::Vec: LoweredVector<Element = Self>,
{
    type Vec;
}

macro_rules! lowered_vector {
    ($vector:ty, $element:ty $(,)?) => {
        unsafe impl LoweredVector for $vector {
            type Element = $element;
        }

        #[allow(clippy::use_self)]
        unsafe impl LoweredVectorElement for $element {
            type Vec = $vector;
        }

        unsafe impl Lift1 for $vector {
            type Target = Vector<$element>;

            fn lift1(self) -> Self::Target {
                unsafe { mem::transmute(self) }
            }
        }
    };
    (: lift: $vector:ty, $element:ty) => {
        impl Lift2 for $vector {
            type Target = Vector<<$element as Lift2>::Target>;

            fn lift2(self) -> Self::Target {
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
);
lowered_vector!(
    :lift:
    target::gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__Object_
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__RegionLayerData__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__RegionLayerData_,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__Visual__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__Visual_,
);
lowered_vector!(
    target::gfc__Vector_gfc__AutoRef_gfc__WorldRegionData__0_gfc__CAllocator_,
    target::gfc__AutoRef_gfc__WorldRegionData_,
);
lowered_vector!(
    target::gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector3_float_gfc__FloatMath_,
);
lowered_vector!(
    :lift:
    target::gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector3_float_gfc__FloatMath_
);
lowered_vector!(
    target::gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector4_float_gfc__FloatMath_,
);
lowered_vector!(
    :lift:
    target::gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    target::gfc__TVector4_float_gfc__FloatMath_
);
lowered_vector!(target::gfc__Vector_unsigned_char_0_gfc__CAllocator_, u8);
lowered_vector!(target::gfc__Vector_unsigned_long_0_gfc__CAllocator_, u32);
lowered_vector!(:lift: target::gfc__Vector_unsigned_long_0_gfc__CAllocator_, u32);
lowered_vector!(target::gfc__Vector_unsigned_short_0_gfc__CAllocator_, u16);
lowered_vector!(:lift: target::gfc__Vector_unsigned_short_0_gfc__CAllocator_, u16);
