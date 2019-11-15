use darksiders1_sys::target;
use std::sync::atomic::Ordering;
use winapi::_core::sync::atomic::AtomicBool;

static PRETENDING: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) {
    PRETENDING.store(true, Ordering::SeqCst);
}

pub unsafe fn world_constructor_hook(world: *mut target::gfc__World) {
    if PRETENDING.load(Ordering::SeqCst) {
        (*world).mInEditor = true;
    }
}
