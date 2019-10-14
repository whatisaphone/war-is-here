use darksiders1_sys::target;

pub type TVector4<N> = na::Vector4<N>;

impl_lift_lower!(TVector4<f32>, target::gfc__TVector4_float_gfc__FloatMath_);
