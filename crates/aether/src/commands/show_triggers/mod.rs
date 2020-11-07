use crate::{hooks::ON_POST_UPDATE_QUEUE, ui};
use std::sync::atomic::{AtomicBool, Ordering};

mod shape;
mod via_immediate;
mod via_world_objects;
mod walk;

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    if toggle_enabled() {
        "now set to true"
    } else {
        "now set to false"
    }
}

fn toggle_enabled() -> bool {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    // `ui` is what does the drawing, so it must be enabled if we are.
    let offset = if enabled { 1 } else { -1 };
    ui::WANT_ENABLED.fetch_add(offset, Ordering::SeqCst);

    if enabled {
        let mut guard = ON_POST_UPDATE_QUEUE.lock();
        guard
            .as_mut()
            .unwrap()
            .push_back(Box::new(via_world_objects::draw));
    }

    enabled
}

pub fn disable() {
    let prev_enabled = ENABLED.fetch_and(false, Ordering::SeqCst);
    if prev_enabled {
        ui::WANT_ENABLED.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn draw(ui: &imgui::Ui<'_>) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    via_immediate::draw(ui);
}
