use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(MeshCache, target::gfc__MeshCache);
struct_wrapper_super!(MeshCache, gfc::ResourceCache);
