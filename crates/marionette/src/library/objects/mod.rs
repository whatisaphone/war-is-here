use crate::darksiders1::gfc;

pub mod gritty_cube;

pub fn override_get_object3d(
    _package_id: i32,
    object_name: &gfc::HString,
) -> Option<gfc::AutoRef<gfc::Object3D>> {
    if object_name == &*gritty_cube::OBJECT_NAME {
        return Some(gritty_cube::build_object());
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
    }
    None
}
