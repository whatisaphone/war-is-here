use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(VisScriptEntity, target::gfc__VisScriptEntity);
struct_wrapper_super!(VisScriptEntity, gfc::Object);

struct_wrapper!(VisScriptModule, target::gfc__VisScriptModule);
struct_wrapper_super!(VisScriptModule, gfc::VisScriptEntity);

impl VisScriptModule {
    pub fn has_variable_in(&self, var_con_id: u32) -> bool {
        unsafe { self.inner.hasVariableIn(var_con_id) }
    }
}
