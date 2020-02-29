use crate::{resource::Resource, utils::init_with};
use winapi::um::d3d11::{ID3D11Texture2D, D3D11_TEXTURE2D_DESC};

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
}
