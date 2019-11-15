use std::fmt;

pub struct Pretty<T>(pub T);

impl fmt::Display for Pretty<&na::Matrix4<f32>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
[[{:.3}, {:.3}, {:.3}, {:.3}]
 [{:.3}, {:.3}, {:.3}, {:.3}]
 [{:.3}, {:.3}, {:.3}, {:.3}]
 [{:.3}, {:.3}, {:.3}, {:.3}]]",
            self.0[(0, 0)],
            self.0[(0, 1)],
            self.0[(0, 2)],
            self.0[(0, 3)],
            self.0[(1, 0)],
            self.0[(1, 1)],
            self.0[(1, 2)],
            self.0[(1, 3)],
            self.0[(2, 0)],
            self.0[(2, 1)],
            self.0[(2, 2)],
            self.0[(2, 3)],
            self.0[(3, 0)],
            self.0[(3, 1)],
            self.0[(3, 2)],
            self.0[(3, 3)],
        )
    }
}

impl fmt::Display for Pretty<&na::Matrix4x1<f32>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:.3}, {:.3}, {:.3}, {:.3}]T",
            self.0[(0, 0)],
            self.0[(1, 0)],
            self.0[(2, 0)],
            self.0[(3, 0)],
        )
    }
}

impl fmt::Display for Pretty<&na::Point3<f32>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.0}, {:.0}, {:.0})", self.0.x, self.0.y, self.0.z)
    }
}

macro_rules! pretty_by_value {
    ($type:ty) => {
        #[allow(clippy::use_self)]
        impl fmt::Display for Pretty<$type> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Pretty(&self.0).fmt(f)
            }
        }
    };
}

pretty_by_value!(na::Matrix4<f32>);
pretty_by_value!(na::Matrix4x1<f32>);
pretty_by_value!(na::Point3<f32>);
