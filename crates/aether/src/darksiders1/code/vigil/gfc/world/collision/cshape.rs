use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(CShape, target::gfc__CShape);
struct_wrapper_super!(CShape, gfc::Object);
impl_reflection!(CShape, target::gfc__CShape___Class);
