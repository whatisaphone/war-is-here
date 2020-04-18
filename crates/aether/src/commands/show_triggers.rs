use crate::{
    darksiders1::{gfc, Lift},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::{
        coordinate_transformer::CoordinateTransformer,
        debug_draw,
        debug_draw_3d,
        geometry::{box_edges, box_vertices, cylinder, icosphere, transform},
        pretty::Pretty,
    },
};
use na::{Isometry, Matrix4, Point3, Transform3, Translation, UnitQuaternion, Vector3};
use ncollide3d::{
    query::PointQuery,
    shape::{Ball, Compound, ConvexHull, Cuboid, Cylinder, ShapeHandle},
    transformation::ToTriMesh,
};
use ordered_float::NotNan;
use std::{
    convert::TryInto,
    f32::consts::FRAC_PI_2,
    sync::atomic::{AtomicBool, Ordering},
};

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    if enabled {
        let mut guard = ON_POST_UPDATE_QUEUE.lock();
        guard.as_mut().unwrap().push_back(Box::new(go));
    }

    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

fn go() {
    walk(&mut |object| {
        if let Some(trigger) = gfc::object_safecast::<gfc::DetectorObject>(object) {
            mark(&trigger);
        }
    });
}

fn walk(visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    walk_group(&world.root(), visitor);

    for (r, _) in world.region_data().iter().enumerate() {
        let region = match world.get_region(r.try_into().unwrap()) {
            Some(x) => x,
            None => continue,
        };

        for (l, _) in region.layer_data().iter().enumerate() {
            let layer = match region.get_layer(l.try_into().unwrap()) {
                Some(x) => x,
                None => continue,
            };

            walk_group(&layer.root(), visitor);
        }

        for object in region.load_regions().iter() {
            visitor(object);
        }
    }
}

fn walk_group(group: &gfc::WorldGroup, visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    for object in group.objects() {
        visitor(object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            walk_group(&group, visitor);
        }
    }
}

fn mark(trigger: &gfc::DetectorObject) {
    let region_id = trigger.get_region_id();
    let layer_id = trigger.get_layer_id();
    let position = trigger.get_position();

    add_marker(region_id, layer_id, position.x, position.y, position.z);

    match get_shape(&trigger) {
        Shape::Aabb(bounds) => {
            for &[p, q] in &box_edges(bounds.min, bounds.max) {
                debug_draw_3d::line(region_id, layer_id, p, q);
            }
        }
        Shape::Box(size, tf) => {
            let origin = Point3::origin();
            let wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
            let transform = Transform3::from_matrix_unchecked(tf);
            for &[p, q] in &wireframe {
                debug_draw_3d::line(region_id, layer_id, transform * p, transform * q);
            }
        }
        Shape::Sphere(radius, center) => {
            for [p, q] in icosphere() {
                let p = center + p.coords * radius;
                let q = center + q.coords * radius;
                debug_draw_3d::line(region_id, layer_id, p, q);
            }
        }
        Shape::Cylinder(radius, length, pos) => {
            for [p, q] in cylinder(24) {
                let p = pos + Vector3::new(p.x * radius, p.y * radius, p.z * length);
                let q = pos + Vector3::new(q.x * radius, q.y * radius, q.z * length);
                debug_draw_3d::line(region_id, layer_id, p, q);
            }
        }
    }
}

fn add_marker(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
    let obj = gfc::AutoRef::new(gfc::StaticObject::new());
    obj.set_region_id(region_id);
    obj.set_layer_id(layer_id);
    obj.set_package_name(&hstring!("vfx_shared"));
    obj.set_object_name(&hstring!("sphere"));
    obj.set_position(&Point3::new(x, y, z));

    if let Some(world) = gfc::OblivionGame::get_instance().get_world() {
        obj.add_object_to_world(&world);
    }
}

// TODO: This is too slow (probably because of cylinders). Caching is required.
const SHOW_CLOSEST: bool = false;

pub fn draw(renderer: &gfc::UIRenderer) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    let transformer = CoordinateTransformer::create();

    let mut triggers = Vec::new();

    walk(&mut |object| {
        let trigger_region = match gfc::object_safecast::<gfc::DetectorObject>(object) {
            Some(o) => o,
            _ => return,
        };

        let position = trigger_region.get_position();
        let screen = transformer.world_to_screen(&position);

        // If position is in front of the camera, and within a certain distance of the
        // player
        if screen.z >= 0.0 && na::distance_squared(&player_pos, &position) < 750_f32.powi(2) {
            let class_name = trigger_region.class().name();
            let class_name = class_name.c_str().to_str().unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y, 2, class_name);

            let object_name = trigger_region.get_name();
            let object_name = object_name.c_str().to_str().unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y + 20.0, 2, object_name);

            match get_shape(&trigger_region) {
                Shape::Aabb(bounds) => {
                    debug_draw::box_wireframe(renderer, &transformer, &bounds);
                }
                Shape::Box(size, tf) => {
                    let origin = Point3::origin();
                    let mut wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
                    transform(&mut wireframe, &Transform3::from_matrix_unchecked(tf));
                    debug_draw::wireframe(renderer, &transformer, &wireframe);
                }
                Shape::Sphere(_radius, _center) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, 2, "sphere");
                }
                Shape::Cylinder(_radius, _length, _pos) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, 2, "cylinder");
                }
            }
        }

        if SHOW_CLOSEST {
            let shape = get_shape(&trigger_region).to_compound();
            let projection = shape.project_point(&Isometry::identity(), &player_pos, false);
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
        let object_name = trigger.get_name();
        let object_name = object_name.c_str().to_str().unwrap_or("<invalid utf-8>");
        bitmap_font::draw_string(renderer, 10.0, 10.0, 2, object_name);
        bitmap_font::draw_string(
            renderer,
            10.0,
            30.0,
            2,
            &format!("{}", Pretty(projection.point)),
        );

        debug_draw::clunky_draw_line(
            renderer,
            transformer.world_to_screen(&player_pos).xy(),
            transformer.world_to_screen(&projection.point).xy(),
        );
    }

    renderer.end();
}

// See `gfc::DetectorObject::doAddToWorld`
fn get_shape(object: &gfc::DetectorObject) -> Shape {
    match object.shape() {
        gfc::PhysicsShapeObject__Detect::Aabb => {
            let bounds = unsafe { (*object.as_ptr()).mBounds.lift_ref().clone() };
            Shape::Aabb(bounds)
        }
        gfc::PhysicsShapeObject__Detect::Box => {
            let size = unsafe { *(*object.as_ptr()).mSize.lift_ref() };
            let transform = object.get_transform();
            Shape::Box(size, transform)
        }
        gfc::PhysicsShapeObject__Detect::Sphere => {
            let radius = unsafe { (*object.as_ptr()).mSize.z } * 0.5;
            let position = object.get_position();
            Shape::Sphere(radius, position)
        }
        gfc::PhysicsShapeObject__Detect::Cylinder => {
            let radius = unsafe { (*object.as_ptr()).mSize.x } * 0.5;
            let length = unsafe { (*object.as_ptr()).mSize.z };
            let position = object.get_position();
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

impl Shape {
    fn to_compound(&self) -> Compound<f32> {
        match self {
            Self::Aabb(bounds) => {
                let center = na::center(&bounds.min, &bounds.max);
                Compound::new(vec![(
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    ShapeHandle::new(Cuboid::new(bounds.max - center)),
                )])
            }
            Self::Box(size, transform) => {
                let vertices = box_vertices(size / 2.0)
                    .iter()
                    .map(|p| Point3::from_homogeneous(transform * p.to_homogeneous()).unwrap())
                    .collect::<Vec<_>>();
                Compound::new(vec![(
                    Isometry::identity(),
                    ShapeHandle::new(ConvexHull::try_from_points(&vertices).unwrap()),
                )])
            }
            &Self::Sphere(radius, center) => {
                Compound::new(vec![(
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    ShapeHandle::new(Ball::new(radius)),
                )])
            }
            &Self::Cylinder(radius, length, pos) => {
                // Cylinder does not implement Shape
                // https://github.com/rustsim/ncollide/issues/216
                let cylinder = Cylinder::new(length / 2.0, radius).to_trimesh(24);
                Compound::new(vec![(
                    Isometry::from_parts(
                        Translation::from(pos.coords),
                        // `nalgebra` uses principal y-axis; Darksiders uses principal z-axis.
                        // Rotate to match.
                        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), FRAC_PI_2),
                    ),
                    ShapeHandle::new(ConvexHull::try_from_points(&cylinder.coords).unwrap()),
                )])
            }
        }
    }
}
