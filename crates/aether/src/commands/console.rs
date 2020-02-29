use crate::library::console;
use std::sync::atomic::Ordering;

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = console::WANT_ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn pump() {
    console::run_frame();
}
