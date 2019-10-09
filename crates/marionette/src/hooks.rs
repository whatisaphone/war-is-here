#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use parking_lot::{Mutex, RwLock};
use pdbindgen_runtime::{main_module, BindArgs};
use std::collections::VecDeque;

static DETOURS: RwLock<Option<GodObject>> = RwLock::new(None);
#[allow(clippy::type_complexity)]
pub static ON_POST_UPDATE_QUEUE: Mutex<Option<VecDeque<Box<dyn FnOnce() + Send>>>> =
    Mutex::new(None);

pub struct GodObject {
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
    gfc__Darksiders__processInputEvent: target::gfc__Darksiders__processInputEvent,
    gfc__MeshCache__getStaticMesh: target::gfc__MeshCache__getStaticMesh,
    gfc__MeshCache__loadMesh: target::gfc__MeshCache__loadMesh,
    gfc__Object3DCache__get: target::gfc__Object3DCache__get,
    gfc__ResourceCache__getResource: target::gfc__ResourceCache__getResource,
    _cleanup: Vec<Box<dyn Send + Sync>>,
}

pub fn install() {
    let mut guard = DETOURS.write();
    assert!(guard.is_none());

    unsafe {
        darksiders1_sys::bind(&BindArgs::create(main_module()));

        macro_rules! hook {
            ($name:ident) => {
                TypedDetour::new(target::$name, hook::$name).unwrap()
            };
        }

        let gfc__Darksiders__onPostUpdateInterval = hook!(gfc__Darksiders__onPostUpdateInterval);
        let gfc__Darksiders__processInputEvent = hook!(gfc__Darksiders__processInputEvent);
        let gfc__MeshCache__getStaticMesh = hook!(gfc__MeshCache__getStaticMesh);
        let gfc__MeshCache__loadMesh = hook!(gfc__MeshCache__loadMesh);
        let gfc__Object3DCache__get = hook!(gfc__Object3DCache__get);
        let gfc__ResourceCache__getResource = hook!(gfc__ResourceCache__getResource);

        *guard = Some(GodObject {
            gfc__Darksiders__onPostUpdateInterval: gfc__Darksiders__onPostUpdateInterval
                .trampoline(),
            gfc__Darksiders__processInputEvent: gfc__Darksiders__processInputEvent.trampoline(),
            gfc__MeshCache__getStaticMesh: gfc__MeshCache__getStaticMesh.trampoline(),
            gfc__MeshCache__loadMesh: gfc__MeshCache__loadMesh.trampoline(),
            gfc__Object3DCache__get: gfc__Object3DCache__get.trampoline(),
            gfc__ResourceCache__getResource: gfc__ResourceCache__getResource.trampoline(),
            _cleanup: vec![
                Box::new(gfc__Darksiders__onPostUpdateInterval),
                Box::new(gfc__Darksiders__processInputEvent),
                Box::new(gfc__MeshCache__getStaticMesh),
                Box::new(gfc__MeshCache__loadMesh),
                Box::new(gfc__Object3DCache__get),
                Box::new(gfc__ResourceCache__getResource),
            ],
        });
        *ON_POST_UPDATE_QUEUE.lock() = Some(VecDeque::new());
    }
}

pub fn uninstall() {
    let mut guard = DETOURS.write();
    assert!(guard.is_some());

    // This drops the `GodObject` inside `guard`, which cleans up the detours.
    *guard = None;
}

mod hook {
    use crate::{
        commands::spawn_cube::{override_get_object3d, override_get_static_mesh},
        darksiders1::gfc,
        hooks::{DETOURS, ON_POST_UPDATE_QUEUE},
    };
    use darksiders1_sys::target;

    pub extern "thiscall" fn gfc__Darksiders__onPostUpdateInterval(
        this: *mut target::gfc__Darksiders,
        deltat: f32,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        {
            let mut guard = ON_POST_UPDATE_QUEUE.lock();
            let on_post_update_queue = guard.as_mut().unwrap();
            while let Some(f) = on_post_update_queue.pop_front() {
                f();
            }
        }

        unsafe { (detours.gfc__Darksiders__onPostUpdateInterval)(this, deltat) }
    }

    pub extern "thiscall" fn gfc__Darksiders__processInputEvent(
        this: *mut target::gfc__Darksiders,
        inputEvent: *const target::keen__InputEvent,
    ) -> bool {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        let result = unsafe { (detours.gfc__Darksiders__processInputEvent)(this, inputEvent) };

        // Setting this flag prevents the game from pausing when you deactivate the
        // window.
        unsafe {
            // work around pdbindgen layout issue
            let this_mGameInBackground = (this as usize + 0x1a6) as *mut bool;
            *this_mGameInBackground = false;
        }

        result
    }

    pub extern "thiscall" fn gfc__MeshCache__getStaticMesh(
        this: *mut target::gfc__MeshCache,
        result: *mut target::gfc__AutoRef_gfc__StaticMesh_,
        packageID: i32,
        meshName: *const target::gfc__HString,
        idx: i32,
    ) -> *mut target::gfc__AutoRef_gfc__StaticMesh_ {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        {
            let mesh_name = unsafe { gfc::HString::from_ptr(meshName) };
            if let Some(ovurride) = override_get_static_mesh(packageID, mesh_name, idx) {
                unsafe {
                    *result = gfc::AutoRef2::lower(ovurride);
                }
                return result;
            }
        }

        unsafe { (detours.gfc__MeshCache__getStaticMesh)(this, result, packageID, meshName, idx) }
    }

    pub extern "thiscall" fn gfc__MeshCache__loadMesh(
        this: *mut target::gfc__MeshCache,
        meshRes: *mut target::gfc__MeshResourceUnopt,
        meshIdx: i32,
        stream: target::gfc__AutoRef_gfc__InputStream_,
        name: target::gfc__HString,
        packageID: i32,
    ) -> i32 {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        unsafe {
            (detours.gfc__MeshCache__loadMesh)(this, meshRes, meshIdx, stream, name, packageID)
        }
    }

    pub extern "thiscall" fn gfc__Object3DCache__get(
        this: *mut target::gfc__Object3DCache,
        result: *mut target::gfc__AutoRef_gfc__Object3D_,
        packageID: i32,
        objectName: *const target::gfc__HString,
    ) -> *mut target::gfc__AutoRef_gfc__Object3D_ {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        {
            let object_name = unsafe { gfc::HString::from_ptr(objectName) };
            if let Some(ovurride) = override_get_object3d(packageID, object_name) {
                unsafe {
                    *result = gfc::AutoRef2::lower(ovurride);
                }
                return result;
            }
        }

        unsafe { (detours.gfc__Object3DCache__get)(this, result, packageID, objectName) }
    }

    pub extern "thiscall" fn gfc__ResourceCache__getResource(
        this: *mut target::gfc__ResourceCache,
        packageId: i32,
        hashedName: *const target::gfc__HString,
    ) -> *mut () {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        let result =
            unsafe { (detours.gfc__ResourceCache__getResource)(this, packageId, hashedName) };

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
