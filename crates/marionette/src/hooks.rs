#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use parking_lot::Mutex;
use pdbindgen_runtime::BindArgs;
use std::collections::VecDeque;

pub static GOD_LOCK: Mutex<Option<GodObject>> = Mutex::new(None);

pub struct GodObject {
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
    gfc__Darksiders__processInputEvent: target::gfc__Darksiders__processInputEvent,
    _cleanup: Vec<Box<dyn Send>>,
    pub on_post_update_queue: VecDeque<Box<dyn FnOnce() + Send>>,
}

pub fn install() {
    let mut guard = GOD_LOCK.lock();
    assert!(guard.is_none());

    unsafe {
        darksiders1_sys::bind(&BindArgs::create());

        let onPostUpdateInterval = TypedDetour::new(
            target::gfc__Darksiders__onPostUpdateInterval,
            hook_gfc__Darksiders__onPostUpdateInterval,
        )
        .unwrap();

        let processInputEvent = TypedDetour::new(
            target::gfc__Darksiders__processInputEvent,
            hook_gfc__Darksiders__processInputEvent,
        )
        .unwrap();

        *guard = Some(GodObject {
            gfc__Darksiders__onPostUpdateInterval: onPostUpdateInterval.trampoline(),
            gfc__Darksiders__processInputEvent: processInputEvent.trampoline(),
            _cleanup: vec![Box::new(onPostUpdateInterval), Box::new(processInputEvent)],
            on_post_update_queue: VecDeque::new(),
        });
    }
}

pub fn uninstall() {
    let mut guard = GOD_LOCK.lock();
    assert!(guard.is_some());

    // This drops the `GodObject` inside `guard`, which cleans up the detours.
    *guard = None;
}

extern "thiscall" fn hook_gfc__Darksiders__onPostUpdateInterval(
    this: *mut target::gfc__Darksiders,
    deltat: f32,
) {
    let mut guard = GOD_LOCK.lock();
    let god = guard.as_mut().unwrap();

    while let Some(f) = god.on_post_update_queue.pop_front() {
        f();
    }

    unsafe { (god.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
}

extern "thiscall" fn hook_gfc__Darksiders__processInputEvent(
    this: *mut target::gfc__Darksiders,
    inputEvent: *const target::keen__InputEvent,
) -> bool {
    let mut guard = GOD_LOCK.lock();
    let god = guard.as_mut().unwrap();

    let result = unsafe { (god.gfc__Darksiders__processInputEvent)(this, inputEvent) };

    // Setting this flag prevents the game from pausing when you deactivate the
    // window.
    unsafe {
        // work around pdbindgen layout issue
        let this_mGameInBackground = (this as usize + 0x1a6) as *mut bool;
        *this_mGameInBackground = false;
    }

    result
}
