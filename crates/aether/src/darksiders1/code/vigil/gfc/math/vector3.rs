use darksiders1_sys::target;
use na::UnitQuaternion;

pub type TVector3<N> = na::Vector3<N>;

impl_lift_lower!(TVector3<f32>, target::gfc__TVector3_float_gfc__FloatMath_);

pub trait TVector3Ext {
    /// Converts Euler angles (in degrees) to a unit quaternion.
    fn to_unit_quaternion(&self) -> UnitQuaternion<f32>;

    /// Converts a unit quaternion to Euler angles (in degrees).
    fn from_unit_quaternion(quat: &UnitQuaternion<f32>) -> Self;
}

impl TVector3Ext for TVector3<f32> {
    fn to_unit_quaternion(&self) -> UnitQuaternion<f32> {
        let roll = self[0].to_radians();
        let pitch = self[1].to_radians();
        let yaw = self[2].to_radians();
        UnitQuaternion::from_euler_angles(roll, pitch, yaw)
    }

    fn from_unit_quaternion(quat: &UnitQuaternion<f32>) -> Self {
        let (roll, pitch, yaw) = quat.euler_angles();
        Self::new(roll.to_degrees(), pitch.to_degrees(), yaw.to_degrees())
    }
}
