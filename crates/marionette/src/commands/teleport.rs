#![allow(clippy::similar_names)]

use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use darksiders1_sys::target;

pub fn run(command: &str) {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => {
            println!("parse error");
            return;
        }
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go(&args) }));
}

fn parse(command: &str) -> Result<Args, ()> {
    let mut words = command.split_ascii_whitespace();
    words.next().ok_or(())?;
    let world = words.next().ok_or(())?.to_string();
    let start_region = words.next().ok_or(())?.to_string();
    let start_point = words.next().unwrap_or("Start").to_string();
    let start_load_region = words.next().unwrap_or("StartLoadRegion").to_string();
    Ok(Args {
        world,
        start_region,
        start_point,
        start_load_region,
    })
}

struct Args {
    world: String,
    start_region: String,
    start_point: String,
    start_load_region: String,
}

unsafe fn go(args: &Args) {
    let teleport_helper = gfc::Singleton::<gfc::TeleportHelper>::get_instance();
    let world = gfc::HString::from_str(&args.world);
    let start_region = gfc::HString::from_str(&args.start_region);
    let start_point = gfc::HString::from_str(&args.start_point);
    let start_load_region = gfc::HString::from_str(&args.start_load_region);
    target::gfc__TeleportHelper__warpToMap(
        teleport_helper.as_ptr(),
        world.as_ptr(),
        start_region.as_ptr(),
        start_point.as_ptr(),
        start_load_region.as_ptr(),
    );
}
