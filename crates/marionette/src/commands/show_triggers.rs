use crate::{
    darksiders1::{gfc, List},
    hooks::ON_POST_UPDATE_QUEUE,
    utils::mem::init_with,
};
use darksiders1_sys::target;
use std::convert::TryFrom;

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    let darksiders = gfc::OblivionGame::get_instance();
    let world = darksiders.get_world();

    #[allow(clippy::cast_ptr_alignment)]
    let root = (*world.as_ptr()).mRoot.p as *mut target::gfc__WorldGroup;
    scan(root);

    let region_data = gfc::Vector::<target::gfc__AutoRef_gfc__WorldRegionData_>::from_ptr(
        &mut (*world.as_ptr()).mRegionData,
    );
    for (r, _) in region_data.iter().enumerate() {
        let r = i32::try_from(r).unwrap();
        let region = init_with(|this| {
            target::gfc__World__getRegion(world.as_ptr(), this, r);
        });
        #[allow(clippy::cast_ptr_alignment)]
        let region = region.p as *mut target::gfc__WorldRegion;
        if region.is_null() {
            continue;
        }

        let layer_data = gfc::Vector::<target::gfc__AutoRef_gfc__RegionLayerData_>::from_ptr(
            &mut (*region).mLayerData,
        );
        for (l, _) in layer_data.iter().enumerate() {
            let l = i32::try_from(l).unwrap();
            let layer = init_with(|this| {
                target::gfc__WorldRegion__getLayer(region, this, l);
            });
            #[allow(clippy::cast_ptr_alignment)]
            let layer = layer.p as *mut target::gfc__RegionLayer;
            if layer.is_null() {
                continue;
            }

            let root = init_with(|this| {
                target::gfc__RegionLayer__getRoot(layer, this);
            });
            #[allow(clippy::cast_ptr_alignment)]
            let root = root.p as *mut target::gfc__WorldGroup;
            scan(root);
        }
    }
}

unsafe fn scan(group: *mut target::gfc__WorldGroup) {
    let objects = &mut (*group).mObjects;
    let objects = List::<target::gfc__AutoRef_gfc__WorldObject_>::from_ptr(objects);
    for (i, object) in objects.iter().enumerate() {
        let object = gfc::Object::from_ptr(object.p as *mut target::gfc__Object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            scan(group.as_ptr());
        }

        if let Some(_trigger) = gfc::object_safecast::<gfc::TriggerRegion>(object) {
            println!("{}: {:?}", i, object.class().name().c_str());
        }
    }
}
