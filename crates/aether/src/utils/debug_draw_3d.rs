use crate::{darksiders1::gfc, library::objects::wireframe_line};
use na::{Point3, Unit, UnitQuaternion, Vector3};

pub fn line(region_id: u16, layer_id: u16, p: Point3<f32>, q: Point3<f32>) {
    let world = unsafe { gfc::OblivionGame::get_instance().get_world() }.unwrap();

    let obj = gfc::AutoRef::new(gfc::StaticObject::new());
    obj.set_region_id(region_id);
    obj.set_layer_id(layer_id);
    obj.set_package_name(&wireframe_line::PACKAGE_NAME);
    obj.set_object_name(&wireframe_line::OBJECT_NAME);
    obj.set_position(&na::center(&p, &q));
    obj.set_rotation(
        &UnitQuaternion::rotation_between(&Vector3::x_axis(), &Unit::new_normalize(q - p))
            .unwrap_or_else(UnitQuaternion::identity),
    );
    obj.set_scale(&Vector3::new(
        (p - q).norm() / wireframe_line::HALF_SIZE / 2.0,
        1.0,
        1.0,
    ));
    obj.add_object_to_world(world);
}
