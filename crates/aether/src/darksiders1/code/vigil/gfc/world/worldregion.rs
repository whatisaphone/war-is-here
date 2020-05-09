use crate::{
    darksiders1::{gfc, Lift, LoweredAutoRef},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(RegionLayerData, target::gfc__RegionLayerData);
struct_wrapper_super!(RegionLayerData, gfc::Object);

struct_wrapper!(WorldRegion, target::gfc__WorldRegion);
struct_wrapper_super!(WorldRegion, gfc::Object);

impl WorldRegion {
    pub fn name(&self) -> &gfc::HString {
        self.inner.mName.lift_ref()
    }

    pub fn layer_data(&self) -> &gfc::Vector<gfc::AutoRef<gfc::RegionLayerData>> {
        unsafe { (*self.as_ptr()).mLayerData.lift_ref() }
    }

    pub fn load_regions(&self) -> &gfc::Vector<gfc::AutoRef<gfc::WorldObject>> {
        unsafe { (*self.as_ptr()).mLoadRegions.lift_ref() }
    }

    pub fn get_layer(&self, idx: i32) -> Option<gfc::AutoRef<gfc::RegionLayer>> {
        let result = unsafe {
            init_with(|p| {
                target::gfc__WorldRegion__getLayer(self.as_ptr(), p, idx);
            })
        };
        if result.ptr().is_null() {
            None
        } else {
            Some(result.lift())
        }
    }
}
