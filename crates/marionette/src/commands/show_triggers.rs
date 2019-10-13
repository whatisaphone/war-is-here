use crate::{
    darksiders1::{gfc, Lift, Lift1, List, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::{coordinate_transformer::CoordinateTransformer, debug_draw, mem::init_with},
};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::{
    convert::TryFrom,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    ENABLED.store(true, Ordering::SeqCst);

    walk(&mut |object| {
        if let Some(trigger) = gfc::object_safecast::<gfc::TriggerRegion>(object) {
            mark(&trigger);
        }
    });
}

unsafe fn walk(visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    let root = (*world.as_ptr()).mRoot.lift_ref();
    walk_group(root, visitor);

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
            walk_group(&root, visitor);
        }
    }
}

unsafe fn walk_group(group: &gfc::WorldGroup, visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let objects = &mut (*group.as_ptr()).mObjects;
    let objects = List::<target::gfc__AutoRef_gfc__WorldObject_>::from_ptr(objects);
    for object in objects {
        let object = object.lift_ref();

        visitor(object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            walk_group(&group, visitor);
        }
    }
}

unsafe fn mark(trigger: &gfc::TriggerRegion) {
    let position = init_with(|p| {
        (*trigger.as_ptr()).getPosition(p);
    });
    add_marker(
        (*trigger.as_ptr()).mRegionID,
        (*trigger.as_ptr()).mLayerID,
        position.x,
        position.y,
        position.z,
    );
}

unsafe fn add_marker(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
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

    if let Some(world) = gfc::OblivionGame::get_instance().get_world() {
        (*obj).addObjectToWorld(world.as_ptr());
    }
}

static ENABLED: AtomicBool = AtomicBool::new(false);

pub unsafe fn draw(renderer: *mut target::gfc__UIRenderer) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    target::gfc__UIRenderer__begin(renderer, true);
    target::gfc__UIRenderer__setMaterial(renderer, (*renderer).mSolidMaterial.ptr());

    let transformer = CoordinateTransformer::create();

    walk(&mut |object| {
        let trigger_region = match gfc::object_safecast::<gfc::TriggerRegion>(object) {
            Some(o) => o,
            _ => return,
        };

        let position = init_with(|p| {
            (*trigger_region.as_ptr()).getPosition(p);
        })
        .lift();
        let screen = transformer.world_to_screen(&position);

        if screen.z >= 0.0 && screen.z < 500.0 {
            let class_name = trigger_region
                .class()
                .name()
                .c_str()
                .to_str()
                .unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y, class_name);

            let object_name = (*trigger_region.as_ptr())
                .mName
                .lift_ref()
                .c_str()
                .to_str()
                .unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y + 20.0, object_name);

            match get_shape(&trigger_region) {
                Shape::Aabb(b0x) => {
                    debug_draw::box_wireframe(renderer, &transformer, &b0x);
                }
                Shape::Other(s) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, s);
                }
            };
        }
    });

    target::gfc__UIRenderer__endRendering(renderer);
}

unsafe fn get_shape(object: &gfc::TriggerRegion) -> Shape {
    match object.shape() {
        gfc::PhysicsShapeObject__Detect::Aabb => {
            Shape::Aabb(std::ptr::read(&(*object.as_ptr()).mBounds))
        }
        gfc::PhysicsShapeObject__Detect::Box => Shape::Other("box"),
        gfc::PhysicsShapeObject__Detect::Sphere => Shape::Other("sphere"),
        gfc::PhysicsShapeObject__Detect::Cylinder => Shape::Other("cylinder"),
    }
}

pub enum Shape {
    Aabb(target::gfc__TBox_float_gfc__FloatMath_),
    Other(&'static str),
}
