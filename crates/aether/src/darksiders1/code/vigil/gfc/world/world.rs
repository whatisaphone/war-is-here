use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(WorldRegionData, target::gfc__WorldRegionData);
struct_wrapper_super!(WorldRegionData, gfc::Object);

struct_wrapper!(World, target::gfc__World);
struct_wrapper_super!(World, gfc::Object);
