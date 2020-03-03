use darksiders1_sys::target;
use std::sync::atomic::{AtomicBool, Ordering};

static PRETENDING: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = PRETENDING.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub unsafe fn world_constructor_hook(world: *mut target::gfc__World) {
    if PRETENDING.load(Ordering::SeqCst) {
        (*world).mInEditor = true;
    }
}
