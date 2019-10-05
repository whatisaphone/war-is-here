#![allow(non_camel_case_types, non_upper_case_globals)]

use super::map;
pub use super::{symbols::*, types::*, types2::*, types3::*};
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
    pub static gfc__OmniLight___Class;
    pub static gfc__WorldGroup___Class;
    pub static gfc__TriggerRegion___Class;
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    pub fn gfc__MeshReader__readObject;
    pub fn gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject_;
    pub fn gfc__OblivionGame__getWorld;
    pub fn gfc__OmniLight__setStaticOnly;
    pub fn gfc__OmniLight__getStaticOnly;
    pub fn gfc__OmniLight__doRemoveFromWorld;
    pub fn gfc__StaticObject__setObjectName;
    pub fn gfc__StaticObject__setPackageName;
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
    pub fn gfc__OmniLight__OmniLight;
    pub fn gfc__OmniLight__getBoundingBox;
    pub fn gfc__OmniLight__doAddToWorld;
    pub fn gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject__2;
    pub fn gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_;
    pub fn gfc__World__getRegion;
    pub fn gfc__ResourceCache__getResource;
    pub fn gfc__WorldGroup__getObjects;
    pub fn gfc__HString__HString;
    pub fn gfc__HString__HString_2;
    pub fn gfc__HString__HString_3;
    pub fn gfc__HString__HString_4;
    pub fn gfc__HString__HString_5;
    pub fn gfc__HString__HString_6;
    pub fn gfc__HString___HString;
    pub fn gfc__HString__c_str;
    pub fn gfc__OOObjectWriter__writeObject;
    pub fn gfc__MemFree;
    pub fn gfc__MemAlloc;
    pub fn gfc__ObjectWriter__ObjectWriter;
    pub fn gfc__ByteOutputStream__ByteOutputStream;
    pub fn gfc__ByteOutputStream___ByteOutputStream;
    pub fn gfc__ByteOutputStream__ByteOutputStream_2;
    pub fn gfc__ClassRegistry__classForName;
    pub fn gfc__RegionLayer__getRoot;
    pub fn gfc__WorldRegion__getLayer;
    pub fn operator_new;
    pub fn gfc__WorldObject__setRegionID;
    pub fn gfc__WorldObject__setLayerID;
    pub fn gfc__ObjectWriter___ObjectWriter;
    pub fn gfc__World__getRegion_2;
    pub fn gfc__Darksiders__processInputEvent;
    pub fn gfc__Darksiders__onPostUpdateInterval;
    pub fn gfc__LoadMapMenu__LoadMapMenu;
    pub fn gfc__WindowHelper__pushWindow;
    pub fn gfc__TeleportHelper__warpToMap;
    pub fn operator_new_2;
    pub fn operator_new_3;
    pub fn operator_new_4;
    pub fn operator_new_5;
}
