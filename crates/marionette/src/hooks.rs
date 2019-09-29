#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use parking_lot::Mutex;
use pdbindgen_runtime::BindArgs;
use std::collections::VecDeque;

pub static GOD_LOCK: Mutex<Option<GodObject>> = Mutex::new(None);

pub struct GodObject {
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
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

        *guard = Some(GodObject {
            gfc__Darksiders__onPostUpdateInterval: onPostUpdateInterval.trampoline(),
            _cleanup: vec![Box::new(onPostUpdateInterval)],
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
