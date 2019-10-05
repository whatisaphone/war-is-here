use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Object3D, target::gfc__Object3D);
inherits!(Object3D, gfc::Object, as_gfc__Object_mut_ptr);
