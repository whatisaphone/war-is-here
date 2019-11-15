use crate::darksiders1::hkReferencedObject;
use darksiders1_sys::target;

#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum hkcdShapeType__ShapeTypeEnum {
    //

    // SPU supported shapes

    //
    ///< hkpSphereShape type.
    Sphere = 0,
    ///< hkpCylinderShape type.
    Cylinder = 1,
    ///< hkpTriangleShape type.
    Triangle = 2,
    ///< hkpBoxShape type.
    Box = 3,
    ///< hkpCapsuleShape type.
    Capsule = 4,
    ///< hkpConvexVerticesShape type.
    ConvexVertices = 5,
    ///< hkpTriSampledHeightFieldCollection type.
    TriSampledHeightFieldCollection = 6,
    ///< hkpTriSampledHeightFieldBvTreeShape type.
    TriSampledHeightFieldBvTree = 7,
    ///< hkpListShape type.
    List = 8,
    ///< hkpMoppBvTreeShape type.
    Mopp = 9,
    ///< hkpConvexTranslateShape type.
    ConvexTranslate = 10,
    ///< hkpConvexTransformShape type.
    ConvexTransform = 11,
    ///< hkpSampledHeightFieldShape type.
    SampledHeightField = 12,
    ///< hkpExtendedMeshShape type.
    ExtendedMesh = 13,
    ///< hkpTransformShape type.
    Transform = 14,
    ///< hkpCompressedMeshShape type.
    CompressedMesh = 15,
    ///< hkpStaticCompoundShape type.
    StaticCompound = 16,
    ///< hkpBvCompressedMeshShape type.
    BvCompressedMesh = 17,
    ///< All shapes which inherit from hkpShapeCollection have this as an
    /// alternate type.
    Collection = 18,
    ///< Custom user type.
    User0 = 19,
    ///< Custom user type.
    User1 = 20,
    ///< Custom user type.
    User2 = 21,
    //

    //	Non-SPU supported shapes

    //
    ///< All shapes which inherit from hkpBvTreeShape have this as an alternate
    /// type.
    BvTree = 22,
    ///< All shapes which inherit from hkpConvexShape have this as an alternate
    /// type.
    Convex = 23,
    ///< DEPRECATED - hkpConvexPieceShape type.
    ConvexPiece = 24,
    ///< DEPRECATED - hkpMultiSphereShape type.
    MultiSphere = 25,
    ///< DEPRECATED - hkpConvexListShape.
    ConvexList = 26,
    ///< A shape collection which only returns triangles as child shapes, e.g.,
    /// hkpMeshShape.
    TriangleCollection = 27,
    ///< hkpHeightFieldShape type.
    HeightField = 28,
    ///< hkpSphereRepShape type.
    SphereRep = 29,
    ///< hkpBvShape type.
    Bv = 30,
    ///< hkpPlaneShape type.
    Plane = 31,
    ///< hkpPhantomCallbackShape type.
    PhantomCallback = 32,
    ///< hkpMultiRayShape type.
    MultiRay = 33,
    ///< Invalid shape
    Invalid = 34,
}

struct_wrapper!(
    /// Base class for all Havok shapes
    hkcdShape,
    target::hkcdShape,
);
struct_wrapper_super!(hkcdShape, hkReferencedObject);
