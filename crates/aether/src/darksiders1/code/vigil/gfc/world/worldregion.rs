use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(RegionLayerData, target::gfc__RegionLayerData);
struct_wrapper_super!(RegionLayerData, gfc::Object);
