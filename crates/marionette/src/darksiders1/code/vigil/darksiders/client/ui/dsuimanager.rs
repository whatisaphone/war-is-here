use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(DSUIManager, target::gfc__DSUIManager);
struct_wrapper_super!(DSUIManager, gfc::_UIManager);
