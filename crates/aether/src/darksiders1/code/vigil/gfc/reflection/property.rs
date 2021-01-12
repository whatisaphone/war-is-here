use crate::{
    darksiders1::{gfc, Lift, Lower},
    utils::mem::init_with,
};
use darksiders1_sys::target;

struct_wrapper!(Property, target::gfc__Property);
struct_wrapper_super!(Property, gfc::IRefObject);

impl Property {
    pub fn get_value(&self, object: &gfc::Object) -> gfc::AutoRef<gfc::Value> {
        unsafe {
            init_with(|p| {
                self.inner.getValue(p, Lower::lower_ptr(object));
            })
        }
        .lift()
    }
}
