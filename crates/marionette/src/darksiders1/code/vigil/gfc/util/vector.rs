use darksiders1_sys::target;
use std::{
    convert::{TryFrom, TryInto},
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
        unsafe { slice::from_raw_parts(self.data, self.size.try_into().unwrap()) }
    }

    pub fn iter(&self) -> Vector__Iterator<'_, T> {
        Vector__Iterator::new(self)
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
    type IntoIter = Vector__Iterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Vector__Iterator::new(self)
    }
}

#[allow(non_camel_case_types)]
pub struct Vector__Iterator<'a, T> {
    current: *mut T,
    end: *mut T,
    _vector: &'a Vector<T>,
}

impl<'a, T> Vector__Iterator<'a, T> {
    fn new(vector: &'a Vector<T>) -> Self {
        let size = usize::try_from(vector.size).unwrap();
        Vector__Iterator {
            current: vector.data,
            end: unsafe { vector.data.add(size) },
            _vector: vector,
        }
    }
}

impl<'a, T> Iterator for Vector__Iterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }
        let result = self.current;
        unsafe {
            self.current = self.current.add(1);
            Some(result.as_ref().unwrap())
        }
    }
}
