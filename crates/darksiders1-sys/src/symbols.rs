#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
#![allow(clippy::unreadable_literal)]

use super::{types::*, types2::*, types3::*, types4::*};

macro_rules! symbols {
    () => {};
    (
        $(
            $(#[$attr:meta])*
            $vis:vis static $name:ident: $type:ty = $section:ident($offset:literal);
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis type $name = $type;
            $(#[$attr])*
            $vis const $name: usize = $offset;
        )*
    };
}

symbols! {
    pub static gfc__WorldObject___Class: *mut gfc__Class = Data(0x88ba8c);
    pub static gfc__WorldGroup___Class: *mut gfc__Class = Data(0x88ba80);
    pub static gfc__CShape___Class: *mut gfc__Class = Data(0x6a2d70);
    pub static gfc__CShapeMesh___Class: *mut gfc__Class = Data(0x6a2d84);
    pub static gfc__CShapeBox___Class: *mut gfc__Class = Data(0x6a2d74);
    pub static gfc__DetectorObject___Class: *mut gfc__Class = Data(0x58c4e0);
    pub static gfc__TriggerRegion___Class: *mut gfc__Class = Data(0x58c510);
    pub static gfc__LoadRegion___Class: *mut gfc__Class = Data(0x58c4f0);
    pub static gfc__Item___Class: *mut gfc__Class = Data(0x56bc10);
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__TeleportHelper = Data(0x55e0ec);
    pub static gfc__Singleton_gfc__KGMeshCache_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__KGMeshCache = Data(0x5559f4);
    pub static gfc__Singleton_gfc__HStringManager_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle: *mut gfc__HStringManager = Data(0x555b14);
    pub static gfc__Singleton_gfc__KGGraphics_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle: *mut gfc__KGGraphics = Data(0xa1aec);
    pub static gfc__Singleton_gfc__ResourceManager_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__ResourceManager = Data(0xa4228);
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__Darksiders = Data(0xa1afc);
    pub static gfc__Singleton_gfc__Object3DCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle: *mut gfc__Object3DCache = Data(0xa1a8c);
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle: *mut gfc__ClassRegistry = Data(0xa4230);
    pub static gfc__Singleton_gfc__PhysMeshCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle: *mut gfc__PhysMeshCache = Data(0xa1a9c);
    pub static gfc__Singleton_gfc__DSUIManager_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__DSUIManager = Data(0xa4258);
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__WindowHelper = Data(0xa1b0c);
    pub static gfc__Vector4Parameter__Vector4Parameter: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) -> *mut gfc__Vector4Parameter = Text(0xfa16b0);
    pub static gfc__UIRenderer__setMaterial: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *mut gfc__Material) = Text(0xfb8320);
    pub static gfc__UIRenderer__fillRect: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32, _: *const gfc__TVector4_float_gfc__FloatMath_, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xfb8480);
    pub static gfc__UIRenderer__drawLine: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32) = Text(0xfb8be0);
    pub static gfc__UIRenderer__beginRendering: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: bool) = Text(0xfb8f80);
    pub static gfc__UIRenderer__endRendering: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfb9020);
    pub static gfc__UIRenderer__pushClip: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32, _: *const gfc__TVector4_float_gfc__FloatMath_, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xfd1930);
    pub static gfc__UIRenderer__popClip: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfd4aa0);
    pub static gfc__Material__setParameter: unsafe extern "thiscall" fn(this: *mut gfc__Material, _: *const gfc__HString, _: gfc__AutoRef_gfc__Parameter_) = Text(0xfe78c0);
    pub static gfc__Material__Material: unsafe extern "thiscall" fn(this: *mut gfc__Material) -> *mut gfc__Material = Text(0xfe9160);
    pub static gfc__Material___Material: unsafe extern "thiscall" fn(this: *mut gfc__Material) = Text(0xfe9250);
    pub static gfc__UIRenderer__UIRenderer: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) -> *mut gfc__UIRenderer = Text(0xfe9430);
    pub static gfc__UIRenderer__updateTime: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfec7d0);
    pub static gfc__MaterialCache__get: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, result: *mut gfc__AutoRef_gfc__Material_, _: i32, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Material_ = Text(0xfee7c0);
    pub static gfc__MaterialCache__get_2: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, result: *mut gfc__AutoRef_gfc__Material_, _: *const gfc__HString, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Material_ = Text(0xff33c0);
    pub static gfc__MeshReader__MeshReader: unsafe extern "thiscall" fn(this: *mut gfc__MeshReader) -> *mut gfc__MeshReader = Text(0xf5f9e0);
    pub static gfc__MeshCache__getStaticMesh: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: i32, _: *const gfc__HString, _: i32) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xf68630);
    pub static gfc__MBSubMesh__MBSubMesh: unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh) -> *mut gfc__MBSubMesh = Text(0xf6baf0);
    pub static gfc__MeshCache__getStaticMesh_2: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: *const gfc__HString, _: *const gfc__HString, _: i32) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xf6fd90);
    pub static gfc__MeshReader__readObject: unsafe extern "thiscall" fn(this: *mut gfc__MeshReader, result: *mut gfc__AutoRef_gfc__Object_, _: gfc__AutoRef_gfc__InputStream_, _: *mut bool) -> *mut gfc__AutoRef_gfc__Object_ = Text(0xf91000);
    pub static gfc__MeshCache__loadMesh: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: *mut gfc__MeshResourceUnopt, _: i32, _: gfc__AutoRef_gfc__InputStream_, _: gfc__HString, _: i32) -> i32 = Text(0xf91df0);
    pub static gfc__UIRenderer__identity: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xea40f0);
    pub static gfc__UIRenderer__multiplyColor: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xea4150);
    pub static gfc___UIManager__draw: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *mut gfc__UIRenderer) = Text(0xed5b70);
    pub static gfc___UIManager__draw_2: unsafe extern "thiscall" fn(this: *mut gfc___UIManager) = Text(0xeec300);
    pub static gfc__OblivionGame__getProjMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, result: *mut gfc__Matrix4) -> *mut gfc__Matrix4 = Text(0xe7f230);
    pub static gfc__OblivionGame__getWorld: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> *mut gfc__World = Text(0xe816a0);
    pub static gfc__OblivionGame__getViewMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, result: *mut gfc__Matrix4) -> *mut gfc__Matrix4 = Text(0xe81980);
    pub static gfc__OblivionGame__update: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32, _: bool) = Text(0xe8d960);
    pub static gfc__StaticObject__setObjectName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d660);
    pub static gfc__StaticObject__setPackageName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d810);
    pub static gfc__StaticObject__StaticObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut gfc__StaticObject = Text(0xe54d60);
    pub static gfc__StaticObject___StaticObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe54eb0);
    pub static gfc__StaticObject__StaticObject_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__String, _: *const gfc__String) -> *mut gfc__StaticObject = Text(0xe54f60);
    pub static gfc__StaticObject__StaticObject_3: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: i32) -> *mut gfc__StaticObject = Text(0xe55020);
    pub static gfc__World__getRegion: unsafe extern "thiscall" fn(this: *const gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0xdd61f0);
    pub static gfc__WorldRegion__preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion) = Text(0xddade0);
    pub static gfc__WorldObject__removeObjectFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xe05600);
    pub static gfc__WorldObject__attachToObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject, _: *const gfc__HString, _: *const gfc__HString, _: u8, _: bool) = Text(0xe0edb0);
    pub static gfc__WorldRegion__removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion) = Text(0xe0fa50);
    pub static gfc__World__World: unsafe extern "thiscall" fn(this: *mut gfc__World) -> *mut gfc__World = Text(0xe17670);
    pub static gfc__Object3DCache__get: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, result: *mut gfc__AutoRef_gfc__Object3D_, _: i32, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Object3D_ = Text(0xd889c0);
    pub static gfc__Object3D__Object3D: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Object3D = Text(0xd9d130);
    pub static gfc__Object3DCache__get_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, result: *mut gfc__AutoRef_gfc__Object3D_, _: *const gfc__HString, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Object3D_ = Text(0xd9deb0);
    pub static gfc__StaticMeshVisual__StaticMeshVisual: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> *mut gfc__StaticMeshVisual = Text(0xdae030);
    pub static gfc__StaticMeshVisual__StaticMeshVisual_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *const gfc__HString, _: i32, _: *const gfc__HString) -> *mut gfc__StaticMeshVisual = Text(0xdae200);
    pub static gfc__StaticMeshVisual___StaticMeshVisual: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) = Text(0xdae3c0);
    pub static gfc__Object3D___Object3D: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xdbbaf0);
    pub static gfc__ResourceManager__getPackageID: unsafe extern "thiscall" fn(this: *const gfc__ResourceManager, _: *const gfc__HString) -> i32 = Text(0xc83fd0);
    pub static gfc__ResourceManager__getPackageID_2: unsafe extern "thiscall" fn(this: *const gfc__ResourceManager, _: i32) -> i32 = Text(0xc84060);
    pub static gfc__ResourceManager__getPermanentID: unsafe extern "thiscall" fn(this: *const gfc__ResourceManager, _: i32) -> i32 = Text(0xc840e0);
    pub static gfc__ResourceManager__getPackageID_3: unsafe extern "thiscall" fn(this: *const gfc__ResourceManager, _: *const gfc__String) -> i32 = Text(0xc97840);
    pub static gfc__ResourceManager__loadPackages: unsafe extern "thiscall" fn(this: *mut gfc__ResourceManager, result: *mut gfc__AutoRef_gfc__PackageMarker_, _: *const gfc__Vector_int_0_gfc__CAllocator_, _: bool, _: *mut gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___) -> *mut gfc__AutoRef_gfc__PackageMarker_ = Text(0xc9e5c0);
    pub static gfc__ResourceCache__getResource: unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: i32, _: *const gfc__HString) -> *mut () = Text(0xca0750);
    pub static gfc__PhysMeshCache__get: unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: i32, _: *const gfc__HString, _: i32, _: f32) -> *mut hkpShape = Text(0xc75710);
    pub static gfc__PhysMeshCache__get_2: unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: *const gfc__HString, _: *const gfc__HString, _: i32, _: f32) -> *mut hkpShape = Text(0xc77260);
    pub static gfc__DebugOutModule__execute: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32) = Text(0xaeb700);
    pub static gfc__InsRun__doPrint: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool = Text(0xaa6f80);
    pub static gfc__HString__HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) -> *mut gfc__HString = Text(0xa63b40);
    pub static gfc__HString__HString_2: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__String) -> *mut gfc__HString = Text(0xa6e4a0);
    pub static gfc__HString__HString_3: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const i8, _: bool) -> *mut gfc__HString = Text(0xa6e550);
    pub static gfc__HString__HString_4: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64) -> *mut gfc__HString = Text(0xa6e600);
    pub static gfc__HString__HString_5: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__HString) -> *mut gfc__HString = Text(0xa6e690);
    pub static gfc__HString__HString_6: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64, _: *const i8, _: i32) -> *mut gfc__HString = Text(0xa6e720);
    pub static gfc__HString___HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa6e790);
    pub static gfc__HString__c_str: unsafe extern "thiscall" fn(this: *const gfc__HString) -> *const i8 = Text(0xa6e8d0);
    pub static gfc__ValueStack__push: unsafe extern "thiscall" fn(this: *mut gfc__ValueStack, _: gfc__AutoRef_gfc__Value_) = Text(0xa56900);
    pub static gfc__ValueStack__pop: unsafe extern "thiscall" fn(this: *mut gfc__ValueStack, result: *mut gfc__AutoRef_gfc__Value_) -> *mut gfc__AutoRef_gfc__Value_ = Text(0xa569c0);
    pub static gfc__ScriptClass__ScriptClass: unsafe extern "thiscall" fn(this: *mut gfc__ScriptClass, _: *const gfc__HString, _: i32, _: *mut gfc__Class) -> *mut gfc__ScriptClass = Text(0xa61fb0);
    pub static gfc__ScriptClass___ScriptClass: unsafe extern "thiscall" fn(this: *mut gfc__ScriptClass) = Text(0xa62140);
    pub static gfc__OOObjectWriter__writeObject: unsafe extern "thiscall" fn(this: *mut gfc__OOObjectWriter, _: gfc__AutoRef_gfc__Object_, _: gfc__AutoRef_gfc__OutputStream_, _: bool) = Text(0xa40140);
    pub static gfc__MemFree: unsafe extern "C" fn(_: u32, _: *mut (), _: *const i8, _: u32) -> *mut () = Text(0xa35880);
    pub static gfc__MemAlloc: unsafe extern "C" fn(_: u32, _: *mut (), _: u32, _: u32, _: u32, _: u32, _: *const i8, _: u32) -> *mut () = Text(0xa37580);
    pub static gfc__ObjectWriter__ObjectWriter: unsafe extern "thiscall" fn(this: *mut gfc__ObjectWriter) -> *mut gfc__ObjectWriter = Text(0xa1cad0);
    pub static gfc__ByteInputStream___ByteInputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) = Text(0xa1f370);
    pub static gfc__ByteInputStream__ByteInputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: *const (), _: u32, _: bool, _: bool) -> *mut gfc__ByteInputStream = Text(0xa21ca0);
    pub static gfc__ByteInputStream__ByteInputStream_2: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: gfc__AutoRef_gfc__InputStream_) -> *mut gfc__ByteInputStream = Text(0xa21d60);
    pub static gfc__ByteInputStream__ByteInputStream_3: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: *const gfc__ByteOutputStream) -> *mut gfc__ByteInputStream = Text(0xa21e90);
    pub static gfc__ByteOutputStream__ByteOutputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) -> *mut gfc__ByteOutputStream = Text(0xa21ef0);
    pub static gfc__ByteOutputStream___ByteOutputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) = Text(0xa21f40);
    pub static gfc__ByteOutputStream__ByteOutputStream_2: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream, _: i32) -> *mut gfc__ByteOutputStream = Text(0xa21fb0);
    pub static gfc__ClassRegistry__classForName: unsafe extern "thiscall" fn(this: *mut gfc__ClassRegistry, _: *const gfc__HString, _: bool, _: bool) -> *mut gfc__Class = Text(0xa16e50);
    pub static gfc__DetectorRegion__bodyEntered: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut gfc__Body) = Text(0x9c58b0);
    pub static gfc__DetectorRegion__bodyExited: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut gfc__Body, _: *mut gfc__WorldObject) = Text(0x9c5ba0);
    pub static gfc__SaveData__setValue: unsafe extern "thiscall" fn(this: *mut gfc__SaveData, _: *const gfc__HString, _: *const gfc__HString) = Text(0x99d580);
    pub static gfc__PhysicsShapeObject__getTransform: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__Matrix4) = Text(0x8ed360);
    pub static gfc__Player__setSpawnPoint: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *const gfc__HString) = Text(0x81ccd0);
    pub static gfc__Player__setSpawnPoint_2: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString) = Text(0x821670);
    pub static gfc__Player__pickupItem: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__Item_) = Text(0x83ab00);
    pub static gfc__Player__pickupItem_2: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__ItemActor_) = Text(0x849000);
    pub static gfc__Inventory__addItem: unsafe extern "thiscall" fn(this: *mut gfc__Inventory, _: gfc__AutoRef_gfc__Item_) -> bool = Text(0x81bdd0);
    pub static gfc__RegionLayer__getRoot: unsafe extern "thiscall" fn(this: *const gfc__RegionLayer, result: *mut gfc__AutoRef_gfc__WorldGroup_) -> *mut gfc__AutoRef_gfc__WorldGroup_ = Text(0x769ed0);
    pub static gfc__WorldRegion__getLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, result: *mut gfc__AutoRef_gfc__RegionLayer_, _: i32) -> *mut gfc__AutoRef_gfc__RegionLayer_ = Text(0x769f60);
    pub static gfc__Vector4Parameter__Vector4Parameter_2: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *const gfc__TVector4_float_gfc__FloatMath_) -> *mut gfc__Vector4Parameter = Text(0x5b0ec0);
    pub static gfc__Vector4Parameter___Vector4Parameter: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) = Text(0x5b0fa0);
    pub static gfc__Vector4Parameter__Vector4Parameter_3: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *const gfc__Vector4Parameter) -> *mut gfc__Vector4Parameter = Text(0x5c7e60);
    pub static gfc__Node3D__getPosition: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0x41d8d0);
    pub static gfc__ObjectWriter___ObjectWriter: unsafe extern "thiscall" fn(this: *mut gfc__ObjectWriter) = Text(0x2103c0);
    pub static gfc__World__getRegion_2: unsafe extern "thiscall" fn(this: *mut gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0x223730);
    pub static gfc__Darksiders__processInputEvent: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *const keen__InputEvent) -> bool = Text(0x27b4b0);
    pub static gfc__Darksiders__doTheMagic: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *const keen__InputEvent, _: *const gfc__HString) -> bool = Text(0x27bed0);
    pub static gfc__DSSaveGameManager__saveGame: unsafe extern "thiscall" fn(this: *mut gfc__DSSaveGameManager, _: i32) = Text(0x294fa0);
    pub static gfc__UIRenderer__translate: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32) = Text(0x1eb170);
    pub static gfc__UIRenderer__rotate: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32) = Text(0x1eb1b0);
    pub static gfc__UIRenderer__scale: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32) = Text(0x1eb1e0);
    pub static gfc__UIRenderer__clearShader: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x1eeab0);
    pub static gfc__LoadMapMenu__LoadMapMenu: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> *mut gfc__LoadMapMenu = Text(0x201d50);
    pub static gfc__WindowHelper__pushWindow: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString) = Text(0x1bc090);
    pub static gfc__TeleportHelper__warpToMap: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString) = Text(0x1bcf30);
    pub static gfc__UIRenderer__end: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xed500);
    pub static gfc__UIRenderer__setColor: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0x10b370);
    pub static gfc__UIRenderer__setSolidMaterial: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x10b3b0);
    pub static gfc__Node3D__getMatrix: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, result: *mut gfc__Matrix4, _: bool) -> *mut gfc__Matrix4 = Text(0x10b870);
    pub static gfc__UIRenderer__pushTransform: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x13b280);
    pub static gfc__UIRenderer__pushParams: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x13b510);
    pub static gfc__UIRenderer__begin: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: bool) = Text(0x143ca0);
    pub static gfc__UIRenderer__popTransform: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x143fc0);
    pub static gfc__UIRenderer__popParams: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x144130);
    pub static gfc__KGGraphics__createStaticMesh: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, result: *mut gfc__AutoRef_gfc__StaticMesh_) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xe1e30);
    pub static gfc__KGGraphics__createStaticMesh_2: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: *mut gfc__MeshBuilder) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xe1ec0);
    pub static gfc__MeshBuilder__MeshBuilder: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) -> *mut gfc__MeshBuilder = Text(0xe4d20);
    pub static gfc__MeshBuilder___MeshBuilder: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) = Text(0xe4e10);
    pub static gfc__String__c_str: unsafe extern "thiscall" fn(this: *const gfc__String) -> *const i8 = Text(0x61f90);
    pub static gfc__String__String: unsafe extern "thiscall" fn(this: *mut gfc__String) -> *mut gfc__String = Text(0x67070);
    pub static gfc__String___String: unsafe extern "thiscall" fn(this: *mut gfc__String) = Text(0x67110);
    pub static gfc__String__String_2: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const gfc__String) -> *mut gfc__String = Text(0x69030);
    pub static gfc__String__String_3: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const gfc__String, _: i32, _: i32) -> *mut gfc__String = Text(0x69060);
    pub static gfc__String__String_4: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const i8) -> *mut gfc__String = Text(0x69530);
    pub static gfc__String__String_5: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const std__basic_string_char_std__char_traits_char__std__allocator_char___) -> *mut gfc__String = Text(0x69570);
    pub static hkpConvexVerticesShape__getOriginalVertices: unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape, _: *mut hkArray_hkVector4f_hkContainerHeapAllocator_) = Text(0x12d4ca0);
}
