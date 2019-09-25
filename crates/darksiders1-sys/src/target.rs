#![allow(non_camel_case_types, non_upper_case_globals)]

use super::map;
pub use super::symbols::*;
pub use super::types::*;
pub use super::types10::*;
pub use super::types11::*;
pub use super::types12::*;
pub use super::types2::*;
pub use super::types3::*;
pub use super::types4::*;
pub use super::types5::*;
pub use super::types6::*;
pub use super::types7::*;
pub use super::types8::*;
pub use super::types9::*;
use std::mem;

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
    pub fn gfc__Darksiders__onPostUpdateInterval;
}
