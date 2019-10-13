#![allow(clippy::cast_precision_loss)]

use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::{Matrix4, Point3};
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
        let screen = self.view_proj * world.to_homogeneous();
        let x = (1.0 + screen.x / screen.w) * self.viewport_width / 2.0;
        let y = (1.0 - screen.y / screen.w) * self.viewport_height / 2.0;
        Point3::new(x, y, screen.z)
    }
}
