use crate::{
    commands::show_triggers::shape::{get_shape, CachedShapeQuery},
    darksiders1::{gfc, gfc::Reflect},
    utils::na_ext::UnitComplexExt,
};
use na::{Isometry3, Point3, Translation, UnitComplex, UnitQuaternion, Vector3};
use ncollide3d::{
    query::{PointQuery, Ray},
    shape::Cuboid,
};
use ordered_float::NotNan;
use std::{cmp::Ordering, collections::BTreeSet, f32::consts::PI};

const MIN_CLOSE_DISTANCE: f32 = 500.0;
const MIN_INSIDE_DISTANCE: f32 = 250.0;

pub struct KeepMinCountOrMinPriority {
    min_count: usize,
    set: BTreeSet<PrioritizedObject>,
}

impl KeepMinCountOrMinPriority {
    pub fn new(min_count: usize) -> Self {
        Self {
            min_count,
            set: BTreeSet::new(),
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = gfc::AutoRef<gfc::DetectorObject>> {
        self.set
            .into_iter()
            .map(|PrioritizedObject { object, .. }| object)
    }

    pub fn feed(&mut self, object: gfc::AutoRef<gfc::DetectorObject>, priority: Priority) {
        self.set.insert(PrioritizedObject { object, priority });
        // Filter as we go.
        if self.set.len() > self.min_count && !self.set.last().unwrap().priority.force_draw() {
            self.set.pop_last();
        }
    }
}

pub fn categorize_object(object: &gfc::DetectorObject) -> Category {
    if object.class().instanceof(gfc::LoadRegion::class()) {
        return Category::LoadRegion;
    }
    Category::Other
}

pub fn prioritize_object(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let distance = broad_phase_distance(object, point);
    if distance > NotNan::new(MIN_CLOSE_DISTANCE).unwrap() {
        return Priority::Far(distance);
    }

    narrow_phase(object, point)
}

pub enum Category {
    LoadRegion,
    Other,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Priority {
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
    let isometry = Isometry3::from_parts(
        Translation::from(bounding_box.center().coords),
        UnitQuaternion::identity(),
    );

    let projection = cuboid.project_point(&isometry, point, true);
    let distance = na::distance(point, &projection.point);
    NotNan::new(distance).unwrap()
}

fn narrow_phase(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let shape = get_shape(object).to_cached_shape_query();
    let projection = shape.project_point(point, false);

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

fn distance_along_xy_plane(shape: &CachedShapeQuery, point: &Point3<f32>) -> Option<f32> {
    // Very rough approximation. Cast 8 rays horizontally to approximate the
    // distance to the edge at the current z position.
    #[allow(clippy::cast_precision_loss)]
    (0..8)
        .map(|i| 2.0 * PI / 8.0 * i as f32)
        .flat_map(|theta| {
            let rot = UnitComplex::new(theta).around_z_axis();
            let vector = rot * Vector3::x();
            shape.toi_with_ray(&Ray::new(*point, vector), f32::INFINITY, false)
        })
        .map(|x| NotNan::new(x).unwrap())
        .min()
        .map(NotNan::into_inner)
}
