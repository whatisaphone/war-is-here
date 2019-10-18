use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Item, target::gfc__Item);
struct_wrapper_super!(Item, gfc::KinematicActor);
impl_reflection!(Item, target::gfc__Item___Class);
