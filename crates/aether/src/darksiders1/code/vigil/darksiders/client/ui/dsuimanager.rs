use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(DSUIManager, target::gfc__DSUIManager);
struct_wrapper_super!(DSUIManager, gfc::_UIManager);

impl gfc::DSUIManager {
    pub fn get_instance() -> &'static Self {
        <gfc::Singleton<Self>>::get_instance()
    }
}
