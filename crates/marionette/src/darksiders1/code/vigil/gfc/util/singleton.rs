use crate::darksiders1::gfc;
use darksiders1_sys::target;
use std::marker::PhantomData;

pub struct Singleton<T> {
    _static: PhantomData<T>,
}

macro_rules! impl_singleton {
    ($type:ty, $symbol:expr $(,)?) => {
        impl Singleton<$type> {
            pub fn get_instance() -> &'static $type {
                unsafe { <$type>::from_ptr(*$symbol) }
            }
        }
    };
}

impl_singleton!(
    gfc::ClassRegistry,
    target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle,
);
impl_singleton!(
    gfc::Darksiders,
    target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle,
);
impl_singleton!(
    gfc::KGMeshCache,
    target::gfc__Singleton_gfc__KGMeshCache_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle,
);
impl_singleton!(
    gfc::Object3DCache,
    target::gfc__Singleton_gfc__Object3DCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle,
);
