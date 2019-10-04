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
        "show_triggers" => commands::show_triggers::run(message),
        "spawn_actor" => commands::spawn_actor::run(message),
        "spawn_static_object" => commands::spawn_static_object::run(message),
        "teleport" => commands::teleport::run(message),
        "shutdown" => return false,
        _ => {
            println!("unknown command");
        }
    }
    true
}
