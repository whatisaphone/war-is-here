#![allow(clippy::similar_names)]

use crate::{
    darksiders1::gfc,
    hooks::ON_POST_UPDATE_QUEUE,
    utils::{ffi::lock_xadd, mem::init_with},
};
use darksiders1_sys::target;
use std::mem;

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
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders).mWorldMgr.p as *mut target::gfc__WorldManager;
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let classname = gfc::HString::from_str(&args.classname);

    let class_registry = *target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    let class =
        target::gfc__ClassRegistry__classForName(class_registry, classname.as_ref(), true, false);

    let obj = init_with(|this| {
        let new_instance = cast_away_pdbindgen_bug((*(*class).__vfptr).newInstance);
        new_instance(class, this);
    });
    lock_xadd(&mut (*obj.p).ReferenceCount, 1);
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.p as *mut target::gfc__KinematicActor;

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

type NewInstanceWrong =
    unsafe extern "thiscall" fn(this: *mut target::gfc__Class) -> target::gfc__AutoRef_gfc__Object_;
type NewInstanceRight = unsafe extern "thiscall" fn(
    this: *mut target::gfc__Class,
    result: *mut target::gfc__AutoRef_gfc__Object_,
) -> *mut target::gfc__AutoRef_gfc__Object_;

unsafe fn cast_away_pdbindgen_bug(new_instance: NewInstanceWrong) -> NewInstanceRight {
    mem::transmute(new_instance)
}
