#[repr(C)]
#[derive(Clone)]
pub struct TVector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> TVector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
