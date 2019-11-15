use crate::darksiders1::{gfc, Lift, LoweredAutoRef};
use darksiders1_sys::target;

struct_wrapper!(Darksiders, target::gfc__Darksiders);
struct_wrapper_super!(Darksiders, gfc::OblivionGame);

impl Darksiders {
    pub fn get_player_actor(&self) -> Option<&gfc::Player> {
        unsafe { self.inner.mPlayerActor.ptr().as_ref().map(Lift::lift_ref) }
    }
}

impl gfc::OblivionGame {
    pub fn get_instance() -> &'static Darksiders {
        gfc::Singleton::<gfc::Darksiders>::get_instance()
    }
}
