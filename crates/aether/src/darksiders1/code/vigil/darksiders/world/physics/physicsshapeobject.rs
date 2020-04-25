use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

struct_wrapper!(PhysicsShapeObject, target::gfc__PhysicsShapeObject);
struct_wrapper_super!(PhysicsShapeObject, gfc::WorldObject);

impl PhysicsShapeObject {
    pub fn shape(&self) -> PhysicsShapeObject__Detect {
        let raw = unsafe { (*self.as_ptr()).mShape };
        PhysicsShapeObject__Detect::try_from(raw).unwrap()
    }

    pub fn get_bounding_box(&self) -> gfc::TBox<f32> {
        unsafe {
            init_with(|p| {
                (*self.as_ptr()).getBoundingBox(p);
            })
        }
        .lift()
    }

    /// Calculates a transformation matrix for the object's rotation and
    /// position.
    pub fn get_transform(&self) -> gfc::Matrix4<f32> {
        unsafe {
            init_with(|p| {
                target::gfc__PhysicsShapeObject__getTransform(self.as_ptr(), p);
            })
        }
        .lift()
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
