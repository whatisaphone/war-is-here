use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(Body, target::gfc__Body);
struct_wrapper_super!(Body, gfc::Object);

impl Body {
    pub fn object(&self) -> Option<&gfc::Object3D> {
        unsafe { Some(self.inner.mObject.as_ref()?.lift_ref()) }
    }
}
