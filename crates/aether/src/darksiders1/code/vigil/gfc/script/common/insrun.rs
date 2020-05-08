use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(InsRun, target::gfc__InsRun);
struct_wrapper_super!(InsRun, gfc::InsExecutor);

impl InsRun {
    pub fn stack(&self) -> &gfc::ValueStack {
        unsafe { (*self.inner.mStack).lift_ref() }
    }
}
