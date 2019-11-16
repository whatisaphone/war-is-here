#![allow(clippy::cast_precision_loss)]

use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::{Isometry3, Matrix4, Point3, Unit, Vector3};
use ncollide3d::{
    query::{PointQuery, Ray, RayCast},
    shape::Plane,
};
use pdbindgen_runtime::StaticCast;

pub struct CoordinateTransformer {
    viewport_width: f32,
    viewport_height: f32,
    view_proj: Matrix4<f32>,
}

impl CoordinateTransformer {
    pub fn create() -> Self {
        unsafe {
            let viewport = gfc::KGGraphics::get_instance().get_viewport();
            let viewport_width = viewport.width() as f32;
            let viewport_height = viewport.height() as f32;

            let darksiders = gfc::OblivionGame::get_instance();
            let view = init_with(|p| {
                target::gfc__OblivionGame__getViewMatrix(darksiders.as_ptr().static_cast(), p);
            })
            .lift();
            let proj = init_with(|p| {
                target::gfc__OblivionGame__getProjMatrix(darksiders.as_ptr().static_cast(), p);
            })
            .lift();

            Self {
                viewport_width,
                viewport_height,
                view_proj: proj * view,
            }
        }
    }

    pub fn world_to_screen(&self, world: &Point3<f32>) -> Point3<f32> {
        let mut screen = self.view_proj * world.to_homogeneous();
        if screen.z < 0.0 {
            screen.x = -screen.x;
            screen.y = -screen.y;
        }
        let x = (1.0 + screen.x / screen.w) * self.viewport_width / 2.0;
        let y = (1.0 - screen.y / screen.w) * self.viewport_height / 2.0;
        Point3::new(x, y, screen.z)
    }

    pub fn clip_line_to_frustum_near_plane(
        &self,
        p: Point3<f32>,
        q: Point3<f32>,
    ) -> Option<[Point3<f32>; 2]> {
        // http://www8.cs.umu.se/kurser/5DV051/HT12/lab/plane_extraction.pdf
        let near = (self.view_proj.row(3) + self.view_proj.row(2)).transpose();

        let near = near / near.xyz().norm();
        let plane = Plane::new(Unit::new_unchecked(near.xyz()));
        // I actually have no idea how this works, but hey, it seems to work.
        let isometry = Isometry3::new(near.xyz() * -(near.w - 10.0), Vector3::zeros());
        let p_inside = !plane.project_point(&isometry, &p, true).is_inside;
        let q_inside = !plane.project_point(&isometry, &q, true).is_inside;
        match (p_inside, q_inside) {
            (true, true) => Some([p, q]),
            (false, false) => None,
            (false, true) => {
                let pq = Ray::new(p, q - p);
                let t = plane.toi_with_ray(&isometry, &pq, false).unwrap();
                let p = pq.point_at(t);
                Some([p, q])
            }
            (true, false) => {
                let qp = Ray::new(q, p - q);
                let t = plane.toi_with_ray(&isometry, &qp, false).unwrap();
                let q = qp.point_at(t);
                Some([p, q])
            }
        }
    }
}
