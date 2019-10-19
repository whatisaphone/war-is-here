use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(TeleportHelper, target::gfc__TeleportHelper);
struct_wrapper_super!(TeleportHelper, gfc::Helper);
