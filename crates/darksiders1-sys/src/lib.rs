#![feature(abi_thiscall)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![allow(clippy::all)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub use bind::bind;

mod bind;
mod map;
mod symbols;
pub mod target;
mod types;
mod types2;
mod types3;
mod types4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
