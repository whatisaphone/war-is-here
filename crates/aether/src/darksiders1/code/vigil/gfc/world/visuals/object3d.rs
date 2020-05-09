use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(Object3D, target::gfc__Object3D);
struct_wrapper_super!(Object3D, gfc::Object);
struct_wrapper_drop!(Object3D, target::gfc__Object3D___Object3D);

impl Object3D {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__Object3D__Object3D(this);
            })
        };
        Self { inner }
    }

    pub fn world_object(&self) -> &gfc::WorldObject {
        unsafe { (*self.inner.mWorldObject).lift_ref() }
    }
}
