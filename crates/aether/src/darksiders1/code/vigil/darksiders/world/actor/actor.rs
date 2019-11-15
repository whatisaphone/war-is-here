use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Actor, target::gfc__Actor);
struct_wrapper_super!(Actor, gfc::WorldObject);
