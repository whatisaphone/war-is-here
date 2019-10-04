use std::convert::TryFrom;

pub struct Vector<T> {
    data: *mut T,
    size: i32,
    _capacity_and_flags: i32,
}

impl<T> Vector<T> {
    pub unsafe fn from_ptr<'a, X>(p: *mut X) -> &'a Self {
        &*(p as *mut Self)
    }

    pub fn iter(&self) -> Vector__Iterator<'_, T> {
        Vector__Iterator::new(self)
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
