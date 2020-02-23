#![allow(clippy::similar_names)]

use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};
use na::Point3;
use once_cell::sync::Lazy;
use std::slice;

pub fn run(command: &str) -> Result<(), &'static str> {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => return Err("parse error"),
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || go(&args)));
    Ok(())
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

pub(super) struct Args {
    pub classname: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub(super) fn go(args: &Args) {
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

    if let Some(sc) = as_script_class(class) {
        // Load the package even if it's already loaded. This is just sheer laziness on
        // my part.
        let package_id = unsafe { (*sc.as_ptr()).mPackageID };
        let resource_manager = <gfc::Singleton<gfc::ResourceManager>>::get_instance();
        let package_id = resource_manager.get_permanent_id(package_id);
        resource_manager.load_packages(slice::from_ref(&package_id), false);
        println!("loading package {}", package_id);
    }

    obj.set_position(&Point3::new(args.x, args.y, args.z));

    if let Some(world) = gfc::OblivionGame::get_instance().get_world() {
        obj.add_object_to_world(&world);
    }
}

// Major hack! I would rather pdbindgen be able to bind to the vftable directly.
static SCRIPTCLASS_VFTABLE: Lazy<usize> = Lazy::new(|| {
    let dummy = gfc::ScriptClass::new(&hstring!("dummy"), -1, None);
    unsafe { (*dummy.as_ptr()).vfptr as usize }
});

fn as_script_class(class: &gfc::Class) -> Option<&gfc::ScriptClass> {
    // Hack! We should be using actual RTTI instead of comparing the vfptr.
    unsafe {
        if (*class.as_ptr()).vfptr as usize == *SCRIPTCLASS_VFTABLE {
            return Some(gfc::ScriptClass::from_ptr(class.as_ptr().cast()));
        }
    }
    None
}
