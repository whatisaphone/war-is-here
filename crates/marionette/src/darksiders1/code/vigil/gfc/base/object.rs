use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Object, target::gfc__Object);

impl Object {
    pub fn class(&self) -> &gfc::Class {
        unsafe {
            let result = ((*self.inner.__vfptr).getClass)(self.as_ptr());
            gfc::Class::from_ptr(result)
        }
    }
}

pub fn object_safecast<To: Reflect>(p: &Object) -> Option<gfc::AutoRef<To>> {
    let class = p.class();
    if !class.instanceof(To::class()) {
        return None;
    }
    Some(unsafe { gfc::AutoRef::from_ptr(p.as_ptr() as *mut target::gfc__IRefObject) })
}

pub unsafe trait Reflect {
    fn class() -> &'static gfc::Class;
}
