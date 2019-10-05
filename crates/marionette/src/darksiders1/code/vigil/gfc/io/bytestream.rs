use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(ByteOutputStream, target::gfc__ByteOutputStream);
inherits!(
    ByteOutputStream,
    gfc::OutputStream,
    as_gfc__OutputStream_mut_ptr,
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

impl Drop for ByteOutputStream {
    fn drop(&mut self) {
        unsafe {
            target::gfc__ByteOutputStream___ByteOutputStream(self.as_ptr());
        }
    }
}
