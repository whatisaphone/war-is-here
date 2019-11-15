use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;
use std::ptr;

struct_wrapper!(ScriptClass, target::gfc__ScriptClass);
struct_wrapper_super!(ScriptClass, gfc::Class);
struct_wrapper_drop!(ScriptClass, target::gfc__ScriptClass___ScriptClass);

impl ScriptClass {
    pub fn new(name: &gfc::HString, package_id: i32, baseclass: Option<&gfc::Class>) -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__ScriptClass__ScriptClass(
                    this,
                    name.as_ptr(),
                    package_id,
                    baseclass.map_or(ptr::null_mut(), gfc::Class::as_ptr),
                );
            })
        };
        Self { inner }
    }
}
