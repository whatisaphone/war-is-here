use crate::darksiders1::hkpShape;
use darksiders1_sys::target;

struct_wrapper!(
    /// This interface produces a set of spheres that represent a very
    /// simplified version of the objects surface. Note: This interface's
    /// function is used by hkpHeightFieldShape implementations
    hkpSphereRepShape,
    target::hkpSphereRepShape,
);
struct_wrapper_super!(hkpSphereRepShape, hkpShape);
