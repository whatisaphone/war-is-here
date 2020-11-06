#![allow(clippy::module_name_repetitions)]

use crate::darksiders1::gfc;
use na::{Isometry, Isometry3, Point3, Translation, UnitQuaternion, Vector3};
use ncollide3d::{
    query::{PointProjection, PointQuery, Ray, RayCast},
    shape::{Ball, Cuboid, Cylinder},
};
use ordered_float::NotNan;
use std::f32::consts::FRAC_PI_2;

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
    pub fn project_point(&self, point: &Point3<f32>, solid: bool) -> PointProjection<f32> {
        let (isometry, collide) = self.to_collide();
        collide.project_point(&isometry, point, solid)
    }

    pub fn toi_with_ray(&self, ray: &Ray<f32>, max_toi: f32, solid: bool) -> Option<f32> {
        let (isometry, collide) = self.to_collide();
        collide.toi_with_ray(&isometry, ray, max_toi, solid)
    }

    fn to_collide(&self) -> (Isometry3<f32>, Box<dyn ShapeQuery>) {
        match self {
            Self::Aabb(bounds) => {
                let center = na::center(&bounds.min, &bounds.max);
                (
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    Box::new(Cuboid::new(bounds.max - center)),
                )
            }
            &Self::Box(size, isometry) => (isometry, Box::new(Cuboid::new(size / 2.0))),
            &Self::Sphere(radius, center) => {
                (
                    Isometry::from_parts(
                        Translation::from(center.coords),
                        UnitQuaternion::identity(),
                    ),
                    Box::new(Ball::new(radius)),
                )
            }
            &Self::Cylinder(radius, length, pos) => {
                (
                    Isometry::from_parts(
                        Translation::from(pos.coords),
                        // `nalgebra` uses principal y-axis; Darksiders uses principal z-axis.
                        // Rotate to match.
                        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), FRAC_PI_2),
                    ),
                    Box::new(Cylinder::new(length / 2.0, radius)),
                )
            }
        }
    }
}

trait ShapeQuery: PointQuery<f32> + RayCast<f32> {}

impl<T> ShapeQuery for T where T: PointQuery<f32> + RayCast<f32> {}

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
