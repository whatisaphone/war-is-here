use crate::utils::mem::init_with;
use std::{convert::TryInto, mem, ptr};
use winapi::{
    shared::winerror::{HRESULT, SUCCEEDED},
    um::d3d11::{D3D11_CPU_ACCESS_READ, D3D11_MAP_READ, D3D11_USAGE_STAGING},
};

pub fn create_staging_texture2d(
    device: &d3d11::Device,
    original: &d3d11::Texture2D,
) -> d3d11::Texture2D {
    let mut desc = original.get_desc_raw();

    desc.Usage = D3D11_USAGE_STAGING;
    desc.BindFlags = 0;
    desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;

    unsafe {
        init_with_hresult(|p| (*device.raw()).CreateTexture2D(&desc, ptr::null(), p))
            .map(|p| d3d11::Texture2D::from_raw(p))
            .unwrap()
    }
}

pub fn copy_and_map_texture2d<'a>(
    context: &'a d3d11::DeviceContext,
    data: &d3d11::Texture2D,
    staging: &'a d3d11::Texture2D,
) -> d3d11::MappedSubresource<'a> {
    let desc = data.get_desc_raw();

    unsafe {
        (*context.raw()).CopyResource(staging.as_resource().raw(), data.as_resource().raw());

        let mapped = init_with(|p| {
            (*context.raw()).Map(staging.as_resource().raw(), 0, D3D11_MAP_READ, 0, p);
        });
        d3d11::MappedSubresource::from_raw(
            context,
            staging.as_resource(),
            0,
            mapped,
            (mapped.RowPitch * desc.Height).try_into().unwrap(),
        )
    }
}

pub unsafe fn init_with_hresult<T>(f: impl FnOnce(*mut T) -> HRESULT) -> Result<T, HRESULT> {
    let mut result = mem::MaybeUninit::uninit();
    let hr = f(result.as_mut_ptr());
    if SUCCEEDED(hr) {
        Ok(result.assume_init())
    } else {
        Err(hr)
    }
}
