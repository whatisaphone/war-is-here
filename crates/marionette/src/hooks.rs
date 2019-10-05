#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use parking_lot::{Mutex, RwLock};
use pdbindgen_runtime::BindArgs;
use std::collections::VecDeque;

pub static GOD_LOCK: RwLock<Option<GodObject>> = RwLock::new(None);
#[allow(clippy::type_complexity)]
pub static ON_POST_UPDATE_QUEUE: Mutex<Option<VecDeque<Box<dyn FnOnce() + Send>>>> =
    Mutex::new(None);

pub struct GodObject {
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
    gfc__Darksiders__processInputEvent: target::gfc__Darksiders__processInputEvent,
    gfc__MeshReader__readObject: target::gfc__MeshReader__readObject,
    gfc__ResourceCache__getResource: target::gfc__ResourceCache__getResource,
    _cleanup: Vec<Box<dyn Send + Sync>>,
}

pub fn install() {
    let mut guard = GOD_LOCK.write();
    assert!(guard.is_none());

    unsafe {
        darksiders1_sys::bind(&BindArgs::create());

        macro_rules! hook {
            ($name:ident) => {
                TypedDetour::new(target::$name, hook::$name).unwrap()
            };
        }

        let gfc__Darksiders__onPostUpdateInterval = hook!(gfc__Darksiders__onPostUpdateInterval);
        let gfc__Darksiders__processInputEvent = hook!(gfc__Darksiders__processInputEvent);
        let gfc__MeshReader__readObject = hook!(gfc__MeshReader__readObject);
        let gfc__ResourceCache__getResource = hook!(gfc__ResourceCache__getResource);

        *guard = Some(GodObject {
            gfc__Darksiders__onPostUpdateInterval: gfc__Darksiders__onPostUpdateInterval
                .trampoline(),
            gfc__Darksiders__processInputEvent: gfc__Darksiders__processInputEvent.trampoline(),
            gfc__MeshReader__readObject: gfc__MeshReader__readObject.trampoline(),
            gfc__ResourceCache__getResource: gfc__ResourceCache__getResource.trampoline(),
            _cleanup: vec![
                Box::new(gfc__Darksiders__onPostUpdateInterval),
                Box::new(gfc__Darksiders__processInputEvent),
                Box::new(gfc__MeshReader__readObject),
                Box::new(gfc__ResourceCache__getResource),
            ],
        });
        *ON_POST_UPDATE_QUEUE.lock() = Some(VecDeque::new());
    }
}

pub fn uninstall() {
    let mut guard = GOD_LOCK.write();
    assert!(guard.is_some());

    // This drops the `GodObject` inside `guard`, which cleans up the detours.
    *guard = None;
}

mod hook {
    use crate::{
        darksiders1::gfc,
        hooks::{GOD_LOCK, ON_POST_UPDATE_QUEUE},
        utils::exfil::dump_object,
    };
    use darksiders1_sys::target;

    pub extern "thiscall" fn gfc__Darksiders__onPostUpdateInterval(
        this: *mut target::gfc__Darksiders,
        deltat: f32,
    ) {
        let guard = GOD_LOCK.read();
        let god = guard.as_ref().unwrap();

        {
            let mut guard = ON_POST_UPDATE_QUEUE.lock();
            let on_post_update_queue = guard.as_mut().unwrap();
            while let Some(f) = on_post_update_queue.pop_front() {
                f();
            }
        }

        unsafe { (god.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
    }

    pub extern "thiscall" fn gfc__Darksiders__processInputEvent(
        this: *mut target::gfc__Darksiders,
        inputEvent: *const target::keen__InputEvent,
    ) -> bool {
        let guard = GOD_LOCK.read();
        let god = guard.as_ref().unwrap();

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

    pub extern "thiscall" fn gfc__MeshReader__readObject(
        this: *mut target::gfc__MeshReader,
        result: *mut target::gfc__AutoRef_gfc__Object_,
        r#in: target::gfc__AutoRef_gfc__InputStream_,
        valid: *mut bool,
    ) -> *mut target::gfc__AutoRef_gfc__Object_ {
        let guard = GOD_LOCK.read();
        let god = guard.as_ref().unwrap();

        let result = unsafe { (god.gfc__MeshReader__readObject)(this, result, r#in, valid) };

        unsafe {
            dump_object(gfc::Object::from_ptr((*result).p.cast()));
        }

        result
    }

    pub extern "thiscall" fn gfc__ResourceCache__getResource(
        this: *mut target::gfc__ResourceCache,
        packageId: i32,
        hashedName: *const target::gfc__HString,
    ) -> *mut () {
        let guard = GOD_LOCK.read();
        let god = guard.as_ref().unwrap();

        let result = unsafe { (god.gfc__ResourceCache__getResource)(this, packageId, hashedName) };

        if result.is_null() {
            let hashed_name = unsafe { gfc::HString::from_ptr(hashedName) };
            println!(
                "failed to load resource. packageId = {}, hashedName = {:?}",
                packageId,
                hashed_name.c_str().to_str().unwrap_or("<invalid utf-8>"),
            );
        }

        result
    }
}
