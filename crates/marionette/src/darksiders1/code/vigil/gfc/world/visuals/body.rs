use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Body, target::gfc__Body);
struct_wrapper_super!(Body, gfc::Object);
