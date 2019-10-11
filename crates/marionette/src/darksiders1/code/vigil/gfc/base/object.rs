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

pub unsafe trait Lift1 {
    type Target;

    fn lift1(self) -> Self::Target;

    fn lift1_ptr(&self) -> *mut Self::Target {
        unsafe { &mut *(self as *const Self as *const Self::Target as *mut Self::Target) }
    }

    unsafe fn lift1_ref(&self) -> &Self::Target {
        &*(self as *const Self as *const Self::Target)
    }

    unsafe fn lift1_mut(&mut self) -> &mut Self::Target {
        &mut *(self as *mut Self as *mut Self::Target)
    }
}

unsafe impl<T: Lift> Lift1 for *const T {
    type Target = *const T::Target;

    fn lift1(self) -> Self::Target {
        self as *const _
    }
}

unsafe impl<T: Lift> Lift1 for *mut T {
    type Target = *mut T::Target;

    fn lift1(self) -> Self::Target {
        self as *mut _
    }
}

unsafe impl<'a, T: Lift> Lift1 for &'a T {
    type Target = &'a T::Target;

    fn lift1(self) -> Self::Target {
        unsafe { &*(self as *const T as *const _) }
    }
}

unsafe impl<'a, T: Lift> Lift1 for &'a mut T {
    type Target = &'a mut T::Target;

    fn lift1(self) -> Self::Target {
        unsafe { &mut *(self as *mut T as *mut _) }
    }
}

pub trait Lift2 {
    type Target;

    fn lift2(self) -> Self::Target;

    fn lift2_ptr(&self) -> *mut Self::Target {
        unsafe { &mut *(self as *const Self as *const Self::Target as *mut Self::Target) }
    }

    unsafe fn lift2_ref(&self) -> &Self::Target {
        &*(self as *const Self as *const Self::Target)
    }

    unsafe fn lift2_mut(&mut self) -> &mut Self::Target {
        &mut *(self as *mut Self as *mut Self::Target)
    }
}

impl<T: Lift> Lift2 for *const T {
    type Target = *const T::Target;

    fn lift2(self) -> Self::Target {
        self as *const _
    }
}

impl<T: Lift> Lift2 for *mut T {
    type Target = *mut T::Target;

    fn lift2(self) -> Self::Target {
        self as *mut _
    }
}

impl<'a, T: Lift> Lift2 for &'a T {
    type Target = &'a T::Target;

    fn lift2(self) -> Self::Target {
        unsafe { &*(self as *const T as *const _) }
    }
}

impl<'a, T: Lift> Lift2 for &'a mut T {
    type Target = &'a mut T::Target;

    fn lift2(self) -> Self::Target {
        unsafe { &mut *(self as *mut T as *mut _) }
    }
}

// Numbers are, obviously, the same on both sides of the divide.
impl_lift_lower!(u8, u8);
impl_lift_lower!(u16, u16);
impl_lift_lower!(u32, u32);

impl_lift_lower2!(u16, u16);
impl_lift_lower2!(u32, u32);
