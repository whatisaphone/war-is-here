#![allow(clippy::similar_names)]

use crate::utils::ffi::lock_xadd;
use darksiders1_sys::target;
use std::{ffi::CString, mem};

pub fn run(command: &str) {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => {
            println!("parse error");
            return;
        }
    };
    unsafe {
        go(&args);
    }
}

fn parse(command: &str) -> Result<Args<'_>, ()> {
    println!("{:?}", command);
    let mut words = command.split_ascii_whitespace();
    words.next().ok_or(())?;
    let classname = words.next().ok_or(())?;
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { classname, x, y, z })
}

struct Args<'a> {
    classname: &'a str,
    x: f32,
    y: f32,
    z: f32,
}

unsafe fn go(args: &Args<'_>) {
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders).mWorldMgr.p as *mut target::gfc__WorldManager;
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let classname_cstring = CString::new(args.classname).unwrap();
    let mut classname_hstring = mem::MaybeUninit::uninit();
    target::gfc__HString__HString_3(
        classname_hstring.as_mut_ptr(),
        classname_cstring.as_ptr() as *const i8,
        true,
    );
    let classname_hstring = classname_hstring.assume_init();

    let class_registry = *target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    let class =
        target::gfc__ClassRegistry__classForName(class_registry, &classname_hstring, true, false);

    let mut obj = mem::MaybeUninit::uninit();
    let new_instance = cast_away_pdbindgen_bug((*(*class).__vfptr).newInstance);
    new_instance(class, obj.as_mut_ptr());
    let obj = obj.assume_init();
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
