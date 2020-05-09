use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(PlayerStatTracker, target::gfc__PlayerStatTracker);
struct_wrapper_super!(PlayerStatTracker, gfc::Object);

impl PlayerStatTracker {
    pub fn total_game_time(&self) -> u32 {
        self.inner.mTotalGameTime
    }

    pub fn delta_time(&self) -> f32 {
        self.inner.mDeltaTime
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn total_game_time_precise(&self) -> f32 {
        self.total_game_time() as f32 + self.delta_time()
    }
}
