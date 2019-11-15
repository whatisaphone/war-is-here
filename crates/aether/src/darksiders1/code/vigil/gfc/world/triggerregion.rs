use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(TriggerRegion, target::gfc__TriggerRegion);
struct_wrapper_super!(TriggerRegion, gfc::DetectorObject);
impl_reflection!(TriggerRegion, target::gfc__TriggerRegion___Class);
