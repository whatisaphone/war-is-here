use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Visual, target::gfc__Visual);
struct_wrapper_super!(Visual, gfc::Object);
