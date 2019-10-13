use darksiders1_sys::target;

pub type TVector4<T> = na::Vector4<T>;

impl_lift_lower!(TVector4<f32>, target::gfc__TVector4_float_gfc__FloatMath_);
