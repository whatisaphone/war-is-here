use crate::darksiders1::{gfc, List};
use darksiders1_sys::target;

struct_wrapper!(WorldGroup, target::gfc__WorldGroup);
struct_wrapper_super!(WorldGroup, gfc::WorldObject);
impl_reflection!(WorldGroup, target::gfc__WorldGroup___Class);

impl WorldGroup {
    pub fn objects(&self) -> &List<'_, gfc::AutoRef<gfc::WorldObject>> {
        unsafe { List::from_ptr(&(*self.as_ptr()).mObjects) }
    }
}
