#![allow(clippy::module_name_repetitions)]

use crate::darksiders1::gfc;
use lru::LruCache;
use na::{Isometry, Isometry3, Point3, Translation, UnitQuaternion, Vector3};
use ncollide3d::{
    shape::{Ball, Compound, ConvexHull, Cuboid, Cylinder, ShapeHandle},
    transformation::ToTriMesh,
};
use once_cell::sync::Lazy;
use ordered_float::NotNan;
use parking_lot::Mutex;
use std::{f32::consts::FRAC_PI_2, sync::Arc};

// See `gfc::DetectorObject::doAddToWorld`
pub fn get_shape(object: &gfc::DetectorObject) -> Shape {
    match object.shape() {
        gfc::PhysicsShapeObject__Detect::Aabb => {
            let bounds = object.bounds().clone();
            Shape::Aabb(bounds)
        }
        gfc::PhysicsShapeObject__Detect::Box => {
            let size = *object.size();
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
            let radius = object.size().z * 0.5;
            let position = object.get_position();
            Shape::Sphere(radius, position)
        }
        gfc::PhysicsShapeObject__Detect::Cylinder => {
            let radius = object.size().x * 0.5;
            let length = object.size().z;
            let position = object.get_position();
            Shape::Cylinder(radius, length, position)
        }
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
    pub fn to_compound_cached(&self) -> Arc<Compound<f32>> {
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
