#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use parking_lot::Mutex;
use pdbindgen_runtime::BindArgs;
use std::mem;

static GOD_LOCK: Mutex<Option<GodObject>> = Mutex::new(None);

struct GodObject {
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
    _cleanup: Vec<Box<dyn Send>>,
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
    let guard = GOD_LOCK.lock();
    let god = guard.as_ref().unwrap();

    unsafe {
        static mut OPENED: bool = false;
        if !OPENED {
            OPENED = true;
            open_load_map_menu();
        }
    }

    unsafe { (god.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
}

fn open_load_map_menu() {
    unsafe {
        let window_helper = *target::gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;

        let mut wndclass = mem::MaybeUninit::uninit();
        target::gfc__HString__HString_3(
            wndclass.as_mut_ptr(),
            b"ui_core/loadmapmenu\0".as_ptr() as *const _,
            false,
        );
        let wndclass = wndclass.assume_init();

        target::gfc__WindowHelper__pushWindow(window_helper, &wndclass);
    }
}
