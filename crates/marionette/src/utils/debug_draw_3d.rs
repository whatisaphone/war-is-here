#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::{
    darksiders1::{gfc, Lower},
    library::objects::gritty_cube,
};
use darksiders1_sys::target;
use na::{Point3, Vector3};

const SCALE: f32 = 10.0;

pub fn chunky_line(p: Point3<f32>, q: Point3<f32>) {
    let world = unsafe { gfc::OblivionGame::get_instance().get_world() }.unwrap();

    for point in clunky_line_3d(p, q) {
        let cube = make_cube(point);
        unsafe {
            (*cube.as_ptr()).addObjectToWorld(world.as_ptr());
        }
    }
}

fn make_cube(p: Point3<f32>) -> gfc::AutoRef<gfc::WorldObject> {
    let obj = gfc::AutoRef::new(gfc::StaticObject::new());

    unsafe {
        target::gfc__StaticObject__setPackageName(obj.as_ptr(), gritty_cube::PACKAGE_NAME.as_ptr());
        target::gfc__StaticObject__setObjectName(obj.as_ptr(), gritty_cube::OBJECT_NAME.as_ptr());
        (*obj.as_ptr()).setPosition(&Lower::lower(p.coords));
        (*obj.as_ptr()).setScale(&Lower::lower(Vector3::new(
            SCALE / 25.0 / 2.0,
            SCALE / 25.0 / 2.0,
            SCALE / 25.0 / 2.0,
        )));
    }

    gfc::AutoRef::cast(obj)
}

fn clunky_line_3d(p: Point3<f32>, q: Point3<f32>) -> impl Iterator<Item = Point3<f32>> {
    let v = q - p;
    let l1_dist = *[(q.x - p.x).abs(), (q.y - p.y).abs(), (q.z - p.z).abs()]
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    let max = (l1_dist / SCALE) as i32 + 1;
    (0..=max).map(move |i| p + v * (i as f32 / max as f32))
}
