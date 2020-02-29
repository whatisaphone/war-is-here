use crate::{DeviceContext, Resource};
use std::slice;
use winapi::{shared::minwindef::UINT, um::d3d11::D3D11_MAPPED_SUBRESOURCE};

pub struct MappedSubresource<'a> {
    context: &'a DeviceContext,
    resource: &'a Resource,
    subresource: UINT,
    raw: D3D11_MAPPED_SUBRESOURCE,
    len: usize,
}

impl<'a> MappedSubresource<'a> {
    #[must_use]
    pub unsafe fn from_raw(
        context: &'a DeviceContext,
        resource: &'a Resource,
        subresource: UINT,
        raw: D3D11_MAPPED_SUBRESOURCE,
        len: usize,
    ) -> Self {
        Self {
            context,
            resource,
            subresource,
            raw,
            len,
        }
    }

    #[must_use]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.raw.pData.cast(), self.len) }
    }

    #[must_use]
    pub fn row_pitch(&self) -> UINT {
        self.raw.RowPitch
    }
}

impl<'a> Drop for MappedSubresource<'a> {
    fn drop(&mut self) {
        unsafe {
            (*self.context.raw()).Unmap(self.resource.raw(), self.subresource);
        }
    }
}
