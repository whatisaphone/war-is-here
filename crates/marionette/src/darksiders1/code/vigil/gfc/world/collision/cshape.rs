use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(CShape, target::gfc__CShape);
struct_wrapper_super!(CShape, gfc::Object);
