use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(OblivionGame, target::gfc__OblivionGame);

impl OblivionGame {
    /// Returns the current world, if there is one. Otherwise (main menu,
    /// loading screen, etc.) returns `None`.
    pub fn get_world(&self) -> Option<gfc::AutoRef<gfc::World>> {
        unsafe {
            let world = target::gfc__OblivionGame__getWorld(self.as_ptr());
            if world.is_null() {
                return None;
            }
            Some(gfc::AutoRef::from_ptr(Lift::lift_ptr(world)))
        }
    }
}
