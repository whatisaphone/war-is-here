use crate::darksiders1::gfc;

pub mod gritty_cube;
pub mod wireframe_line;

pub fn override_get_object3d(
    _package_id: i32,
    object_name: &gfc::HString,
) -> Option<gfc::AutoRef<gfc::Object3D>> {
    if object_name == &*gritty_cube::OBJECT_NAME {
        return Some(gritty_cube::build_object());
    } else if object_name == &*wireframe_line::OBJECT_NAME {
        return Some(wireframe_line::build_object());
    }
    None
}

pub fn override_get_static_mesh(
    _package_id: i32,
    mesh_name: &gfc::HString,
    _idx: i32,
) -> Option<gfc::AutoRef<gfc::StaticMesh>> {
    if mesh_name == &*gritty_cube::MESH_NAME {
        return Some(gritty_cube::build_mesh());
    } else if mesh_name == &*wireframe_line::MESH_NAME {
        return Some(wireframe_line::build_mesh());
    }
    None
}

pub fn override_get_material(
    _package_id: i32,
    _material_name: &gfc::HString,
) -> Option<gfc::AutoRef<gfc::Material>> {
    None
}
