use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(OblivionGame, target::gfc__OblivionGame);

impl OblivionGame {
    pub fn get_world(&self) -> &gfc::World {
        unsafe {
            let world = target::gfc__OblivionGame__getWorld(self.as_ptr());
            gfc::World::from_ptr(world)
        }
    }
}
