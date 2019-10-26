#![allow(clippy::similar_names)]

use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};

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
    let classname = words.next().ok_or(())?.to_string();
    Ok(Args { classname })
}

struct Args {
    classname: String,
}

unsafe fn go(args: &Args) {
    let classname = gfc::HString::from_str(&args.classname);

    let class_registry = gfc::Singleton::<gfc::ClassRegistry>::get_instance();
    let class = match class_registry.class_for_name(&classname, true) {
        Some(class) => class,
        None => {
            println!("class not found");
            return;
        }
    };
    let item = class.new_instance();
    let item = match gfc::object_safecast::<gfc::Item>(&item) {
        Some(obj) => obj,
        None => {
            println!("object is not an Item");
            return;
        }
    };

    let darksiders = gfc::OblivionGame::get_instance();
    if let Some(player) = darksiders.get_player_actor() {
        player.pickup_item(item);
    }
}
