use crate::{
    darksiders1::{gfc, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(StaticObject, target::gfc__StaticObject);
struct_wrapper_super!(StaticObject, gfc::WorldObject);
struct_wrapper_drop!(StaticObject, target::gfc__StaticObject___StaticObject);

impl StaticObject {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__StaticObject__StaticObject(this);
            })
        };
        Self { inner }
    }

    pub fn set_package_name(&self, v: &gfc::HString) {
        unsafe {
            target::gfc__StaticObject__setPackageName(self.as_ptr(), Lower::lower_ptr(v));
        }
    }

    pub fn set_object_name(&self, v: &gfc::HString) {
        unsafe {
            target::gfc__StaticObject__setObjectName(self.as_ptr(), Lower::lower_ptr(v));
        }
    }
}
