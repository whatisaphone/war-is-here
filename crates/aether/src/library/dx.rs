use std::{mem, ptr};
use winapi::{
    shared::winerror::{HRESULT, SUCCEEDED},
    um::d3d11::{D3D11_CPU_ACCESS_READ, D3D11_USAGE_STAGING},
};

pub fn create_staging_texture2d(
    device: &d3d11::Device,
    original: &d3d11::Texture2D,
) -> d3d11::Result<d3d11::Texture2D> {
    let mut desc = original.get_desc_raw();

    desc.Usage = D3D11_USAGE_STAGING;
    desc.BindFlags = 0;
    desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;

    unsafe {
        init_with_hresult(|p| (*device.raw()).CreateTexture2D(&desc, ptr::null(), p))
            .map(|p| d3d11::Texture2D::from_raw(p))
    }
}

pub unsafe fn init_with_hresult<T>(f: impl FnOnce(*mut T) -> HRESULT) -> d3d11::Result<T> {
    let mut result = mem::MaybeUninit::uninit();
    let hr = f(result.as_mut_ptr());
    if SUCCEEDED(hr) {
        Ok(result.assume_init())
    } else {
        Err(hr)
    }
}
