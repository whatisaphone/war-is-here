use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(RegionLayer, target::gfc__RegionLayer);
struct_wrapper_super!(RegionLayer, gfc::Object);

impl RegionLayer {
    pub fn root(&self) -> gfc::AutoRef<gfc::WorldGroup> {
        let result = unsafe {
            init_with(|p| {
                target::gfc__RegionLayer__getRoot(self.as_ptr(), p);
            })
        };
        result.lift()
    }
}
