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
    let darksiders = gfc::Singleton::<gfc::Darksiders>::get_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders.as_ptr()).mWorldMgr.p as *mut target::gfc__WorldManager;
    #[allow(clippy::cast_ptr_alignment)]
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let classname = gfc::HString::from_str(&args.classname);

    let class_registry = *target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    let class =
        target::gfc__ClassRegistry__classForName(class_registry, classname.as_ptr(), true, false);
    let class = gfc::Class::from_ptr(class);

    let obj = class.new_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.as_ptr() as *mut target::gfc__KinematicActor;

    target::gfc__Actor__setPosition(
        (*obj).as_gfc__Actor_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ {
            x: args.x,
            y: args.y,
            z: args.z,
        },
    );

    ((*(*obj).__vfptr).addObjectToWorld)(
        (*(*obj).as_gfc__Actor_mut_ptr()).as_gfc__WorldObject_mut_ptr(),
        world,
    );
}
