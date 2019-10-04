use crate::darksiders1::gfc;
use darksiders1_sys::target;
use winapi::_core::marker::PhantomData;

pub struct Singleton<T> {
    _static: PhantomData<T>,
}

impl Singleton<gfc::Darksiders> {
    pub fn get_instance() -> &'static gfc::Darksiders {
        unsafe {
            gfc::Darksiders::from_ptr(*target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle)
        }
    }
}
