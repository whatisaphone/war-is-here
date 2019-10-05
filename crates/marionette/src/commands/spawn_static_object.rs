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

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
unsafe fn go(args: &Args) {
    for x in (args.x as i32 - 50..=args.x as i32 + 50).step_by(50) {
        for y in (args.y as i32 - 50..=args.y as i32 + 50).step_by(50) {
            once(args, x as f32, y as f32, args.z);
        }
    }
}

unsafe fn once(args: &Args, x: f32, y: f32, z: f32) {
    let package_name = gfc::HString::from_str(&args.package_name);
    let object_name = gfc::HString::from_str(&args.object_name);

    let darksiders = gfc::OblivionGame::get_instance();
    let world = darksiders.get_world();

    let class_registry = gfc::Singleton::<gfc::ClassRegistry>::get_instance();
    let class = class_registry
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();

    let obj = class.new_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.as_ptr() as *mut target::gfc__StaticObject;

    target::gfc__StaticObject__setPackageName(obj, package_name.as_ptr());
    target::gfc__StaticObject__setObjectName(obj, object_name.as_ptr());
    ((*(*obj).__vfptr).setPosition)(
        (*obj).as_gfc__WorldObject_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ { x, y, z },
    );
    ((*(*obj).__vfptr).setScale)(
        (*obj).as_gfc__WorldObject_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ {
            x: args.scale,
            y: args.scale,
            z: args.scale,
        },
    );
    ((*(*obj).__vfptr).preload)((*obj).as_gfc__WorldObject_mut_ptr());
    if x >= -4000.0 {
        let object_3d: *mut target::gfc__Object3D = (*obj).mObject.p as *mut target::gfc__Object3D;
        (*object_3d).mVisuals.mSize = 0;
    }

    ((*(*obj).__vfptr).addObjectToWorld)((*obj).as_gfc__WorldObject_mut_ptr(), world.as_ptr());
}
