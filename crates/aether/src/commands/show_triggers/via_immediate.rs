use crate::{
    commands::show_triggers::{
        shape::{get_shape, Shape},
        walk::walk_world,
    },
    darksiders1::{gfc, gfc::Reflect},
    library::bitmap_font,
    utils::{
        coordinate_transformer::CoordinateTransformer,
        debug_draw,
        geometry::{box_edges, transform},
        na_ext::UnitComplexExt,
    },
};
use itertools::iproduct;
use na::{Isometry, Point3, Translation, UnitComplex, UnitQuaternion, Vector3};
use ncollide3d::{
    query::{PointQuery, Ray, RayCast},
    shape::{Compound, Cuboid},
};
use ordered_float::NotNan;
use std::{cmp::Ordering, collections::BTreeSet, f32::consts::PI};

const MIN_CLOSE_DISTANCE: f32 = 500.0;
const MIN_INSIDE_DISTANCE: f32 = 250.0;

pub fn draw(renderer: &gfc::UIRenderer) {
    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    let transformer = CoordinateTransformer::create();

    // Sort objects into multiple groups, so we can have categories of objects which
    // are always drawn.
    let mut load_regions = KeepMinCountOrMinPriority::new(1);
    let mut others = KeepMinCountOrMinPriority::new(3);

    walk_world(&mut |object| {
        let object = match gfc::object_safecast::<gfc::DetectorObject>(object) {
            Some(o) => o,
            None => return,
        };

        let category = categorize_object(&object);
        let priority = prioritize_object(&object, &player_pos);
        match category {
            Category::LoadRegion => load_regions.feed(object, priority),
            Category::Other => others.feed(object, priority),
        }
    });

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

    for object in load_regions.into_iter() {
        draw_object(renderer, &transformer, &object);
    }
    for object in others.into_iter() {
        draw_object(renderer, &transformer, &object);
    }

    renderer.end();
}

fn draw_object(
    renderer: &gfc::UIRenderer,
    transformer: &CoordinateTransformer,
    object: &gfc::DetectorObject,
) {
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

struct KeepMinCountOrMinPriority {
    min_count: usize,
    set: BTreeSet<PrioritizedObject>,
}

impl KeepMinCountOrMinPriority {
    fn new(min_count: usize) -> Self {
        Self {
            min_count,
            set: BTreeSet::new(),
        }
    }

    fn into_iter(self) -> impl Iterator<Item = gfc::AutoRef<gfc::DetectorObject>> {
        self.set
            .into_iter()
            .map(|PrioritizedObject { object, .. }| object)
    }

    fn feed(&mut self, object: gfc::AutoRef<gfc::DetectorObject>, priority: Priority) {
        self.set.insert(PrioritizedObject { object, priority });
        // Filter as we go.
        if self.set.len() > self.min_count && !self.set.last().unwrap().priority.force_draw() {
            self.set.pop_last();
        }
    }
}

fn categorize_object(object: &gfc::DetectorObject) -> Category {
    if object.class().instanceof(gfc::LoadRegion::class()) {
        return Category::LoadRegion;
    }
    Category::Other
}

fn prioritize_object(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let distance = broad_phase_distance(object, point);
    if distance > NotNan::new(MIN_CLOSE_DISTANCE).unwrap() {
        return Priority::Far(distance);
    }

    narrow_phase(object, point)
}

enum Category {
    LoadRegion,
    Other,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Priority {
    Close(NotNan<f32>),
    InsideClose(NotNan<f32>),
    Far(NotNan<f32>),
    InsideFar(NotNan<f32>),
}

impl Priority {
    fn force_draw(&self) -> bool {
        matches!(self, Self::Close(_) | Self::InsideClose(_))
    }
}

/// Helper struct for sorting objects by priority.
///
/// This compares based on `priority`, and ignores `object` entirely.
struct PrioritizedObject {
    object: gfc::AutoRef<gfc::DetectorObject>,
    priority: Priority,
}

impl PartialEq for PrioritizedObject {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PrioritizedObject {}

impl PartialOrd for PrioritizedObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for PrioritizedObject {
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

fn narrow_phase(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let shape = get_shape(object).to_compound_cached();
    let projection = shape.project_point(&Isometry::identity(), point, false);

    // Attempt to only take into account the xy plane and ignore the z plane. If
    // you're close to a trigger that covers the entire ground, it's useless to
    // display it all the time, so we try to only draw it when you're near the edge
    // (when moving horizontally).
    //
    // Fall back to ordinary distance if we can't figure it out.
    let distance = distance_along_xy_plane(&shape, point)
        .unwrap_or_else(|| na::distance(point, &projection.point));

    if projection.is_inside {
        if distance <= MIN_INSIDE_DISTANCE {
            Priority::InsideClose(NotNan::new(distance).unwrap())
        } else {
            Priority::InsideFar(NotNan::new(distance).unwrap())
        }
    } else if distance <= MIN_CLOSE_DISTANCE {
        Priority::Close(NotNan::new(distance).unwrap())
    } else {
        Priority::Far(NotNan::new(distance).unwrap())
    }
}

fn distance_along_xy_plane(shape: &Compound<f32>, point: &Point3<f32>) -> Option<f32> {
    // Very rough approximation. Cast 8 rays horizontally to approximate the
    // distance to the edge at the current z position.
    iproduct!(
        #[allow(clippy::cast_precision_loss)]
        (0..8).map(|i| 2.0 * PI / 8.0 * i as f32),
        // HACK: do this from the character's feet as well as his head, in case his head is the
        // only thing touching a trigger (for example in CI_03, ci03_mm_trigger_10)
        &[*point, point + Vector3::new(0.0, 0.0, 0.0)]
    )
    .flat_map(|(theta, point)| {
        let rot = UnitComplex::new(theta).around_z_axis();
        let vector = rot * Vector3::x();
        shape.toi_with_ray(&Isometry::identity(), &Ray::new(*point, vector), false)
    })
    .map(|x| NotNan::new(x).unwrap())
    .min()
    .map(NotNan::into_inner)
}
