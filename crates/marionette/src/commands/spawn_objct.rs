#![allow(clippy::similar_names)]

use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use na::Point3;

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
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { classname, x, y, z })
}

struct Args {
    classname: String,
    x: f32,
    y: f32,
    z: f32,
}

unsafe fn go(args: &Args) {
    let classname = gfc::HString::from_str(&args.classname);

    let class = match gfc::Singleton::<gfc::ClassRegistry>::get_instance()
        .class_for_name(&classname, true)
    {
        Some(class) => class,
        None => {
            println!("class not found");
            return;
        }
    };
    let obj = class.new_instance();
    let obj = match gfc::object_safecast::<gfc::WorldObject>(&obj) {
        Some(obj) => obj,
        None => {
            println!("object is not a WorldObject");
            return;
        }
    };

    obj.set_position(&Point3::new(args.x, args.y, args.z));

    if let Some(world) = gfc::OblivionGame::get_instance().get_world() {
        obj.add_object_to_world(world);
    }
}
