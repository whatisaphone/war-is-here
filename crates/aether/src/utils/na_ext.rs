use na::{allocator::Allocator, DefaultAllocator, DimName, Point, Scalar};

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
