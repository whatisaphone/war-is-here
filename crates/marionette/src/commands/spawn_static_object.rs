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
    let package_name = gfc::HString::from_str(&args.package_name);
    let object_name = gfc::HString::from_str(&args.object_name);

    let class = gfc::Singleton::<gfc::ClassRegistry>::get_instance()
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();
    let obj = class.new_instance();
    let obj = obj.as_ptr().cast::<target::gfc__StaticObject>();

    target::gfc__StaticObject__setPackageName(obj, package_name.as_ptr());
    target::gfc__StaticObject__setObjectName(obj, object_name.as_ptr());
    (*obj).setPosition(&target::gfc__TVector3_float_gfc__FloatMath_ {
        x: args.x,
        y: args.y,
        z: args.z,
    });
    (*obj).setScale(&target::gfc__TVector3_float_gfc__FloatMath_ {
        x: args.scale,
        y: args.scale,
        z: args.scale,
    });

    let world = gfc::OblivionGame::get_instance().get_world();
    (*obj).addObjectToWorld(world.as_ptr());
}
