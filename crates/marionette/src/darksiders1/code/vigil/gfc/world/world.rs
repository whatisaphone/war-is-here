use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(World, target::gfc__World);
struct_wrapper_super!(World, gfc::Object);
