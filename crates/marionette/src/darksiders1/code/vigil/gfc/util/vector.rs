use darksiders1_sys::target;
use std::{
    convert::TryInto,
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

    pub unsafe fn from_ptr<'a, X>(p: *mut X) -> &'a Self {
        &*(p as *const Self)
    }

    pub unsafe fn from_ptr_mut<'a, X>(p: *mut X) -> &'a mut Self {
        &mut *(p as *mut Self)
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn is_memory_owned(&self) -> bool {
        (self.capacity_and_flags & Self::OWNED_MASK) != 0
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.is_memory_owned() {
            // TODO: This should run the destructor on each element before freeing the
            // memory (I believe).
            unsafe {
                target::gfc__MemFree(1, self.data as *mut (), ptr::null(), 0);
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
