use crate::{
    commands::show_triggers::{
        shape::{get_shape, Shape},
        walk::walk_world,
    },
    darksiders1::gfc,
    utils::{
        debug_draw_3d,
        meshes::{box_edges, cylinder, icosphere},
    },
};
use na::{Point3, Vector3};

pub fn draw() {
    walk_world(&mut |object| {
        if let Some(trigger) = gfc::object_safecast::<gfc::DetectorObject>(object) {
            mark(&trigger);
        }
    });
}

fn mark(trigger: &gfc::DetectorObject) {
    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    let region_id = trigger.get_region_id();
    let layer_id = trigger.get_layer_id();
    let position = trigger.get_position();

    let add = |object: gfc::AutoRef<gfc::StaticObject>| {
        object.set_region_id(region_id);
        object.set_layer_id(layer_id);
        object.add_object_to_world(&world);
    };

    add(marker(&position));

    match get_shape(&trigger) {
        Shape::Aabb(bounds) => {
            for &[p, q] in &box_edges(bounds.min, bounds.max) {
                add(debug_draw_3d::line(p, q));
            }
        }
        Shape::Box(size, isometry) => {
            let origin = Point3::origin();
            let wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
            for &[p, q] in &wireframe {
                add(debug_draw_3d::line(isometry * p, isometry * q));
            }
        }
        Shape::Sphere(radius, center) => {
            for [p, q] in icosphere() {
                let p = center + p.coords * radius;
                let q = center + q.coords * radius;
                add(debug_draw_3d::line(p, q));
            }
        }
        Shape::Cylinder(radius, length, pos) => {
            for [p, q] in cylinder(24) {
                let p = pos + Vector3::new(p.x * radius, p.y * radius, p.z * length);
                let q = pos + Vector3::new(q.x * radius, q.y * radius, q.z * length);
                add(debug_draw_3d::line(p, q));
            }
        }
    }
}

fn marker(position: &Point3<f32>) -> gfc::AutoRef<gfc::StaticObject> {
    let obj = gfc::AutoRef::new(gfc::StaticObject::new());
    obj.set_package_name(&hstring!("vfx_shared"));
    obj.set_object_name(&hstring!("sphere"));
    obj.set_position(position);
    obj
}
