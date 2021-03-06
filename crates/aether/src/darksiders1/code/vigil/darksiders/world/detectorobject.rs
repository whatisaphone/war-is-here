use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(DetectorObject, target::gfc__DetectorObject);
struct_wrapper_super!(DetectorObject, gfc::PhysicsShapeObject);
impl_reflection!(DetectorObject, target::gfc__DetectorObject___Class);

impl DetectorObject {
    pub fn size(&self) -> &gfc::TVector3<f32> {
        self.inner.mSize.lift_ref()
    }

    pub fn bounds(&self) -> &gfc::TBox<f32> {
        self.inner.mBounds.lift_ref()
    }
}

struct_wrapper!(DetectorRegion, target::gfc__DetectorRegion);
struct_wrapper_super!(DetectorRegion, gfc::PhysicsDetectRegion);

impl DetectorRegion {
    pub fn owner(&self) -> &gfc::DetectorObject {
        unsafe { (*self.inner.mOwner).lift_ref() }
    }
}
