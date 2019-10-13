#![allow(clippy::cast_precision_loss)]

use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::{Matrix4, Vector3, Vector4};
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

    pub fn world_to_screen(&self, world: &Vector3<f32>) -> Vector3<f32> {
        let world_homo = Vector4::new(world.x, world.y, world.z, 1.0);
        let screen = self.view_proj * world_homo;
        let x = (1.0 + screen.x / screen.w) * self.viewport_width / 2.0;
        let y = (1.0 - screen.y / screen.w) * self.viewport_height / 2.0;
        if screen.z >= 0.0 {
            Vector3::new(x, y, screen.z)
        } else {
            Vector3::new(-x, -y, screen.z)
        }
    }
}
