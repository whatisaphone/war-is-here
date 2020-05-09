use crate::darksiders1::{gfc, Lift, Lower};
use darksiders1_sys::target;

pub mod playerstattracker;

struct_wrapper!(Player, target::gfc__Player);
struct_wrapper_super!(Player, gfc::Character);

impl Player {
    pub fn stat_tracker(&self) -> &gfc::PlayerStatTracker {
        self.inner.mStatTracker.lift_ref()
    }

    pub fn pickup_item(&self, item: gfc::AutoRef<gfc::Item>) {
        unsafe {
            target::gfc__Player__pickupItem(self.as_ptr(), Lower::lower(item));
        }
    }
}
