use std::ops::{Add, Mul, Range, Sub};

pub fn lerp<B, N>(t: N, r: Range<B>) -> <B as Add<<N as Mul<<B as Sub>::Output>>::Output>>::Output
where
    B: Add<<N as Mul<<B as Sub>::Output>>::Output> + Sub + Copy,
    N: Mul<<B as Sub>::Output>,
{
    let (x, y) = (r.start, r.end);
    x + t * (y - x)
}

pub fn unlerp(v: f32, r: Range<f32>) -> f32 {
    let (x, y) = (r.start, r.end);
    (v - x) / (y - x)
}
