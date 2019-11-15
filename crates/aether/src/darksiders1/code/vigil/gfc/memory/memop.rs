use darksiders1_sys::target;
use std::ptr;

const HEAP: u32 = 1;

pub fn mem_alloc(size: u32, align: u32) -> *mut () {
    unsafe { target::gfc__MemAlloc(HEAP, ptr::null_mut(), size, align, 0, 0, ptr::null(), 0) }
}

pub unsafe fn mem_free<T>(addr: *mut T) {
    target::gfc__MemFree(HEAP, addr.cast(), ptr::null(), 0);
}
