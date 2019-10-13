use darksiders1_sys::target;

pub type TVector3<T> = na::Vector3<T>;

impl_lift_lower!(TVector3<f32>, target::gfc__TVector3_float_gfc__FloatMath_);
