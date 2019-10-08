use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Mesh, target::gfc__Mesh);
struct_wrapper_super!(Mesh, gfc::IRefObject, as_gfc__IRefObject_mut_ptr);

struct_wrapper!(StaticMesh, target::gfc__StaticMesh);
struct_wrapper_super!(StaticMesh, gfc::Mesh, as_gfc__Mesh_mut_ptr);
