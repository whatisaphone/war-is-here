use std::ops::Sub;

pub struct TRect<T: Copy> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

#[allow(clippy::use_self)]
impl<T: Copy> TRect<T> {
    pub fn ltrb(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> <T as Sub>::Output
    where
        T: Sub,
    {
        self.right - self.left
    }

    pub fn height(&self) -> <T as Sub>::Output
    where
        T: Sub,
    {
        self.bottom - self.top
    }

    pub fn convert<U: Copy>(&self, f: impl Fn(T) -> U) -> TRect<U> {
        TRect {
            left: f(self.left),
            top: f(self.top),
            right: f(self.right),
            bottom: f(self.bottom),
        }
    }
}
