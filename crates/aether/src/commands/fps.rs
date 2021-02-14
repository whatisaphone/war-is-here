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

#[allow(clippy::cast_precision_loss)]
pub fn draw(renderer: &gfc::UIRenderer) {
    let mut counter = COUNTER.lock();
    counter.tick(Instant::now());

    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let text = match counter.fps() {
        Some(n) => format!("{} fps", n),
        None => return,
    };

    let viewport = gfc::KGGraphics::get_instance().get_viewport();

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    bitmap_font::draw_string(renderer, (viewport.width() - 128) as f32, 10.0, 2, &text);

    renderer.end();
}
