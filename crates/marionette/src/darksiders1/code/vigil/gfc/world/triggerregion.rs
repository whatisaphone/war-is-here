use darksiders1_sys::target;

struct_wrapper!(TriggerRegion, target::gfc__TriggerRegion);
struct_wrapper_super!(
    TriggerRegion,
    gfc::DetectorObject,
    as_gfc__DetectorObject_mut_ptr,
);
impl_reflection!(TriggerRegion, target::gfc__TriggerRegion___Class);
