use crate::{darksiders1::gfc, library::bitmap_font, utils::pretty::Pretty};
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(true);

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
    let enabled = ENABLED.load(Ordering::SeqCst);
    if !enabled {
        return;
    }

    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let pos = player.get_position();

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    let s = format!("pos: {}", Pretty(&pos));
    unsafe {
        bitmap_font::draw_string(renderer, 10.0, 10.0, 2, &s);
    }

    renderer.end();
}
