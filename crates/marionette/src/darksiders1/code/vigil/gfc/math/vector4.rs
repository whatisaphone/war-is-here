use darksiders1_sys::target;

#[repr(C)]
#[derive(Clone)]
pub struct TVector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> TVector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl_lift_lower_transmute!(TVector4<f32>, target::gfc__TVector4_float_gfc__FloatMath_);
