use darksiders1_sys::target;
use na::Point3;
use std::fmt::Debug;

#[repr(C)]
#[derive(Clone)]
pub struct TBox<T: Copy + PartialEq + Debug + 'static> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl_lift_lower!(TBox<f32>, target::gfc__TBox_float_gfc__FloatMath_);
