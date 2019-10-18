use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(KinematicActor, target::gfc__KinematicActor);
struct_wrapper_super!(KinematicActor, gfc::Actor);
