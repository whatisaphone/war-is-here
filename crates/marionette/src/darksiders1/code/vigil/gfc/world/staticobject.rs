use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(StaticObject, target::gfc__StaticObject);
struct_wrapper_super!(StaticObject, gfc::WorldObject);
struct_wrapper_drop!(StaticObject, target::gfc__StaticObject___StaticObject);

impl StaticObject {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__StaticObject__StaticObject(this);
            })
        };
        Self { inner }
    }
}
