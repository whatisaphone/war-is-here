use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(KGGraphics, target::gfc__KGGraphics);

impl KGGraphics {
    pub fn get_instance() -> &'static Self {
        gfc::Singleton::<Self>::get_instance()
    }
}
