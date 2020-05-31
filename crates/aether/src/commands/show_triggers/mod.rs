use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

mod shape;
mod via_immediate;
mod via_world_objects;
mod walk;

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = ENABLED.fetch_nand(true, SeqCst);
    let enabled = !prev_enabled;

    if enabled {
        let mut guard = ON_POST_UPDATE_QUEUE.lock();
        guard
            .as_mut()
            .unwrap()
            .push_back(Box::new(via_world_objects::draw));
    }

    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn draw(renderer: &gfc::UIRenderer) {
    if !ENABLED.load(SeqCst) {
        return;
    }

    via_immediate::draw(renderer);
}
