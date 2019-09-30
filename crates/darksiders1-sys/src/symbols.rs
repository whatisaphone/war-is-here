#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
#![allow(clippy::unreadable_literal)]

use super::{
    types::*,
    types10::*,
    types11::*,
    types12::*,
    types13::*,
    types2::*,
    types3::*,
    types4::*,
    types5::*,
    types6::*,
    types7::*,
    types8::*,
    types9::*,
};

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
    pub static gfc__WorldObject___Class: *mut gfc__Class = Data(0x88ba8c);
    pub static gfc__Object3D___Class: *mut gfc__Class = Data(0x88a3e0);
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__TeleportHelper = Data(0x55e0ec);
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__Darksiders = Data(0xa1afc);
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle: *mut gfc__ClassRegistry = Data(0xa4230);
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__WindowHelper = Data(0xa1b0c);
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
    pub static gfc__OmniLight__getColor: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe385a0);
    pub static gfc__OmniLight__setPower: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe385c0);
    pub static gfc__OmniLight__getPower: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe385e0);
    pub static gfc__OmniLight__setCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe385f0);
    pub static gfc__OmniLight__getCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> bool = Text(0xe38600);
    pub static gfc__OmniLight__setAttenStart: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe38610);
    pub static gfc__OmniLight__getAttenStart: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe38630);
    pub static gfc__OmniLight__setAttenEnd: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: f32) = Text(0xe38640);
    pub static gfc__OmniLight__getAttenEnd: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> f32 = Text(0xe38660);
    pub static gfc__OmniLight__setScale: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe38670);
    pub static gfc__OmniLight__getScale: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe386d0);
    pub static gfc__OmniLight__setSize: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe386f0);
    pub static gfc__OmniLight__getSize: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe38750);
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
    pub static gfc__StaticObject__getPosition: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e20);
    pub static gfc__StaticObject__getRotation: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e40);
    pub static gfc__StaticObject__getScale: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe54e60);
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
    pub static gfc__WorldObject__setObjectID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u32) = Text(0xdca1a0);
    pub static gfc__WorldObject__setEventGroupID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32) = Text(0xdca7b0);
    pub static gfc__WorldObject__getEventGroupID: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> i32 = Text(0xdca7c0);
    pub static gfc__WorldObject__update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32) = Text(0xdcc2f0);
    pub static gfc__WorldObject__cloneLayerInfo: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject) = Text(0xdcc300);
    pub static gfc__WorldObject__getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32 = Text(0xdcc320);
    pub static gfc__WorldObject__getEventHandlers: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject__EventHandlerNode = Text(0xdcc360);
    pub static gfc__WorldObject__addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World) = Text(0xdcc370);
    pub static gfc__WorldObject__getWorldObjectID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__WorldObject__ID = Text(0xdcc390);
    pub static gfc__WorldObject__getScriptEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Environment = Text(0xdcc3b0);
    pub static gfc__WorldObject__onChildDetachedFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject) = Text(0xdcc3d0);
    pub static gfc__WorldObject__setPosition: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: f32, _: f32) = Text(0xdd6260);
    pub static gfc__WorldObject__WorldObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xdd86d0);
    pub static gfc__WorldObject__removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xdd87d0);
    pub static gfc__WorldObject__getAnimation: unsafe extern "thiscall" fn(this: *const gfc__WorldObject, _: i32) -> gfc__AutoRef_gfc__Animation_ = Text(0xdd8820);
    pub static gfc__WorldObject__addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World) = Text(0xdd8880);
    pub static gfc__WorldObject__getRegion: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldRegion = Text(0xdd88c0);
    pub static gfc__WorldObject__getNodeRotation: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xdd8940);
    pub static gfc__WorldObject__attachEventHandler: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__EventHandler) = Text(0xde7270);
    pub static gfc__WorldObject__detachAllEventHandlers: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xde73d0);
    pub static gfc__WorldObject__releaseEventHandlers: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xde7430);
    pub static gfc__WorldObject__addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xde74c0);
    pub static gfc__WorldObject__getNodePosition: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xde76e0);
    pub static gfc__WorldObject__detachEventHandler: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__EventHandler) = Text(0xdf9dd0);
    pub static gfc__WorldObject__executeEvent: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString) = Text(0xdf9e50);
    pub static gfc__WorldObject__visScriptCall: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString, _: *const gfc__HString) = Text(0xdf9e70);
    pub static gfc__WorldObject__onDetachedFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject) = Text(0xdf9fe0);
    pub static gfc__WorldObject__executeEvent1P: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString, _: gfc__AutoRef_gfc__Value_) = Text(0xe054d0);
    pub static gfc__WorldObject__executeEvent2P: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString, _: gfc__AutoRef_gfc__Value_, _: gfc__AutoRef_gfc__Value_) = Text(0xe05550);
    pub static gfc__WorldObject__removeObjectFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xe05600);
    pub static gfc__WorldObject__onAttachedToObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject, _: *const gfc__TVector3_float_gfc__FloatMath_, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe05610);
    pub static gfc__WorldObject__setLightGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u32) = Text(0xe0eba0);
    pub static gfc__WorldObject__removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xe0ebc0);
    pub static gfc__WorldObject__attachToObjectOffset: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject, _: *const gfc__HString, _: *const gfc__HString, _: u8, _: bool, _: *const gfc__TVector3_float_gfc__FloatMath_, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xe0ed10);
    pub static gfc__WorldObject__attachToObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject, _: *const gfc__HString, _: *const gfc__HString, _: u8, _: bool) = Text(0xe0edb0);
    pub static gfc__WorldObject__detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0xe0ee30);
    pub static gfc__WorldObject__isAttachedToObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0xe0eee0);
    pub static gfc__WorldObject__getAttachedParent: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject = Text(0xe0ef40);
    pub static gfc__WorldObject__hasAttachedObjects: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0xe0efd0);
    pub static gfc__WorldObject__getAttachedObjects: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_ = Text(0xe0f030);
    pub static gfc__WorldObject__getAttachedTransform: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__Matrix4 = Text(0xe0f0b0);
    pub static gfc__WorldObject__getAttachedPosition: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe0f130);
    pub static gfc__WorldObject__getAttachedRotation: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0xe0f190);
    pub static gfc__Object3D__getLODDistances: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Vector_float_0_gfc__CAllocator_ = Text(0xd7b330);
    pub static gfc__Object3D__setLODFloor: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: u32) = Text(0xd7bbe0);
    pub static gfc__Object3D__getHighPriority: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> bool = Text(0xd83df0);
    pub static gfc__WorldObject__getDisabled: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0xd84e50);
    pub static gfc__Object3D__getSkeleton: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Skeleton3D = Text(0xd888d0);
    pub static gfc__Object3D__getVisualCount: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> i32 = Text(0xd888e0);
    pub static gfc__Object3D__getVisualAt: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: i32) -> gfc__AutoRef_gfc__Visual_ = Text(0xd888f0);
    pub static gfc__Object3D__containsVisualAt: unsafe extern "thiscall" fn(this: *const gfc__Object3D, _: i32) -> bool = Text(0xd88920);
    pub static gfc__Object3D__getVisualKeys: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> gfc__AutoRef_gfc__Value_ = Text(0xd88930);
    pub static gfc__Object3D__setRagdoll: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: gfc__AutoRef_gfc__RagdollMapper_) = Text(0xd945d0);
    pub static gfc__Object3D__Object3D: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xd9d130);
    pub static gfc__Object3D__setSkeleton: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__Skeleton3D) = Text(0xd9d210);
    pub static gfc__Object3D__setNoShadows: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: bool) = Text(0xd9d270);
    pub static gfc__Object3D__setCharacterShadow: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: bool) = Text(0xd9d2b0);
    pub static gfc__Object3D__setMaterialOverride: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString, _: gfc__AutoRef_gfc__Material_) = Text(0xd9d2e0);
    pub static gfc__Object3D__setMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString, _: gfc__AutoRef_gfc__Material_) = Text(0xd9d3a0);
    pub static gfc__Object3D__clearMaterialOverride: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xd9d460);
    pub static gfc__Object3D__setObjectColor: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0xd9d490);
    pub static gfc__Object3D__setFadeValue: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: f32) = Text(0xd9d4d0);
    pub static gfc__Object3D__getLODLevel: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__TVector3_float_gfc__FloatMath_) -> u32 = Text(0xd9d520);
    pub static gfc__Object3D__preload: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: i32) = Text(0xd9d680);
    pub static gfc__Object3D__getBodyByName: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString) -> *mut gfc__Body = Text(0xd9d7d0);
    pub static gfc__Object3D__getAnimationController: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__AnimationController = Text(0xd9d8b0);
    pub static gfc__Object3D__getParticleController: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__ParticleController = Text(0xd9d990);
    pub static gfc__Object3D__applyImpulseToBodyByName: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString, _: f32, _: f32, _: f32) = Text(0xd9da70);
    pub static gfc__Object3D__stopAllAnimations: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: f32) = Text(0xd9de50);
    pub static gfc__Object3D__stopEffect: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: i32) = Text(0xd9de80);
    pub static gfc__Object3D__stopAllEffects: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xd9dea0);
    pub static gfc__Object3D__containsVisual: unsafe extern "thiscall" fn(this: *const gfc__Object3D, _: gfc__AutoRef_gfc__Visual_) -> bool = Text(0xdabc90);
    pub static gfc__Object3D__getVisualValues: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> gfc__AutoRef_gfc__Value_ = Text(0xdabce0);
    pub static gfc__Object3D__addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__World, _: bool) = Text(0xdabd10);
    pub static gfc__Object3D__addToWorld_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__WorldObject, _: bool) = Text(0xdabf10);
    pub static gfc__Object3D__convertBodiesToDebris: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xdabf30);
    pub static gfc__Object3D__getConstrainedBodies: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__RigidBody, _: *mut gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_) = Text(0xdabf40);
    pub static gfc__Object3D__walkConstraintChain: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__RigidBody, _: *mut gfc__ConstraintSetup) -> *mut gfc__RigidBody = Text(0xdac1c0);
    pub static gfc__Object3D__addVisual: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: gfc__AutoRef_gfc__Visual_) = Text(0xdb5270);
    pub static gfc__Object3D__removeVisual: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: gfc__AutoRef_gfc__Visual_) = Text(0xdb5420);
    pub static gfc__Object3D__setupBodySystems: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> bool = Text(0xdb55a0);
    pub static gfc__Object3D__removeVisualAt: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: i32) = Text(0xdb9cd0);
    pub static gfc__Object3D__clearVisualList: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xdb9d10);
    pub static gfc__Object3D__assignFrom: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__Object3D) = Text(0xdb9f20);
    pub static gfc__Object3D__cloneObject: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__ObjectCloner, _: gfc__AutoRef_gfc__Object_) = Text(0xdba780);
    pub static gfc__Object3D__setMaterialOverride_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString, _: *const gfc__HString) = Text(0xdc24d0);
    pub static gfc__Object3D__setMaterial_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString, _: *const gfc__HString) = Text(0xdc2590);
    pub static gfc__Object3D__removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) = Text(0xdc2650);
    pub static gfc__Object3D__playEffect: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString) -> i32 = Text(0xdc2820);
    pub static gfc__Object3D__setWorld: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__World) = Text(0xcd7af0);
    pub static gfc__Object3D__getWorldObject: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> *mut gfc__WorldObject = Text(0xc21be0);
    pub static gfc__Object3D__getConstraints: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Vector_gfc__AutoRef_gfc__Constraint__0_gfc__CAllocator_ = Text(0xc21bf0);
    pub static gfc__Object3D__getIsAggregatable: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> bool = Text(0xc32370);
    pub static gfc__Class__getContextFlags: unsafe extern "thiscall" fn(this: *const gfc__Class) -> u8 = Text(0xa9ac70);
    pub static gfc__HString__HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa63b40);
    pub static gfc__HString__HString_2: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__String) = Text(0xa6e4a0);
    pub static gfc__HString__HString_3: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const i8, _: bool) = Text(0xa6e550);
    pub static gfc__HString__HString_4: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64) = Text(0xa6e600);
    pub static gfc__HString__HString_5: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__HString) = Text(0xa6e690);
    pub static gfc__HString__HString_6: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64, _: *const i8, _: i32) = Text(0xa6e720);
    pub static gfc__HString___HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa6e790);
    pub static gfc__Class__getMethodCount: unsafe extern "thiscall" fn(this: *const gfc__Class) -> i32 = Text(0xa39ce0);
    pub static gfc__Class__getMethodAt: unsafe extern "thiscall" fn(this: *const gfc__Class, _: i32) -> *mut gfc__Method = Text(0xa39cf0);
    pub static gfc__MemAlloc: unsafe extern "C" fn(_: u32, _: *mut (), _: u32, _: u32, _: u32, _: u32, _: *const i8, _: u32) -> *mut () = Text(0xa37580);
    pub static gfc__Class__getPropertyCount: unsafe extern "thiscall" fn(this: *const gfc__Class) -> i32 = Text(0xa1f010);
    pub static gfc__Class__setContextFlags: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: u8) = Text(0xa12230);
    pub static gfc__Class__getPropertyByName: unsafe extern "thiscall" fn(this: *const gfc__Class, _: *const gfc__HString, _: *mut *const gfc__Class) -> *mut gfc__Property = Text(0xa124a0);
    pub static gfc__Class__getPropertyByName_2: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const gfc__HString, _: *mut *mut gfc__Class) -> *mut gfc__Property = Text(0xa124d0);
    pub static gfc__Class__getPropertyByID: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const u64, _: *mut *mut gfc__Class) -> *mut gfc__Property = Text(0xa124e0);
    pub static gfc__Class__getMethodByName: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const gfc__HString) -> *mut gfc__Method = Text(0xa124f0);
    pub static gfc__Class__instanceof: unsafe extern "thiscall" fn(this: *const gfc__Class, _: *const gfc__Class) -> bool = Text(0xa13990);
    pub static gfc__Class__instanceof_2: unsafe extern "thiscall" fn(this: *const gfc__Class, _: *const gfc__HString) -> bool = Text(0xa139b0);
    pub static gfc__Class__newInstance: unsafe extern "thiscall" fn(this: *mut gfc__Class) -> gfc__AutoRef_gfc__Object_ = Text(0xa139f0);
    pub static gfc__Class__getPropertyAt: unsafe extern "thiscall" fn(this: *const gfc__Class, _: i32) -> *mut gfc__Property = Text(0xa13a30);
    pub static gfc__Class__getPropertyByID_2: unsafe extern "thiscall" fn(this: *const gfc__Class, _: *const u64, _: *mut *const gfc__Class) -> *mut gfc__Property = Text(0xa16c90);
    pub static gfc__Class__getMethodByID: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const u64) -> *mut gfc__Method = Text(0xa16d60);
    pub static gfc__ClassRegistry__classForName: unsafe extern "thiscall" fn(this: *mut gfc__ClassRegistry, _: *const gfc__HString, _: bool, _: bool) -> *mut gfc__Class = Text(0xa16e50);
    pub static gfc__Class__Class: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const gfc__HString) = Text(0xa18460);
    pub static gfc__Class__getTypeID: unsafe extern "thiscall" fn(this: *mut gfc__Class) -> i32 = Text(0xa18510);
    pub static gfc__Class__getName: unsafe extern "thiscall" fn(this: *const gfc__Class) -> *const gfc__HString = Text(0xa18520);
    pub static gfc__Class__getParent: unsafe extern "thiscall" fn(this: *const gfc__Class) -> *mut gfc__Class = Text(0xa18530);
    pub static gfc__Class__isAbstract: unsafe extern "thiscall" fn(this: *const gfc__Class) -> bool = Text(0xa18540);
    pub static gfc__Class__Class_2: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const gfc__HString, _: gfc__AutoRef_gfc__Constructor_, _: u8) = Text(0xa186d0);
    pub static gfc__Class__addProperty: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *mut gfc__Property) = Text(0xa1a690);
    pub static gfc__Class__addMethod: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *mut gfc__Method) = Text(0xa1a880);
    pub static gfc__WorldObject__getObjectID: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> u32 = Text(0x962c50);
    pub static gfc__WorldObject__setPosition_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x908fc0);
    pub static gfc__WorldObject__getPosition: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0x91a910);
    pub static gfc__WorldObject__getHide: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x8e8220);
    pub static gfc__WorldObject__getFreeze: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x8e8230);
    pub static gfc__WorldObject__getLocked: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x8e8240);
    pub static gfc__WorldObject__getSelected: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x8e8250);
    pub static gfc__Object3D__getWorld: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> *mut gfc__World = Text(0x81c7b0);
    pub static gfc__Player__Player: unsafe extern "thiscall" fn(this: *mut gfc__Player) = Text(0x8593b0);
    pub static gfc__WorldObject__getLightGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> u32 = Text(0x755d10);
    pub static gfc__Object3D__setWorldObject: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *mut gfc__WorldObject) = Text(0x756040);
    pub static gfc__Object3D__getClass: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> *mut gfc__Class = Text(0x764160);
    pub static gfc__Actor__setPosition: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x7a5540);
    pub static gfc__WorldObject__getWorld: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> *mut gfc__World = Text(0x64b9b0);
    pub static gfc__Object3D__getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> i32 = Text(0x5b1000);
    pub static gfc__Object3D__getRagdoll: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> gfc__AutoRef_gfc__RagdollMapper_ = Text(0x577fd0);
    pub static gfc__WorldObject__doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x49c220);
    pub static gfc__WorldObject__doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x49c230);
    pub static gfc__WorldObject__getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> *mut gfc__Class = Text(0x4cbe20);
    pub static gfc__WorldObject__setRotation: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x4cbe30);
    pub static gfc__WorldObject__setScale: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x4cbe40);
    pub static gfc__WorldObject__getBoundingBox: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TBox_float_gfc__FloatMath_) = Text(0x4cbe50);
    pub static gfc__WorldObject__getRotation: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0x4cbea0);
    pub static gfc__WorldObject__getScale: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> gfc__TVector3_float_gfc__FloatMath_ = Text(0x4cbec0);
    pub static gfc__Object3D__setHighPriority: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: bool) = Text(0x474da0);
    pub static gfc__Object3D__getWorld_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__World = Text(0x3b5fe0);
    pub static gfc__WorldObject__isAttached: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x297210);
    pub static gfc__WorldObject__placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x29fd80);
    pub static gfc__WorldObject__droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x29fd90);
    pub static gfc__WorldObject__updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32) = Text(0x29fda0);
    pub static gfc__WorldObject__render: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Renderer, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x29fdb0);
    pub static gfc__WorldObject__drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x29fdc0);
    pub static gfc__WorldObject__isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0x29fdd0);
    pub static gfc__WorldObject__inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0x29fde0);
    pub static gfc__WorldObject__isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0x29fdf0);
    pub static gfc__WorldObject__getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject = Text(0x29fe00);
    pub static gfc__WorldObject__setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D) = Text(0x29fe10);
    pub static gfc__WorldObject__removeFromPathFinding: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint) = Text(0x29fe20);
    pub static gfc__WorldObject__overrideHitEffect: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool = Text(0x29fe30);
    pub static gfc__WorldObject__supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0x29fe40);
    pub static gfc__WorldObject__staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool = Text(0x29fe50);
    pub static gfc__WorldObject__getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32 = Text(0x29fe60);
    pub static gfc__WorldObject__initStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__StaticLightingObjectOpt) -> bool = Text(0x29fe70);
    pub static gfc__WorldObject__clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x29fe80);
    pub static gfc__WorldObject__setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa550);
    pub static gfc__WorldObject__setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa570);
    pub static gfc__WorldObject__setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa590);
    pub static gfc__WorldObject__setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa5b0);
    pub static gfc__WorldObject__setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa5d0);
    pub static gfc__WorldObject__setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa5f0);
    pub static gfc__WorldObject__setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa610);
    pub static gfc__WorldObject__setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool) = Text(0x2aa630);
    pub static gfc__WorldObject__getSettled: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> bool = Text(0x2aa650);
    pub static gfc__WorldObject__addObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_) = Text(0x2aa660);
    pub static gfc__WorldObject__removeObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_) = Text(0x2aa690);
    pub static gfc__WorldObject__preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x2c1790);
    pub static gfc__WorldObject__begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) = Text(0x2c17a0);
    pub static gfc__WorldObject__playSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32, _: *mut gfc__TVector3_float_gfc__FloatMath_) -> i32 = Text(0x2c17b0);
    pub static gfc__WorldObject__playSound_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__SoundDesc, _: *mut gfc__TVector3_float_gfc__FloatMath_) -> i32 = Text(0x2c17c0);
    pub static gfc__WorldObject__stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32) = Text(0x2c17d0);
    pub static gfc__WorldObject__getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D = Text(0x2c17e0);
    pub static gfc__WorldObject__setRegionID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u16) = Text(0x20ac80);
    pub static gfc__WorldObject__getRegionID: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> u16 = Text(0x20ac90);
    pub static gfc__WorldObject__setLayerID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u16) = Text(0x20aca0);
    pub static gfc__WorldObject__getLayerID: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> u16 = Text(0x20acb0);
    pub static gfc__WorldObject__setName: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString) = Text(0x20acc0);
    pub static gfc__Darksiders__processInputEvent: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *const keen__InputEvent) -> bool = Text(0x27b4b0);
    pub static gfc__Darksiders__onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32) = Text(0x295c90);
    pub static gfc__WorldObject__getName: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> *const gfc__HString = Text(0x1e40b0);
    pub static gfc__LoadMapMenu__LoadMapMenu: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) = Text(0x201d50);
    pub static gfc__WindowHelper__pushWindow: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString) = Text(0x1bc090);
    pub static gfc__TeleportHelper__warpToMap: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString, _: *const gfc__HString) = Text(0x1bcf30);
    pub static gfc__WorldObject__getWorld_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__World = Text(0xed660);
    pub static gfc__Object3D__getBodies: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Vector_gfc__AutoRef_gfc__Body__0_gfc__CAllocator_ = Text(0xee3a0);
    pub static gfc__Object3D__getVisuals: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Vector_gfc__AutoRef_gfc__Visual__0_gfc__CAllocator_ = Text(0xee3b0);
    pub static gfc__Graphics__getInstance: unsafe extern "C" fn() -> *mut gfc__Graphics = Text(0xd55d0);
}
