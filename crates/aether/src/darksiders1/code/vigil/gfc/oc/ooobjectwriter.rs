use crate::{
    darksiders1::{gfc, Lift, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use pdbindgen_runtime::StaticCast;
use std::ptr;

struct_wrapper!(OOObjectWriter, target::gfc__OOObjectWriter);
struct_wrapper_super!(OOObjectWriter, gfc::ObjectWriter);

impl OOObjectWriter {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this: *mut target::gfc__OOObjectWriter| {
                // Rewrite of missing constructor
                target::gfc__ObjectWriter__ObjectWriter(this.static_cast());
                ptr::write(
                    &mut (*this).mObjectDatabase,
                    Lower::lower(<gfc::Vector<gfc::AutoRef<gfc::Object>>>::new()),
                );
                ptr::write(&mut (*this).mWriteDefaults, false);
            })
        };
        Self { inner }
    }

    pub fn write_object(
        &mut self,
        object: &gfc::Object,
        output: gfc::AutoRef<gfc::OutputStream>,
        write_defaults: bool,
    ) {
        unsafe {
            target::gfc__OOObjectWriter__writeObject(
                self.as_ptr(),
                Lower::lower(gfc::AutoRef::from_ptr(object)),
                Lower::lower(output),
                write_defaults,
            );
        }
    }
}

impl Drop for OOObjectWriter {
    fn drop(&mut self) {
        // Rewrite of missing destructor
        unsafe {
            ptr::drop_in_place((*self.as_ptr()).mObjectDatabase.lift_mut());
            target::gfc__ObjectWriter___ObjectWriter(self.as_ptr().static_cast())
        }
    }
}
