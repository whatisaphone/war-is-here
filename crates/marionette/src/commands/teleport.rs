#![allow(clippy::similar_names)]

use crate::{darksiders1::gfc, hooks::GOD_LOCK};
use darksiders1_sys::target;

pub fn run(command: &str) {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => {
            println!("parse error");
            return;
        }
    };
    let mut guard = GOD_LOCK.lock();
    guard
        .as_mut()
        .unwrap()
        .on_post_update_queue
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
    let teleport_helper = *target::gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    let world = gfc::HString::from_str(&args.world);
    let start_region = gfc::HString::from_str(&args.start_region);
    let start_point = gfc::HString::from_str(&args.start_point);
    let start_load_region = gfc::HString::from_str(&args.start_load_region);
    target::gfc__TeleportHelper__warpToMap(
        teleport_helper,
        world.as_ref(),
        start_region.as_ref(),
        start_point.as_ref(),
        start_load_region.as_ref(),
    );
}
