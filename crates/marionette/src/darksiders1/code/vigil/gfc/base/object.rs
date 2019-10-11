use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Object, target::gfc__Object);
struct_wrapper_super!(Object, gfc::IRefObject);

impl Object {
    pub fn class(&self) -> &gfc::Class {
        unsafe {
            let result = ((*self.inner.vfptr).getClass)(self.as_ptr());
            gfc::Class::from_ptr(result)
        }
    }
}

pub fn object_safecast<To: Reflect>(p: &Object) -> Option<gfc::AutoRef<To>> {
    let class = p.class();
    if !class.instanceof(To::class()) {
        return None;
    }
    Some(unsafe { gfc::AutoRef::from_ptr(p as *const Object as *mut Object as *mut To) })
}

pub unsafe trait Reflect: AsRef<gfc::IRefObject> {
    fn class() -> &'static gfc::Class;
}

pub trait Lift: Sized
where
    Self::Target: Lower<Target = Self>,
{
    type Target;

    fn lift(this: *const Self) -> *mut Self::Target {
        this as *const _ as *mut _
    }
}

pub trait Lower: Sized
where
    Self::Target: Lift<Target = Self>,
{
    type Target;

    fn lower(this: *const Self) -> *mut Self::Target {
        this as *const _ as *mut _
    }
}

// Numbers are, obviously, the same on both sides of the divide.
impl_lift_lower!(u8, u8);
impl_lift_lower!(u16, u16);
impl_lift_lower!(u32, u32);
