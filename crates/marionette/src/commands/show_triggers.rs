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
    for object in objects {
        let object = gfc::Object::from_ptr(object.p as *mut target::gfc__Object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            scan(group.as_ptr());
        }

        if let Some(trigger) = gfc::object_safecast::<gfc::TriggerRegion>(object) {
            let position = init_with(|this| {
                ((*(*trigger.as_ptr()).__vfptr).getPosition)(
                    (*(*(*trigger.as_ptr()).as_gfc__DetectorObject_mut_ptr())
                        .as_gfc__PhysicsShapeObject_mut_ptr())
                    .as_gfc__WorldObject_mut_ptr(),
                    this,
                );
            });
            mark(
                (*group).mRegionID,
                (*group).mLayerID,
                position.x,
                position.y,
                position.z,
            );
        }
    }
}

unsafe fn mark(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
    let darksiders = gfc::OblivionGame::get_instance();
    let world = darksiders.get_world();

    let class_registry = gfc::Singleton::<gfc::ClassRegistry>::get_instance();
    let class = class_registry
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();

    let obj = class.new_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.as_ptr() as *mut target::gfc__StaticObject;

    target::gfc__WorldObject__setRegionID((*obj).as_gfc__WorldObject_mut_ptr(), region_id);
    target::gfc__WorldObject__setLayerID((*obj).as_gfc__WorldObject_mut_ptr(), layer_id);
    target::gfc__StaticObject__setPackageName(obj, hstring!("vfx_shared").as_ptr());
    target::gfc__StaticObject__setObjectName(obj, hstring!("sphere").as_ptr());

    ((*(*obj).__vfptr).setPosition)(
        (*obj).as_gfc__WorldObject_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ { x, y, z },
    );

    ((*(*obj).__vfptr).addObjectToWorld)((*obj).as_gfc__WorldObject_mut_ptr(), world.as_ptr());
}
