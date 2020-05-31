use na::{
    allocator::Allocator,
    DefaultAllocator,
    DimName,
    Point,
    RealField,
    Scalar,
    UnitComplex,
    UnitQuaternion,
    Vector3,
};

pub trait PointExt<N: Scalar, D: DimName>
where
    DefaultAllocator: Allocator<N, D>,
{
    fn map<N2: Scalar>(&self, f: impl FnMut(N) -> N2) -> Point<N2, D>
    where
        DefaultAllocator: Allocator<N2, D>;
}

impl<N: Scalar, D: DimName> PointExt<N, D> for Point<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    fn map<N2: Scalar>(&self, f: impl FnMut(N) -> N2) -> Point<N2, D>
    where
        DefaultAllocator: Allocator<N2, D>,
    {
        self.coords.map(f).into()
    }
}

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
