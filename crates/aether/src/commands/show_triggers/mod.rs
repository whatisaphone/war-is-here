use crate::{commands::show_triggers::via_immediate::DrawOptions, hooks::ON_POST_UPDATE_QUEUE, ui};
use std::{
    borrow::Cow,
    sync::atomic::{AtomicBool, Ordering},
};

mod shape;
mod via_immediate;
mod via_world_objects;
mod walk;

static ENABLED: AtomicBool = AtomicBool::new(false);
static DRAW_CYLINDERS_SPHERES: AtomicBool = AtomicBool::new(true);
static DRAW_ALL_LOAD_REGIONS: AtomicBool = AtomicBool::new(false);

pub fn run(command: &str) -> Cow<'static, str> {
    match parse(command) {
        Command::ToggleDraw => {
            toggle_enabled();
            format_status().into()
        }
        Command::ToggleRound => {
            DRAW_CYLINDERS_SPHERES.fetch_nand(true, Ordering::SeqCst);
            format_status().into()
        }
        Command::ToggleAllLoadRegions => {
            DRAW_ALL_LOAD_REGIONS.fetch_nand(true, Ordering::SeqCst);
            format_status().into()
        }
        Command::Usage => {
            "\
Subcommands:
/draw_triggers
/draw_triggers round
/draw_triggers all_load_regions"
                .into()
        }
    }
}

fn parse(command: &str) -> Command {
    let mut words = command.split_ascii_whitespace();
    words.next().unwrap(); // skip command name
    match words.next() {
        None => Command::ToggleDraw,
        Some("round") => Command::ToggleRound,
        Some("all_load_regions") => Command::ToggleAllLoadRegions,
        Some(_) => Command::Usage,
    }
}

enum Command {
    ToggleDraw,
    ToggleRound,
    ToggleAllLoadRegions,
    Usage,
}

pub fn run_toggle_round(_command: &str) -> &'static str {
    let prev_enabled = DRAW_CYLINDERS_SPHERES.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

fn format_status() -> String {
    format!(
        "Drawing: {}
Cylinders/spheres: {}
All load regions: {}",
        ENABLED.load(Ordering::SeqCst),
        DRAW_CYLINDERS_SPHERES.load(Ordering::SeqCst),
        DRAW_ALL_LOAD_REGIONS.load(Ordering::SeqCst)
    )
}

pub fn run_mark(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(via_world_objects::draw));
}

fn toggle_enabled() -> bool {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    // `ui` is what does the drawing, so it must be enabled if we are.
    let offset = if enabled { 1 } else { -1 };
    ui::WANT_ENABLED.fetch_add(offset, Ordering::SeqCst);

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

    via_immediate::draw(ui, DrawOptions {
        draw_cylinders_spheres: DRAW_CYLINDERS_SPHERES.load(Ordering::SeqCst),
        all_load_regions: DRAW_ALL_LOAD_REGIONS.load(Ordering::SeqCst),
    });
}
