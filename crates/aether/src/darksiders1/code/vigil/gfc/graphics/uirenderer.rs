use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(UIRenderer, target::gfc__UIRenderer);

impl UIRenderer {
    pub fn solid_material(&self) -> &gfc::Material {
        unsafe { (*self.as_ptr()).mSolidMaterial.lift_ref() }
    }

    pub fn begin(&self, clearstencil: bool) {
        unsafe {
            target::gfc__UIRenderer__begin(self.as_ptr(), clearstencil);
        }
    }

    pub fn end(&self) {
        unsafe {
            target::gfc__UIRenderer__end(self.as_ptr());
        }
    }

    pub fn set_material(&self, material: &gfc::Material) {
        unsafe {
            target::gfc__UIRenderer__setMaterial(self.as_ptr(), material.as_ptr());
        }
    }
}
