pub struct UnsafeSend<T>(T);

unsafe impl<T> Send for UnsafeSend<T> {}

impl<T> UnsafeSend<T> {
    pub const fn new(x: T) -> Self {
        Self(x)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
