use std::mem;

pub unsafe fn init_with<T>(f: impl FnOnce(*mut T)) -> T {
    let mut result = mem::MaybeUninit::uninit();
    f(result.as_mut_ptr());
    result.assume_init()
}
