use crate::commands;
use std::{
    net::{Ipv4Addr, UdpSocket},
    str,
};

pub fn start() {
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 12345)).unwrap();

    let mut buf = vec![0; 4096];
    while let Ok(bytes) = socket.recv(&mut buf) {
        let message = &buf[..bytes];
        if !handle_message(message) {
            break;
        }
    }
}

fn handle_message(message: &[u8]) -> bool {
    let message = str::from_utf8(message).unwrap_or(":(");
    let command = message.split_ascii_whitespace().next().unwrap_or(":(");
    match command {
        "load_map_menu" => commands::load_map_menu::run(message),
        "move_player" => commands::move_player::run(message),
        "pickup_item" => commands::pickup_item::run(message),
        "pretend_editor" => commands::pretend_editor::run(message),
        "show_collision" => commands::show_collision::run(message),
        "show_triggers" => commands::show_triggers::run(message),
        "shutdown" => return false,
        "spawn_humans" => commands::spawn_humans::run(message),
        "spawn_object" => commands::spawn_objct::run(message),
        "spawn_static_object" => commands::spawn_static_object::run(message),
        "teleport" => commands::teleport::run(message),
        _ => {
            println!("unknown command");
        }
    }
    true
}
