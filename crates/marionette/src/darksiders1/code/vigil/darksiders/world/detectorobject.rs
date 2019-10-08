use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(DetectorObject, target::gfc__DetectorObject);
struct_wrapper_super!(
    DetectorObject,
    gfc::PhysicsShapeObject,
    as_gfc__PhysicsShapeObject_mut_ptr,
);
