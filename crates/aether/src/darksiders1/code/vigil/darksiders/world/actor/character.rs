use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(Character, target::gfc__Character);
struct_wrapper_super!(Character, gfc::KinematicActor);

impl Character {
    pub fn get_inventory(&self) -> &gfc::Inventory {
        self.inner.mInventory.lift_ref()
    }
}
