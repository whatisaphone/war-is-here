use crate::darksiders1::gfc;
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn pump() {
    let enabled = ENABLED.load(Ordering::SeqCst);
    if !enabled {
        return;
    }

    let darksiders = gfc::OblivionGame::get_instance();
    let player = match darksiders.get_player_actor() {
        Some(p) => p,
        None => return,
    };

    let jump_down = unsafe { (*player.as_ptr()).mMoveInput.JumpDown };
    if !jump_down {
        return;
    }

    let limit = 350.0; // PlayerJumpDesc.JumpImpulse is 350.
    if player.gravity() < limit {
        player.set_gravity(limit);
    }
}
