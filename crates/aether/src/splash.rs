use crate::{
    darksiders1::{gfc, LoweredAutoRef},
    library::bitmap_font,
};
use darksiders1_sys::target;
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

    unsafe {
        target::gfc__UIRenderer__begin(renderer.as_ptr(), true);
        target::gfc__UIRenderer__setMaterial(
            renderer.as_ptr(),
            (*renderer.as_ptr()).mSolidMaterial.ptr(),
        );
    }

    unsafe {
        bitmap_font::draw_string(renderer, 100.0, 100.0, 8, text);
    }

    unsafe {
        target::gfc__UIRenderer__endRendering(renderer.as_ptr());
    }
}
