#![feature(abi_thiscall, const_transmute)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(dead_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub use bind::bind;

mod bind;
mod map;
mod symbols;
pub mod target;
mod types;
mod types10;
mod types11;
mod types12;
mod types13;
mod types14;
mod types15;
mod types16;
mod types17;
mod types18;
mod types19;
mod types2;
mod types3;
mod types4;
mod types5;
mod types6;
mod types7;
mod types8;
mod types9;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
