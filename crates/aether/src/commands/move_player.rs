use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use na::Point3;

pub fn run(command: &str) -> Result<(), &'static str> {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => return Err("/move_player [by] <x> <y> <z>"),
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || go(&args)));
    Ok(())
}

fn parse(command: &str) -> Result<Args, ()> {
    let mut words = command.split_ascii_whitespace().peekable();
    words.next().ok_or(())?;
    let relative = words.peek() == Some(&"by");
    if relative {
        words.next();
    }
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { relative, x, y, z })
}

#[derive(Debug)]
struct Args {
    relative: bool,
    x: f32,
    y: f32,
    z: f32,
}

fn go(args: &Args) {
    let darksiders = gfc::OblivionGame::get_instance();
    let player = match darksiders.get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let mut pos = if args.relative {
        player.get_position()
    } else {
        Point3::origin()
    };
    pos.x += args.x;
    pos.y += args.y;
    pos.z += args.z;
    player.set_position(&pos);
}
