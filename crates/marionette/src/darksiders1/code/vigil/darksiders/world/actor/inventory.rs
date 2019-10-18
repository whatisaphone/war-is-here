use crate::darksiders1::{gfc, Lower};
use darksiders1_sys::target;

struct_wrapper!(Inventory, target::gfc__Inventory);
struct_wrapper_super!(Inventory, gfc::Object);

impl Inventory {
    pub fn add_item(&self, item: gfc::AutoRef<gfc::Item>) -> Result<(), ()> {
        let result =
            unsafe { target::gfc__Inventory__addItem(Lower::lower_ptr(self), Lower::lower(item)) };
        if result { Ok(()) } else { Err(()) }
    }
}
