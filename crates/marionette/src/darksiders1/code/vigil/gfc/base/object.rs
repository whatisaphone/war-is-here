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

pub trait Lift
where
    Self::Target: Lower<Target = Self>,
{
    type Target: ?Sized;

    fn lift(this: *mut Self) -> *mut Self::Target;
    fn lower(this: *mut Self::Target) -> *mut Self;
}

pub trait Lower
where
    Self::Target: Lift<Target = Self>,
{
    type Target: ?Sized;

    fn lift(this: *mut Self::Target) -> *mut Self {
        Self::Target::lift(this)
    }

    fn lower(this: *mut Self) -> *mut Self::Target {
        Self::Target::lower(this)
    }
}

// Numbers are, obviously, the same on both sides of the divide.
impl_lift_lower_transmute!(u8, u8);
impl_lift_lower_transmute!(u16, u16);
impl_lift_lower_transmute!(u32, u32);
