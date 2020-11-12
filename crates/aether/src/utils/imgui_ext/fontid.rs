use imgui::{Font, FontId};
use std::mem;

pub trait FontIdExt {
    unsafe fn from_raw(font: *const Font) -> Self;
}

impl FontIdExt for FontId {
    unsafe fn from_raw(font: *const Font) -> Self {
        // This is unsound without `repr(C)`!
        mem::transmute(font)
    }
}
