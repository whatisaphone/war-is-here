use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use std::{mem, ptr};

struct_wrapper!(Class, target::gfc__Class);
struct_wrapper_super!(Class, gfc::IRefObject);

impl Class {
    pub fn new_instance(&self) -> gfc::AutoRef<gfc::Object> {
        let obj = unsafe {
            init_with(|p| {
                self.inner.newInstance(p);
            })
        };
        obj.lift()
    }

    pub fn name(&self) -> &gfc::HString {
        unsafe {
            let result = self.inner.getName();
            gfc::HString::from_ptr(result)
        }
    }

    pub fn is_abstract(&self) -> bool {
        unsafe { self.inner.isAbstract() }
    }

    pub fn instanceof(&self, class: &Self) -> bool {
        unsafe { self.inner.instanceof(class.as_ptr()) }
    }

    pub fn get_property_by_name(
        &self,
        name: &gfc::HString,
        out_prop_class: Option<&mut Option<&gfc::Class>>,
    ) -> Option<&gfc::Property> {
        unsafe {
            let out_prop_class: *mut *const target::gfc__Class = match out_prop_class {
                Some(p) => mem::transmute(*p),
                None => ptr::null_mut(),
            };

            self.inner
                .getPropertyByName(name.as_ptr(), out_prop_class)
                .as_ref()
                .map(Lift::lift_ref)
        }
    }
}

struct_wrapper!(ClassRegistry, target::gfc__ClassRegistry);

impl ClassRegistry {
    pub fn class_for_name(
        &self,
        classname: &gfc::HString,
        use_loaders: bool,
    ) -> Option<&gfc::Class> {
        unsafe {
            let result = target::gfc__ClassRegistry__classForName(
                self.as_ptr(),
                classname.as_ptr(),
                use_loaders,
                false, // The `quiet` param has no effect. Probably #ifdef'd away.
            );
            result.as_ref().map(|p| gfc::Class::from_ptr(p))
        }
    }
}
