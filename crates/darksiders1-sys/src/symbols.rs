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
    pub static gfc__OmniLight___Class: *mut gfc__Class = Data(0x88cf24);
    pub static gfc__WorldGroup___Class: *mut gfc__Class = Data(0x88ba80);
    pub static gfc__TriggerRegion___Class: *mut gfc__Class = Data(0x58c510);
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__TeleportHelper = Data(0x55e0ec);
    pub static gfc__Singleton_gfc__KGMeshCache_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__KGMeshCache = Data(0x5559f4);
    pub static gfc__Singleton_gfc__KGGraphics_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle: *mut gfc__KGGraphics = Data(0xa1aec);
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__Darksiders = Data(0xa1afc);
    pub static gfc__Singleton_gfc__Object3DCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle: *mut gfc__Object3DCache = Data(0xa1a8c);
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle: *mut gfc__ClassRegistry = Data(0xa4230);
    pub static gfc__Singleton_gfc__DSUIManager_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__DSUIManager = Data(0xa4258);
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__WindowHelper = Data(0xa1b0c);
    pub static gfc__Vector4Parameter__Vector4Parameter: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) = Text(0xfa16b0);
    pub static gfc__UIRenderer__setMaterial: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *mut gfc__Material) = Text(0xfb8320);
    pub static gfc__UIRenderer__fillRect: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32, _: *const gfc__TVector4_float_gfc__FloatMath_, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xfb8480);
    pub static gfc__UIRenderer__drawLine: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32) = Text(0xfb8be0);
    pub static gfc__UIRenderer__beginRendering: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: bool) = Text(0xfb8f80);
    pub static gfc__UIRenderer__endRendering: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfb9020);
    pub static gfc__UIRenderer__pushClip: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32, _: f32, _: f32, _: *const gfc__TVector4_float_gfc__FloatMath_, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xfd1930);
    pub static gfc__UIRenderer__popClip: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfd4aa0);
    pub static gfc__Material__setParameter: unsafe extern "thiscall" fn(this: *mut gfc__Material, _: *const gfc__HString, _: gfc__AutoRef_gfc__Parameter_) = Text(0xfe78c0);
    pub static gfc__Material__Material: unsafe extern "thiscall" fn(this: *mut gfc__Material) = Text(0xfe9160);
    pub static gfc__Material___Material: unsafe extern "thiscall" fn(this: *mut gfc__Material) = Text(0xfe9250);
    pub static gfc__UIRenderer__UIRenderer: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfe9430);
    pub static gfc__UIRenderer__updateTime: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xfec7d0);
    pub static gfc__MaterialCache__get: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, result: *mut gfc__AutoRef_gfc__Material_, _: i32, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Material_ = Text(0xfee7c0);
    pub static gfc__MaterialCache__get_2: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, result: *mut gfc__AutoRef_gfc__Material_, _: *const gfc__HString, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Material_ = Text(0xff33c0);
    pub static gfc__MeshReader__MeshReader: unsafe extern "thiscall" fn(this: *mut gfc__MeshReader) = Text(0xf5f9e0);
    pub static gfc__MeshCache__getStaticMesh: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: i32, _: *const gfc__HString, _: i32) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xf68630);
    pub static gfc__MBSubMesh__MBSubMesh: unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh) = Text(0xf6baf0);
    pub static gfc__MeshCache__getStaticMesh_2: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: *const gfc__HString, _: *const gfc__HString, _: i32) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xf6fd90);
    pub static gfc__MeshReader__readObject: unsafe extern "thiscall" fn(this: *mut gfc__MeshReader, result: *mut gfc__AutoRef_gfc__Object_, _: gfc__AutoRef_gfc__InputStream_, _: *mut bool) -> *mut gfc__AutoRef_gfc__Object_ = Text(0xf91000);
    pub static gfc__MeshCache__loadMesh: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: *mut gfc__MeshResourceUnopt, _: i32, _: gfc__AutoRef_gfc__InputStream_, _: gfc__HString, _: i32) -> i32 = Text(0xf91df0);
    pub static gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject_: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_, _: *mut gfc__IRefObject) = Text(0xf1b740);
    pub static gfc__UIRenderer__identity: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xea40f0);
    pub static gfc__UIRenderer__multiplyColor: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0xea4150);
    pub static gfc___UIManager__draw: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *mut gfc__UIRenderer) = Text(0xed5b70);
    pub static gfc___UIManager__draw_2: unsafe extern "thiscall" fn(this: *mut gfc___UIManager) = Text(0xeec300);
    pub static gfc__OblivionGame__getProjMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, result: *mut gfc__Matrix4) -> *mut gfc__Matrix4 = Text(0xe7f230);
    pub static gfc__OblivionGame__getWorld: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> *mut gfc__World = Text(0xe816a0);
    pub static gfc__OblivionGame__getViewMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, result: *mut gfc__Matrix4) -> *mut gfc__Matrix4 = Text(0xe81980);
    pub static gfc__OmniLight__setStaticOnly: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe26960);
    pub static gfc__OmniLight__getStaticOnly: unsafe extern "thiscall" fn(this: *const gfc__OmniLight) -> bool = Text(0xe26970);
    pub static gfc__OmniLight__doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe271c0);
    pub static gfc__StaticObject__setObjectName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d660);
    pub static gfc__StaticObject__setPackageName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d810);
    pub static gfc__OmniLight__getClass: unsafe extern "thiscall" fn(this: *const gfc__OmniLight) -> *mut gfc__Class = Text(0xe38550);
    pub static gfc__OmniLight__setStatic: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe38560);
    pub static gfc__OmniLight__getStatic: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> bool = Text(0xe38570);
    pub static gfc__OmniLight__setColor: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe38580);
    pub static gfc__OmniLight__getColor: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe385a0);
    pub static gfc__OmniLight__setPower: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe385c0);
    pub static gfc__OmniLight__getPower: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe385e0);
    pub static gfc__OmniLight__setCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe385f0);
    pub static gfc__OmniLight__getCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> bool = Text(0xe38600);
    pub static gfc__OmniLight__setAttenStart: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe38610);
    pub static gfc__OmniLight__getAttenStart: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe38630);
    pub static gfc__OmniLight__setAttenEnd: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe38640);
    pub static gfc__OmniLight__getAttenEnd: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe38660);
    pub static gfc__OmniLight__setScale: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe38670);
    pub static gfc__OmniLight__getScale: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe386d0);
    pub static gfc__OmniLight__setSize: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe386f0);
    pub static gfc__OmniLight__getSize: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe38750);
    pub static gfc__OmniLight__preload: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe38770);
    pub static gfc__OmniLight__OmniLight: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4c9b0);
    pub static gfc__OmniLight__getBoundingBox: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *mut gfc__TBox_float_gfc__FloatMath_) = Text(0xe4cae0);
    pub static gfc__OmniLight__doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4cd00);
    pub static gfc__StaticObject__StaticObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe54d60);
    pub static gfc__StaticObject___StaticObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe54eb0);
    pub static gfc__StaticObject__StaticObject_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__String, _: *const gfc__String) = Text(0xe54f60);
    pub static gfc__StaticObject__StaticObject_3: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: i32) = Text(0xe55020);
    pub static gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject__2: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_) = Text(0xdd24b0);
    pub static gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_) = Text(0xdd24c0);
    pub static gfc__World__getRegion: unsafe extern "thiscall" fn(this: *const gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0xdd61f0);
    pub static gfc__Object3DCache__get: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, result: *mut gfc__AutoRef_gfc__Object3D_, _: i32, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Object3D_ = Text(0xd889c0);
    pub static gfc__Object3D__Object3D: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xd9d130);
    pub static gfc__Object3DCache__get_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, result: *mut gfc__AutoRef_gfc__Object3D_, _: *const gfc__HString, _: *const gfc__HString) -> *mut gfc__AutoRef_gfc__Object3D_ = Text(0xd9deb0);
    pub static gfc__StaticMeshVisual__StaticMeshVisual: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) = Text(0xdae030);
    pub static gfc__StaticMeshVisual__StaticMeshVisual_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *const gfc__HString, _: i32, _: *const gfc__HString) = Text(0xdae200);
    pub static gfc__StaticMeshVisual___StaticMeshVisual: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) = Text(0xdae3c0);
    pub static gfc__Object3D___Object3D: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xdbbaf0);
    pub static gfc__ResourceCache__getResource: unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: i32, _: *const gfc__HString) -> *mut () = Text(0xca0750);
    pub static gfc__WorldGroup__getObjects: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> *mut List_gfc__AutoRef_gfc__WorldObject___ = Text(0xb27f30);
    pub static gfc__HString__HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa63b40);
    pub static gfc__HString__HString_2: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__String) = Text(0xa6e4a0);
    pub static gfc__HString__HString_3: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const i8, _: bool) = Text(0xa6e550);
    pub static gfc__HString__HString_4: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64) = Text(0xa6e600);
    pub static gfc__HString__HString_5: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__HString) = Text(0xa6e690);
    pub static gfc__HString__HString_6: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64, _: *const i8, _: i32) = Text(0xa6e720);
    pub static gfc__HString___HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa6e790);
    pub static gfc__HString__c_str: unsafe extern "thiscall" fn(this: *const gfc__HString) -> *const i8 = Text(0xa6e8d0);
    pub static gfc__OOObjectWriter__writeObject: unsafe extern "thiscall" fn(this: *mut gfc__OOObjectWriter, _: gfc__AutoRef_gfc__Object_, _: gfc__AutoRef_gfc__OutputStream_, _: bool) = Text(0xa40140);
    pub static gfc__MemFree: unsafe extern "C" fn(_: u32, _: *mut (), _: *const i8, _: u32) -> *mut () = Text(0xa35880);
    pub static gfc__MemAlloc: unsafe extern "C" fn(_: u32, _: *mut (), _: u32, _: u32, _: u32, _: u32, _: *const i8, _: u32) -> *mut () = Text(0xa37580);
    pub static gfc__ObjectWriter__ObjectWriter: unsafe extern "thiscall" fn(this: *mut gfc__ObjectWriter) = Text(0xa1cad0);
    pub static gfc__ByteInputStream___ByteInputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) = Text(0xa1f370);
    pub static gfc__ByteInputStream__ByteInputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: *const (), _: u32, _: bool, _: bool) = Text(0xa21ca0);
    pub static gfc__ByteInputStream__ByteInputStream_2: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: gfc__AutoRef_gfc__InputStream_) = Text(0xa21d60);
    pub static gfc__ByteInputStream__ByteInputStream_3: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: *const gfc__ByteOutputStream) = Text(0xa21e90);
    pub static gfc__ByteOutputStream__ByteOutputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) = Text(0xa21ef0);
    pub static gfc__ByteOutputStream___ByteOutputStream: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) = Text(0xa21f40);
    pub static gfc__ByteOutputStream__ByteOutputStream_2: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream, _: i32) = Text(0xa21fb0);
    pub static gfc__ClassRegistry__classForName: unsafe extern "thiscall" fn(this: *mut gfc__ClassRegistry, _: *const gfc__HString, _: bool, _: bool) -> *mut gfc__Class = Text(0xa16e50);
    pub static gfc__PhysicsShapeObject__getTransform: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__Matrix4) = Text(0x8ed360);
    pub static gfc__RegionLayer__getRoot: unsafe extern "thiscall" fn(this: *const gfc__RegionLayer, result: *mut gfc__AutoRef_gfc__WorldGroup_) -> *mut gfc__AutoRef_gfc__WorldGroup_ = Text(0x769ed0);
    pub static gfc__WorldRegion__getLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, result: *mut gfc__AutoRef_gfc__RegionLayer_, _: i32) -> *mut gfc__AutoRef_gfc__RegionLayer_ = Text(0x769f60);
    pub static gfc__Vector4Parameter__Vector4Parameter_2: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0x5b0ec0);
    pub static gfc__Vector4Parameter___Vector4Parameter: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) = Text(0x5b0fa0);
    pub static gfc__Vector4Parameter__Vector4Parameter_3: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *const gfc__Vector4Parameter) = Text(0x5c7e60);
    pub static gfc__WorldObject__setRegionID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u16) = Text(0x20ac80);
    pub static gfc__WorldObject__setLayerID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u16) = Text(0x20aca0);
    pub static gfc__ObjectWriter___ObjectWriter: unsafe extern "thiscall" fn(this: *mut gfc__ObjectWriter) = Text(0x2103c0);
    pub static gfc__World__getRegion_2: unsafe extern "thiscall" fn(this: *mut gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0x223730);
    pub static gfc__Darksiders__processInputEvent: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *const keen__InputEvent) -> bool = Text(0x27b4b0);
    pub static gfc__Darksiders__onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32) = Text(0x295c90);
    pub static gfc__UIRenderer__translate: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32) = Text(0x1eb170);
    pub static gfc__UIRenderer__rotate: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32) = Text(0x1eb1b0);
    pub static gfc__UIRenderer__scale: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: f32, _: f32) = Text(0x1eb1e0);
    pub static gfc__UIRenderer__clearShader: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x1eeab0);
    pub static gfc__LoadMapMenu__LoadMapMenu: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) = Text(0x201d50);
    pub static gfc__WindowHelper__pushWindow: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString) = Text(0x1bc090);
    pub static gfc__TeleportHelper__warpToMap: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString) = Text(0x1bcf30);
    pub static gfc__UIRenderer__end: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0xed500);
    pub static gfc__UIRenderer__setColor: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: *const gfc__TVector4_float_gfc__FloatMath_) = Text(0x10b370);
    pub static gfc__UIRenderer__setSolidMaterial: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x10b3b0);
    pub static gfc__UIRenderer__pushTransform: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x13b280);
    pub static gfc__UIRenderer__pushParams: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x13b510);
    pub static gfc__UIRenderer__begin: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer, _: bool) = Text(0x143ca0);
    pub static gfc__UIRenderer__popTransform: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x143fc0);
    pub static gfc__UIRenderer__popParams: unsafe extern "thiscall" fn(this: *mut gfc__UIRenderer) = Text(0x144130);
    pub static gfc__KGGraphics__createStaticMesh: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, result: *mut gfc__AutoRef_gfc__StaticMesh_) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xe1e30);
    pub static gfc__KGGraphics__createStaticMesh_2: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, result: *mut gfc__AutoRef_gfc__StaticMesh_, _: *mut gfc__MeshBuilder) -> *mut gfc__AutoRef_gfc__StaticMesh_ = Text(0xe1ec0);
    pub static gfc__MeshBuilder__MeshBuilder: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) = Text(0xe4d20);
    pub static gfc__MeshBuilder___MeshBuilder: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) = Text(0xe4e10);
    pub static gfc__String__String: unsafe extern "thiscall" fn(this: *mut gfc__String) = Text(0x67070);
    pub static gfc__String___String: unsafe extern "thiscall" fn(this: *mut gfc__String) = Text(0x67110);
    pub static gfc__String__String_2: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const gfc__String) = Text(0x69030);
    pub static gfc__String__String_3: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const gfc__String, _: i32, _: i32) = Text(0x69060);
    pub static gfc__String__String_4: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const i8) = Text(0x69530);
    pub static gfc__String__String_5: unsafe extern "thiscall" fn(this: *mut gfc__String, _: *const std__basic_string_char_std__char_traits_char__std__allocator_char___) = Text(0x69570);
}
