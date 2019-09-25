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

    bind!(text, gfc__Darksiders__onPostUpdateInterval);
}
