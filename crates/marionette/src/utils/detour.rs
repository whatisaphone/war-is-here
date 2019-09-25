use detour::{Function, RawDetour};
use std::{marker::PhantomData, mem};

#[allow(clippy::module_name_repetitions)]
pub struct TypedDetour<F: Function>(RawDetour, PhantomData<F>);

impl<F: Function> TypedDetour<F> {
    pub unsafe fn new(target: F, detour: F) -> Result<Self, detour::Error> {
        let detour = RawDetour::new(mem::transmute_copy(&target), mem::transmute_copy(&detour))?;
        detour.enable()?;
        Ok(Self(detour, PhantomData))
    }

    pub unsafe fn trampoline(&self) -> F {
        mem::transmute_copy(&self.0.trampoline())
    }
}

impl<F: Function> Drop for TypedDetour<F> {
    fn drop(&mut self) {
        unsafe {
            self.0.disable().unwrap();
        }
    }
}
