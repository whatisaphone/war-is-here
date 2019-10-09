use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(WorldObject, target::gfc__WorldObject);
struct_wrapper_super!(WorldObject, gfc::Object);
