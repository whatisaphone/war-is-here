use std::{borrow::Cow, str};

pub mod console;
pub mod dump_hstrings;
pub mod editor_mode;
pub mod fps;
pub mod infinite_jump;
pub mod load_map_menu;
pub mod load_package;
pub mod log_events;
pub mod move_player;
pub mod pickup_item;
pub mod show_bird_kills;
pub mod show_collision;
pub mod show_player_pos;
pub mod show_triggers;
pub mod spawn_humans;
pub mod spawn_objct;
pub mod spawn_static_object;
pub mod teleport;

pub enum RunResult {
    Ok,
    Response(Cow<'static, str>),
    Shutdown,
}

// Hide "/draw_triggers_round" since the /draw_triggers subcommand is preferred.
// Hide "/shutdown" since it doesn't work via GUI
const HELP: &str = "\
Available commands:
/clear          /load_map_menu    /show_collision
/console        /load_package     /show_player_pos
/draw_triggers  /log_events       /spawn_humans
/editor_mode    /mark_triggers    /spawn_object
/fps            /move_player      /spawn_static_object
/help           /pickup_item      /teleport
/infinite_jump  /show_bird_kills
";

pub const COMMANDS: &[&str] = &[
    "/clear",
    "/console",
    "/draw_triggers",
    "/draw_triggers_round",
    "/dump_hstrings",
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
    "/show_bird_kills",
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
        "draw_triggers" => show_triggers::run(message).into(),
        "draw_triggers_round" => show_triggers::run_toggle_round(message).into(),
        "dump_hstrings" => dump_hstrings::run(message).into(),
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
        "show_bird_kills" => show_bird_kills::run(message).into(),
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
        Self::Response(response.into())
    }
}

impl From<Cow<'static, str>> for RunResult {
    fn from(response: Cow<'static, str>) -> Self {
        Self::Response(response)
    }
}

impl From<Result<(), &'static str>> for RunResult {
    fn from(result: Result<(), &'static str>) -> Self {
        match result {
            Ok(()) => Self::Ok,
            Err(message) => Self::Response(message.into()),
        }
    }
}
