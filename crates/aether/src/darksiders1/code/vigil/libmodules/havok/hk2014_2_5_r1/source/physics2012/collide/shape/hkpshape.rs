use crate::darksiders1::hkpShapeBase;
use darksiders1_sys::target;

struct_wrapper!(
    /// The base class for narrowphase collision detection objects.
    /// All narrowphase collision detection is performed between pairs of
    /// hkpShape objects by creating appropriate hkpCollisionAgent objects.
    /// An hkpShape can be a simple shape such as a box or sphere, a shape with
    /// additional transform information, or a compound shape made up of
    /// simpler hkShapes. hkpShape instances can be shared within or even
    /// between rigid bodies. See the hkpShape subclasses for more details.
    hkpShape,
    target::hkpShape,
);
struct_wrapper_super!(hkpShape, hkpShapeBase);
