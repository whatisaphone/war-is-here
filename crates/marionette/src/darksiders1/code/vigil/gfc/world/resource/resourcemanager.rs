use crate::{
    darksiders1::{gfc, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use std::ptr;

struct_wrapper!(ResourceManager, target::gfc__ResourceManager);

impl ResourceManager {
    pub fn load_packages(&self, package_ids: &[i32], high_priority: bool) {
        unsafe {
            init_with(|p| {
                target::gfc__ResourceManager__loadPackages(
                    self.as_ptr(),
                    p,
                    Lower::lower_ptr(gfc::Vector__SliceAdapter::new(package_ids).as_vector()),
                    high_priority,
                    ptr::null_mut(),
                );
            });
        }
        // TODO: the above returns an AutoRef, we should probably at least free
        // the memory.
    }

    pub fn get_package_id(&self, package_name: &gfc::HString) -> i32 {
        unsafe {
            target::gfc__ResourceManager__getPackageID(
                self.as_ptr(),
                Lower::lower_ptr(package_name),
            )
        }
    }

    pub fn get_permanent_id(&self, runtime_id: i32) -> i32 {
        unsafe { target::gfc__ResourceManager__getPermanentID(self.as_ptr(), runtime_id) }
    }
}
