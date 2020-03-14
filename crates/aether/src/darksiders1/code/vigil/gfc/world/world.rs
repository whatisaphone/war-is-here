use crate::{
    darksiders1::{gfc, Lift, LoweredAutoRef},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(WorldRegionData, target::gfc__WorldRegionData);
struct_wrapper_super!(WorldRegionData, gfc::Object);

struct_wrapper!(World, target::gfc__World);
struct_wrapper_super!(World, gfc::Object);

impl World {
    pub fn root(&self) -> &gfc::WorldGroup {
        unsafe { (*self.as_ptr()).mRoot.lift_ref() }
    }

    pub fn region_data(&self) -> &gfc::Vector<gfc::AutoRef<gfc::WorldRegionData>> {
        unsafe { (*self.as_ptr()).mRegionData.lift_ref() }
    }

    pub fn get_region(&self, idx: i32) -> Option<gfc::AutoRef<gfc::WorldRegion>> {
        let result = unsafe {
            init_with(|p| {
                target::gfc__World__getRegion(self.as_ptr(), p, idx);
            })
        };
        if result.ptr().is_null() {
            None
        } else {
            Some(result.lift())
        }
    }
}
