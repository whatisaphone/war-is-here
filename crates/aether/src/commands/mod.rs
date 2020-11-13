use std::str;

pub mod console;
pub mod editor_mode;
pub mod fps;
pub mod infinite_jump;
pub mod load_map_menu;
pub mod load_package;
pub mod log_events;
pub mod move_player;
pub mod pickup_item;
pub mod show_collision;
pub mod show_player_pos;
pub mod show_triggers;
pub mod spawn_humans;
pub mod spawn_objct;
pub mod spawn_static_object;
pub mod teleport;

pub enum RunResult {
    Ok,
    Response(&'static str),
    Shutdown,
}

// Hide "/shutdown" since it doesn't work via GUI
const HELP: &str = "\
Available commands:
/clear                /infinite_jump  /show_collision
/console              /load_map_menu  /show_player_pos
/draw_triggers        /load_package   /spawn_humans
/draw_triggers_round  /log_events     /spawn_object
/editor_mode          /mark_triggers  /spawn_static_object
/fps                  /move_player    /teleport
/help                 /pickup_item
";

pub const COMMANDS: &[&str] = &[
    "/clear",
    "/console",
    "/draw_triggers",
    "/draw_triggers_round",
    "/editor_mode",
    "/fps",
    "/help",
    "/infinite_jump",
    "/load_map_menu",
    "/load_package",
    "/log_events",
    "/mark_triggers",
    "/move_player",
    "/pickup_item",
    "/show_collision",
    "/show_player_pos",
    // Skip "/shutdown" since it doesn't work via GUI
    "/spawn_humans",
    "/spawn_object",
    "/spawn_static_object",
    "/teleport",
];

pub fn run(message: &[u8]) -> RunResult {
    let message = str::from_utf8(message).unwrap_or(":(");
    let command = message.split_ascii_whitespace().next().unwrap_or(":(");
    #[allow(clippy::unit_arg)]
    match command {
        "console" => console::run(message).into(),
        "draw_triggers" => show_triggers::run_draw(message).into(),
        "draw_triggers_round" => show_triggers::run_draw_round(message).into(),
        "editor_mode" => editor_mode::run(message).into(),
        "fps" => fps::run(message).into(),
        "help" => HELP.into(),
        "infinite_jump" => infinite_jump::run(message).into(),
        "load_map_menu" => load_map_menu::run(message).into(),
        "load_package" => load_package::run(message).into(),
        "log_events" => log_events::run(message).into(),
        "mark_triggers" => show_triggers::run_mark(message).into(),
        "move_player" => move_player::run(message).into(),
        "pickup_item" => pickup_item::run(message).into(),
        "show_collision" => show_collision::run(message).into(),
        "show_player_pos" => show_player_pos::run(message).into(),
        "shutdown" => RunResult::Shutdown,
        "spawn_humans" => spawn_humans::run(message).into(),
        "spawn_object" => spawn_objct::run(message).into(),
        "spawn_static_object" => spawn_static_object::run(message).into(),
        "teleport" => teleport::run(message).into(),
        _ => "invalid command".into(),
    }
}

impl From<()> for RunResult {
    fn from(_: ()) -> Self {
        Self::Ok
    }
}

impl From<&'static str> for RunResult {
    fn from(response: &'static str) -> Self {
        Self::Response(response)
    }
}

impl From<Result<(), &'static str>> for RunResult {
    fn from(result: Result<(), &'static str>) -> Self {
        match result {
            Ok(()) => Self::Ok,
            Err(message) => Self::Response(message),
        }
    }
}
