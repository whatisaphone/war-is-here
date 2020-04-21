use darksiders1_sys::target;
use na::{Point3, RealField, Scalar};

#[repr(C)]
#[derive(Clone)]
pub struct TBox<N: Scalar + RealField> {
    pub min: Point3<N>,
    pub max: Point3<N>,
}

impl_lift_lower!(TBox<f32>, target::gfc__TBox_float_gfc__FloatMath_);

impl<N: Scalar + RealField> TBox<N> {
    pub fn new(min: Point3<N>, max: Point3<N>) -> Self {
        Self { min, max }
    }

    pub fn center(&self) -> Point3<N> {
        na::center(&self.min, &self.max)
    }
}
