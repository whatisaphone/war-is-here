use crate::darksiders1::hkBaseObject;
use darksiders1_sys::target;

struct_wrapper!(
    /// Base for all classes in the Havok SDK.
    /// All core SDK objects that can be owned by multiple owners inherit from
    /// this class - rigid bodies, constraints, and actions are all
    /// hkReferencedObjects and any object that is memory managed by Havok
    /// also inherits from it.
    hkReferencedObject,
    target::hkReferencedObject,
);
struct_wrapper_super!(hkReferencedObject, hkBaseObject);
