use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;
use std::mem;

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
                let new_instance = cast_away_pdbindgen_bug((*self.inner.__vfptr).newInstance);
                new_instance(self.as_ptr(), this);
            });
            gfc::AutoRefObject::from_raw(obj)
        }
    }

    pub fn as_ptr(&self) -> *mut target::gfc__Class {
        &self.inner as *const _ as *mut _
    }
}

type NewInstanceWrong =
    unsafe extern "thiscall" fn(this: *mut target::gfc__Class) -> target::gfc__AutoRef_gfc__Object_;
type NewInstanceRight = unsafe extern "thiscall" fn(
    this: *mut target::gfc__Class,
    result: *mut target::gfc__AutoRef_gfc__Object_,
) -> *mut target::gfc__AutoRef_gfc__Object_;

unsafe fn cast_away_pdbindgen_bug(new_instance: NewInstanceWrong) -> NewInstanceRight {
    mem::transmute(new_instance)
}
