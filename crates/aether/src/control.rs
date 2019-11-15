use crate::commands;
use std::{
    io,
    io::{BufRead, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

const RANDOM_PORT: u16 = 53508; // chosen by fair dice roll. guaranteed to be random.

pub fn start() {
    let server = TcpListener::bind((Ipv4Addr::LOCALHOST, RANDOM_PORT)).unwrap();

    while let Ok((client, _)) = server.accept() {
        let terminate = !handle_client(client);
        if terminate {
            break;
        }
    }
}

fn handle_client(mut client: TcpStream) -> bool {
    let mut reader = io::BufReader::new(client.try_clone().unwrap());

    let mut buf = Vec::with_capacity(32);
    match reader.read_until(b'\n', &mut buf) {
        Ok(n) if n > 0 => {}
        _ => return false,
    }

    let result = commands::run(&buf);
    match result {
        commands::RunResult::Ok => {
            let _ = client.write_all("\u{1f44d}\n".as_bytes()); // 👍
        }
        commands::RunResult::Response(response) => {
            let _ = client.write_all(response.as_bytes());
            let _ = client.write_all(b"\n");
        }
        commands::RunResult::Shutdown => return false,
    }
    true
}
