use darksiders1_sys::target;

struct_wrapper!(
    /// Base class for all Havok classes that have virtual functions.
    /// In gcc2 for instance, if the virtual base class has data in it the
    /// vtable is placed after the data, whereas most other compilers always
    /// have the vtable at the start. Thus we have an empty virtual base
    /// class to force the vtable to always be at the start of the derived
    /// objects. All Havok managed objects inherit from a sub class of this,
    /// hkReferencedObject that stores the memory size and the reference
    /// count info (if used).
    hkBaseObject,
    target::hkBaseObject,
);
