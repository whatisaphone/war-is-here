use crate::wrap::blob::Blob;
use std::{convert::TryInto, ffi::CStr, mem, ptr};
use winapi::{shared::winerror::FAILED, um::d3dcompiler};

pub fn compile(source: &str, entrypoint: &CStr, target: &CStr) -> Result<Blob, String> {
    #[cfg(debug_assertions)]
    let flags = d3dcompiler::D3DCOMPILE_DEBUG
        | d3dcompiler::D3DCOMPILE_ENABLE_STRICTNESS
        | d3dcompiler::D3DCOMPILE_OPTIMIZATION_LEVEL0
        | d3dcompiler::D3DCOMPILE_WARNINGS_ARE_ERRORS;
    #[cfg(not(debug_assertions))]
    let flags = d3dcompiler::D3DCOMPILE_OPTIMIZATION_LEVEL3;

    unsafe {
        let mut code = mem::MaybeUninit::uninit();
        let mut error = mem::MaybeUninit::uninit();
        let hr = d3dcompiler::D3DCompile(
            source.as_ptr().cast(),
            source.len().try_into().unwrap(),
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
            entrypoint.as_ptr(),
            target.as_ptr(),
            flags,
            0,
            code.as_mut_ptr(),
            error.as_mut_ptr(),
        );

        if FAILED(hr) {
            let error = Blob::from_raw(error.assume_init());
            return Err(String::from_utf8_lossy(error.buffer()).into_owned());
        }

        Ok(Blob::from_raw(code.assume_init()))
    }
}
