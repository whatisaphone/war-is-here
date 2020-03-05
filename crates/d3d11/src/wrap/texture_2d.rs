use crate::{
    utils::{init_with, init_with_hresult},
    DeviceContext,
    MappedSubresource,
    Resource,
    Result,
};
use std::convert::TryInto;
use winapi::{
    shared::minwindef::UINT,
    um::d3d11::{ID3D11Texture2D, D3D11_MAP, D3D11_TEXTURE2D_DESC},
};

wrap_iunknown!(pub Texture2D, *mut ID3D11Texture2D);
iunknown_cast!(Texture2D::as_resource() -> Resource);

impl Texture2D {
    #[must_use]
    pub fn get_desc_raw(&self) -> D3D11_TEXTURE2D_DESC {
        unsafe {
            init_with(|p| {
                (*self.0).GetDesc(p);
            })
        }
    }

    pub fn map<'a>(
        &'a self,
        context: &'a DeviceContext,
        subresource: UINT,
        typ: D3D11_MAP,
        flags: UINT,
    ) -> Result<MappedSubresource<'a>> {
        let desc = self.get_desc_raw();
        unsafe {
            let mapped = init_with_hresult(|p| {
                (*context.raw()).Map(self.as_resource().raw(), subresource, typ, flags, p)
            })?;
            Ok(MappedSubresource::from_raw(
                context,
                self.as_resource(),
                subresource,
                mapped,
                (mapped.RowPitch * desc.Height).try_into().unwrap(),
            ))
        }
    }
}
