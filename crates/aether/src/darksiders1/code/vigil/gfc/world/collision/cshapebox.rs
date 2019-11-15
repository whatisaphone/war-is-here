use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;
use na::Vector3;

struct_wrapper!(CShapeBox, target::gfc__CShapeBox);
struct_wrapper_super!(CShapeBox, gfc::CShape);
impl_reflection!(CShapeBox, target::gfc__CShapeBox___Class);

impl CShapeBox {
    pub fn size(&self) -> &Vector3<f32> {
        self.inner.mSize.lift_ref()
    }
}
