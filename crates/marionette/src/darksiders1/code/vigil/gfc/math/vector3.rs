use darksiders1_sys::target;

pub type TVector3<N> = na::Vector3<N>;

impl_lift_lower!(TVector3<f32>, target::gfc__TVector3_float_gfc__FloatMath_);
