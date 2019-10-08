use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Object3DCache, target::gfc__Object3DCache);
struct_wrapper_super!(
    Object3DCache,
    gfc::ResourceCache,
    as_gfc__ResourceCache_mut_ptr,
);
