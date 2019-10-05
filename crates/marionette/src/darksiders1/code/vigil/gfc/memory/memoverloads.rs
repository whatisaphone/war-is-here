use darksiders1_sys::target;
use std::{convert::TryFrom, mem, ptr};

pub fn new<T>(ctor: impl FnOnce() -> T) -> *mut T {
    let size = u32::try_from(mem::size_of::<T>()).unwrap();
    unsafe {
        let result = target::operator_new_2(size) as *mut T;
        ptr::write(result, ctor());
        result
    }
}
