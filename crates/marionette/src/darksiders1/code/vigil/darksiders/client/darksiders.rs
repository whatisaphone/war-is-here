use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Darksiders, target::gfc__Darksiders);
inherits!(Darksiders, gfc::OblivionGame, as_gfc__OblivionGame_mut_ptr);

impl gfc::OblivionGame {
    pub fn get_instance() -> &'static Darksiders {
        gfc::Singleton::<gfc::Darksiders>::get_instance()
    }
}
