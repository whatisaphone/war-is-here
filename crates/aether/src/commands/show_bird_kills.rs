use crate::{darksiders1::gfc, library::bitmap_font};
use std::{
    convert::TryInto,
    sync::atomic::{AtomicBool, Ordering},
};

static ENABLED: AtomicBool = AtomicBool::new(false);

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
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let darksiders = <gfc::Singleton<gfc::Darksiders>>::get_instance();
    let player = match darksiders.get_player_actor() {
        Some(player) => player,
        None => return,
    };

    // Only show the counter during the relevant level
    let world = match darksiders.get_world() {
        Some(world) => world,
        None => return,
    };
    let region_name = hstring!("CI_BirdPath");
    let in_bird_path = world
        .region_data()
        .iter()
        .enumerate()
        .flat_map(|(r, _)| world.get_region(r.try_into().unwrap()))
        .any(|r| r.name() == &region_name);
    if !in_bird_path {
        return;
    }

    let viewport = gfc::KGGraphics::get_instance().get_viewport();

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    bitmap_font::draw_string(
        renderer,
        (viewport.width() - 320) as f32,
        120.0,
        2,
        &format!("Kills: {}", player.stat_tracker().number_of_kills_on_bird()),
    );

    renderer.end();
}
