use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(_UIControl, target::gfc___UIControl);
struct_wrapper_super!(_UIControl, gfc::Object);

impl _UIControl {
    pub fn name(&self) -> &gfc::HString {
        self.inner.mName.lift_ref()
    }
}
