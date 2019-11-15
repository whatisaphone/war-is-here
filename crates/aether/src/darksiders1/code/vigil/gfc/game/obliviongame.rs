use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(OblivionGame, target::gfc__OblivionGame);

impl OblivionGame {
    /// Returns the current world, if there is one. Otherwise (main menu,
    /// loading screen, etc.) returns `None`.
    ///
    /// # Safety
    ///
    /// Only call this from inside the postUpdate game hook. The reference might
    /// become invalid between different calls to the hook.
    pub unsafe fn get_world(&self) -> Option<&gfc::World> {
        let world = target::gfc__OblivionGame__getWorld(self.as_ptr());
        if world.is_null() {
            return None;
        }
        Some(gfc::World::from_ptr(world))
    }
}
