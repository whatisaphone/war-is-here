#![allow(non_snake_case)]

use crate::utils::{
    detour::TypedDetour,
    ffi::{lock_xadd, new},
};
use darksiders1_sys::target;
use parking_lot::Mutex;
use pdbindgen_runtime::BindArgs;

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
            shine_a_light();
        }
    }

    unsafe { (god.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
}

#[allow(dead_code)]
unsafe fn player_pos() {
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let player = (*darksiders).mPlayerActor.p as *mut target::gfc__Player;
    let position = ((*(*player).__vfptr).getPosition)(player as *mut _);
    eprintln!("position.x = {:?}", position.x);
    eprintln!("position.y = {:?}", position.y);
    eprintln!("position.z = {:?}", position.z);
}

unsafe fn shine_a_light() {
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders).mWorldMgr.p as *mut target::gfc__WorldManager;
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let light = new(|this| target::gfc__OmniLight__OmniLight(this));
    lock_xadd(&mut (*light).ReferenceCount, 100); // huge number so memory isn't freed (workaround for bug in `new`)

    eprintln!("light.mName.mHash = {:016x}", (*light).mName.mHash);
    eprintln!("light.mWorld as u32 = {:08x}", (*light).mWorld as u32);
    eprintln!("light.mEnabled = {:?}", (*light).mEnabled);

    target::gfc__WorldObject__setPosition(
        (*(*light).as_gfc__WorldLight_mut_ptr()).as_gfc__WorldObject_mut_ptr(),
        -4000.0,
        -28000.0,
        100.0,
    );
    target::gfc__OmniLight__setPower(&mut *light, 300.0);
    target::gfc__OmniLight__setAttenEnd(&mut *light, 1000.0);

    target::gfc__WorldObject__addToWorld(
        (*(*light).as_gfc__WorldLight_mut_ptr()).as_gfc__WorldObject_mut_ptr(),
        world,
    );
}
