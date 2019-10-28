use crate::{
    darksiders1::{gfc, Lift, Lift1, List, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::{
        coordinate_transformer::CoordinateTransformer,
        debug_draw,
        debug_draw_3d,
        geometry::{box_edges, icosphere},
        mem::init_with,
        pretty::Pretty,
    },
};
use darksiders1_sys::target;
use na::{Isometry, Matrix4, Point3, Translation, UnitQuaternion, Vector3};
use ncollide3d::{
    query::PointQuery,
    shape::{Ball, Compound, Cuboid, ShapeHandle},
};
use ordered_float::NotNan;
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
    let position = trigger.get_position();

    add_marker(
        (*trigger.as_ptr()).mRegionID,
        (*trigger.as_ptr()).mLayerID,
        position.x,
        position.y,
        position.z,
    );

    match get_shape(&trigger) {
        Shape::Aabb(bounds) => {
            for &[p, q] in &box_edges(bounds.min, bounds.max) {
                debug_draw_3d::line(p, q);
            }
        }
        Shape::Box(size, transform) => {
            for &[p, q] in &box_edges(Point3::origin() - size / 2.0, Point3::origin() + size / 2.0)
            {
                let p = Point3::from_homogeneous(transform * p.to_homogeneous()).unwrap();
                let q = Point3::from_homogeneous(transform * q.to_homogeneous()).unwrap();
                debug_draw_3d::line(p, q);
            }
        }
        Shape::Sphere(radius, center) => {
            for [p, q] in icosphere() {
                debug_draw_3d::line(center + p.coords * radius, center + q.coords * radius);
            }
        }
        Shape::Cylinder(_radius, _length, _pos) => {}
    }
}

fn add_marker(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
    let obj = gfc::AutoRef::new(gfc::StaticObject::new());

    unsafe {
        target::gfc__WorldObject__setRegionID(obj.as_ptr().static_cast(), region_id);
        target::gfc__WorldObject__setLayerID(obj.as_ptr().static_cast(), layer_id);
        target::gfc__StaticObject__setPackageName(obj.as_ptr(), hstring!("vfx_shared").as_ptr());
        target::gfc__StaticObject__setObjectName(obj.as_ptr(), hstring!("sphere").as_ptr());
    }
    obj.set_position(&Point3::new(x, y, z));

    if let Some(world) = unsafe { gfc::OblivionGame::get_instance().get_world() } {
        obj.add_object_to_world(world);
    }
}

static ENABLED: AtomicBool = AtomicBool::new(false);

pub unsafe fn draw(renderer: &gfc::UIRenderer) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    target::gfc__UIRenderer__begin(renderer.as_ptr(), true);
    target::gfc__UIRenderer__setMaterial(
        renderer.as_ptr(),
        (*renderer.as_ptr()).mSolidMaterial.ptr(),
    );

    let transformer = CoordinateTransformer::create();

    let mut triggers = Vec::new();

    walk(&mut |object| {
        let trigger_region = match gfc::object_safecast::<gfc::TriggerRegion>(object) {
            Some(o) => o,
            _ => return,
        };

        let position = trigger_region.get_position();
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
                Shape::Aabb(bounds) => {
                    debug_draw::box_wireframe(renderer, &transformer, &bounds);
                }
                Shape::Box(_size, _transform) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, "box");
                }
                Shape::Sphere(_radius, _center) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, "sphere");
                }
                Shape::Cylinder(_radius, _length, _pos) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, "cylinder");
                }
            }
        }

        let shape = match get_shape(&trigger_region) {
            Shape::Aabb(bounds) => {
                let center = na::center(&bounds.min, &bounds.max);
                Some(Compound::new(vec![(
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    ShapeHandle::new(Cuboid::new(bounds.max - center)),
                )]))
            }
            Shape::Box(_size, _transform) => None,
            Shape::Sphere(radius, center) => {
                Some(Compound::new(vec![(
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    ShapeHandle::new(Ball::new(radius)),
                )]))
            }
            Shape::Cylinder(_radius, _length, _pos) => None,
        };

        let projection = match shape {
            Some(shape) => Some(shape.project_point(&Isometry::identity(), &player_pos, false)),
            None => None,
        };

        if let Some(projection) = projection {
            triggers.push((trigger_region, projection));
        }
    });

    let closest_trigger = triggers
        .into_iter()
        .filter(|(_region, projection)| !projection.is_inside)
        .min_by_key(|(_region, projection)| {
            NotNan::new(na::distance_squared(&player_pos, &projection.point)).unwrap()
        });
    if let Some((trigger, projection)) = closest_trigger {
        let object_name = (*trigger.as_ptr())
            .mName
            .lift_ref()
            .c_str()
            .to_str()
            .unwrap_or("<invalid utf-8>");
        bitmap_font::draw_string(renderer, 10.0, 10.0, object_name);
        bitmap_font::draw_string(
            renderer,
            10.0,
            30.0,
            &format!("{}", Pretty(projection.point)),
        );

        debug_draw::clunky_draw_line(
            renderer,
            transformer.world_to_screen(&player_pos).xy(),
            transformer.world_to_screen(&projection.point).xy(),
        );
    }

    target::gfc__UIRenderer__endRendering(renderer.as_ptr());
}

// See `gfc::DetectorObject::doAddToWorld`
unsafe fn get_shape(object: &gfc::TriggerRegion) -> Shape {
    match object.shape() {
        gfc::PhysicsShapeObject__Detect::Aabb => {
            let bounds = (*object.as_ptr()).mBounds.lift_ref().clone();
            Shape::Aabb(bounds)
        }
        gfc::PhysicsShapeObject__Detect::Box => {
            let size = *(*object.as_ptr()).mSize.lift_ref();
            let transform = object.get_transform();
            Shape::Box(size, transform)
        }
        gfc::PhysicsShapeObject__Detect::Sphere => {
            let radius = (*object.as_ptr()).mSize.z * 0.5;
            let position = Point3::from(*(*object.as_ptr()).mPosition.lift_ref());
            Shape::Sphere(radius, position)
        }
        gfc::PhysicsShapeObject__Detect::Cylinder => {
            let radius = (*object.as_ptr()).mSize.x * 0.5;
            let length = (*object.as_ptr()).mSize.z;
            let position = Point3::from(*(*object.as_ptr()).mPosition.lift_ref());
            Shape::Cylinder(radius, length, position)
        }
    }
}

pub enum Shape {
    Aabb(gfc::TBox<f32>),
    Box(Vector3<f32>, Matrix4<f32>),
    Sphere(f32, Point3<f32>),
    Cylinder(f32, f32, Point3<f32>),
}
