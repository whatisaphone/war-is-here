use crate::{darksiders1::gfc, ui};
use imgui::{im_str, ImStr, ImString};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use scopeguard::guard;
use std::{
    cell::Cell,
    collections::VecDeque,
    convert::TryInto,
    sync::atomic::{AtomicBool, Ordering},
};

// TODO: don't hardcode screen size
const WINDOW_LEFT: f32 = 0.0;
const WINDOW_TOP: f32 = 480.0;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 240.0;

const TIMESTAMP_COLOR: [f32; 4] = [1.0, 0.75, 0.0, 1.0];

static ENABLED: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::new()));

pub fn run(_command: &str) -> &'static str {
    if toggle_enabled() {
        STATE.lock().log(im_str!("event logging enabled"));
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn disable() {
    let prev_enabled = ENABLED.fetch_and(false, Ordering::SeqCst);
    if prev_enabled {
        ui::WANT_ENABLED.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn toggle_enabled() -> bool {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    // `ui` is what draws the event log, so it must be enabled if the console is.
    let offset = if enabled { 1 } else { -1 };
    ui::WANT_ENABLED.fetch_add(offset, Ordering::SeqCst);

    enabled
}

pub fn draw(ui: &imgui::Ui<'_>) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    imgui_token_guard! {
        ui.push_style_color(imgui::StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.0]);
    }

    // Bit of a hack here to prevent flickering when removing entries.
    //
    // imgui's scrolling methods seem to use the previous frame's measurements
    // instead of the current frame, so naively deleting an item would cause a
    // flicker of empty space. We can prevent the flicker by setting the new scroll
    // position a frame prior to actually modifying the list.
    let num_old_entries = state.count_prune();
    if num_old_entries != 0 {
        state.need_scroll.set(true);
    }
    let scroll_pos = if state.need_scroll.get() {
        Some((state.entries.len() - num_old_entries).saturating_sub(1))
    } else {
        None
    };

    imgui::Window::new(im_str!("Event Log"))
        .position([WINDOW_LEFT, WINDOW_TOP], imgui::Condition::Always)
        .size([WINDOW_WIDTH, WINDOW_HEIGHT], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .build(ui, || {
            // Force entries to appear at the bottom of the window.
            imgui::ChildWindow::new("spacer")
                .size([0.0, 0.0])
                .build(ui, || {});

            for (i, entry) in state.entries.iter().enumerate() {
                draw_text_shadow(ui, &entry.timestamp_text);
                ui.text_colored(TIMESTAMP_COLOR, &entry.timestamp_text);
                ui.same_line(0.0);
                draw_text_shadow(ui, &entry.text);
                ui.text(&entry.text);
                if Some(i) == scroll_pos {
                    ui.set_scroll_here_y_with_ratio(1.0);
                    state.need_scroll.set(false);
                }
            }
        });

    // Prune the list _after_ drawing to prevent flicker, as per above comment.
    state.entries.drain(..num_old_entries);
}

/// Draws a text shadow without affecting the cursor position. Does not draw the
/// actual text.
fn draw_text_shadow(ui: &imgui::Ui<'_>, text: &ImStr) {
    let [x, y] = ui.cursor_pos();

    ui.set_cursor_pos([x + 2.0, y + 2.0]);
    ui.text_colored([0.0, 0.0, 0.0, 1.0], &text);

    ui.set_cursor_pos([x + 1.0, y + 1.0]);
    ui.text_colored([0.0, 0.0, 0.0, 1.0], &text);

    ui.set_cursor_pos([x, y]);
}

struct State {
    entries: VecDeque<Entry>,
    need_scroll: Cell<bool>,
}

impl State {
    fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            need_scroll: Cell::new(false),
        }
    }

    fn log(&mut self, text: impl Into<ImString>) {
        let darksiders = <gfc::Singleton<gfc::Darksiders>>::get_instance();
        let timestamp = darksiders.get_player_actor().map_or(0.0, |player| {
            player.stat_tracker().total_game_time_precise()
        });
        let entry = Entry::new(timestamp, text.into());
        self.entries.push_back(entry);
        self.need_scroll.set(true);
    }

    fn count_prune(&mut self) -> usize {
        let limit = now() - 5.0;
        self.entries
            .iter()
            .take_while(|e| e.timestamp < limit)
            .count()
    }
}

fn now() -> f32 {
    let darksiders = <gfc::Singleton<gfc::Darksiders>>::get_instance();
    let player = match darksiders.get_player_actor() {
        Some(x) => x,
        None => return 0.0,
    };
    player.stat_tracker().total_game_time_precise()
}

struct Entry {
    timestamp: f32,
    timestamp_text: ImString,
    text: ImString,
}

impl Entry {
    fn new(timestamp: f32, text: impl Into<ImString>) -> Self {
        Self {
            timestamp,
            timestamp_text: format_timestamp(timestamp).into(),
            text: text.into(),
        }
    }
}

fn format_timestamp(timestamp: f32) -> String {
    #[allow(clippy::cast_possible_truncation)]
    let total_ms = (timestamp * 1000.0) as i32;
    let ms = total_ms % 1000;
    let total_sec = total_ms / 1000;
    let sec = total_sec % 60;
    let total_min = total_sec / 60;
    let min = total_min % 60;
    let total_hours = total_min / 60;
    format!("{}:{:02}:{:02}.{:03}", total_hours, min, sec, ms)
}

pub fn hook_debugoutmodule_execute(module: &gfc::DebugOutModule, actionid: u32) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    match actionid.try_into() {
        Ok(gfc::DebugOutModule__Actions::In) => {
            // Loosely based on the real `gfc::DebugOutModule::execute`. It throws the
            // string away without using it (probably `#ifdef`-ed away) so I recreated the
            // logic.
            let mut message = String::with_capacity(64);
            message.push_str("DebugOutModule: ");
            message.push_str(
                module
                    .message()
                    .c_str()
                    .to_str()
                    .unwrap_or("<invalid utf-8>"),
            );
            if module.has_variable_in(gfc::DebugOutModule__Variables::Values.into()) {
                message.push_str(": ( TODO connected variables )");
            }

            state.log(message);
        }
        _ => {}
    }
}

pub fn hook_detectorregion_body_entered(detector: &gfc::DetectorRegion, body: &gfc::Body) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    let object = match body.object() {
        Some(o) => o,
        None => return,
    };
    let object_name = object.world_object().get_name();
    if object_name == &hstring!("War") {
        state.log(format!(
            "{:?} entered region {:?}",
            object_name.c_str(),
            detector.owner().get_name().c_str(),
        ));
    }
}

pub fn hook_detectorregion_body_exited(detector: &gfc::DetectorRegion, body: &gfc::Body) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    let object = match body.object() {
        Some(o) => o,
        None => return,
    };
    let object_name = object.world_object().get_name();
    if object_name == &hstring!("War") {
        state.log(format!(
            "{:?} exited region {:?}",
            object_name.c_str(),
            detector.owner().get_name().c_str(),
        ));
    }
}

pub fn hook_dssavegamemanager_save_game() {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();
    state.log(im_str!("save game"));
}

pub fn hook_insrun_do_print(run: &gfc::InsRun) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    // Peek the value by temporarily popping.
    let val = run.stack().pop();
    let val = guard(val, |val| run.stack().push(val));

    let pr = val.get_string();
    state.log(format!(
        "print: {}",
        pr.c_str().to_str().unwrap_or("<invalid utf-8>"),
    ));
}

pub fn hook_player_set_spawn_point(
    world: &gfc::HString,
    region: &gfc::HString,
    spawn_point: &gfc::HString,
    spawn_load_region: &gfc::HString,
) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();
    state.log(format!(
        "setSpawnPoint {:?} {:?} {:?} {:?}",
        world.c_str(),
        region.c_str(),
        spawn_point.c_str(),
        spawn_load_region.c_str(),
    ));
}

pub fn hook_savedata_set_value(key: &gfc::HString, value: &gfc::HString) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();
    state.log(format!(
        "set save value {:?} to {:?}",
        key.c_str(),
        value.c_str(),
    ));
}

pub fn hook_worldregion_preload(region: &gfc::WorldRegion) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let mut state = STATE.lock();

    state.log(format!(
        "Finished loading WorldRegion {:?}",
        region.name().c_str(),
    ));
}
