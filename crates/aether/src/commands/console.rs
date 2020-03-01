use crate::console::WANT_ENABLED;
use std::sync::atomic::Ordering;

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = WANT_ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}
