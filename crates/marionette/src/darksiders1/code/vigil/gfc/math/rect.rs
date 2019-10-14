use std::ops::Sub;

pub struct TRect<N: Copy> {
    pub left: N,
    pub top: N,
    pub right: N,
    pub bottom: N,
}

#[allow(clippy::use_self)]
impl<N: Copy> TRect<N> {
    pub fn ltrb(left: N, top: N, right: N, bottom: N) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> <N as Sub>::Output
    where
        N: Sub,
    {
        self.right - self.left
    }

    pub fn height(&self) -> <N as Sub>::Output
    where
        N: Sub,
    {
        self.bottom - self.top
    }

    pub fn convert<To: Copy>(&self, f: impl Fn(N) -> To) -> TRect<To> {
        TRect {
            left: f(self.left),
            top: f(self.top),
            right: f(self.right),
            bottom: f(self.bottom),
        }
    }
}
