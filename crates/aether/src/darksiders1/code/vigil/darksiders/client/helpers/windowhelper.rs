use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(WindowHelper, target::gfc__WindowHelper);
struct_wrapper_super!(WindowHelper, gfc::Helper);

impl WindowHelper {
    pub fn push_window(&self, wndclass: &gfc::HString) {
        unsafe {
            target::gfc__WindowHelper__pushWindow(self.as_ptr(), wndclass.as_ptr());
        }
    }

    pub fn get_window(&self) -> gfc::AutoRef<gfc::_UIControl> {
        self.inner.mWindow.lift_ref().clone()
    }
}
