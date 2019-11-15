use crate::darksiders1::hkcdShape;
use darksiders1_sys::target;

struct_wrapper!(
    /// Base interface for all physics shapes.
    /// Each virtual member function is implemented as a stub in this class, and
    /// should be reimplemented by derived shapes as required. Subsets of
    /// these functions may be registered on different SPU ELFs for each shape.
    hkpShapeBase,
    target::hkpShapeBase,
);
struct_wrapper_super!(hkpShapeBase, hkcdShape);
