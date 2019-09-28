use std::{
    mem,
    sync::atomic::{AtomicI32, Ordering},
};

pub unsafe fn new<T>(ctor: impl FnOnce(*mut T)) -> *mut T {
    // BAD: this allocates memory on the wrong heap.
    let mut boxx = Box::new(mem::MaybeUninit::uninit());
    ctor(boxx.as_mut_ptr());
    &mut *mem::transmute::<Box<mem::MaybeUninit<T>>, Box<T>>(boxx)
}

pub fn lock_xadd(target: &mut i32, n: i32) -> i32 {
    atomic(target).fetch_add(n, Ordering::SeqCst)
}

pub fn atomic(x: &mut i32) -> &mut AtomicI32 {
    unsafe { &mut *(x as *mut i32 as *mut AtomicI32) }
}
