#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

use crate::{errors::assert_or_last_os_error, utf16, win32::Processes};
use scopeguard::defer;
use std::{io, iter, mem, path::Path, ptr};
use winapi::{
    shared::minwindef::{DWORD, FALSE},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        memoryapi::{VirtualAllocEx, VirtualFreeEx, WriteProcessMemory},
        processthreadsapi::{CreateRemoteThread, GetExitCodeThread, OpenProcess},
        synchapi::WaitForSingleObject,
        winbase::WAIT_OBJECT_0,
        winnt::{MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE, PROCESS_ALL_ACCESS},
    },
};

pub fn inject(exe_name: &str, dll_path: &Path) -> io::Result<()> {
    let pid = match find_process_id_by_exe_name(exe_name)? {
        Some(pid) => pid,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "process not found")),
    };
    inject_load_library(pid, dll_path)
}

// Case-insensitive
fn find_process_id_by_exe_name(exe_name: &str) -> Result<Option<DWORD>, io::Error> {
    let exe_name_utf16: Vec<_> = exe_name.to_ascii_lowercase().encode_utf16().collect();

    for mut entry in Processes::snapshot()? {
        let entry_exe = utf16::sans_null_terminator(&mut entry.szExeFile);
        utf16::make_ascii_lowercase(entry_exe);
        if entry_exe == &*exe_name_utf16 {
            return Ok(Some(entry.th32ProcessID));
        }
    }
    Ok(None)
}

#[allow(clippy::shadow_unrelated)]
fn inject_load_library(pid: DWORD, dll_path: &Path) -> io::Result<()> {
    let process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };
    assert_or_last_os_error(!process.is_null())?;
    defer! {
        let result = unsafe { CloseHandle(process) };
        assert_or_last_os_error(result != 0).unwrap();
    }

    let dll_path_utf16: Vec<_> = dll_path
        .to_string_lossy()
        .encode_utf16()
        .chain(iter::once(0)) // Null terminator
        .collect();
    let size = dll_path_utf16.len() * mem::size_of::<u16>();

    let address =
        unsafe { VirtualAllocEx(process, ptr::null_mut(), size, MEM_COMMIT, PAGE_READWRITE) };
    assert_or_last_os_error(!address.is_null())?;
    defer! {
        let result = unsafe { VirtualFreeEx(process, address, 0, MEM_RELEASE) };
        assert_or_last_os_error(result != 0).unwrap();
    }

    let result = unsafe {
        WriteProcessMemory(
            process,
            address,
            dll_path_utf16.as_ptr().cast(),
            size,
            ptr::null_mut(),
        )
    };
    assert_or_last_os_error(result != 0)?;

    let kernel32 = unsafe { GetModuleHandleW(KERNEL32_UTF16_Z.as_ptr()) };
    assert_or_last_os_error(!kernel32.is_null())?;

    let load_library = unsafe { GetProcAddress(kernel32, "LoadLibraryW\0".as_ptr().cast()) };
    assert_or_last_os_error(!load_library.is_null())?;

    let thread = unsafe {
        CreateRemoteThread(
            process,
            ptr::null_mut(),
            0,
            mem::transmute(load_library),
            address,
            0,
            ptr::null_mut(),
        )
    };
    assert_or_last_os_error(!thread.is_null())?;
    defer! {
        let result = unsafe { CloseHandle(thread) };
        assert_or_last_os_error(result != 0).unwrap();
    }

    let result = unsafe { WaitForSingleObject(thread, 5000) }; // 5000ms timeout
    assert_or_last_os_error(result == WAIT_OBJECT_0)?;

    let mut exit_code = 0;
    let result = unsafe { GetExitCodeThread(thread, &mut exit_code) };
    assert_or_last_os_error(result != 0)?;

    if exit_code == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "remote LoadLibrary failed",
        ));
    }
    Ok(())
}

// The string "kernel32", UTF-16 and null-terminated. Whew.
const KERNEL32_UTF16_Z: &[u16] = &[
    'k' as u16,
    'e' as u16,
    'r' as u16,
    'n' as u16,
    'e' as u16,
    'l' as u16,
    '3' as u16,
    '2' as u16,
    '\0' as u16,
];
