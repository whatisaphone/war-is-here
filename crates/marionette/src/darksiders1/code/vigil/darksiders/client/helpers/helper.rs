use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Helper, target::gfc__Helper);
struct_wrapper_super!(Helper, gfc::Object);
