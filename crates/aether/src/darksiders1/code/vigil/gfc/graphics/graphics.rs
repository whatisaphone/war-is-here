use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(Graphics, target::gfc__Graphics);

impl Graphics {
    pub fn get_viewport(&self) -> gfc::TRect<i32> {
        let [left, top, right, bottom] = unsafe {
            init_with(|p: *mut [i32; 4]| {
                self.inner
                    .getViewport(&mut (*p)[0], &mut (*p)[1], &mut (*p)[2], &mut (*p)[3]);
            })
        };
        gfc::TRect::ltrb(left, top, right, bottom)
    }
}
