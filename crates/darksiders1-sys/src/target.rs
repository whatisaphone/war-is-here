#![allow(non_camel_case_types, non_upper_case_globals)]

use super::map;
pub use super::{
    symbols::*,
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
use std::{mem, ptr};

fn transmute_hack() {
    panic!("need to call `bind` first");
}

macro_rules! symbols {
    () => {};
    (
        $($(#[$attr:meta])* pub static $name:ident;)+
    ) => {
        $(
            $(#[$attr])*
            pub static mut $name: *mut map::$name = ptr::null_mut();
            $(#[$attr])*
            pub type $name = map::$name;
        )*
    };
    (
        $($(#[$attr:meta])* pub fn $name:ident;)+
    ) => {
        $(
            $(#[$attr])*
            pub static mut $name: map::$name = unsafe { mem::transmute(transmute_hack as *mut ()) };
            $(#[$attr])*
            pub type $name = map::$name;
        )*
    };
    (
        $($(#[$attr:meta])* pub static $name:ident;)+
        pub fn $($tail:tt)*
    ) => {
        symbols!($($(#[$attr])* pub static $name;)+);
        symbols!(pub fn $($tail)*);
    };
}

symbols! {
    pub static gfc__StaticObject___Class;
    pub static gfc__OmniLight___Class;
    pub static gfc__WorldObject___Class;
    pub static gfc__Object3D___Class;
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    pub fn gfc__OmniLight__setStaticOnly;
    pub fn gfc__OmniLight__getStaticOnly;
    pub fn gfc__OmniLight__doRemoveFromWorld;
    pub fn gfc__StaticObject__setObjectName;
    pub fn gfc__StaticObject__getObjectName;
    pub fn gfc__StaticObject__setPackageName;
    pub fn gfc__StaticObject__getPackageName;
    pub fn gfc__StaticObject__setAORayLength;
    pub fn gfc__StaticObject__getNoShadows;
    pub fn gfc__StaticObject__setBakeLighting;
    pub fn gfc__StaticObject__getBakeLighting;
    pub fn gfc__OmniLight__getClass;
    pub fn gfc__OmniLight__setStatic;
    pub fn gfc__OmniLight__getStatic;
    pub fn gfc__OmniLight__setColor;
    pub fn gfc__OmniLight__getColor;
    pub fn gfc__OmniLight__setPower;
    pub fn gfc__OmniLight__getPower;
    pub fn gfc__OmniLight__setCastShadows;
    pub fn gfc__OmniLight__getCastShadows;
    pub fn gfc__OmniLight__setAttenStart;
    pub fn gfc__OmniLight__getAttenStart;
    pub fn gfc__OmniLight__setAttenEnd;
    pub fn gfc__OmniLight__getAttenEnd;
    pub fn gfc__OmniLight__setScale;
    pub fn gfc__OmniLight__getScale;
    pub fn gfc__OmniLight__setSize;
    pub fn gfc__OmniLight__getSize;
    pub fn gfc__OmniLight__preload;
    pub fn gfc__StaticObject__init;
    pub fn gfc__StaticObject__setNoShadows;
    pub fn gfc__StaticObject__setPosition;
    pub fn gfc__StaticObject__setRotation;
    pub fn gfc__StaticObject__setScale;
    pub fn gfc__StaticObject__doAddToWorld;
    pub fn gfc__StaticObject__supportsStaticLighting;
    pub fn gfc__OmniLight__OmniLight;
    pub fn gfc__OmniLight__getBoundingBox;
    pub fn gfc__OmniLight__doAddToWorld;
    pub fn gfc__StaticObject__StaticObject;
    pub fn gfc__StaticObject__getClass;
    pub fn gfc__StaticObject__getPosition;
    pub fn gfc__StaticObject__getRotation;
    pub fn gfc__StaticObject__getScale;
    pub fn gfc__StaticObject__getObject;
    pub fn gfc__StaticObject__staticLightingIsDynamic;
    pub fn gfc__StaticObject__getAORayLength;
    pub fn gfc__StaticObject__StaticObject_2;
    pub fn gfc__StaticObject__StaticObject_3;
    pub fn gfc__StaticObject__setObject;
    pub fn gfc__StaticObject__getBoundingBox;
    pub fn gfc__StaticObject__doRemoveFromWorld;
    pub fn gfc__StaticObject__invalidateRenderNodes;
    pub fn gfc__StaticObject__initStaticLighting;
    pub fn gfc__StaticObject__clearStaticLighting;
    pub fn gfc__StaticObject__preload;
    pub fn gfc__StaticObject__getPackageID;
    pub fn gfc__WorldObject__setObjectID;
    pub fn gfc__WorldObject__setEventGroupID;
    pub fn gfc__WorldObject__getEventGroupID;
    pub fn gfc__WorldObject__update;
    pub fn gfc__WorldObject__cloneLayerInfo;
    pub fn gfc__WorldObject__getPackageID;
    pub fn gfc__WorldObject__getEventHandlers;
    pub fn gfc__WorldObject__addToWorld;
    pub fn gfc__WorldObject__getWorldObjectID;
    pub fn gfc__WorldObject__getScriptEnvironment;
    pub fn gfc__WorldObject__onChildDetachedFromObject;
    pub fn gfc__WorldObject__setPosition;
    pub fn gfc__WorldObject__WorldObject;
    pub fn gfc__WorldObject__removeFromGroup;
    pub fn gfc__WorldObject__getAnimation;
    pub fn gfc__WorldObject__addObjectToWorld;
    pub fn gfc__WorldObject__getRegion;
    pub fn gfc__WorldObject__getNodeRotation;
    pub fn gfc__WorldObject__attachEventHandler;
    pub fn gfc__WorldObject__detachAllEventHandlers;
    pub fn gfc__WorldObject__releaseEventHandlers;
    pub fn gfc__WorldObject__addToLayer;
    pub fn gfc__WorldObject__getNodePosition;
    pub fn gfc__WorldObject__detachEventHandler;
    pub fn gfc__WorldObject__executeEvent;
    pub fn gfc__WorldObject__visScriptCall;
    pub fn gfc__WorldObject__onDetachedFromObject;
    pub fn gfc__WorldObject__executeEvent1P;
    pub fn gfc__WorldObject__executeEvent2P;
    pub fn gfc__WorldObject__removeObjectFromWorld;
    pub fn gfc__WorldObject__onAttachedToObject;
    pub fn gfc__WorldObject__setLightGroup;
    pub fn gfc__WorldObject__removeFromWorld;
    pub fn gfc__WorldObject__attachToObjectOffset;
    pub fn gfc__WorldObject__attachToObject;
    pub fn gfc__WorldObject__detachFromObject;
    pub fn gfc__WorldObject__isAttachedToObject;
    pub fn gfc__WorldObject__getAttachedParent;
    pub fn gfc__WorldObject__hasAttachedObjects;
    pub fn gfc__WorldObject__getAttachedObjects;
    pub fn gfc__WorldObject__getAttachedTransform;
    pub fn gfc__WorldObject__getAttachedPosition;
    pub fn gfc__WorldObject__getAttachedRotation;
    pub fn gfc__Object3D__getLODDistances;
    pub fn gfc__Object3D__setLODFloor;
    pub fn gfc__Object3D__getHighPriority;
    pub fn gfc__WorldObject__getDisabled;
    pub fn gfc__Object3D__getSkeleton;
    pub fn gfc__Object3D__getVisualCount;
    pub fn gfc__Object3D__getVisualAt;
    pub fn gfc__Object3D__containsVisualAt;
    pub fn gfc__Object3D__getVisualKeys;
    pub fn gfc__Object3D__setRagdoll;
    pub fn gfc__Object3D__Object3D;
    pub fn gfc__Object3D__setSkeleton;
    pub fn gfc__Object3D__setNoShadows;
    pub fn gfc__Object3D__setCharacterShadow;
    pub fn gfc__Object3D__setMaterialOverride;
    pub fn gfc__Object3D__setMaterial;
    pub fn gfc__Object3D__clearMaterialOverride;
    pub fn gfc__Object3D__setObjectColor;
    pub fn gfc__Object3D__setFadeValue;
    pub fn gfc__Object3D__getLODLevel;
    pub fn gfc__Object3D__preload;
    pub fn gfc__Object3D__getBodyByName;
    pub fn gfc__Object3D__getAnimationController;
    pub fn gfc__Object3D__getParticleController;
    pub fn gfc__Object3D__applyImpulseToBodyByName;
    pub fn gfc__Object3D__stopAllAnimations;
    pub fn gfc__Object3D__stopEffect;
    pub fn gfc__Object3D__stopAllEffects;
    pub fn gfc__Object3D__containsVisual;
    pub fn gfc__Object3D__getVisualValues;
    pub fn gfc__Object3D__addToWorld;
    pub fn gfc__Object3D__addToWorld_2;
    pub fn gfc__Object3D__convertBodiesToDebris;
    pub fn gfc__Object3D__getConstrainedBodies;
    pub fn gfc__Object3D__walkConstraintChain;
    pub fn gfc__Object3D__addVisual;
    pub fn gfc__Object3D__removeVisual;
    pub fn gfc__Object3D__setupBodySystems;
    pub fn gfc__Object3D__removeVisualAt;
    pub fn gfc__Object3D__clearVisualList;
    pub fn gfc__Object3D__assignFrom;
    pub fn gfc__Object3D__cloneObject;
    pub fn gfc__Object3D__setMaterialOverride_2;
    pub fn gfc__Object3D__setMaterial_2;
    pub fn gfc__Object3D__removeFromWorld;
    pub fn gfc__Object3D__playEffect;
    pub fn gfc__Object3D__setWorld;
    pub fn gfc__Object3D__getWorldObject;
    pub fn gfc__Object3D__getConstraints;
    pub fn gfc__Object3D__getIsAggregatable;
    pub fn gfc__Class__getContextFlags;
    pub fn gfc__HString__HString;
    pub fn gfc__HString__HString_2;
    pub fn gfc__HString__HString_3;
    pub fn gfc__HString__HString_4;
    pub fn gfc__HString__HString_5;
    pub fn gfc__HString__HString_6;
    pub fn gfc__HString___HString;
    pub fn gfc__Class__getMethodCount;
    pub fn gfc__Class__getMethodAt;
    pub fn gfc__MemAlloc;
    pub fn gfc__Class__getPropertyCount;
    pub fn gfc__Class__setContextFlags;
    pub fn gfc__Class__getPropertyByName;
    pub fn gfc__Class__getPropertyByName_2;
    pub fn gfc__Class__getPropertyByID;
    pub fn gfc__Class__getMethodByName;
    pub fn gfc__Class__instanceof;
    pub fn gfc__Class__instanceof_2;
    pub fn gfc__Class__newInstance;
    pub fn gfc__Class__getPropertyAt;
    pub fn gfc__Class__getPropertyByID_2;
    pub fn gfc__Class__getMethodByID;
    pub fn gfc__ClassRegistry__classForName;
    pub fn gfc__Class__Class;
    pub fn gfc__Class__getTypeID;
    pub fn gfc__Class__getName;
    pub fn gfc__Class__getParent;
    pub fn gfc__Class__isAbstract;
    pub fn gfc__Class__Class_2;
    pub fn gfc__Class__addProperty;
    pub fn gfc__Class__addMethod;
    pub fn gfc__WorldObject__getObjectID;
    pub fn gfc__WorldObject__setPosition_2;
    pub fn gfc__WorldObject__getPosition;
    pub fn gfc__WorldObject__getHide;
    pub fn gfc__WorldObject__getFreeze;
    pub fn gfc__WorldObject__getLocked;
    pub fn gfc__WorldObject__getSelected;
    pub fn gfc__Object3D__getWorld;
    pub fn gfc__Player__Player;
    pub fn gfc__WorldObject__getLightGroup;
    pub fn gfc__Object3D__setWorldObject;
    pub fn gfc__Object3D__getClass;
    pub fn gfc__Actor__setPosition;
    pub fn gfc__WorldObject__getWorld;
    pub fn gfc__Object3D__getPackageID;
    pub fn gfc__Object3D__getRagdoll;
    pub fn gfc__WorldObject__doAddToWorld;
    pub fn gfc__WorldObject__doRemoveFromWorld;
    pub fn gfc__WorldObject__getClass;
    pub fn gfc__WorldObject__setRotation;
    pub fn gfc__WorldObject__setScale;
    pub fn gfc__WorldObject__getBoundingBox;
    pub fn gfc__WorldObject__getRotation;
    pub fn gfc__WorldObject__getScale;
    pub fn gfc__Object3D__setHighPriority;
    pub fn gfc__Object3D__getWorld_2;
    pub fn gfc__WorldObject__isAttached;
    pub fn gfc__WorldObject__placedInEditor;
    pub fn gfc__WorldObject__droppedToGroundInEditor;
    pub fn gfc__WorldObject__updatePost;
    pub fn gfc__WorldObject__render;
    pub fn gfc__WorldObject__drawDebug;
    pub fn gfc__WorldObject__isGroup;
    pub fn gfc__WorldObject__inGroup;
    pub fn gfc__WorldObject__isRoot;
    pub fn gfc__WorldObject__getGroup;
    pub fn gfc__WorldObject__setObject;
    pub fn gfc__WorldObject__removeFromPathFinding;
    pub fn gfc__WorldObject__overrideHitEffect;
    pub fn gfc__WorldObject__supportsStaticLighting;
    pub fn gfc__WorldObject__staticLightingIsDynamic;
    pub fn gfc__WorldObject__getAORayLength;
    pub fn gfc__WorldObject__initStaticLighting;
    pub fn gfc__WorldObject__clearStaticLighting;
    pub fn gfc__WorldObject__setHide;
    pub fn gfc__WorldObject__setFreeze;
    pub fn gfc__WorldObject__setLocked;
    pub fn gfc__WorldObject__setSelected;
    pub fn gfc__WorldObject__setDisabled;
    pub fn gfc__WorldObject__setDisplayNames;
    pub fn gfc__WorldObject__setError;
    pub fn gfc__WorldObject__setSettled;
    pub fn gfc__WorldObject__getSettled;
    pub fn gfc__WorldObject__addObject;
    pub fn gfc__WorldObject__removeObject;
    pub fn gfc__WorldObject__preload;
    pub fn gfc__WorldObject__begin;
    pub fn gfc__WorldObject__playSound;
    pub fn gfc__WorldObject__playSound_2;
    pub fn gfc__WorldObject__stopSound;
    pub fn gfc__WorldObject__getObject;
    pub fn gfc__WorldObject__setRegionID;
    pub fn gfc__WorldObject__getRegionID;
    pub fn gfc__WorldObject__setLayerID;
    pub fn gfc__WorldObject__getLayerID;
    pub fn gfc__WorldObject__setName;
    pub fn gfc__Darksiders__onPostUpdateInterval;
    pub fn gfc__WorldObject__getName;
    pub fn gfc__LoadMapMenu__LoadMapMenu;
    pub fn gfc__WindowHelper__pushWindow;
    pub fn gfc__WorldObject__getWorld_2;
    pub fn gfc__Object3D__getBodies;
    pub fn gfc__Object3D__getVisuals;
    pub fn gfc__Graphics__getInstance;
}
