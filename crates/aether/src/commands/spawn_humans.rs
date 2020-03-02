use crate::commands::spawn_objct;
use na::Point3;
use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub fn run(command: &str) -> Result<(), &'static str> {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => return Err("/spawn_humans <x> <y> <z>"),
    };

    *STATE.lock() = Some(State {
        remaining: 150,
        next_time: Instant::now(),
        position: Point3::new(args.x, args.y, args.z),
    });
    Ok(())
}

fn parse(command: &str) -> Result<Args, ()> {
    let mut words = command.split_ascii_whitespace();
    words.next().ok_or(())?;
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { x, y, z })
}

struct Args {
    x: f32,
    y: f32,
    z: f32,
}

const WAIT: Duration = Duration::from_millis(200);

static STATE: Mutex<Option<State>> = Mutex::new(None);

struct State {
    remaining: i32,
    next_time: Instant,
    position: Point3<f32>,
}

pub fn pump() {
    let mut guard = STATE.lock();
    let state = match *guard {
        Some(ref mut s) => s,
        None => return,
    };
    let now = Instant::now();
    if state.remaining > 0 && now >= state.next_time {
        state.remaining -= 1;
        state.next_time = now + WAIT;
        spawn_human(&state.position);
    }
}

fn spawn_human(position: &Point3<f32>) {
    let mut rng = thread_rng();

    let classname = match rng.gen_range(0, 3) {
        0 => "mayhem_cop/mayhem_cop",
        1 => "crowdfemale1/crowdfemale",
        2 => "crowdmale1/crowdmale",
        _ => unreachable!(),
    };

    spawn_objct::go(&spawn_objct::Args {
        classname: classname.to_string(),
        x: position.x + rng.gen_range(-100.0, 100.0),
        y: position.y + rng.gen_range(-100.0, 100.0),
        z: position.z + rng.gen_range(-50.0, 50.0),
    });
}
