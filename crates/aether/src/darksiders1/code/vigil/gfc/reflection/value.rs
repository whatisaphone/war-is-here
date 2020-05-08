use crate::{
    darksiders1::{gfc, Lift},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(Value, target::gfc__Value);
struct_wrapper_super!(Value, gfc::IRefObject);

impl Value {
    pub fn get_string(&self) -> gfc::String {
        unsafe {
            init_with(|p| {
                self.inner.getString(p);
            })
        }
        .lift()
    }
}
