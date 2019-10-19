use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target::{self, hkpRigidBody};

struct_wrapper!(RigidBody, target::gfc__RigidBody);
struct_wrapper_super!(RigidBody, gfc::Body);

impl RigidBody {
    pub fn shape(&self) -> &gfc::CShape {
        self.inner.mShape.lift_ref()
    }

    pub fn rigid_body_raw(&self) -> &hkpRigidBody {
        // work around pdbindgen bug
        unsafe {
            &**self
                .as_ptr()
                .cast::<u8>()
                .add(52)
                .cast::<*mut hkpRigidBody>()
                .as_ref()
                .unwrap()
        }
    }
}
