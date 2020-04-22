use crate::{darksiders1::gfc, library::bitmap_font, utils::fps_counter::FPSCounter};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

static ENABLED: AtomicBool = AtomicBool::new(cfg!(debug_assertions));
static COUNTER: Lazy<Mutex<FPSCounter>> = Lazy::new(|| Mutex::new(FPSCounter::new()));

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn draw(renderer: &gfc::UIRenderer) {
    let mut counter = COUNTER.lock();
    counter.tick(Instant::now());

    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let fps = match counter.fps() {
        Some(n) => format!("{} fps", n),
        None => return,
    };

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    // Assume a resolution of 1280x720
    bitmap_font::draw_string(renderer, 1168.0, 10.0, 2, &fps);

    renderer.end();
}
