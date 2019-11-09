#![allow(non_snake_case)]

use crate::utils::detour::TypedDetour;
use darksiders1_sys::target;
use detour::RawDetour;
use parking_lot::{Mutex, RwLock};
use pdbindgen_runtime::{main_module, BindArgs};
use std::collections::VecDeque;

static DETOURS: RwLock<Option<Detours>> = RwLock::new(None);
#[allow(clippy::type_complexity)]
pub static ON_POST_UPDATE_QUEUE: Mutex<Option<VecDeque<Box<dyn FnOnce() + Send>>>> =
    Mutex::new(None);

struct Detours {
    gfc___UIManager__draw: target::gfc___UIManager__draw,
    gfc__Darksiders__onPostUpdateInterval: target::gfc__Darksiders__onPostUpdateInterval,
    gfc__Darksiders__processInputEvent: target::gfc__Darksiders__processInputEvent,
    gfc__MaterialCache__get: target::gfc__MaterialCache__get,
    gfc__MeshCache__getStaticMesh: target::gfc__MeshCache__getStaticMesh,
    gfc__MeshCache__loadMesh: target::gfc__MeshCache__loadMesh,
    gfc__Object3DCache__get: target::gfc__Object3DCache__get,
    gfc__ResourceCache__getResource: target::gfc__ResourceCache__getResource,
    gfc__World__World: target::gfc__World__World,
    detours: Vec<RawDetour>,
}

pub fn install() {
    let mut guard = DETOURS.write();
    assert!(guard.is_none());

    unsafe {
        darksiders1_sys::bind(&BindArgs::create(main_module()).unwrap());

        macro_rules! hook {
            ($($name:ident),* $(,)?) => {
                $(
                    let $name = TypedDetour::create(target::$name, hook::$name).unwrap();
                )*

                *guard = Some(Detours {
                    $($name: $name.trampoline()),*,
                    detours: vec![
                        $($name.into_inner()),*,
                    ],
                });
            };
        }

        hook!(
            gfc___UIManager__draw,
            gfc__Darksiders__onPostUpdateInterval,
            gfc__Darksiders__processInputEvent,
            gfc__MaterialCache__get,
            gfc__MeshCache__getStaticMesh,
            gfc__MeshCache__loadMesh,
            gfc__Object3DCache__get,
            gfc__ResourceCache__getResource,
            gfc__World__World,
        );

        *ON_POST_UPDATE_QUEUE.lock() = Some(VecDeque::new());
    }
}

pub fn uninstall() {
    let guard = DETOURS.write();
    let detours = guard.as_ref().unwrap();
    assert!(guard.is_some());

    for detour in &detours.detours {
        unsafe { detour.disable() }.unwrap();
    }
}

pub fn cleanup() {
    let mut guard = DETOURS.write();
    assert!(guard.is_some());

    drop(guard.take());
}

mod hook {
    use crate::{
        commands::{pretend_editor, show_collision, show_triggers, spawn_humans},
        darksiders1::{Lift, Lower},
        hooks::{DETOURS, ON_POST_UPDATE_QUEUE},
        library::objects::{
            override_get_material,
            override_get_object3d,
            override_get_static_mesh,
        },
    };
    use darksiders1_sys::target;

    pub unsafe extern "thiscall" fn gfc___UIManager__draw(
        this: *mut target::gfc___UIManager,
        renderer: *mut target::gfc__UIRenderer,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc___UIManager__draw)(this, renderer);

        show_triggers::draw((*renderer).lift_ref());
        show_collision::draw((*renderer).lift_ref());
    }

    pub unsafe extern "thiscall" fn gfc__Darksiders__onPostUpdateInterval(
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

        spawn_humans::pump();

        (detours.gfc__Darksiders__onPostUpdateInterval)(this, deltat)
    }

    pub unsafe extern "thiscall" fn gfc__Darksiders__processInputEvent(
        this: *mut target::gfc__Darksiders,
        inputEvent: *const target::keen__InputEvent,
    ) -> bool {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        let result = (detours.gfc__Darksiders__processInputEvent)(this, inputEvent);

        // Setting this flag prevents the game from pausing when you deactivate the
        // window.
        (*this).mGameInBackground = false;

        result
    }

    pub unsafe extern "thiscall" fn gfc__MaterialCache__get(
        this: *mut target::gfc__MaterialCache,
        result: *mut target::gfc__AutoRef_gfc__Material_,
        packageID: i32,
        materialName: *const target::gfc__HString,
    ) -> *mut target::gfc__AutoRef_gfc__Material_ {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        let material_name = (*materialName).lift_ref();
        if let Some(ovurride) = override_get_material(packageID, material_name) {
            *result = Lower::lower(ovurride);
            return result;
        }

        (detours.gfc__MaterialCache__get)(this, result, packageID, materialName)
    }

    pub unsafe extern "thiscall" fn gfc__MeshCache__getStaticMesh(
        this: *mut target::gfc__MeshCache,
        result: *mut target::gfc__AutoRef_gfc__StaticMesh_,
        packageID: i32,
        meshName: *const target::gfc__HString,
        idx: i32,
    ) -> *mut target::gfc__AutoRef_gfc__StaticMesh_ {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        {
            let mesh_name = (*meshName).lift_ref();
            if let Some(ovurride) = override_get_static_mesh(packageID, mesh_name, idx) {
                *result = Lower::lower(ovurride);
                return result;
            }
        }

        (detours.gfc__MeshCache__getStaticMesh)(this, result, packageID, meshName, idx)
    }

    pub unsafe extern "thiscall" fn gfc__MeshCache__loadMesh(
        this: *mut target::gfc__MeshCache,
        meshRes: *mut target::gfc__MeshResourceUnopt,
        meshIdx: i32,
        stream: target::gfc__AutoRef_gfc__InputStream_,
        name: target::gfc__HString,
        packageID: i32,
    ) -> i32 {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc__MeshCache__loadMesh)(this, meshRes, meshIdx, stream, name, packageID)
    }

    pub unsafe extern "thiscall" fn gfc__Object3DCache__get(
        this: *mut target::gfc__Object3DCache,
        result: *mut target::gfc__AutoRef_gfc__Object3D_,
        packageID: i32,
        objectName: *const target::gfc__HString,
    ) -> *mut target::gfc__AutoRef_gfc__Object3D_ {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        {
            let object_name = (*objectName).lift_ref();
            if let Some(ovurride) = override_get_object3d(packageID, object_name) {
                *result = Lower::lower(ovurride);
                return result;
            }
        }

        (detours.gfc__Object3DCache__get)(this, result, packageID, objectName)
    }

    pub unsafe extern "thiscall" fn gfc__ResourceCache__getResource(
        this: *mut target::gfc__ResourceCache,
        packageId: i32,
        hashedName: *const target::gfc__HString,
    ) -> *mut () {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        let result = (detours.gfc__ResourceCache__getResource)(this, packageId, hashedName);

        if result.is_null() {
            let hashed_name = (*hashedName).lift_ref();
            println!(
                "failed to load resource. packageId = {}, hashedName = {:?}",
                packageId,
                hashed_name.c_str().to_str().unwrap_or("<invalid utf-8>"),
            );
        }

        result
    }

    pub unsafe extern "thiscall" fn gfc__World__World(
        this: *mut target::gfc__World,
    ) -> *mut target::gfc__World {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc__World__World)(this);

        pretend_editor::world_constructor_hook(this);

        this
    }
}
