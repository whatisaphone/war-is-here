#![allow(clippy::cast_precision_loss)]

use crate::{
    darksiders1::{gfc, Lift},
    utils::{
        geometry::{LineSegment3, Plane3},
        mem::init_with,
    },
};
use darksiders1_sys::target;
use na::{Matrix4, Point3, RowVector4, Vector3};
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

    pub fn clip_line_segment_to_frustum_near_plane(
        &self,
        p: &Point3<f32>,
        q: &Point3<f32>,
    ) -> Option<[Point3<f32>; 2]> {
        // Extract the near plane of the frustum
        //
        // http://www8.cs.umu.se/kurser/5DV051/HT12/lab/plane_extraction.pdf

        fn plane(v: RowVector4<f32>) -> Plane3 {
            Plane3::new(Vector3::new(v.x, v.y, v.z), v.w)
        }

        let mut near = plane(self.view_proj.row(3) + self.view_proj.row(2));
        // A fudge factor is needed. Why?
        near.d -= 25.0;

        // Clip the segment to the plane

        let mut segment = LineSegment3::new(*p, *q);
        segment = segment.clip_to_plane(&near)?;
        Some([segment.p, segment.q])
    }
}
