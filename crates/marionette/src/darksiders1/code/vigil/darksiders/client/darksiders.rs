use crate::darksiders1::{gfc, Lift, LoweredAutoRef};
use darksiders1_sys::target;

struct_wrapper!(Darksiders, target::gfc__Darksiders);
struct_wrapper_super!(Darksiders, gfc::OblivionGame);

impl Darksiders {
    pub fn get_player_actor(&self) -> Option<&gfc::Player> {
        // workaround pdbindgen layout bug
        unsafe {
            let player_actor = (self as *const Self)
                .cast::<u8>()
                .add(0x1b8)
                .cast::<target::gfc__AutoRef_gfc__Player_>();
            (*player_actor).ptr().as_ref().map(Lift::lift_ref)
        }
    }
}

impl gfc::OblivionGame {
    pub fn get_instance() -> &'static Darksiders {
        gfc::Singleton::<gfc::Darksiders>::get_instance()
    }
}
