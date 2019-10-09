#![allow(dead_code)]

pub use code::vigil::gfc::{memory::memoverloads::Heap, util::list::List};

mod code;
pub mod gfc;

include!(concat!(env!("OUT_DIR"), "/cast.rs"));
