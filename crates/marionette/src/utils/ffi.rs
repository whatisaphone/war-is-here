use darksiders1_sys::target;
use std::{
    convert::TryInto,
    mem,
    ptr,
    sync::atomic::{AtomicI32, Ordering},
};

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

pub fn lock_xadd(target: &mut i32, n: i32) -> i32 {
    atomic(target).fetch_add(n, Ordering::SeqCst)
}

pub fn atomic(x: &mut i32) -> &mut AtomicI32 {
    unsafe { &mut *(x as *mut i32 as *mut AtomicI32) }
}
