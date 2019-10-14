use darksiders1_sys::target;

pub type TVector3<N> = na::Vector3<N>;

impl_lift_lower!(TVector3<f32>, target::gfc__TVector3_float_gfc__FloatMath_);

pub trait TVector3Ext {
    /// Converts a unit quaternion to Euler angles (in degrees).
    fn from_unit_quaternion(quat: &na::UnitQuaternion<f32>) -> Self;
}

impl TVector3Ext for TVector3<f32> {
    fn from_unit_quaternion(quat: &na::UnitQuaternion<f32>) -> Self {
        let (roll, pitch, yaw) = quat.euler_angles();
        Self::new(roll.to_degrees(), pitch.to_degrees(), yaw.to_degrees())
    }
}
