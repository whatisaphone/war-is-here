use crate::{
    darksiders1::{gfc, Lift, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(ValueStack, target::gfc__ValueStack);

impl ValueStack {
    pub fn push(&self, val: gfc::AutoRef<gfc::Value>) {
        unsafe {
            target::gfc__ValueStack__push(self.as_ptr(), Lower::lower(val));
        }
    }

    pub fn pop(&self) -> gfc::AutoRef<gfc::Value> {
        // TODO: this will probably crash if the stack is empty, should return an
        // `Option` instead.
        unsafe {
            init_with(|p| {
                target::gfc__ValueStack__pop(self.as_ptr(), p);
            })
        }
        .lift()
    }
}
