use na::{Matrix4, RealField, Transform3, UnitComplex, UnitQuaternion, Vector3};

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

pub trait Transform3Ext<N: RealField> {
    fn from_scaling(scaling: &Vector3<N>) -> Self;
}

impl<N: RealField> Transform3Ext<N> for Transform3<N> {
    fn from_scaling(scaling: &Vector3<N>) -> Self {
        #[rustfmt::skip]
            let matrix = Matrix4::new(
            scaling[0], N::zero(), N::zero(), N::zero(),
            N::zero(), scaling[1], N::zero(), N::zero(),
            N::zero(), N::zero(), scaling[2], N::zero(),
            N::zero(), N::zero(), N::zero(), N::one(),
        );
        Self::from_matrix_unchecked(matrix)
    }
}
