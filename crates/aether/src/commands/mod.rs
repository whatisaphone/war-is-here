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
/clear          /load_map_menu    /show_player_pos
/console        /load_package     /show_triggers
/editor_mode    /log_events       /spawn_humans
/fps            /move_player      /spawn_object
/help           /pickup_item      /spawn_static_object
/infinite_jump  /show_collision   /teleport
";

pub const COMMANDS: &[&str] = &[
    "/clear",
    "/console",
    "/editor_mode",
    "/fps",
    "/help",
    "/infinite_jump",
    "/load_map_menu",
    "/load_package",
    "/log_events",
    "/move_player",
    "/pickup_item",
    "/show_collision",
    "/show_player_pos",
    "/show_triggers",
    // Skip "/shutdown" since it doesn't work via GUI
    "/spawn_humans",
    "/spawn_object",
    "/spawn_static_object",
    "/teleport",
];

pub fn run(message: &[u8]) -> RunResult {
    let message = str::from_utf8(message).unwrap_or(":(");
    let command = message.split_ascii_whitespace().next().unwrap_or(":(");
    match command {
        "console" => RunResult::Response(console::run(message)),
        "editor_mode" => RunResult::Response(editor_mode::run(message)),
        "fps" => RunResult::Response(fps::run(message)),
        "help" => RunResult::Response(HELP),
        "infinite_jump" => RunResult::Response(infinite_jump::run(message)),
        "load_map_menu" => {
            load_map_menu::run(message);
            RunResult::Ok
        }
        "load_package" => load_package::run(message).into(),
        "log_events" => RunResult::Response(log_events::run(message)),
        "move_player" => move_player::run(message).into(),
        "pickup_item" => pickup_item::run(message).into(),
        "show_collision" => RunResult::Response(show_collision::run(message)),
        "show_player_pos" => RunResult::Response(show_player_pos::run(message)),
        "show_triggers" => RunResult::Response(show_triggers::run(message)),
        "shutdown" => RunResult::Shutdown,
        "spawn_humans" => spawn_humans::run(message).into(),
        "spawn_object" => spawn_objct::run(message).into(),
        "spawn_static_object" => spawn_static_object::run(message).into(),
        "teleport" => teleport::run(message).into(),
        _ => RunResult::Response("invalid command"),
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
