use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(ObjectWriter, target::gfc__ObjectWriter);
struct_wrapper_super!(ObjectWriter, gfc::IRefObject, as_gfc__IRefObject_mut_ptr);
