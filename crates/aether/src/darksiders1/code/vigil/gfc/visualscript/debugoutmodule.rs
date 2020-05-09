use crate::darksiders1::{gfc, Lift};
use darksiders1_sys::target;
use num_enum::{IntoPrimitive, TryFromPrimitive};

struct_wrapper!(DebugOutModule, target::gfc__DebugOutModule);
struct_wrapper_super!(DebugOutModule, gfc::VisScriptModule);

#[repr(u32)]
#[derive(TryFromPrimitive)]
pub enum DebugOutModule__Actions {
    In = 0,
}

#[repr(u32)]
pub enum DebugOutModule__Events {
    Out = 0,
}

#[repr(u32)]
#[derive(IntoPrimitive)]
pub enum DebugOutModule__Variables {
    Values = 0,
}

impl DebugOutModule {
    pub fn message(&self) -> &gfc::HString {
        self.inner.mMessage.lift_ref()
    }
}
