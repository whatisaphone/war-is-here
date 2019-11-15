use crate::{
    darksiders1::{gfc, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(Parameter, target::gfc__Parameter);
struct_wrapper_super!(Parameter, gfc::Object);

struct_wrapper!(Material, target::gfc__Material);
struct_wrapper_super!(Material, gfc::Object);
struct_wrapper_drop!(Material, target::gfc__Material___Material);

impl Material {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__Material__Material(this);
            })
        };
        Self { inner }
    }

    pub fn set_parameter(&self, name: &gfc::HString, value: gfc::AutoRef<Parameter>) {
        unsafe {
            target::gfc__Material__setParameter(
                self.as_ptr(),
                Lower::lower_ptr(name),
                Lower::lower(value),
            );
        }
    }
}

struct_wrapper!(Vector4Parameter, target::gfc__Vector4Parameter);
struct_wrapper_super!(Vector4Parameter, gfc::Parameter);
struct_wrapper_drop!(
    Vector4Parameter,
    target::gfc__Vector4Parameter___Vector4Parameter
);

impl Vector4Parameter {
    pub fn new(value: &gfc::TVector4<f32>) -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__Vector4Parameter__Vector4Parameter_2(this, Lower::lower_ptr(value));
            })
        };
        Self { inner }
    }
}
