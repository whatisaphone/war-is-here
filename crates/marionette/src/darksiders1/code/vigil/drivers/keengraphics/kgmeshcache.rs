use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(KGMeshCache, target::gfc__KGMeshCache);
struct_wrapper_super!(KGMeshCache, gfc::MeshCache, as_gfc__MeshCache_mut_ptr);
