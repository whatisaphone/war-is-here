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
            let object = gfc::AutoRef::<gfc::Object>::from_ptr(object.as_ptr().static_cast());
            let output = gfc::AutoRef::<gfc::OutputStream>::from_ptr(output.as_ptr().static_cast());
            target::gfc__OOObjectWriter__writeObject(
                self.as_ptr(),
                autoref_cast!(object, target::gfc__AutoRef_gfc__Object_),
                autoref_cast!(output, target::gfc__AutoRef_gfc__OutputStream_),
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
