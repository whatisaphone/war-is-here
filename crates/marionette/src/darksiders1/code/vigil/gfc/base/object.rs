use darksiders1_sys::target;

pub struct Object {
    inner: target::gfc__Object,
}

impl Object {
    pub unsafe fn from_ptr<'a>(inner: *const target::gfc__Object) -> &'a Self {
        &*(inner as *const _)
    }

    pub fn as_ptr(&self) -> *mut target::gfc__Object {
        &self.inner as *const _ as *mut _
    }
}
