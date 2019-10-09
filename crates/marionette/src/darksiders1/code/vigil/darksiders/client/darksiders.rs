use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Darksiders, target::gfc__Darksiders);
struct_wrapper_super!(Darksiders, gfc::OblivionGame);

impl gfc::OblivionGame {
    pub fn get_instance() -> &'static Darksiders {
        gfc::Singleton::<gfc::Darksiders>::get_instance()
    }
}
