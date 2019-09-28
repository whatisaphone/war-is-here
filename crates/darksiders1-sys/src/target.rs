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
    pub static gfc__OmniLight___Class;
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    pub fn gfc__OmniLight__setStaticOnly;
    pub fn gfc__OmniLight__getStaticOnly;
    pub fn gfc__OmniLight__doRemoveFromWorld;
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
    pub fn gfc__WorldObject__addToWorld;
    pub fn gfc__WorldObject__setPosition;
    pub fn gfc__HString__HString;
    pub fn gfc__HString__HString_2;
    pub fn gfc__HString__HString_3;
    pub fn gfc__HString__HString_4;
    pub fn gfc__HString__HString_5;
    pub fn gfc__HString__HString_6;
    pub fn gfc__MemAlloc;
    pub fn gfc__WorldObject__setPosition_2;
    pub fn gfc__Player__Player;
    pub fn gfc__Darksiders__onPostUpdateInterval;
    pub fn gfc__LoadMapMenu__LoadMapMenu;
    pub fn gfc__WindowHelper__pushWindow;
    pub fn gfc__Graphics__getInstance;
}
