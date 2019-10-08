use darksiders1_sys::target;

struct_wrapper!(WorldGroup, target::gfc__WorldGroup);
struct_wrapper_super!(WorldGroup, gfc::WorldObject, as_gfc__WorldObject_mut_ptr);
impl_reflection!(WorldGroup, target::gfc__WorldGroup___Class);
