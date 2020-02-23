use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(Graphics, target::gfc__Graphics);

impl Graphics {
    pub fn get_viewport(&self) -> gfc::TRect<i32> {
        unsafe {
            init_with(|p: *mut gfc::TRect<i32>| {
                self.inner.getViewport(
                    &mut (*p).left,
                    &mut (*p).top,
                    &mut (*p).right,
                    &mut (*p).bottom,
                );
            })
        }
    }
}
