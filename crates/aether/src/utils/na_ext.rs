use na::{RealField, UnitComplex, UnitQuaternion, Vector3};

pub trait UnitComplexExt<N> {
    fn around_z_axis(&self) -> UnitQuaternion<N>
    where
        N: RealField;
}

impl<N> UnitComplexExt<N> for UnitComplex<N> {
    fn around_z_axis(&self) -> UnitQuaternion<N>
    where
        N: RealField,
    {
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.angle())
    }
}
