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
    pub static gfc__OmniLight___Class: *mut gfc__Class = Data(0x88cf24);
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle: *mut gfc__Darksiders = Data(0xa1afc);
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle: *mut gfc__WindowHelper = Data(0xa1b0c);
    pub static gfc__OmniLight__setStaticOnly: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: bool) = Text(0xe26960);
    pub static gfc__OmniLight__getStaticOnly: unsafe extern "thiscall" fn(this: *const gfc__OmniLight) -> bool = Text(0xe26970);
    pub static gfc__OmniLight__doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe271c0);
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
    pub static gfc__OmniLight__OmniLight: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4c9b0);
    pub static gfc__OmniLight__getBoundingBox: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight, _: *mut gfc__TBox_float_gfc__FloatMath_) = Text(0xe4cae0);
    pub static gfc__OmniLight__doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__OmniLight) = Text(0xe4cd00);
    pub static gfc__WorldObject__addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World) = Text(0xdcc370);
    pub static gfc__WorldObject__setPosition: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: f32, _: f32) = Text(0xdd6260);
    pub static gfc__HString__HString: unsafe extern "thiscall" fn(this: *mut gfc__HString) = Text(0xa63b40);
    pub static gfc__HString__HString_2: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__String) = Text(0xa6e4a0);
    pub static gfc__HString__HString_3: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const i8, _: bool) = Text(0xa6e550);
    pub static gfc__HString__HString_4: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64) = Text(0xa6e600);
    pub static gfc__HString__HString_5: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: *const gfc__HString) = Text(0xa6e690);
    pub static gfc__HString__HString_6: unsafe extern "thiscall" fn(this: *mut gfc__HString, _: u64, _: *const i8, _: i32) = Text(0xa6e720);
    pub static gfc__WorldObject__setPosition_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__TVector3_float_gfc__FloatMath_) = Text(0x908fc0);
    pub static gfc__Player__Player: unsafe extern "thiscall" fn(this: *mut gfc__Player) = Text(0x8593b0);
    pub static gfc__Darksiders__onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32) = Text(0x295c90);
    pub static gfc__LoadMapMenu__LoadMapMenu: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) = Text(0x201d50);
    pub static gfc__WindowHelper__pushWindow: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString) = Text(0x1bc090);
    pub static gfc__Graphics__getInstance: unsafe extern "C" fn() -> *mut gfc__Graphics = Text(0xd55d0);
}
