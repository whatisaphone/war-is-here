use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;

struct_wrapper!(Character, target::gfc__Character);
struct_wrapper_super!(Character, gfc::KinematicActor);

impl Character {
    pub fn get_inventory(&self) -> &gfc::Inventory {
        // workaround pdbindgen layout bug
        unsafe {
            let inventory = (self as *const Self)
                .cast::<u8>()
                .add(0x264)
                .cast::<target::gfc__AutoRef_gfc__Inventory_>();
            (*inventory).lift_ref()
        }
    }
}
