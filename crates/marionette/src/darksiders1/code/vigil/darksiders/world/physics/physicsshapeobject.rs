use crate::darksiders1::gfc;
use darksiders1_sys::target;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

struct_wrapper!(PhysicsShapeObject, target::gfc__PhysicsShapeObject);
struct_wrapper_super!(PhysicsShapeObject, gfc::WorldObject);

impl PhysicsShapeObject {
    pub const DETECT_AABB: i32 = 0;
    pub const DETECT_BOX: i32 = 1;
    pub const DETECT_SPHERE: i32 = 2;
    pub const DETECT_CYLINDER: i32 = 3;

    pub fn shape(&self) -> PhysicsShapeObject__Detect {
        let raw = unsafe { (*self.as_ptr()).mShape };
        PhysicsShapeObject__Detect::try_from(raw).unwrap()
    }
}

#[derive(TryFromPrimitive)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum PhysicsShapeObject__Detect {
    Aabb = 0,
    Box = 1,
    Sphere = 2,
    Cylinder = 3,
}
