use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::{Matrix4, Point3};

struct_wrapper!(Node3D, target::gfc__Node3D);
struct_wrapper_super!(Node3D, gfc::Object);

struct_wrapper!(Skeleton3D, target::gfc__Skeleton3D);
struct_wrapper_super!(Skeleton3D, gfc::Node3D);

impl Node3D {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        let result = unsafe {
            init_with(|p| {
                target::gfc__Node3D__getMatrix(self.as_ptr(), p, false);
            })
        };
        result.lift()
    }

    pub fn get_position(&self) -> Point3<f32> {
        let result = unsafe {
            init_with(|p| {
                target::gfc__Node3D__getPosition(self.as_ptr(), p);
            })
        };
        Point3::from(result.lift())
    }
}
