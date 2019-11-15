use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(EquippableItem, target::gfc__EquippableItem);
struct_wrapper_super!(EquippableItem, gfc::Item);
