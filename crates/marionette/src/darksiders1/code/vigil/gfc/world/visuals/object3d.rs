use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(Object3D, target::gfc__Object3D);
struct_wrapper_drop!(Object3D, target::gfc__Object3D___Object3D);
inherits!(Object3D, gfc::Object, as_gfc__Object_mut_ptr);

impl Object3D {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__Object3D__Object3D(this);
            })
        };
        Self { inner }
    }
}
