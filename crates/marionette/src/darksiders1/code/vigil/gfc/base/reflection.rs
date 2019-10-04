use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

#[repr(transparent)]
pub struct Class {
    inner: target::gfc__Class,
}

impl Class {
    pub unsafe fn from_ptr<'a>(inner: *const target::gfc__Class) -> &'a Self {
        &*(inner as *const _)
    }

    pub fn new_instance(&self) -> gfc::AutoRefObject {
        unsafe {
            let obj = init_with(|this| {
                ((*self.inner.__vfptr).newInstance)(self.as_ptr(), this);
            });
            gfc::AutoRefObject::from_raw(obj)
        }
    }

    pub fn as_ptr(&self) -> *mut target::gfc__Class {
        &self.inner as *const _ as *mut _
    }
}
