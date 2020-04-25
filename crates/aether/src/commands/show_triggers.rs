use crate::{
    darksiders1::{gfc, Lift},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::{
        coordinate_transformer::CoordinateTransformer,
        debug_draw,
        debug_draw_3d,
        geometry::{box_edges, cylinder, icosphere, transform},
        na_ext::PointExt,
    },
};
use lru::LruCache;
use na::{Isometry, Isometry3, Point3, Translation, UnitQuaternion, Vector3};
use ncollide3d::{
    query::PointQuery,
    shape::{Ball, Compound, ConvexHull, Cuboid, Cylinder, ShapeHandle},
    transformation::ToTriMesh,
};
use once_cell::sync::Lazy;
use ordered_float::NotNan;
use parking_lot::Mutex;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    convert::TryInto,
    f32::consts::FRAC_PI_2,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

const MIN_COUNT: usize = 4;
const MIN_CLOSE_DISTANCE: f32 = 500.0;
const MIN_INSIDE_DISTANCE: f32 = 250.0;

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = ENABLED.fetch_nand(true, SeqCst);
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
        Shape::Box(size, isometry) => {
            let origin = Point3::origin();
            let wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
            for &[p, q] in &wireframe {
                debug_draw_3d::line(region_id, layer_id, isometry * p, isometry * q);
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

pub fn draw(renderer: &gfc::UIRenderer) {
    if !ENABLED.load(SeqCst) {
        return;
    }

    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    let transformer = CoordinateTransformer::create();

    let mut triggers = BTreeSet::new();

    walk(&mut |object| {
        let trigger_region = match gfc::object_safecast::<gfc::DetectorObject>(object) {
            Some(o) => o,
            _ => return,
        };

        let eval = evaluate_object(trigger_region, &player_pos);
        triggers.insert(eval);
        // Filter which objects to draw as we go.
        if triggers.len() > MIN_COUNT && !triggers.last().unwrap().evaluation.force_draw() {
            triggers.pop_last();
        }
    });

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    // The set is already sorted and trimmed.
    for EvaluatedObject { object, .. } in &triggers {
        let position = object.get_position();
        let screen = transformer.world_to_screen(&position);

        // Only draw text if the position is in front of the camera.
        let draw_text = screen.z > 0.0;
        if draw_text {
            let class_name = object.class().name();
            let class_name = class_name.c_str().to_str().unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y, 2, class_name);

            let object_name = object.get_name();
            let object_name = object_name.c_str().to_str().unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y + 20.0, 2, object_name);
        }

        match get_shape(&object) {
            Shape::Aabb(bounds) => {
                debug_draw::box_wireframe(renderer, &transformer, &bounds);
            }
            Shape::Box(size, isometry) => {
                let origin = Point3::origin();
                let mut wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
                transform(&mut wireframe, &na::convert(isometry));
                debug_draw::wireframe(renderer, &transformer, &wireframe);
            }
            Shape::Sphere(_radius, _center) => {
                if draw_text {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, 2, "sphere");
                }
            }
            Shape::Cylinder(_radius, _length, _pos) => {
                if draw_text {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, 2, "cylinder");
                }
            }
        }
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
            // Note: The game technically uses `getTransform()` here, but if we use that,
            // later on we have to use `ConvexHull` instead of `Cuboid`, and there seems to
            // be a `ConvexHull` performance bug where when you stand in certain spots,
            // calling `project_point` even just once can take several seconds to complete.
            //
            // So instead, return an isometry that copies what `getTransform()` does
            // internally.
            let isometry = Isometry::from_parts(
                Translation::from(object.get_position() - Point3::origin()),
                object.get_rotation(),
            );
            Shape::Box(size, isometry)
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

fn evaluate_object(
    object: gfc::AutoRef<gfc::DetectorObject>,
    point: &Point3<f32>,
) -> EvaluatedObject {
    let distance = broad_phase_distance(&object, point);
    if distance > NotNan::new(MIN_CLOSE_DISTANCE).unwrap() {
        return EvaluatedObject {
            object,
            evaluation: Evaluation::Far(distance),
        };
    }

    let evaluation = narrow_phase(&object, point);
    EvaluatedObject { object, evaluation }
}

struct EvaluatedObject {
    object: gfc::AutoRef<gfc::DetectorObject>,
    evaluation: Evaluation,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Evaluation {
    Close(NotNan<f32>),
    InsideClose(NotNan<f32>),
    Far(NotNan<f32>),
    InsideFar(NotNan<f32>),
}

impl Evaluation {
    fn force_draw(&self) -> bool {
        matches!(self, Self::Close(_) | Self::InsideClose(_))
    }
}

impl PartialEq for EvaluatedObject {
    fn eq(&self, other: &Self) -> bool {
        self.evaluation == other.evaluation
    }
}

impl Eq for EvaluatedObject {}

impl PartialOrd for EvaluatedObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.evaluation.partial_cmp(&other.evaluation)
    }
}

impl Ord for EvaluatedObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn broad_phase_distance(object: &gfc::DetectorObject, point: &Point3<f32>) -> NotNan<f32> {
    let bounding_box = object.get_bounding_box();

    let cuboid = Cuboid::new((bounding_box.max - bounding_box.min) / 2.0);
    let isometry = Isometry::from_parts(
        Translation::from(bounding_box.center().coords),
        UnitQuaternion::identity(),
    );

    let projection = cuboid.project_point(&isometry, point, true);
    let distance = na::distance(point, &projection.point);
    NotNan::new(distance).unwrap()
}

fn narrow_phase(object: &gfc::DetectorObject, point: &Point3<f32>) -> Evaluation {
    let shape = get_shape(object).to_compound_cached();
    let projection = shape.project_point(&Isometry::identity(), point, false);
    let distance = na::distance(point, &projection.point);
    if projection.is_inside {
        if distance <= MIN_INSIDE_DISTANCE {
            Evaluation::InsideClose(NotNan::new(distance).unwrap())
        } else {
            Evaluation::InsideFar(NotNan::new(distance).unwrap())
        }
    } else if distance <= MIN_CLOSE_DISTANCE {
        Evaluation::Close(NotNan::new(distance).unwrap())
    } else {
        Evaluation::Far(NotNan::new(distance).unwrap())
    }
}

pub enum Shape {
    Aabb(gfc::TBox<f32>),
    Box(Vector3<f32>, Isometry3<f32>),
    Sphere(f32, Point3<f32>),
    Cylinder(f32, f32, Point3<f32>),
}

impl Shape {
    /// Since `to_compound` is too expensive to run on every frame, we keep a
    /// cache.
    fn to_compound_cached(&self) -> Arc<Compound<f32>> {
        // This is super sloppy. If the cache is too small, FPS will drop. The tradeoff
        // is memory usage.
        static CACHE: Lazy<Mutex<LruCache<TotalShape, Arc<Compound<f32>>>>> =
            Lazy::new(|| Mutex::new(LruCache::new(1000)));

        let mut cache = CACHE.lock();

        let key = self.into();
        if let Some(result) = cache.get(&key) {
            return result.clone();
        }

        let result = Arc::new(self.to_compound());
        cache.put(key, result.clone());
        result
    }

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
            &Self::Box(size, isometry) => {
                Compound::new(vec![(isometry, ShapeHandle::new(Cuboid::new(size / 2.0)))])
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

/// A hashable version of `Shape`.
#[derive(Eq, PartialEq, Hash)]
enum TotalShape {
    Aabb(Point3<NotNan<f32>>, Point3<NotNan<f32>>),
    Box(
        Vector3<NotNan<f32>>,
        Vector3<NotNan<f32>>,
        Vector3<NotNan<f32>>,
    ),
    Sphere(NotNan<f32>, Point3<NotNan<f32>>),
    Cylinder(NotNan<f32>, NotNan<f32>, Point3<NotNan<f32>>),
}

impl From<&Shape> for TotalShape {
    fn from(s: &Shape) -> Self {
        fn not_nan(x: f32) -> NotNan<f32> {
            NotNan::new(x).unwrap()
        }

        match s {
            Shape::Aabb(bounds) => {
                TotalShape::Aabb(bounds.min.map(not_nan), bounds.max.map(not_nan))
            }
            Shape::Box(
                size,
                Isometry {
                    rotation,
                    translation,
                    ..
                },
            ) => {
                TotalShape::Box(
                    size.map(not_nan),
                    rotation.vector().map(not_nan),
                    translation.vector.map(not_nan),
                )
            }
            &Shape::Sphere(radius, center) => {
                TotalShape::Sphere(not_nan(radius), center.map(not_nan))
            }
            &Shape::Cylinder(radius, length, pos) => {
                TotalShape::Cylinder(not_nan(radius), not_nan(length), pos.map(not_nan))
            }
        }
    }
}
