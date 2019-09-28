#![allow(clippy::transmute_ptr_to_ptr)]

use super::{map, target};
use pdbindgen_runtime::BindArgs;
use std::mem;

pub unsafe fn bind(args: &BindArgs) {
    macro_rules! bind {
        ($section:ident, $name:ident) => {
            target::$name = mem::transmute(args.$section.add(map::$name));
        };
    }

    bind!(data, gfc__OmniLight___Class);
    bind!(
        data,
        gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(data, gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle);
    bind!(text, gfc__OmniLight__setStaticOnly);
    bind!(text, gfc__OmniLight__getStaticOnly);
    bind!(text, gfc__OmniLight__doRemoveFromWorld);
    bind!(text, gfc__OmniLight__getClass);
    bind!(text, gfc__OmniLight__setStatic);
    bind!(text, gfc__OmniLight__getStatic);
    bind!(text, gfc__OmniLight__setColor);
    bind!(text, gfc__OmniLight__getColor);
    bind!(text, gfc__OmniLight__setPower);
    bind!(text, gfc__OmniLight__getPower);
    bind!(text, gfc__OmniLight__setCastShadows);
    bind!(text, gfc__OmniLight__getCastShadows);
    bind!(text, gfc__OmniLight__setAttenStart);
    bind!(text, gfc__OmniLight__getAttenStart);
    bind!(text, gfc__OmniLight__setAttenEnd);
    bind!(text, gfc__OmniLight__getAttenEnd);
    bind!(text, gfc__OmniLight__setScale);
    bind!(text, gfc__OmniLight__getScale);
    bind!(text, gfc__OmniLight__setSize);
    bind!(text, gfc__OmniLight__getSize);
    bind!(text, gfc__OmniLight__preload);
    bind!(text, gfc__OmniLight__OmniLight);
    bind!(text, gfc__OmniLight__getBoundingBox);
    bind!(text, gfc__OmniLight__doAddToWorld);
    bind!(text, gfc__WorldObject__addToWorld);
    bind!(text, gfc__WorldObject__setPosition);
    bind!(text, gfc__HString__HString);
    bind!(text, gfc__HString__HString_2);
    bind!(text, gfc__HString__HString_3);
    bind!(text, gfc__HString__HString_4);
    bind!(text, gfc__HString__HString_5);
    bind!(text, gfc__HString__HString_6);
    bind!(text, gfc__WorldObject__setPosition_2);
    bind!(text, gfc__Player__Player);
    bind!(text, gfc__Darksiders__onPostUpdateInterval);
    bind!(text, gfc__LoadMapMenu__LoadMapMenu);
    bind!(text, gfc__WindowHelper__pushWindow);
    bind!(text, gfc__Graphics__getInstance);
}
