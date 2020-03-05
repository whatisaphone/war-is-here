use crate::Result;
use std::mem;
use winapi::shared::winerror::{HRESULT, SUCCEEDED};

pub unsafe fn init_with<T>(f: impl FnOnce(*mut T)) -> T {
    let mut result = mem::MaybeUninit::uninit();
    f(result.as_mut_ptr());
    result.assume_init()
}

pub unsafe fn init_with_hresult<T>(f: impl FnOnce(*mut T) -> HRESULT) -> Result<T> {
    let mut result = mem::MaybeUninit::uninit();
    let hr = f(result.as_mut_ptr());
    if SUCCEEDED(hr) {
        Ok(result.assume_init())
    } else {
        Err(hr)
    }
}
