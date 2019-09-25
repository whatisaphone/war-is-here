#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
#![allow(clippy::unreadable_literal)]

use super::types::*;
use super::types10::*;
use super::types11::*;
use super::types12::*;
use super::types2::*;
use super::types3::*;
use super::types4::*;
use super::types5::*;
use super::types6::*;
use super::types7::*;
use super::types8::*;
use super::types9::*;

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
    pub static gfc__Darksiders__onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32) = Text(0x295c90);
}
