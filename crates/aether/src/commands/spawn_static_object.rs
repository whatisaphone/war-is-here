use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use na::{Point3, Vector3};

pub fn run(command: &str) -> Result<(), &'static str> {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => return Err("parse error"),
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go(&args) }));
    Ok(())
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

    let obj = gfc::AutoRef::new(gfc::StaticObject::new());

    obj.set_package_name(&package_name);
    obj.set_object_name(&object_name);
    obj.set_position(&Point3::new(args.x, args.y, args.z));
    obj.set_scale(&Vector3::new(args.scale, args.scale, args.scale));

    if let Some(world) = gfc::OblivionGame::get_instance().get_world() {
        obj.add_object_to_world(world);
    }
}
