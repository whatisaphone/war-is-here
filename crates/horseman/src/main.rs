#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::env;

mod errors;
mod inject;
mod utf16;
mod win32;

fn main() {
    let dll_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("aether.dll");
    inject::inject("darksiders1.exe", &dll_path).unwrap();
}
