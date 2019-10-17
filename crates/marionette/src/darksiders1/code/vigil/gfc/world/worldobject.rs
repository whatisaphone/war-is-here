use crate::{
    darksiders1::{
        gfc::{self, TVector3Ext},
        Lift,
        Lower,
    },
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::{Point3, UnitQuaternion, Vector3};

struct_wrapper!(WorldObject, target::gfc__WorldObject);
struct_wrapper_super!(WorldObject, gfc::Object);
impl_reflection!(WorldObject, target::gfc__WorldObject___Class);

impl WorldObject {
    pub fn set_position(&self, pos: &Point3<f32>) {
        unsafe {
            self.inner.setPosition(Lower::lower_ptr(&pos.coords));
        }
    }

    pub fn get_position(&self) -> Point3<f32> {
        let pos = unsafe {
            init_with(|p| {
                self.inner.getPosition(p);
            })
        };
        Point3::from(pos.lift())
    }

    pub fn set_rotation(&self, rot: &UnitQuaternion<f32>) {
        unsafe {
            self.inner
                .setRotation(&Lower::lower(Vector3::from_unit_quaternion(&rot)));
        }
    }

    pub fn set_scale(&self, scale: &Vector3<f32>) {
        unsafe {
            self.inner.setScale(Lower::lower_ptr(scale));
        }
    }

    pub fn add_object_to_world(&self, world: &gfc::World) {
        unsafe {
            self.inner.addObjectToWorld(Lower::lower_ptr(world));
        }
    }
}
