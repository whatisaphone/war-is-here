use crate::darksiders1::{gfc, Lift, Lower};
use darksiders1_sys::target;
use na::Vector4;

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

    pub fn fill_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        uv0: &Vector4<f32>,
        uv1: &Vector4<f32>,
    ) {
        unsafe {
            target::gfc__UIRenderer__fillRect(
                self.as_ptr(),
                x,
                y,
                width,
                height,
                Lower::lower_ptr(uv0),
                Lower::lower_ptr(uv1),
            );
        }
    }
}
