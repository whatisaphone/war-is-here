use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(RigidBody, target::gfc__RigidBody);
struct_wrapper_super!(RigidBody, gfc::Body);

impl RigidBody {
    pub fn shape(&self) -> &gfc::CShape {
        self.inner.mShape.lift_ref()
    }
}
