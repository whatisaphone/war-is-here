#![allow(non_snake_case)]

use crate::utils::{detour::TypedDetour, ffi::lock_xadd};
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
            conjure_object();
        }
    }

    unsafe { (god.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
}

unsafe fn conjure_object() {
    let darksiders = *target::gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    #[allow(clippy::cast_ptr_alignment)]
    let world_mgr = (*darksiders).mWorldMgr.p as *mut target::gfc__WorldManager;
    let world = (*world_mgr).mWorld.p as *mut target::gfc__World;

    let class_registry = *target::gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    let class = target::gfc__ClassRegistry__classForName(
        class_registry,
        &hstring!("vulgrim_chime/vulgrim_chime_medium"),
        true,
        false,
    );
    let mut chime = mem::MaybeUninit::uninit();
    let newInstance = mem::transmute::<
        unsafe extern "thiscall" fn(
            this: *mut target::gfc__Class,
        ) -> target::gfc__AutoRef_gfc__Object_,
        unsafe extern "thiscall" fn(
            this: *mut target::gfc__Class,
            result: *mut target::gfc__AutoRef_gfc__Object_,
        ) -> *mut target::gfc__AutoRef_gfc__Object_,
    >((*(*class).__vfptr).newInstance);
    newInstance(class, chime.as_mut_ptr());
    let chime = chime.assume_init();
    lock_xadd(&mut (*chime.p).ReferenceCount, 1);

    #[allow(clippy::cast_ptr_alignment)]
    let chime = chime.p as *mut target::gfc__Actor;

    target::gfc__Actor__setPosition(chime, &target::gfc__TVector3_float_gfc__FloatMath_ {
        x: -4000.0,
        y: -28000.0,
        z: 200.0,
    });

    ((*(*chime).__vfptr).addObjectToWorld)((*chime).as_gfc__WorldObject_mut_ptr(), world);
}
