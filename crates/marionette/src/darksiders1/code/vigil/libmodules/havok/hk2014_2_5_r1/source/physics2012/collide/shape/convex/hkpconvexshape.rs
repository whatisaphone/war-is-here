use crate::darksiders1::hkpSphereRepShape;
use darksiders1_sys::target;

struct_wrapper!(
    /// An interface shape class that allows its implementations to work with
    /// GJK. It also holds an extra radius to allow for shells around objects.
    hkpConvexShape,
    target::hkpConvexShape,
);
struct_wrapper_super!(hkpConvexShape, hkpSphereRepShape);
