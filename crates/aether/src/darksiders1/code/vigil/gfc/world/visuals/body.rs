use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(Body, target::gfc__Body);
struct_wrapper_super!(Body, gfc::Object);

impl Body {
    pub fn object(&self) -> &gfc::Object3D {
        unsafe { (*self.inner.mObject).lift_ref() }
    }
}
