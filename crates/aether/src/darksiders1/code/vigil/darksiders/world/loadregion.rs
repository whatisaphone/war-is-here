use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(LoadRegion, target::gfc__LoadRegion);
struct_wrapper_super!(LoadRegion, gfc::DetectorObject);
impl_reflection!(LoadRegion, target::gfc__LoadRegion___Class);
