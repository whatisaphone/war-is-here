#![allow(non_snake_case)]

use crate::{
    commands::{console, log_events},
    ui,
    utils::detour::TypedDetour,
};
use darksiders1_sys::target;
use detour::RawDetour;
use parking_lot::{Mutex, RwLock};
use pdbindgen_runtime::{main_module, BindArgs};
use std::{collections::VecDeque, sync::atomic::Ordering, thread, time::Duration};

static DETOURS: RwLock<Option<Detours>> = RwLock::new(None);
#[allow(clippy::type_complexity)]
pub static ON_POST_UPDATE_QUEUE: Mutex<Option<VecDeque<Box<dyn FnOnce() + Send>>>> =
    Mutex::new(None);

struct Detours {
    gfc___UIManager__draw: target::gfc___UIManager__draw,
    gfc__Darksiders__processInputEvent: target::gfc__Darksiders__processInputEvent,
    gfc__DSSaveGameManager__saveGame: target::gfc__DSSaveGameManager__saveGame,
    gfc__DebugOutModule__execute: target::gfc__DebugOutModule__execute,
    gfc__DetectorRegion__bodyEntered: target::gfc__DetectorRegion__bodyEntered,
    gfc__DetectorRegion__bodyExited: target::gfc__DetectorRegion__bodyExited,
    gfc__InsRun__doPrint: target::gfc__InsRun__doPrint,
    gfc__MaterialCache__get: target::gfc__MaterialCache__get,
    gfc__MeshCache__getStaticMesh: target::gfc__MeshCache__getStaticMesh,
    gfc__MeshCache__loadMesh: target::gfc__MeshCache__loadMesh,
    gfc__Object3DCache__get: target::gfc__Object3DCache__get,
    gfc__OblivionGame__update: target::gfc__OblivionGame__update,
    gfc__Player__setSpawnPoint_2: target::gfc__Player__setSpawnPoint_2,
    gfc__ResourceCache__getResource: target::gfc__ResourceCache__getResource,
    gfc__SaveData__setValue: target::gfc__SaveData__setValue,
    gfc__World__World: target::gfc__World__World,
    gfc__WorldRegion__preload: target::gfc__WorldRegion__preload,
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
            gfc__DSSaveGameManager__saveGame,
            gfc__Darksiders__processInputEvent,
            gfc__DebugOutModule__execute,
            gfc__DetectorRegion__bodyEntered,
            gfc__DetectorRegion__bodyExited,
            gfc__InsRun__doPrint,
            gfc__MaterialCache__get,
            gfc__MeshCache__getStaticMesh,
            gfc__MeshCache__loadMesh,
            gfc__Object3DCache__get,
            gfc__OblivionGame__update,
            gfc__Player__setSpawnPoint_2,
            gfc__ResourceCache__getResource,
            gfc__SaveData__setValue,
            gfc__World__World,
            gfc__WorldRegion__preload,
        );
    }

    *ON_POST_UPDATE_QUEUE.lock() = Some(VecDeque::new());
}

pub fn uninstall() {
    // Disable anything that requires the ui to be loaded.
    console::hide();
    log_events::disable();

    // Wait for the UI to become cleaned up. This must happen on the main thread.
    while ui::IS_ENABLED.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(10));
    }

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
        commands::{
            editor_mode,
            fps,
            infinite_jump,
            log_events,
            show_collision,
            show_player_pos,
            show_triggers,
            spawn_humans,
        },
        darksiders1::{gfc, Lift, Lower},
        hooks::{DETOURS, ON_POST_UPDATE_QUEUE},
        library::objects::{
            override_get_material,
            override_get_object3d,
            override_get_static_mesh,
        },
        splash,
        ui,
    };
    use darksiders1_sys::target;

    pub unsafe extern "thiscall" fn gfc___UIManager__draw(
        this: *mut target::gfc___UIManager,
        renderer: *mut target::gfc__UIRenderer,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc___UIManager__draw)(this, renderer);

        let renderer = (*renderer).lift_ref();
        splash::draw(renderer);
        fps::draw(renderer);
        show_player_pos::draw(renderer);
        show_triggers::draw(renderer);
        show_collision::draw(renderer);

        ui::pump();
    }

    // Return `false` to swallow the event, or `true` to continue processing
    // normally.
    #[allow(clippy::shadow_unrelated)]
    pub unsafe extern "thiscall" fn gfc__Darksiders__processInputEvent(
        this: *mut target::gfc__Darksiders,
        inputEvent: *const target::keen__InputEvent,
    ) -> bool {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        // Restore access to The Amazing Secret.
        let window_helper = <gfc::Singleton<gfc::WindowHelper>>::get_instance();
        let current_window = window_helper.get_window();
        let result =
            target::gfc__Darksiders__doTheMagic(this, inputEvent, current_window.name().as_ptr());
        if !result {
            return false;
        }

        let result = (detours.gfc__Darksiders__processInputEvent)(this, inputEvent);

        // Setting this flag prevents the game from pausing when you deactivate the
        // window.
        (*this).mGameInBackground = false;

        if !result {
            return false;
        }

        if ui::handle_input_event(inputEvent) == ui::InputHandled::Swallow {
            return false;
        }

        true
    }

    pub unsafe extern "thiscall" fn gfc__DebugOutModule__execute(
        this: *mut target::gfc__DebugOutModule,
        actionid: u32,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc__DebugOutModule__execute)(this, actionid);

        log_events::hook_debugoutmodule_execute((*this).lift_ref(), actionid);
    }

    pub unsafe extern "thiscall" fn gfc__DetectorRegion__bodyEntered(
        this: *mut target::gfc__DetectorRegion,
        body: *mut target::gfc__Body,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_detectorregion_body_entered((*this).lift_ref(), (*body).lift_ref());

        (detours.gfc__DetectorRegion__bodyEntered)(this, body);
    }

    pub unsafe extern "thiscall" fn gfc__DetectorRegion__bodyExited(
        this: *mut target::gfc__DetectorRegion,
        body: *mut target::gfc__Body,
        wobject: *mut target::gfc__WorldObject,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_detectorregion_body_exited((*this).lift_ref(), (*body).lift_ref());

        (detours.gfc__DetectorRegion__bodyExited)(this, body, wobject);
    }

    pub unsafe extern "thiscall" fn gfc__DSSaveGameManager__saveGame(
        this: *mut target::gfc__DSSaveGameManager,
        slot: i32,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_dssavegamemanager_save_game();

        (detours.gfc__DSSaveGameManager__saveGame)(this, slot);
    }

    pub unsafe extern "thiscall" fn gfc__InsRun__doPrint(this: *mut target::gfc__InsRun) -> bool {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_insrun_do_print((*this).lift_ref());

        (detours.gfc__InsRun__doPrint)(this)
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
        if let Some(ovrrride) = override_get_material(packageID, material_name) {
            *result = Lower::lower(ovrrride);
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
            if let Some(ovrrride) = override_get_static_mesh(packageID, mesh_name, idx) {
                *result = Lower::lower(ovrrride);
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
            if let Some(ovrrride) = override_get_object3d(packageID, object_name) {
                *result = Lower::lower(ovrrride);
                return result;
            }
        }

        (detours.gfc__Object3DCache__get)(this, result, packageID, objectName)
    }

    pub unsafe extern "thiscall" fn gfc__OblivionGame__update(
        this: *mut target::gfc__OblivionGame,
        timescale: f32,
        noInputUpdate: bool,
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
        infinite_jump::pump();

        (detours.gfc__OblivionGame__update)(this, timescale, noInputUpdate)
    }

    pub unsafe extern "thiscall" fn gfc__Player__setSpawnPoint_2(
        this: *mut target::gfc__Player,
        world: *const target::gfc__HString,
        region: *const target::gfc__HString,
        spawnpoint: *const target::gfc__HString,
        spawnloadregion: *const target::gfc__HString,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_player_set_spawn_point(
            (*world).lift_ref(),
            (*region).lift_ref(),
            (*spawnpoint).lift_ref(),
            (*spawnloadregion).lift_ref(),
        );

        (detours.gfc__Player__setSpawnPoint_2)(this, world, region, spawnpoint, spawnloadregion);
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

    pub unsafe extern "thiscall" fn gfc__SaveData__setValue(
        this: *mut target::gfc__SaveData,
        key: *const target::gfc__HString,
        value: *const target::gfc__HString,
    ) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        log_events::hook_savedata_set_value((*key).lift_ref(), (*value).lift_ref());

        (detours.gfc__SaveData__setValue)(this, key, value);
    }

    pub unsafe extern "thiscall" fn gfc__World__World(
        this: *mut target::gfc__World,
    ) -> *mut target::gfc__World {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc__World__World)(this);

        editor_mode::world_constructor_hook(this);

        this
    }

    pub unsafe extern "thiscall" fn gfc__WorldRegion__preload(this: *mut target::gfc__WorldRegion) {
        let guard = DETOURS.read();
        let detours = guard.as_ref().unwrap();

        (detours.gfc__WorldRegion__preload)(this);

        log_events::hook_worldregion_preload((*this).lift_ref());
    }
}
