use crate::{darksiders1::gfc, library::bitmap_font};
use parking_lot::Mutex;
use std::time::{Duration, Instant};

static STARTED: Mutex<Option<Instant>> = Mutex::new(None);

pub fn draw(renderer: &gfc::UIRenderer) {
    let mut guard = STARTED.lock();
    let now = Instant::now();
    let started = *guard.get_or_insert(now);

    let elapsed = now - started;
    let wait = Duration::from_millis(500);
    let text = match elapsed {
        e if e < wait * 2 => "war",
        e if e < wait * 3 => "",
        e if e < wait * 5 => "is",
        e if e < wait * 6 => "",
        e if e < wait * 8 => "here",
        _ => "",
    };

    if text == "" {
        return;
    }

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    bitmap_font::draw_string(renderer, 100.0, 100.0, 8, text);

    renderer.end();
}
