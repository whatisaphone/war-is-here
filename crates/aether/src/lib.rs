#![feature(abi_thiscall, map_first_last, vec_into_raw_parts)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(clippy::single_match, clippy::single_match_else)]
#![cfg_attr(feature = "strict", deny(warnings))]

use crate::utils::marker::UnsafeSend;
use std::{env, thread, time::Duration};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{
        consoleapi::AllocConsole,
        libloaderapi::FreeLibraryAndExitThread,
        wincon::FreeConsole,
        winnt::DLL_PROCESS_ATTACH,
    },
};

#[macro_use]
mod macros;

mod commands;
mod control;
mod darksiders1;
mod hooks;
mod library;
mod splash;
mod ui;
mod utils;

#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            let hinst = UnsafeSend::new(hinst_dll);
            thread::spawn(move || init(hinst.into_inner()));
            TRUE
        }
        _ => TRUE,
    }
}

fn init(hinst: HINSTANCE) {
    unsafe {
        #[cfg(debug_assertions)]
        AllocConsole();
    }

    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    println!("Hello world :)");
    hooks::install();

    // This will pump messages until the user requests us to terminate.
    control::start();

    println!("Goodbye world :(");
    hooks::uninstall();

    // Release the lock, and let any code that was waiting on it return. This sleep
    // doesn't actually seem to be necessary, but better safe than sorry.
    thread::sleep(Duration::from_millis(100));

    hooks::cleanup();

    unsafe {
        #[cfg(debug_assertions)]
        FreeConsole();

        FreeLibraryAndExitThread(hinst, 0);
    }
}
