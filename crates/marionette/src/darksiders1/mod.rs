#![allow(dead_code)]

pub use code::vigil::gfc::{
    base::{
        autoref::LoweredAutoRef,
        object::{Lift, Lower},
    },
    memory::memoverloads::Heap,
    util::list::List,
};

mod code;
pub mod gfc;

include!(concat!(env!("OUT_DIR"), "/cast.rs"));
