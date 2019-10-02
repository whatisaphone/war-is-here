use darksiders1_sys::target;
use std::{convert::TryInto, mem, ptr};

#[allow(dead_code)]
pub unsafe fn new<T>(ctor: impl FnOnce(*mut T)) -> *mut T {
    let alloc = target::gfc__MemAlloc(
        1,
        ptr::null_mut(),
        mem::size_of::<T>().try_into().unwrap(),
        mem::align_of::<T>().try_into().unwrap(),
        0,
        0,
        ptr::null(),
        0,
    ) as *mut T;
    ctor(alloc);
    alloc
}
