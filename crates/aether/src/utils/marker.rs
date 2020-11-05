use std::ops::{Deref, DerefMut};

pub struct UnsafeSend<T>(T);

unsafe impl<T> Send for UnsafeSend<T> {}

impl<T> UnsafeSend<T> {
    pub const unsafe fn new(x: T) -> Self {
        Self(x)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for UnsafeSend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for UnsafeSend<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
