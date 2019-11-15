use detour::{Function, RawDetour};
use std::{marker::PhantomData, mem};

#[allow(clippy::module_name_repetitions)]
pub struct TypedDetour<F: Function>(RawDetour, PhantomData<F>);

impl<F: Function> TypedDetour<F> {
    pub unsafe fn create(target: F, detour: F) -> detour::Result<Self> {
        let detour = RawDetour::new(mem::transmute_copy(&target), mem::transmute_copy(&detour))?;
        detour.enable()?;
        Ok(Self(detour, PhantomData))
    }

    pub fn into_inner(self) -> RawDetour {
        self.0
    }

    pub unsafe fn trampoline(&self) -> F {
        mem::transmute_copy(&self.0.trampoline())
    }
}
