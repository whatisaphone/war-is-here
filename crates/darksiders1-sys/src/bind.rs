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

    bind!(data, gfc__StaticObject___Class);
    bind!(data, gfc__OmniLight___Class);
    bind!(data, gfc__WorldGroup___Class);
    bind!(data, gfc__TriggerRegion___Class);
    bind!(
        data,
        gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(
        data,
        gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(data, gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle);
    bind!(data, gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle);
    bind!(
        text,
        gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject_
    );
    bind!(text, gfc__OblivionGame__getWorld);
    bind!(text, gfc__OmniLight__setStaticOnly);
    bind!(text, gfc__OmniLight__getStaticOnly);
    bind!(text, gfc__OmniLight__doRemoveFromWorld);
    bind!(text, gfc__StaticObject__setObjectName);
    bind!(text, gfc__StaticObject__getObjectName);
    bind!(text, gfc__StaticObject__setPackageName);
    bind!(text, gfc__StaticObject__getPackageName);
    bind!(text, gfc__StaticObject__setAORayLength);
    bind!(text, gfc__StaticObject__getNoShadows);
    bind!(text, gfc__StaticObject__setBakeLighting);
    bind!(text, gfc__StaticObject__getBakeLighting);
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
    bind!(text, gfc__StaticObject__init);
    bind!(text, gfc__StaticObject__setNoShadows);
    bind!(text, gfc__StaticObject__setPosition);
    bind!(text, gfc__StaticObject__setRotation);
    bind!(text, gfc__StaticObject__setScale);
    bind!(text, gfc__StaticObject__doAddToWorld);
    bind!(text, gfc__StaticObject__supportsStaticLighting);
    bind!(text, gfc__OmniLight__OmniLight);
    bind!(text, gfc__OmniLight__getBoundingBox);
    bind!(text, gfc__OmniLight__doAddToWorld);
    bind!(text, gfc__StaticObject__StaticObject);
    bind!(text, gfc__StaticObject__getClass);
    bind!(text, gfc__StaticObject__getPosition);
    bind!(text, gfc__StaticObject__getRotation);
    bind!(text, gfc__StaticObject__getScale);
    bind!(text, gfc__StaticObject__getObject);
    bind!(text, gfc__StaticObject__staticLightingIsDynamic);
    bind!(text, gfc__StaticObject__getAORayLength);
    bind!(text, gfc__StaticObject__StaticObject_2);
    bind!(text, gfc__StaticObject__StaticObject_3);
    bind!(text, gfc__StaticObject__setObject);
    bind!(text, gfc__StaticObject__getBoundingBox);
    bind!(text, gfc__StaticObject__doRemoveFromWorld);
    bind!(text, gfc__StaticObject__invalidateRenderNodes);
    bind!(text, gfc__StaticObject__initStaticLighting);
    bind!(text, gfc__StaticObject__clearStaticLighting);
    bind!(text, gfc__StaticObject__preload);
    bind!(text, gfc__StaticObject__getPackageID);
    bind!(
        text,
        gfc__AutoRef_gfc__IRefObject___AutoRef_gfc__IRefObject__2
    );
    bind!(
        text,
        gfc__AutoRef_gfc__IRefObject____AutoRef_gfc__IRefObject_
    );
    bind!(text, gfc__World__getRegion);
    bind!(text, gfc__ResourceCache__getResource);
    bind!(text, gfc__WorldGroup__getObjects);
    bind!(text, gfc__HString__HString);
    bind!(text, gfc__HString__HString_2);
    bind!(text, gfc__HString__HString_3);
    bind!(text, gfc__HString__HString_4);
    bind!(text, gfc__HString__HString_5);
    bind!(text, gfc__HString__HString_6);
    bind!(text, gfc__HString___HString);
    bind!(text, gfc__HString__c_str);
    bind!(text, gfc__MemAlloc);
    bind!(text, gfc__ClassRegistry__classForName);
    bind!(text, gfc__RegionLayer__getRoot);
    bind!(text, gfc__WorldRegion__getLayer);
    bind!(text, gfc__Actor__setPosition);
    bind!(text, gfc__WorldObject__setRegionID);
    bind!(text, gfc__WorldObject__setLayerID);
    bind!(text, gfc__World__getRegion_2);
    bind!(text, gfc__Darksiders__processInputEvent);
    bind!(text, gfc__Darksiders__onPostUpdateInterval);
    bind!(text, gfc__LoadMapMenu__LoadMapMenu);
    bind!(text, gfc__WindowHelper__pushWindow);
    bind!(text, gfc__TeleportHelper__warpToMap);
}
