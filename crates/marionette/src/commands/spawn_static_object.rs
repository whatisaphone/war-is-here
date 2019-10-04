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
    let package_name = words.next().ok_or(())?.to_string();
    let object_name = words.next().ok_or(())?.to_string();
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let scale = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args {
        package_name,
        object_name,
        x,
        y,
        z,
        scale,
    })
}

#[derive(Debug)]
struct Args {
    package_name: String,
    object_name: String,
    x: f32,
    y: f32,
    z: f32,
    scale: f32,
}

unsafe fn go(args: &Args) {
    // I have no idea what region or layer to use, so let's try them all!
    for r in 0..10 {
        for l in 0..10 {
            once(args, r, l);
        }
    }
}

unsafe fn once(args: &Args, region_id: u16, layer_id: u16) {
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders).mWorldMgr.p as *mut target::gfc__WorldManager;
    #[allow(clippy::cast_ptr_alignment)]
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let package_name = gfc::HString::from_str(&args.package_name);
    let object_name = gfc::HString::from_str(&args.object_name);

    let class_registry = *target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    let class = target::gfc__ClassRegistry__classForName(
        class_registry,
        hstring!("StaticObject").as_ptr(),
        true,
        false,
    );
    let class = gfc::Class::from_ptr(class);

    let obj = class.new_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.as_ptr() as *mut target::gfc__StaticObject;

    target::gfc__WorldObject__setRegionID((*obj).as_gfc__WorldObject_mut_ptr(), region_id);
    target::gfc__WorldObject__setLayerID((*obj).as_gfc__WorldObject_mut_ptr(), layer_id);
    target::gfc__StaticObject__setPackageName(obj, package_name.as_ptr());
    target::gfc__StaticObject__setObjectName(obj, object_name.as_ptr());
    target::gfc__StaticObject__setPosition(obj, &target::gfc__TVector3_float_gfc__FloatMath_ {
        x: args.x,
        y: args.y,
        z: args.z,
    });
    target::gfc__StaticObject__setScale(obj, &target::gfc__TVector3_float_gfc__FloatMath_ {
        x: args.scale,
        y: args.scale,
        z: args.scale,
    });

    ((*(*obj).__vfptr).addObjectToWorld)((*obj).as_gfc__WorldObject_mut_ptr(), world);
}
