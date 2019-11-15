use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(PhysMeshCache, target::gfc__PhysMeshCache);
struct_wrapper_super!(PhysMeshCache, gfc::ResourceCache);
