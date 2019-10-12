use crate::{
    darksiders1::{gfc, Lift, Lift1, List, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    utils::mem::init_with,
};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::convert::TryFrom;

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    let world = gfc::OblivionGame::get_instance().get_world();

    let root = (*world.as_ptr()).mRoot.lift_ref();
    scan(root);

    let region_data = (*world.as_ptr()).mRegionData.lift1_ref();
    for (r, _) in region_data.iter().enumerate() {
        let r = i32::try_from(r).unwrap();
        let region = init_with(|p| {
            target::gfc__World__getRegion(world.as_ptr(), p, r);
        });
        let region = region.ptr();
        if region.is_null() {
            continue;
        }

        let layer_data = (*region).mLayerData.lift1_ref();
        for (l, _) in layer_data.iter().enumerate() {
            let l = i32::try_from(l).unwrap();
            let layer = init_with(|p| {
                target::gfc__WorldRegion__getLayer(region, p, l);
            });
            let layer = layer.ptr();
            if layer.is_null() {
                continue;
            }

            let root = init_with(|p| {
                target::gfc__RegionLayer__getRoot(layer, p);
            });
            let root = root.lift();
            scan(&root);
        }
    }
}

unsafe fn scan(group: &gfc::WorldGroup) {
    let objects = &mut (*group.as_ptr()).mObjects;
    let objects = List::<target::gfc__AutoRef_gfc__WorldObject_>::from_ptr(objects);
    for object in objects {
        let object = object.lift_ref();

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            scan(&group);
        }

        if let Some(trigger) = gfc::object_safecast::<gfc::TriggerRegion>(object) {
            let position = init_with(|p| {
                (*trigger.as_ptr()).getPosition(p);
            });
            mark(
                (*trigger.as_ptr()).mRegionID,
                (*trigger.as_ptr()).mLayerID,
                position.x,
                position.y,
                position.z,
            );
        }
    }
}

unsafe fn mark(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
    let class = gfc::Singleton::<gfc::ClassRegistry>::get_instance()
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();
    let obj = class.new_instance();
    let obj = obj.as_ptr().cast::<target::gfc__StaticObject>();

    target::gfc__WorldObject__setRegionID(obj.static_cast(), region_id);
    target::gfc__WorldObject__setLayerID(obj.static_cast(), layer_id);
    target::gfc__StaticObject__setPackageName(obj, hstring!("vfx_shared").as_ptr());
    target::gfc__StaticObject__setObjectName(obj, hstring!("sphere").as_ptr());
    (*obj).setPosition(&target::gfc__TVector3_float_gfc__FloatMath_ { x, y, z });

    let world = gfc::OblivionGame::get_instance().get_world();
    (*obj).addObjectToWorld(world.as_ptr());
}
