use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(CShapeMesh, target::gfc__CShapeMesh);
struct_wrapper_super!(CShapeMesh, gfc::CShape);
impl_reflection!(CShapeMesh, target::gfc__CShapeMesh___Class);

impl CShapeMesh {
    pub fn mesh_name(&self) -> &gfc::HString {
        self.inner.mMeshName.lift_ref()
    }

    pub fn mesh_id(&self) -> i32 {
        self.inner.mMeshID
    }
}
