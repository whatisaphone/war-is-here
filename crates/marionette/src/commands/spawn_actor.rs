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

    let class = gfc::Singleton::<gfc::ClassRegistry>::get_instance()
        .class_for_name(&classname, true)
        .unwrap();
    let obj = class.new_instance();
    let obj = obj.as_ptr().cast::<target::gfc__KinematicActor>();

    ((*(*obj).vfptr).setPosition)(obj, &target::gfc__TVector3_float_gfc__FloatMath_ {
        x: args.x,
        y: args.y,
        z: args.z,
    });

    let world = gfc::OblivionGame::get_instance().get_world();
    ((*(*obj).vfptr).addObjectToWorld)(obj, world.as_ptr());
}
