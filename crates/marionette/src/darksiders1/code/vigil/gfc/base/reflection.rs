use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(Class, target::gfc__Class);

impl Class {
    pub fn new_instance(&self) -> gfc::AutoRef<gfc::Object> {
        unsafe {
            let obj = init_with(|this| {
                ((*self.inner.__vfptr).newInstance)(self.as_ptr(), this);
            });
            gfc::AutoRef::from_raw(autoref_transmute!(obj))
        }
    }

    pub fn name(&self) -> &gfc::HString {
        unsafe {
            let result = ((*self.inner.__vfptr).getName)(self.as_ptr());
            gfc::HString::from_ptr(result)
        }
    }

    pub fn instanceof(&self, class: &Self) -> bool {
        unsafe { ((*self.inner.__vfptr).instanceof)(self.as_ptr(), class.as_ptr()) }
    }
}
