use crate::errors::assert_or_last_os_error;
use std::{convert::TryInto, io, mem, ptr};
use winapi::um::{
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    tlhelp32::{
        CreateToolhelp32Snapshot,
        Process32FirstW,
        Process32NextW,
        PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
    winnt::HANDLE,
};

pub struct Processes {
    snapshot: HANDLE,
    first: bool,
}

impl Processes {
    pub fn snapshot() -> io::Result<Self> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        assert_or_last_os_error(snapshot != INVALID_HANDLE_VALUE)?;

        Ok(Self {
            snapshot,
            first: true,
        })
    }
}

impl Drop for Processes {
    fn drop(&mut self) {
        let result = unsafe { CloseHandle(self.snapshot) };
        assert_or_last_os_error(result != 0).unwrap();
    }
}

impl Iterator for Processes {
    type Item = PROCESSENTRY32W;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = <mem::MaybeUninit<PROCESSENTRY32W>>::uninit();
        let filled = unsafe {
            ptr::write(
                &mut (*result.as_mut_ptr()).dwSize,
                mem::size_of::<PROCESSENTRY32W>().try_into().unwrap(),
            );
            if self.first {
                self.first = false;
                Process32FirstW(self.snapshot, result.as_mut_ptr())
            } else {
                Process32NextW(self.snapshot, result.as_mut_ptr())
            }
        };
        if filled == 0 {
            return None;
        }
        Some(unsafe { result.assume_init() })
    }
}
