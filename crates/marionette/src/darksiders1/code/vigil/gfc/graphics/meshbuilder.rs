use crate::{darksiders1::gfc, utils::mem::init_with};
use darksiders1_sys::target;

struct_wrapper!(MeshBuilder, target::gfc__MeshBuilder);
struct_wrapper_super!(MeshBuilder, gfc::Object);
struct_wrapper_drop!(MeshBuilder, target::gfc__MeshBuilder___MeshBuilder);

impl MeshBuilder {
    pub fn new() -> Self {
        let inner = unsafe {
            init_with(|this| {
                target::gfc__MeshBuilder__MeshBuilder(this);
            })
        };
        Self { inner }
    }
}
