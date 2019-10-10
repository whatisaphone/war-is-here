use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::{mem, ptr};

struct_wrapper!(OOObjectWriter, target::gfc__OOObjectWriter);
struct_wrapper_super!(OOObjectWriter, gfc::ObjectWriter);

type OOObjectWriterObjectDatabase = gfc::Vector<gfc::AutoRef<gfc::Object>>;

impl OOObjectWriter {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this: *mut target::gfc__OOObjectWriter| {
                // Rewrite of missing constructor
                target::gfc__ObjectWriter__ObjectWriter(this.static_cast());
                ptr::write(
                    &mut (*this).mObjectDatabase,
                    mem::transmute(OOObjectWriterObjectDatabase::new()),
                );
                ptr::write(&mut (*this).mWriteDefaults, false);
            })
        };
        Self { inner }
    }

    pub fn write_object(
        &mut self,
        object: &gfc::Object,
        output: &mut gfc::OutputStream,
        write_defaults: bool,
    ) {
        unsafe {
            target::gfc__OOObjectWriter__writeObject(
                self.as_ptr(),
                gfc::AutoRef::lower(gfc::AutoRef::from_ptr(object)),
                gfc::AutoRef::lower(gfc::AutoRef::from_ptr(output)),
                write_defaults,
            );
        }
    }
}

impl Drop for OOObjectWriter {
    fn drop(&mut self) {
        // Rewrite of missing destructor
        unsafe {
            ptr::drop_in_place(OOObjectWriterObjectDatabase::from_ptr_mut(
                &mut (*self.as_ptr()).mObjectDatabase,
            ));
            target::gfc__ObjectWriter___ObjectWriter(self.as_ptr().static_cast())
        }
    }
}
