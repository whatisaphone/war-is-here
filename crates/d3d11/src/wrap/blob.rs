use std::slice;
use winapi::um::d3dcommon::ID3DBlob;

wrap_iunknown!(pub Blob, *mut ID3DBlob);

impl Blob {
    #[must_use]
    pub fn buffer(&self) -> &[u8] {
        unsafe {
            let ptr = (*self.0).GetBufferPointer().cast::<u8>();
            let len = (*self.0).GetBufferSize();
            slice::from_raw_parts(ptr, len)
        }
    }
}
