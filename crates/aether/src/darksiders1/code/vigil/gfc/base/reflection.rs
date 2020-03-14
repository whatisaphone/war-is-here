use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;

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
