use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(WorldGroup, target::gfc__WorldGroup);
struct_wrapper_super!(WorldGroup, gfc::WorldObject);
impl_reflection!(WorldGroup, target::gfc__WorldGroup___Class);
