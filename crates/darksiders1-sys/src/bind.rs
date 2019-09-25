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

    bind!(data, gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle);
    bind!(text, gfc__HString__HString);
    bind!(text, gfc__HString__HString_2);
    bind!(text, gfc__HString__HString_3);
    bind!(text, gfc__HString__HString_4);
    bind!(text, gfc__HString__HString_5);
    bind!(text, gfc__HString__HString_6);
    bind!(text, gfc__Darksiders__onPostUpdateInterval);
    bind!(text, gfc__LoadMapMenu__LoadMapMenu);
    bind!(text, gfc__WindowHelper__pushWindow);
}
