use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(PhysicsShapeObject, target::gfc__PhysicsShapeObject);
struct_wrapper_super!(
    PhysicsShapeObject,
    gfc::WorldObject,
    as_gfc__WorldObject_mut_ptr,
);
