use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;
use std::convert::TryFrom;

struct_wrapper!(ByteInputStream, target::gfc__ByteInputStream);
struct_wrapper_super!(ByteInputStream, gfc::InputStream);
struct_wrapper_drop!(
    ByteInputStream,
    target::gfc__ByteInputStream___ByteInputStream,
);

impl ByteInputStream {
    pub fn new(buffer: &'static [u8]) -> Self {
        let take_local_copy = false;
        let take_ownership = false;
        let inner = unsafe {
            init_with(|this| {
                target::gfc__ByteInputStream__ByteInputStream(
                    this,
                    buffer.as_ptr().cast(),
                    u32::try_from(buffer.len()).unwrap(),
                    take_local_copy,
                    take_ownership,
                )
            })
        };
        Self { inner }
    }
}

struct_wrapper!(ByteOutputStream, target::gfc__ByteOutputStream);
struct_wrapper_super!(ByteOutputStream, gfc::OutputStream);
struct_wrapper_drop!(
    ByteOutputStream,
    target::gfc__ByteOutputStream___ByteOutputStream,
);

impl ByteOutputStream {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__ByteOutputStream__ByteOutputStream(this);
            })
        };
        Self { inner }
    }

    fn output(&self) -> &gfc::Vector<u8> {
        unsafe { gfc::Vector::from_ptr(&mut (*self.as_ptr()).mOutput) }
    }

    pub fn bytes(&self) -> &[u8] {
        self.output().as_slice()
    }
}
