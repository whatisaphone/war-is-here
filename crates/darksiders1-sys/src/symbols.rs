#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
#![allow(clippy::unreadable_literal)]

use super::{types::*, types2::*, types3::*};

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
    pub static gfc__StaticObject___Class: *mut gfc__Class = Data(0x88cf58);
    pub static gfc__OmniLight___Class: *mut gfc__Class = Data(0x88cf24);
    pub static gfc__WorldGroup___Class: *mut gfc__Class = Data(0x88ba80);
    pub static gfc__TriggerRegion___Class: *mut gfc__Class = Data(0x58c510);
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__TeleportHelper = Data(0x55e0ec);
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__Darksiders = Data(0xa1afc);
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle: *mut gfc__ClassRegistry = Data(0xa4230);
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__WindowHelper = Data(0xa1b0c);
    pub static gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject_: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_, _: *mut gfc__IRefObject) = Text(0xf1b740);
    pub static gfc__OmniLight__setStaticOnly: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe26960);
    pub static gfc__OmniLight__getStaticOnly: unsafe extern "thiscall" fn(this: *const gfc__OmniLight) -> bool = Text(0xe26970);
    pub static gfc__OmniLight__doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe271c0);
    pub static gfc__StaticObject__setObjectName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d660);
    pub static gfc__StaticObject__getObjectName: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> *const gfc__HString = Text(0xe2d680);
    pub static gfc__StaticObject__setPackageName: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString) = Text(0xe2d810);
    pub static gfc__StaticObject__getPackageName: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> *const gfc__HString = Text(0xe2d830);
    pub static gfc__StaticObject__setAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: i32) = Text(0xe2dc80);
    pub static gfc__StaticObject__getNoShadows: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> bool = Text(0xe33770);
    pub static gfc__StaticObject__setBakeLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool) = Text(0xe33780);
    pub static gfc__StaticObject__getBakeLighting: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> bool = Text(0xe337a0);
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
    pub static gfc__StaticObject__init: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe3cde0);
    pub static gfc__StaticObject__setNoShadows: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool) = Text(0xe3ce20);
    pub static gfc__StaticObject__setPosition: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe3ce60);
    pub static gfc__StaticObject__setRotation: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe3ce90);
    pub static gfc__StaticObject__setScale: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe3cec0);
    pub static gfc__StaticObject__doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe3cf20);
    pub static gfc__StaticObject__supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool = Text(0xe3cf40);
    pub static gfc__OmniLight__OmniLight: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4c9b0);
    pub static gfc__OmniLight__getBoundingBox: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *mut gfc__TBox_float_gfc__FloatMath_) = Text(0xe4cae0);
    pub static gfc__OmniLight__doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4cd00);
    pub static gfc__StaticObject__StaticObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe54d60);
    pub static gfc__StaticObject__getClass: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> *mut gfc__Class = Text(0xe54e10);
    pub static gfc__StaticObject__getPosition: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e20);
    pub static gfc__StaticObject__getRotation: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e40);
    pub static gfc__StaticObject__getScale: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, result: *mut gfc__TVector3_float_gfc__FloatMath_) -> *mut gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e60);
    pub static gfc__StaticObject__getObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut gfc__Object3D = Text(0xe54e80);
    pub static gfc__StaticObject__staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool = Text(0xe54e90);
    pub static gfc__StaticObject__getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> i32 = Text(0xe54ea0);
    pub static gfc__StaticObject__StaticObject_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__String, _: *const gfc__String) = Text(0xe54f60);
    pub static gfc__StaticObject__StaticObject_3: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: i32) = Text(0xe55020);
    pub static gfc__StaticObject__setObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__Object3D) = Text(0xe550e0);
    pub static gfc__StaticObject__getBoundingBox: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__TBox_float_gfc__FloatMath_) = Text(0xe55140);
    pub static gfc__StaticObject__doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe553c0);
    pub static gfc__StaticObject__invalidateRenderNodes: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe55420);
    pub static gfc__StaticObject__initStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__StaticLightingObjectOpt) -> bool = Text(0xe554f0);
    pub static gfc__StaticObject__clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe55610);
    pub static gfc__StaticObject__preload: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) = Text(0xe7c4c0);
    pub static gfc__StaticObject__getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> i32 = Text(0xe7c600);
    pub static gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject__2: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_) = Text(0xdd24b0);
    pub static gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_: unsafe extern "thiscall" fn(this: *mut gfc__AutoRef_gfc__IRefObject_) = Text(0xdd24c0);
    pub static gfc__World__getRegion: unsafe extern "thiscall" fn(this: *const gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0xdd61f0);
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
    pub static gfc__MemAlloc: unsafe extern "C" fn(_: u32, _: *mut (), _: u32, _: u32, _: u32, _: u32, _: *const i8, _: u32) -> *mut () = Text(0xa37580);
    pub static gfc__ClassRegistry__classForName: unsafe extern "thiscall" fn(this: *mut gfc__ClassRegistry, _: *const gfc__HString, _: bool, _: bool) -> *mut gfc__Class = Text(0xa16e50);
    pub static gfc__RegionLayer__getRoot: unsafe extern "thiscall" fn(this: *const gfc__RegionLayer, result: *mut gfc__AutoRef_gfc__WorldGroup_) -> *mut gfc__AutoRef_gfc__WorldGroup_ = Text(0x769ed0);
    pub static gfc__WorldRegion__getLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, result: *mut gfc__AutoRef_gfc__RegionLayer_, _: i32) -> *mut gfc__AutoRef_gfc__RegionLayer_ = Text(0x769f60);
    pub static gfc__Actor__setPosition: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x7a5540);
    pub static gfc__World__getRegion_2: unsafe extern "thiscall" fn(this: *mut gfc__World, result: *mut gfc__AutoRef_gfc__WorldRegion_, _: i32) -> *mut gfc__AutoRef_gfc__WorldRegion_ = Text(0x223730);
    pub static gfc__Darksiders__processInputEvent: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *const keen__InputEvent) -> bool = Text(0x27b4b0);
    pub static gfc__Darksiders__onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32) = Text(0x295c90);
    pub static gfc__LoadMapMenu__LoadMapMenu: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) = Text(0x201d50);
    pub static gfc__WindowHelper__pushWindow: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString) = Text(0x1bc090);
    pub static gfc__TeleportHelper__warpToMap: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString) = Text(0x1bcf30);
}
