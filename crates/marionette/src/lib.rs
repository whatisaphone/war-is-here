#![feature(abi_thiscall)]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::{thread, time::Duration};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{
        consoleapi::AllocConsole,
        libloaderapi::FreeLibraryAndExitThread,
        wincon::FreeConsole,
        winnt::DLL_PROCESS_ATTACH,
    },
};

mod hooks;
mod utils;

#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            struct UnsafeSend<T>(T);
            unsafe impl<T> Send for UnsafeSend<T> {}

            let hinst = UnsafeSend(hinst_dll);
            thread::spawn(move || init(hinst.0));
            TRUE
        }
        _ => TRUE,
    }
}

fn init(hinst: HINSTANCE) {
    unsafe {
        AllocConsole();
    }

    println!("Hello world :)");
    thread::sleep(Duration::from_millis(100));
    hooks::install();

    thread::sleep(Duration::from_millis(5000));

    println!("Goodbye world :(");
    hooks::uninstall();
    thread::sleep(Duration::from_millis(100));

    unsafe {
        FreeConsole();
        FreeLibraryAndExitThread(hinst, 0);
    }
}
