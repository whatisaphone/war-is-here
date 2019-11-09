#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types::*, types2::*, types3::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

#[repr(C)]
pub struct hkpBroadPhaseCastCollector {
    pub vfptr: *const hkpBroadPhaseCastCollector__vftable,
}

impl hkpBroadPhaseCastCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBroadPhaseHandle(
        &self,
        a1: *const hkpBroadPhaseHandle,
        a2: i32,
    ) -> f32 {
        ((*self.vfptr).addBroadPhaseHandle)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpBroadPhaseCastCollector__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseCastCollector, _: u32) -> *mut (),
    pub addBroadPhaseHandle: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseCastCollector,
        _: *const hkpBroadPhaseHandle,
        _: i32,
    ) -> f32,
}

#[repr(C)]
pub struct hkpBroadPhase__hkpCastRayInput {
    pub m_from: hkVector4f,
    pub m_numCasts: i32,
    pub m_toBase: *const hkVector4f,
    pub m_toStriding: i32,
    pub m_aabbCacheInfo: *const i8,
}

#[repr(C)]
pub struct hkpBroadPhase__hkpCastAabbInput {
    pub m_from: hkVector4f,
    pub m_to: hkVector4f,
    pub m_halfExtents: hkVector4f,
    pub m_aabbCacheInfo: *const i8,
    __pdbindgen_padding: [u8; 12],
}

#[repr(C)]
pub struct hkpCollisionFilter {
    // hkBaseObject
    pub vfptr: *const hkpCollisionFilter__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpCollidableCollidableFilter
    pub vfptr_2: *const hkpCollisionFilter__vftable,
    // hkpShapeCollectionFilter
    pub vfptr_3: *const hkpCollisionFilter__vftable,
    // hkpRayShapeCollectionFilter
    pub vfptr_4: *const hkpCollisionFilter__vftable,
    // hkpRayCollidableFilter
    pub vfptr_5: *const hkpCollisionFilter__vftable,
    // hkpCollisionFilter
    pub m_prepad: [u32; 2],
    pub m_type: hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_,
    pub m_postpad: [u32; 3],
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpCollisionFilter {}

unsafe impl UpcastToNop<hkBaseObject> for hkpCollisionFilter {}

unsafe impl UpcastTo<hkpCollidableCollidableFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpCollidableCollidableFilter {
        (p as usize + 0x8) as *const _
    }
}

unsafe impl UpcastTo<hkpShapeCollectionFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpShapeCollectionFilter {
        (p as usize + 0xc) as *const _
    }
}

unsafe impl UpcastTo<hkpRayShapeCollectionFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpRayShapeCollectionFilter {
        (p as usize + 0x10) as *const _
    }
}

unsafe impl UpcastTo<hkpRayCollidableFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpRayCollidableFilter {
        (p as usize + 0x14) as *const _
    }
}

impl hkpCollisionFilter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isCollisionEnabled(
        &self,
        result: *mut hkBool,
        a2: *const hkpWorldRayCastInput,
        a3: *const hkpCollidable,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn numShapeKeyHitsLimitBreached(
        &self,
        a1: *const hkpCollisionInput,
        a2: *const hkpCdBody,
        a3: *const hkpCdBody,
        a4: *const hkpBvTreeShape,
        a5: *mut hkAabb,
        a6: *mut u32,
        a7: i32,
    ) -> i32 {
        ((*self.vfptr).numShapeKeyHitsLimitBreached)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        )
    }

    pub unsafe extern "thiscall" fn __vecDelDtor_2(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn init(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).init)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpCollisionFilter__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: u32) -> *mut (),
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpCollisionFilter,
        result: *mut hkBool,
        _: *const hkpWorldRayCastInput,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
    pub numShapeKeyHitsLimitBreached: unsafe extern "thiscall" fn(
        this: *const hkpCollisionFilter,
        _: *const hkpCollisionInput,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpBvTreeShape,
        _: *mut hkAabb,
        _: *mut u32,
        _: i32,
    ) -> i32,
    pub __vecDelDtor_2:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: u32) -> *mut (),
    pub init: unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpCdPoint {
    pub m_contact: hkContactPoint,
    pub m_unweldedNormal: hkVector4f,
    pub m_cdBodyA: *const hkpCdBody,
    pub m_cdBodyB: *const hkpCdBody,
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct hkpSphereRepShape {
    // hkBaseObject
    pub vfptr: *const hkpSphereRepShape__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkcdShape
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    // hkpShapeBase
    // hkpShape
    pub m_userData: u32,
    // hkpSphereRepShape
}

unsafe impl UpcastToNop<hkpShape> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSphereRepShape {}

impl hkpSphereRepShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isConvex(&self) -> bool {
        ((*self.vfptr).isConvex)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAabb(
        &self,
        a1: *const hkTransformf,
        a2: f32,
        a3: *mut hkAabb,
    ) {
        ((*self.vfptr).getAabb)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRay(
        &self,
        result: *mut hkBool,
        a2: *const hkpShapeRayCastInput,
        a3: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool {
        ((*self.vfptr).castRay)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayWithCollector(
        &self,
        a1: *const hkpShapeRayCastInput,
        a2: *const hkpCdBody,
        a3: *mut hkpRayHitCollector,
    ) {
        ((*self.vfptr).castRayWithCollector)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayBundle(
        &self,
        result: *mut hkVector4fComparison,
        a2: *const hkpShapeRayBundleCastInput,
        a3: *mut hkpShapeRayBundleCastOutput,
        a4: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison {
        ((*self.vfptr).castRayBundle)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSupportingVertex(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkcdVertex,
    ) {
        ((*self.vfptr).getSupportingVertex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertVertexIdsToVertices(
        &self,
        a1: *const u16,
        a2: i32,
        a3: *mut hkcdVertex,
    ) {
        ((*self.vfptr).convertVertexIdsToVertices)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCentre(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getCentre)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumCollisionSpheres(&self) -> i32 {
        ((*self.vfptr).getNumCollisionSpheres)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCollisionSpheres(
        &self,
        a1: *mut hkSphere,
    ) -> *const hkSphere {
        ((*self.vfptr).getCollisionSpheres)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn weldContactPoint(
        &self,
        a1: *mut u16,
        a2: *mut u8,
        a3: *mut hkVector4f,
        a4: *const hkTransformf,
        a5: *const hkpConvexShape,
        a6: *const hkTransformf,
        a7: *mut hkVector4f,
    ) -> i32 {
        ((*self.vfptr).weldContactPoint)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6, a7)
    }

    pub unsafe extern "thiscall" fn getContainer(&self) -> *const hkpShapeContainer {
        ((*self.vfptr).getContainer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaximumProjection(&self, a1: *const hkVector4f) -> f32 {
        ((*self.vfptr).getMaximumProjection)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn calcSizeForSpu(
        &self,
        a1: *const hkpShape__CalcSizeForSpuInput,
        a2: i32,
    ) -> i32 {
        ((*self.vfptr).calcSizeForSpu)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpSphereRepShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpSphereRepShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpSphereRepShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpSphereRepShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
}

#[repr(C)]
pub struct hkpConvexShape {
    // hkBaseObject
    pub vfptr: *const hkpConvexShape__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkcdShape
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    // hkpShapeBase
    // hkpShape
    pub m_userData: u32,
    // hkpSphereRepShape
    // hkpConvexShape
    pub m_radius: f32,
}

unsafe impl UpcastToNop<hkpSphereRepShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkpShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpConvexShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexShape {}

impl hkpConvexShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isConvex(&self) -> bool {
        ((*self.vfptr).isConvex)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAabb(
        &self,
        a1: *const hkTransformf,
        a2: f32,
        a3: *mut hkAabb,
    ) {
        ((*self.vfptr).getAabb)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRay(
        &self,
        result: *mut hkBool,
        a2: *const hkpShapeRayCastInput,
        a3: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool {
        ((*self.vfptr).castRay)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayWithCollector(
        &self,
        a1: *const hkpShapeRayCastInput,
        a2: *const hkpCdBody,
        a3: *mut hkpRayHitCollector,
    ) {
        ((*self.vfptr).castRayWithCollector)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayBundle(
        &self,
        result: *mut hkVector4fComparison,
        a2: *const hkpShapeRayBundleCastInput,
        a3: *mut hkpShapeRayBundleCastOutput,
        a4: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison {
        ((*self.vfptr).castRayBundle)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSupportingVertex(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkcdVertex,
    ) {
        ((*self.vfptr).getSupportingVertex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertVertexIdsToVertices(
        &self,
        a1: *const u16,
        a2: i32,
        a3: *mut hkcdVertex,
    ) {
        ((*self.vfptr).convertVertexIdsToVertices)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCentre(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getCentre)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumCollisionSpheres(&self) -> i32 {
        ((*self.vfptr).getNumCollisionSpheres)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCollisionSpheres(
        &self,
        a1: *mut hkSphere,
    ) -> *const hkSphere {
        ((*self.vfptr).getCollisionSpheres)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn weldContactPoint(
        &self,
        a1: *mut u16,
        a2: *mut u8,
        a3: *mut hkVector4f,
        a4: *const hkTransformf,
        a5: *const hkpConvexShape,
        a6: *const hkTransformf,
        a7: *mut hkVector4f,
    ) -> i32 {
        ((*self.vfptr).weldContactPoint)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6, a7)
    }

    pub unsafe extern "thiscall" fn getContainer(&self) -> *const hkpShapeContainer {
        ((*self.vfptr).getContainer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaximumProjection(&self, a1: *const hkVector4f) -> f32 {
        ((*self.vfptr).getMaximumProjection)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn calcSizeForSpu(
        &self,
        a1: *const hkpShape__CalcSizeForSpuInput,
        a2: i32,
    ) -> i32 {
        ((*self.vfptr).calcSizeForSpu)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getFirstVertex(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getFirstVertex)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(&self) -> i32 {
        ((*self.vfptr).getSize)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpConvexShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpConvexShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpConvexShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConvexShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub getFirstVertex:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *mut hkVector4f),
    pub getSize: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> i32,
}

#[repr(C)]
pub struct hkpShapeRayBundleCastInput {
    pub m_from: hkFourTransposedPointsf,
    pub m_to: hkFourTransposedPointsf,
    pub m_filterInfo: u32,
    pub m_userData: u32,
    pub m_rayShapeCollectionFilter: *const hkpRayShapeCollectionFilter,
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct hkpAabbCastCollector {
    pub vfptr: *const hkpAabbCastCollector__vftable,
    __pdbindgen_padding: [u8; 12],
    pub m_earlyOutFraction: hkSimdFloat32,
}

impl hkpAabbCastCollector {
    pub unsafe extern "thiscall" fn addHit(&self, a1: u32) {
        ((*self.vfptr).addHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpAabbCastCollector__vftable {
    pub addHit: unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32) -> *mut (),
}

#[repr(C)]
pub struct hkpConvexVerticesConnectivity {
    // hkBaseObject
    pub vfptr: *const hkpConvexVerticesConnectivity__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConvexVerticesConnectivity
    pub m_vertexIndices: hkArray_unsigned_short_hkContainerHeapAllocator_,
    pub m_numVerticesPerFace: hkArray_unsigned_char_hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexVerticesConnectivity {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexVerticesConnectivity {}

impl hkpConvexVerticesConnectivity {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpConvexVerticesConnectivity__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesConnectivity) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesConnectivity),
    pub clear: unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity),
}

#[repr(C)]
pub struct hkpConvexListFilter {
    // hkBaseObject
    pub vfptr: *const hkpConvexListFilter__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConvexListFilter
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexListFilter {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexListFilter {}

impl hkpConvexListFilter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getConvexListCollisionType(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpCollisionInput,
    ) -> hkpConvexListFilter__ConvexListCollisionType {
        ((*self.vfptr).getConvexListCollisionType)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpConvexListFilter__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConvexListFilter, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConvexListFilter),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConvexListFilter) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConvexListFilter),
    pub getConvexListCollisionType:
        unsafe extern "thiscall" fn(
            this: *const hkpConvexListFilter,
            _: *const hkpCdBody,
            _: *const hkpCdBody,
            _: *const hkpCollisionInput,
        ) -> hkpConvexListFilter__ConvexListCollisionType,
}

#[repr(C)]
pub struct hkEnum_enum_hkpWorldCinfo__BroadPhaseType_signed_char_ {
    pub m_storage: i8,
}

#[repr(C)]
pub struct hkEnum_enum_hkpWorldCinfo__ContactPointGeneration_signed_char_ {
    pub m_storage: i8,
}

#[repr(C)]
pub struct hkpCollidableAddedEvent {
    pub m_phantom: *const hkpPhantom,
    pub m_collidable: *const hkpCollidable,
    pub m_collidableAccept: hkpCollidableAccept,
}

#[repr(C)]
pub struct hkpPhantom {
    // hkBaseObject
    pub vfptr: *const hkpPhantom__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldObject
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
    // hkpPhantom
    pub m_overlapListeners: hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_,
    pub m_phantomListeners: hkArray_hkpPhantomListener___hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkpWorldObject> for hkpPhantom {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpPhantom {}

unsafe impl UpcastToNop<hkBaseObject> for hkpPhantom {}

impl hkpPhantom {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setShape(
        &self,
        a1: *const hkpShape,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).setShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShape(
        &self,
        a1: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).updateShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMotionState(&self) -> *mut hkMotionState {
        ((*self.vfptr).getMotionState)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpPhantomType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn calcAabb(&self, a1: *mut hkAabb) {
        ((*self.vfptr).calcAabb)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addOverlappingCollidable(&self, a1: *mut hkpCollidable) {
        ((*self.vfptr).addOverlappingCollidable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isOverlappingCollidableAdded(
        &self,
        result: *mut hkBool,
        a2: *const hkpCollidable,
    ) -> *mut hkBool {
        ((*self.vfptr).isOverlappingCollidableAdded)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn removeOverlappingCollidable(&self, a1: *mut hkpCollidable) {
        ((*self.vfptr).removeOverlappingCollidable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn ensureDeterministicOrder(&self) {
        ((*self.vfptr).ensureDeterministicOrder)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clone(&self) -> *mut hkpPhantom {
        ((*self.vfptr).clone)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateShapeCollectionFilter(&self) {
        ((*self.vfptr).updateShapeCollectionFilter)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deallocateInternalArrays(&self) {
        ((*self.vfptr).deallocateInternalArrays)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpPhantom__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpPhantom),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpPhantom,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpPhantom,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState: unsafe extern "thiscall" fn(this: *mut hkpPhantom) -> *mut hkMotionState,
    pub getType: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> hkpPhantomType,
    pub calcAabb: unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkAabb),
    pub addOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkpCollidable),
    pub isOverlappingCollidableAdded: unsafe extern "thiscall" fn(
        this: *mut hkpPhantom,
        result: *mut hkBool,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
    pub removeOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkpCollidable),
    pub ensureDeterministicOrder: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub clone: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> *mut hkpPhantom,
    pub updateShapeCollectionFilter: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
}

#[repr(C)]
pub struct hkpWorld {
    // hkBaseObject
    pub vfptr: *const hkpWorld__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorld
    pub m_simulation: *mut hkpSimulation,
    __pdbindgen_padding: [u8; 4],
    pub m_gravity: hkVector4f,
    pub m_fixedIsland: *mut hkpSimulationIsland,
    pub m_fixedRigidBody: *mut hkpRigidBody,
    pub m_activeSimulationIslands: hkArray_hkpSimulationIsland___hkContainerHeapAllocator_,
    pub m_inactiveSimulationIslands: hkArray_hkpSimulationIsland___hkContainerHeapAllocator_,
    pub m_dirtySimulationIslands: hkArray_hkpSimulationIsland___hkContainerHeapAllocator_,
    pub m_maintenanceMgr: *mut hkpWorldMaintenanceMgr,
    pub m_memoryWatchDog: hkRefPtr_hkWorldMemoryAvailableWatchDog_,
    pub m_assertOnRunningOutOfSolverMemory: hkBool,
    pub m_broadPhaseType: hkEnum_enum_hkpWorldCinfo__BroadPhaseType_signed_char_,
    pub m_broadPhase: *mut hkpBroadPhase,
    pub m_broadPhaseDispatcher: *mut hkpTypedBroadPhaseDispatcher,
    pub m_phantomBroadPhaseListener: *mut hkpPhantomBroadPhaseListener,
    pub m_entityEntityBroadPhaseListener: *mut hkpEntityEntityBroadPhaseListener,
    pub m_broadPhaseBorderListener: *mut hkpBroadPhaseBorderListener,
    pub m_multithreadedSimulationJobData: *mut hkpMtThreadStructure,
    pub m_collisionInput: *mut hkpProcessCollisionInput,
    pub m_collisionFilter: *mut hkpCollisionFilter,
    pub m_collisionDispatcher: *mut hkpCollisionDispatcher,
    pub m_convexListFilter: *mut hkpConvexListFilter,
    pub m_pendingOperations: *mut hkpWorldOperationQueue,
    pub m_pendingOperationsCount: i32,
    pub m_pendingBodyOperationsCount: i32,
    pub m_criticalOperationsLockCount: i32,
    pub m_criticalOperationsLockCountForPhantoms: i32,
    pub m_blockExecutingPendingOperations: hkBool,
    pub m_criticalOperationsAllowed: hkBool,
    pub m_pendingOperationQueues: *mut hkpDebugInfoOnPendingOperationQueues,
    pub m_pendingOperationQueueCount: i32,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_processActionsInSingleThread: hkBool,
    pub m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: hkBool,
    pub m_minDesiredIslandSize: u32,
    pub m_modifyConstraintCriticalSection: *mut hkCriticalSection,
    pub m_isLocked: i32,
    pub m_islandDirtyListCriticalSection: *mut hkCriticalSection,
    pub m_propertyMasterLock: *mut hkCriticalSection,
    pub m_wantSimulationIslands: hkBool,
    pub m_snapCollisionToConvexEdgeThreshold: f32,
    pub m_snapCollisionToConcaveEdgeThreshold: f32,
    pub m_enableToiWeldRejection: hkBool,
    pub m_wantDeactivation: hkBool,
    pub m_shouldActivateOnRigidBodyTransformChange: hkBool,
    pub m_deactivationReferenceDistance: f32,
    pub m_toiCollisionResponseRotateNormal: f32,
    pub m_maxSectorsPerMidphaseCollideTask: i32,
    pub m_maxSectorsPerNarrowphaseCollideTask: i32,
    pub m_processToisMultithreaded: hkBool,
    pub m_maxEntriesPerToiMidphaseCollideTask: i32,
    pub m_maxEntriesPerToiNarrowphaseCollideTask: i32,
    pub m_maxNumToiCollisionPairsSinglethreaded: i32,
    pub m_simulationType: hkEnum_enum_hkpWorldCinfo__SimulationType_int_,
    pub m_numToisTillAllowedPenetrationSimplifiedToi: f32,
    pub m_numToisTillAllowedPenetrationToi: f32,
    pub m_numToisTillAllowedPenetrationToiHigher: f32,
    pub m_numToisTillAllowedPenetrationToiForced: f32,
    pub m_lastEntityUid: u32,
    pub m_lastIslandUid: u32,
    pub m_lastConstraintUid: u32,
    pub m_phantoms: hkArray_hkpPhantom___hkContainerHeapAllocator_,
    pub m_actionListeners: hkArray_hkpActionListener___hkContainerHeapAllocator_,
    pub m_entityListeners: hkArray_hkpEntityListener___hkContainerHeapAllocator_,
    pub m_phantomListeners: hkArray_hkpPhantomListener___hkContainerHeapAllocator_,
    pub m_constraintListeners: hkArray_hkpConstraintListener___hkContainerHeapAllocator_,
    pub m_worldDeletionListeners: hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_,
    pub m_islandActivationListeners:
        hkArray_hkpIslandActivationListener___hkContainerHeapAllocator_,
    pub m_worldPostSimulationListeners:
        hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_,
    pub m_worldPostIntegrateListeners:
        hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_,
    pub m_worldPostCollideListeners:
        hkArray_hkpWorldPostCollideListener___hkContainerHeapAllocator_,
    pub m_islandPostIntegrateListeners:
        hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_,
    pub m_islandPostCollideListeners:
        hkArray_hkpIslandPostCollideListener___hkContainerHeapAllocator_,
    pub m_contactListeners: hkArray_hkpContactListener___hkContainerHeapAllocator_,
    pub m_contactImpulseLimitBreachedListeners:
        hkArray_hkpContactImpulseLimitBreachedListener___hkContainerHeapAllocator_,
    pub m_worldExtensions: hkArray_hkpWorldExtension___hkContainerHeapAllocator_,
    pub m_violatedConstraintArray: *mut hkpViolatedConstraintArray,
    pub m_broadPhaseBorder: *mut hkpBroadPhaseBorder,
    pub m_destructionWorld: *mut hkdWorld,
    pub m_npWorld: *mut hknpWorld,
    __pdbindgen_padding_2: [u8; 8],
    pub m_dynamicsStepInfo: hkpWorldDynamicsStepInfo,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_broadPhaseExtents: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_3: [u8; 32],
    pub m_broadPhaseNumMarkers: i32,
    pub m_sizeOfToiEventQueue: i32,
    pub m_broadPhaseQuerySize: i32,
    pub m_broadPhaseUpdateSize: i32,
    pub m_contactPointGeneration: hkEnum_enum_hkpWorldCinfo__ContactPointGeneration_signed_char_,
    pub m_useCompoundSpuElf: hkBool,
    __pdbindgen_padding_4: [u8; 14],
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpWorld {}

unsafe impl UpcastToNop<hkBaseObject> for hkpWorld {}

impl hkpWorld {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpWorld__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpWorld, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpWorld),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpWorld) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpWorld),
}

#[repr(C)]
pub struct hkpWorldDynamicsStepInfo {
    pub m_stepInfo: hkStepInfo,
    pub m_solverInfo: hkpSolverInfo,
}

#[repr(C)]
pub struct hkEnum_enum_hkpWorldCinfo__SimulationType_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpRigidBody {
    // hkBaseObject
    pub vfptr: *const hkpRigidBody__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldObject
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
    // hkpEntity
    pub m_material: hkpMaterial,
    pub m_limitContactImpulseUtilAndFlag: *mut (),
    pub m_damageMultiplier: f32,
    pub m_breakableBody: *mut hkpBreakableBody,
    pub m_solverData: u32,
    pub m_storageIndex: u16,
    pub m_contactPointCallbackDelay: u16,
    pub m_constraintsMaster: hkSmallArray_hkConstraintInternal_,
    pub m_constraintsSlave: hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_,
    pub m_constraintRuntime: hkArray_unsigned_char_hkContainerHeapAllocator_,
    pub m_simulationIsland: *mut hkpSimulationIsland,
    pub m_autoRemoveLevel: i8,
    pub m_numShapeKeysInContactPointProperties: u8,
    pub m_responseModifierFlags: u8,
    pub m_uid: u32,
    pub m_spuCollisionCallback: hkpEntity__SpuCollisionCallback,
    __pdbindgen_padding: [u8; 4],
    pub m_motion: hkpMaxSizeMotion,
    pub m_contactListeners: hkSmallArray_hkpContactListener___,
    pub m_actions: hkSmallArray_hkpAction___,
    pub m_localFrame: hkRefPtr_hkLocalFrame_,
    pub m_extendedListeners: *mut hkpEntity__ExtendedListeners,
    pub m_npData: u32,
    // hkpRigidBody
    __pdbindgen_padding_2: [u8; 4],
}

unsafe impl UpcastToNop<hkpEntity> for hkpRigidBody {}

unsafe impl UpcastToNop<hkpWorldObject> for hkpRigidBody {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpRigidBody {}

unsafe impl UpcastToNop<hkBaseObject> for hkpRigidBody {}

impl hkpRigidBody {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setShape(
        &self,
        a1: *const hkpShape,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).setShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShape(
        &self,
        a1: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).updateShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMotionState(&self) -> *mut hkMotionState {
        ((*self.vfptr).getMotionState)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deallocateInternalArrays(&self) {
        ((*self.vfptr).deallocateInternalArrays)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clone(&self) -> *mut hkpRigidBody {
        ((*self.vfptr).clone)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpRigidBody__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpRigidBody, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpRigidBody),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpRigidBody) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpRigidBody),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpRigidBody,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpRigidBody,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState: unsafe extern "thiscall" fn(this: *mut hkpRigidBody) -> *mut hkMotionState,
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpRigidBody),
    pub clone: unsafe extern "thiscall" fn(this: *const hkpRigidBody) -> *mut hkpRigidBody,
}

#[repr(C)]
pub struct hkEnum_enum_hkpAgentNnTrackType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpAgentNnTrack {
    pub m_bytesUsedInLastSector: u16,
    pub m_nnTrackType: hkEnum_enum_hkpAgentNnTrackType_unsigned_char_,
    pub m_padding: u8,
    pub m_sectors: hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkpSimulationIsland {
    // hkBaseObject
    pub vfptr: *const hkpSimulationIsland__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintOwner
    pub m_constraintInfo: hkpConstraintInfo,
    // hkpSimulationIsland
    pub m_world: *mut hkpWorld,
    pub m_numConstraints: i32,
    pub m_storageIndex: u16,
    pub m_dirtyListIndex: u16,
    pub m_splitCheckFrameCounter: u8,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_splitCheckRequested: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_isSparse: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_actionListCleanupNeeded: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_allowIslandLocking: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_isInActiveIslandsArray: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_activeMark: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_tryToIncreaseIslandSizeMark: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_inIntegrateJob: compile_error!("unimplemented type kind"),
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_timeSinceLastHighFrequencyCheck: f32,
    pub m_timeSinceLastLowFrequencyCheck: f32,
    pub m_actions: hkArray_hkpAction___hkContainerHeapAllocator_,
    pub m_timeOfDeactivation: f32,
    pub m_entities: hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_,
    pub m_midphaseAgentTrack: hkpAgentNnTrack,
    pub m_narrowphaseAgentTrack: hkpAgentNnTrack,
}

unsafe impl UpcastToNop<hkpConstraintOwner> for hkpSimulationIsland {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimulationIsland {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimulationIsland {}

impl hkpSimulationIsland {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addConstraintToCriticalLockedIsland(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).addConstraintToCriticalLockedIsland)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeConstraintFromCriticalLockedIsland(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).removeConstraintFromCriticalLockedIsland)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCallbackRequest(
        &self,
        a1: *mut hkpConstraintInstance,
        a2: i32,
    ) {
        ((*self.vfptr).addCallbackRequest)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn checkAccessRw(&self) {
        ((*self.vfptr).checkAccessRw)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpSimulationIsland__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpSimulationIsland) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpSimulationIsland),
    pub addConstraintToCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: *mut hkpConstraintInstance),
    pub removeConstraintFromCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: *mut hkpConstraintInstance),
    pub addCallbackRequest: unsafe extern "thiscall" fn(
        this: *mut hkpSimulationIsland,
        _: *mut hkpConstraintInstance,
        _: i32,
    ),
    pub checkAccessRw: unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland),
}

#[repr(C)]
pub struct hkpCollisionEvent {
    pub m_source: hkpCollisionEvent__CallbackSource,
    pub m_bodies: [*mut hkpRigidBody; 2],
    pub m_contactMgr: *mut hkpSimpleConstraintContactMgr,
}

#[repr(C)]
pub struct hkpContactPointEvent {
    // hkpCollisionEvent
    pub m_source: hkpCollisionEvent__CallbackSource,
    pub m_bodies: [*mut hkpRigidBody; 2],
    pub m_contactMgr: *mut hkpSimpleConstraintContactMgr,
    // hkpContactPointEvent
    pub m_type: hkpContactPointEvent__Type,
    pub m_contactPoint: *mut hkContactPoint,
    pub m_contactPointProperties: *mut hkpContactPointProperties,
    pub m_firingCallbacksForFullManifold: hkBool,
    pub m_firstCallbackForFullManifold: hkBool,
    pub m_lastCallbackForFullManifold: hkBool,
    pub m_separatingVelocity: *mut f32,
    pub m_rotateNormal: *mut f32,
    pub m_shapeKeyStorage: *mut u32,
    pub m_accumulators: [*mut hkpVelocityAccumulator; 2],
}

unsafe impl UpcastToNop<hkpCollisionEvent> for hkpContactPointEvent {}

#[repr(C)]
pub struct hkpDynamicsContactMgr {
    // hkBaseObject
    pub vfptr: *const hkpDynamicsContactMgr__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
    // hkpDynamicsContactMgr
    pub m_world: *mut hkpWorld,
}

unsafe impl UpcastToNop<hkpContactMgr> for hkpDynamicsContactMgr {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpDynamicsContactMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpDynamicsContactMgr {}

impl hkpDynamicsContactMgr {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addContactPointImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: *const hkpGskCache,
        a6: *mut hkContactPoint,
    ) -> u16 {
        ((*self.vfptr).addContactPointImpl)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn reserveContactPointsImpl(
        &self,
        result: *mut hkResult,
        a2: i32,
    ) -> *mut hkResult {
        ((*self.vfptr).reserveContactPointsImpl)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn removeContactPointImpl(
        &self,
        a1: u16,
        a2: *mut hkpConstraintOwner,
    ) {
        ((*self.vfptr).removeContactPointImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processContactImpl(
        &self,
        a1: *const hkpCollidable,
        a2: *const hkpCollidable,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionData,
    ) {
        ((*self.vfptr).processContactImpl)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn addToiImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: f32,
        a6: *mut hkContactPoint,
        a7: *const hkpGskCache,
        a8: *mut f32,
        a9: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept {
        ((*self.vfptr).addToiImpl)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
        )
    }

    pub unsafe extern "thiscall" fn removeToiImpl(
        &self,
        a1: *mut hkpConstraintOwner,
        a2: *mut hkpContactPointProperties,
    ) {
        ((*self.vfptr).removeToiImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn cleanup(&self) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getContactPointProperties(
        &self,
        a1: u16,
    ) -> *mut hkpContactPointProperties {
        ((*self.vfptr).getContactPointProperties)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContactPoint(&self, a1: u16) -> *mut hkContactPoint {
        ((*self.vfptr).getContactPoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAllContactPointIds(
        &self,
        a1: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getAllContactPointIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpContactMgr__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseBeginCallback(
        &self,
        a1: *const hkContactPoint,
        a2: *mut hkpSimpleConstraintInfoInitInput,
        a3: *mut hkpBodyVelocity,
        a4: *mut hkpSimpleConstraintInfoInitInput,
        a5: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseBeginCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseEndCallback(
        &self,
        a1: *const hkContactPoint,
        a2: f32,
        a3: *mut hkpSimpleConstraintInfoInitInput,
        a4: *mut hkpBodyVelocity,
        a5: *mut hkpSimpleConstraintInfoInitInput,
        a6: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseEndCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
        )
    }

    pub unsafe extern "thiscall" fn getConstraintInstance(&self) -> *mut hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn fireCallbacksForEarliestToi(
        &self,
        result: *mut hkBool,
        a2: *mut hkpToiEvent,
        a3: *mut f32,
    ) -> *mut hkBool {
        ((*self.vfptr).fireCallbacksForEarliestToi)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn confirmToi(
        &self,
        a1: *mut hkpToiEvent,
        a2: f32,
        a3: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).confirmToi)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpDynamicsContactMgr__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpDynamicsContactMgr) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpDynamicsContactMgr),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        result: *mut hkResult,
        _: i32,
    ) -> *mut hkResult,
    pub removeContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: u16,
        _: *mut hkpConstraintOwner,
    ),
    pub processContactImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionData,
    ),
    pub addToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: f32,
        _: *mut hkContactPoint,
        _: *const hkpGskCache,
        _: *mut f32,
        _: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept,
    pub removeToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *mut hkpConstraintOwner,
        _: *mut hkpContactPointProperties,
    ),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr),
    pub getContactPointProperties: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: u16,
    )
        -> *mut hkpContactPointProperties,
    pub getContactPoint: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: u16,
    ) -> *mut hkContactPoint,
    pub getAllContactPointIds: unsafe extern "thiscall" fn(
        this: *const hkpDynamicsContactMgr,
        _: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ),
    pub getType:
        unsafe extern "thiscall" fn(this: *const hkpDynamicsContactMgr) -> hkpContactMgr__Type,
    pub toiCollisionResponseBeginCallback: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkContactPoint,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub toiCollisionResponseEndCallback: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkContactPoint,
        _: f32,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub getConstraintInstance:
        unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr) -> *mut hkpConstraintInstance,
    pub fireCallbacksForEarliestToi: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        result: *mut hkBool,
        _: *mut hkpToiEvent,
        _: *mut f32,
    ) -> *mut hkBool,
    pub confirmToi: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *mut hkpToiEvent,
        _: f32,
        _: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ),
}

#[repr(C)]
pub struct hkpModifierConstraintAtom {
    // hkpConstraintAtom
    pub m_type: hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_,
    // hkpModifierConstraintAtom
    __pdbindgen_padding: [u8; 14],
    pub m_modifierAtomSize: u16,
    pub m_childSize: u16,
    pub m_child: *mut hkpConstraintAtom,
    pub m_pad: [u32; 2],
}

unsafe impl UpcastToNop<hkpConstraintAtom> for hkpModifierConstraintAtom {}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr {
    // hkBaseObject
    pub vfptr: *const hkpSimpleConstraintContactMgr__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
    // hkpDynamicsContactMgr
    pub m_world: *mut hkpWorld,
    // hkpSimpleConstraintContactMgr
    pub m_reservedContactPoints: u16,
    pub m_contactPointCallbackDelay: u16,
    pub m_contactConstraintData: hkpSimpleContactConstraintData,
    pub m_constraint: hkpConstraintInstance,
    pub m_pad: [u32; 1],
}

unsafe impl UpcastToNop<hkpDynamicsContactMgr> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkpContactMgr> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimpleConstraintContactMgr {}

impl hkpSimpleConstraintContactMgr {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addContactPointImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: *const hkpGskCache,
        a6: *mut hkContactPoint,
    ) -> u16 {
        ((*self.vfptr).addContactPointImpl)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn reserveContactPointsImpl(
        &self,
        result: *mut hkResult,
        a2: i32,
    ) -> *mut hkResult {
        ((*self.vfptr).reserveContactPointsImpl)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn removeContactPointImpl(
        &self,
        a1: u16,
        a2: *mut hkpConstraintOwner,
    ) {
        ((*self.vfptr).removeContactPointImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processContactImpl(
        &self,
        a1: *const hkpCollidable,
        a2: *const hkpCollidable,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionData,
    ) {
        ((*self.vfptr).processContactImpl)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn addToiImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: f32,
        a6: *mut hkContactPoint,
        a7: *const hkpGskCache,
        a8: *mut f32,
        a9: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept {
        ((*self.vfptr).addToiImpl)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
        )
    }

    pub unsafe extern "thiscall" fn removeToiImpl(
        &self,
        a1: *mut hkpConstraintOwner,
        a2: *mut hkpContactPointProperties,
    ) {
        ((*self.vfptr).removeToiImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn cleanup(&self) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getContactPointProperties(
        &self,
        a1: u16,
    ) -> *mut hkpContactPointProperties {
        ((*self.vfptr).getContactPointProperties)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContactPoint(&self, a1: u16) -> *mut hkContactPoint {
        ((*self.vfptr).getContactPoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAllContactPointIds(
        &self,
        a1: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getAllContactPointIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpContactMgr__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseBeginCallback(
        &self,
        a1: *const hkContactPoint,
        a2: *mut hkpSimpleConstraintInfoInitInput,
        a3: *mut hkpBodyVelocity,
        a4: *mut hkpSimpleConstraintInfoInitInput,
        a5: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseBeginCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseEndCallback(
        &self,
        a1: *const hkContactPoint,
        a2: f32,
        a3: *mut hkpSimpleConstraintInfoInitInput,
        a4: *mut hkpBodyVelocity,
        a5: *mut hkpSimpleConstraintInfoInitInput,
        a6: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseEndCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
        )
    }

    pub unsafe extern "thiscall" fn getConstraintInstance(&self) -> *mut hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn fireCallbacksForEarliestToi(
        &self,
        result: *mut hkBool,
        a2: *mut hkpToiEvent,
        a3: *mut f32,
    ) -> *mut hkBool {
        ((*self.vfptr).fireCallbacksForEarliestToi)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn confirmToi(
        &self,
        a1: *mut hkpToiEvent,
        a2: f32,
        a3: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).confirmToi)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getConstraintInstance_2(&self) -> *const hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance_2)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpSimpleConstraintContactMgr) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpSimpleConstraintContactMgr),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        result: *mut hkResult,
        _: i32,
    ) -> *mut hkResult,
    pub removeContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
        _: *mut hkpConstraintOwner,
    ),
    pub processContactImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionData,
    ),
    pub addToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: f32,
        _: *mut hkContactPoint,
        _: *const hkpGskCache,
        _: *mut f32,
        _: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept,
    pub removeToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *mut hkpConstraintOwner,
        _: *mut hkpContactPointProperties,
    ),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr),
    pub getContactPointProperties: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
    )
        -> *mut hkpContactPointProperties,
    pub getContactPoint: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
    ) -> *mut hkContactPoint,
    pub getAllContactPointIds: unsafe extern "thiscall" fn(
        this: *const hkpSimpleConstraintContactMgr,
        _: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ),
    pub getType: unsafe extern "thiscall" fn(
        this: *const hkpSimpleConstraintContactMgr,
    ) -> hkpContactMgr__Type,
    pub toiCollisionResponseBeginCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkContactPoint,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub toiCollisionResponseEndCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkContactPoint,
        _: f32,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub getConstraintInstance: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
    ) -> *mut hkpConstraintInstance,
    pub fireCallbacksForEarliestToi: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        result: *mut hkBool,
        _: *mut hkpToiEvent,
        _: *mut f32,
    ) -> *mut hkBool,
    pub confirmToi: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *mut hkpToiEvent,
        _: f32,
        _: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ),
    pub getConstraintInstance_2: unsafe extern "thiscall" fn(
        this: *const hkpSimpleConstraintContactMgr,
    ) -> *const hkpConstraintInstance,
}

#[repr(C)]
pub struct hkpContactPointAddedEvent {
    pub m_bodyA: *const hkpCdBody,
    pub m_bodyB: *const hkpCdBody,
    pub m_type: hkEnum_enum_hkpContactPointAddedEvent__Type_int_,
    pub m_callbackFiredFrom: *mut hkpEntity,
    pub m_contactPoint: *const hkContactPoint,
    pub m_gskCache: *const hkpGskCache,
    pub m_contactPointProperties: *mut hkpContactPointProperties,
    pub m_projectedVelocity: f32,
    pub m_status: hkpContactPointAccept,
    pub m_internalContactMgr: *mut hkpDynamicsContactMgr,
    pub m_collisionInput: *const hkpProcessCollisionInput,
    pub m_collisionOutput: *mut hkpProcessCollisionOutput,
}

#[repr(C)]
pub struct hkpContactPointConfirmedEvent {
    pub m_collidableA: *const hkpCollidable,
    pub m_collidableB: *const hkpCollidable,
    pub m_callbackFiredFrom: *mut hkpEntity,
    pub m_contactPoint: *mut hkContactPoint,
    pub m_contactPointProperties: *mut hkpContactPointProperties,
    pub m_rotateNormal: f32,
    pub m_projectedVelocity: f32,
    pub m_type: hkEnum_enum_hkpContactPointAddedEvent__Type_int_,
    pub m_contactData: *const hkpSimpleContactConstraintData,
}

#[repr(C)]
pub struct hkEnum_enum_hkpContactPointAddedEvent__Type_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpCollisionListener {
    // hkpContactListener
    pub vfptr: *const hkpCollisionListener__vftable,
    // hkpCollisionListener
}

unsafe impl UpcastToNop<hkpContactListener> for hkpCollisionListener {}

impl hkpCollisionListener {
    pub unsafe extern "thiscall" fn contactPointCallback(&self, a1: *const hkpContactPointEvent) {
        ((*self.vfptr).contactPointCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collisionAddedCallback(&self, a1: *const hkpCollisionEvent) {
        ((*self.vfptr).collisionAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collisionRemovedCallback(&self, a1: *const hkpCollisionEvent) {
        ((*self.vfptr).collisionRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointAddedCallback(
        &self,
        a1: *mut hkpContactPointAddedEvent,
    ) {
        ((*self.vfptr).contactPointAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointRemovedCallback(
        &self,
        a1: *mut hkpContactPointRemovedEvent,
    ) {
        ((*self.vfptr).contactPointRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactProcessCallback(&self, a1: *mut hkpContactProcessEvent) {
        ((*self.vfptr).contactProcessCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointConfirmedCallback(
        &self,
        a1: *mut hkpContactPointConfirmedEvent,
    ) {
        ((*self.vfptr).contactPointConfirmedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpCollisionListener__vftable {
    pub contactPointCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *const hkpContactPointEvent,
    ),
    pub collisionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: *const hkpCollisionEvent),
    pub collisionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: *const hkpCollisionEvent),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: u32) -> *mut (),
    pub contactPointAddedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointAddedEvent,
    ),
    pub contactPointRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointRemovedEvent,
    ),
    pub contactProcessCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactProcessEvent,
    ),
    pub contactPointConfirmedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointConfirmedEvent,
    ),
}

#[repr(C)]
pub struct hkpCollisionAgentConfig {
    pub m_iterativeLinearCastEarlyOutDistance: f32,
    pub m_iterativeLinearCastMaxIterations: i32,
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct hkpLinearCastCollisionInput {
    // hkpCollisionInput
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    __pdbindgen_padding: [u8; 4],
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
    // hkpLinearCastCollisionInput
    pub m_path: hkVector4f,
    pub m_maxExtraPenetration: f32,
    pub m_cachedPathLength: f32,
    pub m_config: *mut hkpCollisionAgentConfig,
    __pdbindgen_padding_2: [u8; 4],
}

unsafe impl UpcastToNop<hkpCollisionInput> for hkpLinearCastCollisionInput {}

#[repr(C)]
pub struct hkpShapeRayCastOutput {
    // hkpShapeRayCastCollectorOutput
    pub m_normal: hkVector4f,
    pub m_hitFraction: f32,
    pub m_extraInfo: i32,
    pub m_pad: [i32; 2],
    // hkpShapeRayCastOutput
    pub m_shapeKeys: [u32; 8],
    pub m_shapeKeyIndex: i32,
    __pdbindgen_padding: [u8; 12],
}

unsafe impl UpcastToNop<hkpShapeRayCastCollectorOutput> for hkpShapeRayCastOutput {}

#[repr(C)]
pub struct hkpCollisionInput {
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    __pdbindgen_padding: [u8; 4],
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
}

#[repr(C)]
pub struct hkThreadPool {
    // hkBaseObject
    pub vfptr: *const hkThreadPool__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkThreadPool
}

unsafe impl UpcastToNop<hkReferencedObject> for hkThreadPool {}

unsafe impl UpcastToNop<hkBaseObject> for hkThreadPool {}

impl hkThreadPool {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn processJobQueue(&self, a1: *mut hkJobQueue, a2: hkJobType) {
        ((*self.vfptr).processJobQueue)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processTaskQueue(&self, a1: *mut hkDefaultTaskQueue) {
        ((*self.vfptr).processTaskQueue)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isProcessing(&self) -> bool {
        ((*self.vfptr).isProcessing)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn waitForCompletion(&self) {
        ((*self.vfptr).waitForCompletion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNumThreads(&self) -> i32 {
        ((*self.vfptr).getNumThreads)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setNumThreads(&self, a1: i32) {
        ((*self.vfptr).setNumThreads)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn appendTimerData(
        &self,
        a1: *mut hkArrayBase_hkTimerData_,
        a2: *mut hkMemoryAllocator,
    ) {
        ((*self.vfptr).appendTimerData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn clearTimerData(&self) {
        ((*self.vfptr).clearTimerData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn gcThreadMemoryOnNextCompletion(&self) {
        ((*self.vfptr).gcThreadMemoryOnNextCompletion)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkThreadPool__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkThreadPool, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkThreadPool),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkThreadPool) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkThreadPool),
    pub processJobQueue:
        unsafe extern "thiscall" fn(this: *mut hkThreadPool, _: *mut hkJobQueue, _: hkJobType),
    pub processTaskQueue:
        unsafe extern "thiscall" fn(this: *mut hkThreadPool, _: *mut hkDefaultTaskQueue),
    pub isProcessing: unsafe extern "thiscall" fn(this: *const hkThreadPool) -> bool,
    pub waitForCompletion: unsafe extern "thiscall" fn(this: *mut hkThreadPool),
    pub getNumThreads: unsafe extern "thiscall" fn(this: *const hkThreadPool) -> i32,
    pub setNumThreads: unsafe extern "thiscall" fn(this: *mut hkThreadPool, _: i32),
    pub appendTimerData: unsafe extern "thiscall" fn(
        this: *mut hkThreadPool,
        _: *mut hkArrayBase_hkTimerData_,
        _: *mut hkMemoryAllocator,
    ),
    pub clearTimerData: unsafe extern "thiscall" fn(this: *mut hkThreadPool),
    pub gcThreadMemoryOnNextCompletion: unsafe extern "thiscall" fn(this: *mut hkThreadPool),
}

#[repr(C)]
pub struct hkpProcessCollisionOutput {
    // hkpProcessCollisionData
    pub m_firstFreeContactPoint: hkPadSpu_hkpProcessCdPoint___,
    pub m_constraintOwner: hkPadSpu_hkpConstraintOwner___,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_contactPoints: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 12296],
    pub m_toi: hkpProcessCollisionData__ToiInfo,
    // hkpProcessCollisionOutput
    pub m_potentialContacts: hkPadSpu_hkpProcessCollisionOutput__PotentialInfo___,
    __pdbindgen_padding_2: [u8; 12],
}

unsafe impl UpcastToNop<hkpProcessCollisionData> for hkpProcessCollisionOutput {}

#[repr(C)]
pub struct hkpGskCache {
    pub m_vertices: [u16; 4],
    pub m_dimA: u8,
    pub m_dimB: u8,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_maxDimA: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_maxDimB: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 1],
    pub m_gskFlags: u8,
}

#[repr(C)]
pub struct hkpWorldRayCastInput {
    pub m_from: hkVector4f,
    pub m_to: hkVector4f,
    pub m_enableShapeCollectionFilter: hkBool,
    pub m_filterInfo: u32,
    pub m_userData: u32,
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct hkpShapeRayCastCollectorOutput {
    pub m_normal: hkVector4f,
    pub m_hitFraction: f32,
    pub m_extraInfo: i32,
    pub m_pad: [i32; 2],
}

#[repr(C)]
pub struct hkpCollidable {
    // hkpCdBody
    pub m_shape: *const hkpShape,
    pub m_shapeKey: u32,
    pub m_motion: *const (),
    pub m_parent: *const hkpCdBody,
    // hkpCollidable
    pub m_ownerOffset: i8,
    pub m_forceCollideOntoPpu: u8,
    pub m_shapeSizeOnSpu: u16,
    pub m_broadPhaseHandle: hkpTypedBroadPhaseHandle,
    pub m_boundingVolumeData: hkpCollidable__BoundingVolumeData,
    pub m_allowedPenetrationDepth: f32,
}

unsafe impl UpcastToNop<hkpCdBody> for hkpCollidable {}

#[repr(C)]
pub struct hkpCollisionDispatcher__ShapeInheritance {
    pub m_primaryType: hkcdShapeType__ShapeTypeEnum,
    pub m_alternateType: hkcdShapeType__ShapeTypeEnum,
}

#[repr(C)]
pub struct hkpBreakableMaterial {
    // hkBaseObject
    pub vfptr: *const hkpBreakableMaterial__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableMaterial
    pub m_strength: f32,
    pub m_typeAndFlags: i32,
    pub m_properties: *mut hkRefCountedProperties,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBreakableMaterial {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBreakableMaterial {}

impl hkpBreakableMaterial {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn createInverseMapping(&self, a1: *const hkcdShape) {
        ((*self.vfptr).createInverseMapping)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn duplicate(&self) -> *mut hkpBreakableMaterial {
        ((*self.vfptr).duplicate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDefaultMapping(&self) {
        ((*self.vfptr).setDefaultMapping)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSubShapeMaterialIndex(
        &self,
        a1: *const hkcdShape,
        a2: u32,
    ) -> i16 {
        ((*self.vfptr).getSubShapeMaterialIndex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getShapeKeyMaterial(
        &self,
        a1: *const hkcdShape,
        a2: u32,
    ) -> *mut hkpBreakableMaterial {
        ((*self.vfptr).getShapeKeyMaterial)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertShapeKeyToSubShapeId(&self, a1: u32) -> u32 {
        ((*self.vfptr).convertShapeKeyToSubShapeId)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn convertShapeKeysToSubShapeIds(
        &self,
        a1: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).convertShapeKeysToSubShapeIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn disableSubShapes(
        &self,
        a1: *mut hkcdShape,
        a2: *const i16,
        a3: i32,
    ) {
        ((*self.vfptr).disableSubShapes)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getNumSubMaterials(&self) -> i32 {
        ((*self.vfptr).getNumSubMaterials)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getShapeKeysForSubShapes(
        &self,
        a1: *const hkcdShape,
        a2: *const u32,
        a3: i32,
        a4: *mut hkpBreakableMaterial__ShapeKeyCollector,
    ) {
        ((*self.vfptr).getShapeKeysForSubShapes)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSubShapeMaterialIndices(
        &self,
        a1: *const hkcdShape,
        a2: *const hkArray_unsigned_int_hkContainerHeapAllocator_,
        a3: *mut hkArray_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getSubShapeMaterialIndices)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpBreakableMaterial__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial),
    pub createInverseMapping:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial, _: *const hkcdShape),
    pub duplicate:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial) -> *mut hkpBreakableMaterial,
    pub setDefaultMapping: unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial),
    pub getSubShapeMaterialIndex: unsafe extern "thiscall" fn(
        this: *const hkpBreakableMaterial,
        _: *const hkcdShape,
        _: u32,
    ) -> i16,
    pub getShapeKeyMaterial: unsafe extern "thiscall" fn(
        this: *const hkpBreakableMaterial,
        _: *const hkcdShape,
        _: u32,
    ) -> *mut hkpBreakableMaterial,
    pub convertShapeKeyToSubShapeId:
        unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial, _: u32) -> u32,
    pub convertShapeKeysToSubShapeIds: unsafe extern "thiscall" fn(
        this: *const hkpBreakableMaterial,
        _: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ),
    pub disableSubShapes: unsafe extern "thiscall" fn(
        this: *mut hkpBreakableMaterial,
        _: *mut hkcdShape,
        _: *const i16,
        _: i32,
    ),
    pub getNumSubMaterials: unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial) -> i32,
    pub getShapeKeysForSubShapes: unsafe extern "thiscall" fn(
        this: *const hkpBreakableMaterial,
        _: *const hkcdShape,
        _: *const u32,
        _: i32,
        _: *mut hkpBreakableMaterial__ShapeKeyCollector,
    ),
    pub getSubShapeMaterialIndices: unsafe extern "thiscall" fn(
        this: *const hkpBreakableMaterial,
        _: *const hkcdShape,
        _: *const hkArray_unsigned_int_hkContainerHeapAllocator_,
        _: *mut hkArray_short_hkContainerHeapAllocator_,
    ),
}

#[repr(C)]
pub struct hkpBreakableMaterial__ShapeKeyCollector {
    pub vfptr: *const hkpBreakableMaterial__ShapeKeyCollector__vftable,
}

impl hkpBreakableMaterial__ShapeKeyCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addShapeKey(&self, a1: u32) {
        ((*self.vfptr).addShapeKey)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addShapeKeyBatch(&self, a1: *const u32, a2: i32) {
        ((*self.vfptr).addShapeKeyBatch)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn addContiguousShapeKeyRange(&self, a1: u32, a2: i32) {
        ((*self.vfptr).addContiguousShapeKeyRange)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpBreakableMaterial__ShapeKeyCollector__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpBreakableMaterial__ShapeKeyCollector,
        _: u32,
    ) -> *mut (),
    pub addShapeKey:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial__ShapeKeyCollector, _: u32),
    pub addShapeKeyBatch: unsafe extern "thiscall" fn(
        this: *mut hkpBreakableMaterial__ShapeKeyCollector,
        _: *const u32,
        _: i32,
    ),
    pub addContiguousShapeKeyRange: unsafe extern "thiscall" fn(
        this: *mut hkpBreakableMaterial__ShapeKeyCollector,
        _: u32,
        _: i32,
    ),
}

#[repr(C)]
pub struct hkRefPtr_hkpBreakableMaterial_ {
    pub m_pntr: *mut hkpBreakableMaterial,
}

#[repr(C)]
pub struct hkpBreakableShape {
    // hkBaseObject
    pub vfptr: *const hkpBreakableShape__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableShape
    pub m_physicsShape: hkRefPtr_hkcdShape_const__,
    pub m_material: hkRefPtr_hkpBreakableMaterial_,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBreakableShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBreakableShape {}

impl hkpBreakableShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpBreakableShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpBreakableShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpBreakableShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpBreakableShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBreakableShape),
}

#[repr(C)]
pub struct hkRefPtr_hkcdShape_const__ {
    pub m_pntr: *const hkcdShape,
}

#[repr(C)]
pub struct hkpBreakableBody {
    // hkBaseObject
    pub vfptr: *const hkpBreakableBody__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableBody
    pub m_controller: hkRefPtr_hkpBreakableBody__Controller_,
    pub m_breakableShape: hkRefPtr_hkpBreakableShape_const__,
    pub m_bodyTypeAndFlags: u8,
    pub m_constraintStrength: hkHalf,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBreakableBody {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBreakableBody {}

impl hkpBreakableBody {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn cloneBreakableBody(
        &self,
        a1: *mut hkpRigidBody,
    ) -> *mut hkpBreakableBody {
        ((*self.vfptr).cloneBreakableBody)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpBreakableBody__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpBreakableBody, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpBreakableBody),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpBreakableBody) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBreakableBody),
    pub cloneBreakableBody: unsafe extern "thiscall" fn(
        this: *const hkpBreakableBody,
        _: *mut hkpRigidBody,
    ) -> *mut hkpBreakableBody,
}

#[repr(C)]
pub struct hkpBreakableBody__Controller {
    // hkBaseObject
    pub vfptr: *const hkpBreakableBody__Controller__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableBody__Controller
    pub m_breakingImpulse: f32,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBreakableBody__Controller {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBreakableBody__Controller {}

impl hkpBreakableBody__Controller {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpBreakableBody__Controller__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableBody__Controller, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableBody__Controller),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpBreakableBody__Controller) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpBreakableBody__Controller),
}

#[repr(C)]
pub struct hkRefPtr_hkpBreakableBody__Controller_ {
    pub m_pntr: *mut hkpBreakableBody__Controller,
}

#[repr(C)]
pub struct hkRefPtr_hkpBreakableShape_const__ {
    pub m_pntr: *const hkpBreakableShape,
}

#[repr(C)]
pub struct hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpBodyOperationEntry_
    pub m_data: *mut hkpBodyOperationEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpBodyOperationEntry_>
    for hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpBodyOperation__UpdateInfo {
    pub m_bodyIsDeleted: hkBool,
    pub m_bodyIsInWorld: hkBool,
}

#[repr(C)]
pub struct hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_ {
    // hkArrayBase_hkWorldOperation__BiggestOperation_
    pub m_data: *mut hkWorldOperation__BiggestOperation,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkWorldOperation__BiggestOperation_>
    for hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkWorldOperation__BiggestOperation {
    // hkWorldOperation__BaseOperation
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
    // hkWorldOperation__BiggestOperation
    pub dummy: [u32; 7],
}

unsafe impl UpcastToNop<hkWorldOperation__BaseOperation> for hkWorldOperation__BiggestOperation {}

#[repr(C)]
pub struct hkWorldOperation__BaseOperation {
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
}

#[repr(C)]
pub struct hkpBodyOperation {
    // hkBaseObject
    pub vfptr: *const hkpBodyOperation__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBodyOperation
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBodyOperation {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBodyOperation {}

impl hkpBodyOperation {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn execute(
        &self,
        a1: *mut hkpRigidBody,
        a2: *mut hkpBodyOperation__UpdateInfo,
    ) {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpBodyOperation__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpBodyOperation, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpBodyOperation),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpBodyOperation) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBodyOperation),
    pub execute: unsafe extern "thiscall" fn(
        this: *mut hkpBodyOperation,
        _: *mut hkpRigidBody,
        _: *mut hkpBodyOperation__UpdateInfo,
    ),
}

#[repr(C)]
pub struct hkpSimulation {
    // hkBaseObject
    pub vfptr: *const hkpSimulation__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpSimulation
    pub m_determinismCheckFrameCounter: u32,
    pub m_world: *mut hkpWorld,
    pub m_lastProcessingStep: hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_,
    pub m_currentTime: f32,
    pub m_currentPsiTime: f32,
    pub m_physicsDeltaTime: f32,
    pub m_simulateUntilTime: f32,
    pub m_frameMarkerPsiSnap: f32,
    pub m_previousStepResult: u32,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimulation {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimulation {}

impl hkpSimulation {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepDeltaTime(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).stepDeltaTime)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn integrate(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).integrate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collide(&self) -> hkpStepResult {
        ((*self.vfptr).collide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn advanceTime(&self) -> hkpStepResult {
        ((*self.vfptr).advanceTime)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepBeginSt(
        &self,
        a1: *mut hkJobQueue,
        a2: f32,
    ) -> hkpStepResult {
        ((*self.vfptr).stepBeginSt)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn finishMtStep(
        &self,
        a1: *mut hkJobQueue,
        a2: *mut hkThreadPool,
    ) -> hkpStepResult {
        ((*self.vfptr).finishMtStep)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMultithreadConfig(&self, a1: *mut hkpMultithreadConfig) {
        ((*self.vfptr).getMultithreadConfig)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMultithreadConfig(
        &self,
        a1: *const hkpMultithreadConfig,
        a2: *mut hkJobQueue,
    ) {
        ((*self.vfptr).setMultithreadConfig)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn collideEntitiesDiscrete(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: *const hkStepInfo,
        a5: hkpSimulation__FindContacts,
    ) {
        ((*self.vfptr).collideEntitiesDiscrete)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn resetCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: hkpSimulation__ResetCollisionInformation,
    ) {
        ((*self.vfptr).resetCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
        )
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
        )
    }

    pub unsafe extern "thiscall" fn removeCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).removeCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reintegrateAndRecollideEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: i32,
    ) {
        ((*self.vfptr).reintegrateAndRecollideEntities)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn collideInternal(&self, a1: *const hkStepInfo) {
        ((*self.vfptr).collideInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn warpTime(&self, a1: f32) {
        ((*self.vfptr).warpTime)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpSimulation__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpSimulation),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpSimulation) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpSimulation),
    pub stepDeltaTime:
        unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: f32) -> hkpStepResult,
    pub integrate: unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: f32) -> hkpStepResult,
    pub collide: unsafe extern "thiscall" fn(this: *mut hkpSimulation) -> hkpStepResult,
    pub advanceTime: unsafe extern "thiscall" fn(this: *mut hkpSimulation) -> hkpStepResult,
    pub stepBeginSt: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut hkJobQueue,
        _: f32,
    ) -> hkpStepResult,
    pub finishMtStep: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut hkJobQueue,
        _: *mut hkThreadPool,
    ) -> hkpStepResult,
    pub getMultithreadConfig:
        unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: *mut hkpMultithreadConfig),
    pub setMultithreadConfig: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *const hkpMultithreadConfig,
        _: *mut hkJobQueue,
    ),
    pub collideEntitiesDiscrete: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: *const hkStepInfo,
        _: hkpSimulation__FindContacts,
    ),
    pub resetCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: hkpSimulation__ResetCollisionInformation,
    ),
    pub assertThereIsNoCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
    ),
    pub removeCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: *mut hkpAgentNnEntry),
    pub assertThereIsNoCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: *mut hkpAgentNnEntry),
    pub reintegrateAndRecollideEntities: unsafe extern "thiscall" fn(
        this: *mut hkpSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: i32,
    ),
    pub collideInternal:
        unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: *const hkStepInfo),
    pub warpTime: unsafe extern "thiscall" fn(this: *mut hkpSimulation, _: f32),
}

#[repr(C)]
pub struct hkpWorldOperationQueue {
    pub m_pending: hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_,
    pub m_world: *mut hkpWorld,
    pub m_islandMerges: hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_,
    pub m_pendingBodyOperations: hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkpDebugInfoOnPendingOperationQueues {
    pub m_pending: *mut hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_,
    pub m_nextPendingOperationIndex: i32,
    pub m_nextQueue: *mut hkpDebugInfoOnPendingOperationQueues,
    pub m_prevQueue: *mut hkpDebugInfoOnPendingOperationQueues,
}

#[repr(C)]
pub struct hkEnum_enum_hkWorldOperation__Type_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpBodyOperationEntry {
    pub m_entity: *mut hkpEntity,
    pub m_operation: *mut hkpBodyOperation,
    pub m_priority: i32,
    pub m_hint: hkpBodyOperation__ExecutionState,
}

#[repr(C)]
pub struct hkArrayBase_hkWorldOperation__BiggestOperation_ {
    pub m_data: *mut hkWorldOperation__BiggestOperation,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpBodyOperationEntry_ {
    pub m_data: *mut hkpBodyOperationEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpConstraintTrackerData {
    // hkBaseObject
    pub vfptr: *const hkpConstraintTrackerData__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintTrackerData
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConstraintTrackerData {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConstraintTrackerData {}

impl hkpConstraintTrackerData {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn print(&self, a1: *mut hkStringBuf) {
        ((*self.vfptr).print)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpConstraintTrackerData__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintTrackerData, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintTrackerData),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConstraintTrackerData) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpConstraintTrackerData),
    pub print:
        unsafe extern "thiscall" fn(this: *const hkpConstraintTrackerData, _: *mut hkStringBuf),
}

#[repr(C)]
pub struct hkpConstraintBrokenEvent {
    pub m_world: *mut hkpWorld,
    pub m_constraintInstance: *mut hkpConstraintInstance,
    pub m_eventSource: *const hkClass,
    pub m_eventSourceDetails: *mut hkpConstraintTrackerData,
    pub m_actualImpulse: f32,
    pub m_impulseLimit: f32,
}

#[repr(C)]
pub struct hkpConstraintRepairedEvent {
    pub m_world: *mut hkpWorld,
    pub m_constraintInstance: *mut hkpConstraintInstance,
    pub m_eventSource: *const hkClass,
    pub m_eventSourceDetails: *mut hkpConstraintTrackerData,
}

#[repr(C)]
pub struct hkpConstraintListener {
    pub vfptr: *const hkpConstraintListener__vftable,
}

impl hkpConstraintListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintAddedCallback(&self, a1: *mut hkpConstraintInstance) {
        ((*self.vfptr).constraintAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintRemovedCallback(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).constraintRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintDeletedCallback(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).constraintDeletedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintViolatedCallback(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).constraintViolatedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintBreakingCallback(
        &self,
        a1: *const hkpConstraintBrokenEvent,
    ) {
        ((*self.vfptr).constraintBreakingCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn constraintRepairedCallback(
        &self,
        a1: *const hkpConstraintRepairedEvent,
    ) {
        ((*self.vfptr).constraintRepairedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpConstraintListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintListener, _: u32) -> *mut (),
    pub constraintAddedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *mut hkpConstraintInstance,
    ),
    pub constraintRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *mut hkpConstraintInstance,
    ),
    pub constraintDeletedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *mut hkpConstraintInstance,
    ),
    pub constraintViolatedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *mut hkpConstraintInstance,
    ),
    pub constraintBreakingCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *const hkpConstraintBrokenEvent,
    ),
    pub constraintRepairedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintListener,
        _: *const hkpConstraintRepairedEvent,
    ),
}

#[repr(C)]
pub struct hkContactPointMaterial {
    pub m_userData: u32,
    pub m_friction: hkUFloat8,
    pub m_restitution: u8,
    pub m_maxImpulse: hkUFloat8,
    pub m_flags: u8,
}

#[repr(C)]
pub struct hkpShapePhantom {
    // hkBaseObject
    pub vfptr: *const hkpShapePhantom__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldObject
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
    // hkpPhantom
    pub m_overlapListeners: hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_,
    pub m_phantomListeners: hkArray_hkpPhantomListener___hkContainerHeapAllocator_,
    // hkpShapePhantom
    pub m_motionState: hkMotionState,
}

unsafe impl UpcastToNop<hkpPhantom> for hkpShapePhantom {}

unsafe impl UpcastToNop<hkpWorldObject> for hkpShapePhantom {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpShapePhantom {}

unsafe impl UpcastToNop<hkBaseObject> for hkpShapePhantom {}

impl hkpShapePhantom {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setShape(
        &self,
        a1: *const hkpShape,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).setShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShape(
        &self,
        a1: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).updateShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMotionState(&self) -> *mut hkMotionState {
        ((*self.vfptr).getMotionState)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpPhantomType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn calcAabb(&self, a1: *mut hkAabb) {
        ((*self.vfptr).calcAabb)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addOverlappingCollidable(&self, a1: *mut hkpCollidable) {
        ((*self.vfptr).addOverlappingCollidable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isOverlappingCollidableAdded(
        &self,
        result: *mut hkBool,
        a2: *const hkpCollidable,
    ) -> *mut hkBool {
        ((*self.vfptr).isOverlappingCollidableAdded)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn removeOverlappingCollidable(&self, a1: *mut hkpCollidable) {
        ((*self.vfptr).removeOverlappingCollidable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn ensureDeterministicOrder(&self) {
        ((*self.vfptr).ensureDeterministicOrder)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clone(&self) -> *mut hkpPhantom {
        ((*self.vfptr).clone)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateShapeCollectionFilter(&self) {
        ((*self.vfptr).updateShapeCollectionFilter)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deallocateInternalArrays(&self) {
        ((*self.vfptr).deallocateInternalArrays)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setPositionAndLinearCast(
        &self,
        a1: *const hkVector4f,
        a2: *const hkpLinearCastInput,
        a3: *mut hkpCdPointCollector,
        a4: *mut hkpCdPointCollector,
    ) {
        ((*self.vfptr).setPositionAndLinearCast)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn setTransformAndLinearCast(
        &self,
        a1: *const hkTransformf,
        a2: *const hkpLinearCastInput,
        a3: *mut hkpCdPointCollector,
        a4: *mut hkpCdPointCollector,
    ) {
        ((*self.vfptr).setTransformAndLinearCast)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getClosestPoints(
        &self,
        a1: *mut hkpCdPointCollector,
        a2: *const hkpCollisionInput,
    ) {
        ((*self.vfptr).getClosestPoints)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPenetrations(
        &self,
        a1: *mut hkpCdBodyPairCollector,
        a2: *const hkpCollisionInput,
    ) {
        ((*self.vfptr).getPenetrations)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpShapePhantom__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpShapePhantom) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpShapePhantom),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState:
        unsafe extern "thiscall" fn(this: *mut hkpShapePhantom) -> *mut hkMotionState,
    pub getType: unsafe extern "thiscall" fn(this: *const hkpShapePhantom) -> hkpPhantomType,
    pub calcAabb: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom, _: *mut hkAabb),
    pub addOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpShapePhantom, _: *mut hkpCollidable),
    pub isOverlappingCollidableAdded: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        result: *mut hkBool,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
    pub removeOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpShapePhantom, _: *mut hkpCollidable),
    pub ensureDeterministicOrder: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom),
    pub clone: unsafe extern "thiscall" fn(this: *const hkpShapePhantom) -> *mut hkpPhantom,
    pub updateShapeCollectionFilter: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom),
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpShapePhantom),
    pub setPositionAndLinearCast: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *const hkVector4f,
        _: *const hkpLinearCastInput,
        _: *mut hkpCdPointCollector,
        _: *mut hkpCdPointCollector,
    ),
    pub setTransformAndLinearCast: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *const hkTransformf,
        _: *const hkpLinearCastInput,
        _: *mut hkpCdPointCollector,
        _: *mut hkpCdPointCollector,
    ),
    pub getClosestPoints: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *mut hkpCdPointCollector,
        _: *const hkpCollisionInput,
    ),
    pub getPenetrations: unsafe extern "thiscall" fn(
        this: *mut hkpShapePhantom,
        _: *mut hkpCdBodyPairCollector,
        _: *const hkpCollisionInput,
    ),
}

#[repr(C)]
pub struct hkpCollisionInput__Aabb32Info {
    pub m_bitOffsetLow: hkVector4f,
    pub m_bitOffsetHigh: hkVector4f,
    pub m_bitScale: hkVector4f,
}

#[repr(C)]
pub struct hkpCollisionDispatcher {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_debugAgent2Table")]
#[repr(C)]
pub struct hkpCollisionDispatcher {
    // hkBaseObject
    pub vfptr: *const hkpCollisionDispatcher__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpCollisionDispatcher
    pub m_defaultCollisionAgent: *mut unsafe extern "C" fn(
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpCollisionInput,
        _: *mut hkpContactMgr,
    ) -> *mut hkpCollisionAgent,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_contactMgrFactory: compile_error!("unimplemented sizeof array"),
    __pdbindgen_padding: [u8; 260],
    pub m_hasAlternateType: [u32; 35],
    pub m_numAgent2Types: i32,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_agent2Types: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_agent2TypesPred: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_agent2Func: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 3732],
    pub m_numAgent3Types: i32,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_agent3Types: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_agent3TypesPred: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_agent3Func: compile_error!("unimplemented class layout"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_collisionQualityTable: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_collisionQualityInfo: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_3: [u8; 4248],
    pub m_collisionAgentRegistered: hkBool,
    pub m_agent3Registered: hkBool,
    pub m_midphaseAgent3Registered: hkBool,
    pub m_checkEnabled: hkBool,
    pub m_shapeInheritance:
        hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_debugAgent2Table: *mut compile_error!("unimplemented class layout"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_debugAgent2TablePred: *mut compile_error!("unimplemented class layout"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_debugAgent3Table: *mut compile_error!("unimplemented class layout"),
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_debugAgent3TablePred: *mut compile_error!("unimplemented class layout"),
    pub m_expectedMaxLinearVelocity: f32,
    pub m_expectedMinPsiDeltaTime: f32,
    __pdbindgen_padding_4: [u8; 8],
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpCollisionDispatcher {}

unsafe impl UpcastToNop<hkBaseObject> for hkpCollisionDispatcher {}

#[cfg(pdb_issue = "error in field m_debugAgent2Table")]
impl hkpCollisionDispatcher {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpCollisionDispatcher__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionDispatcher, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionDispatcher),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpCollisionDispatcher) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpCollisionDispatcher),
}

#[repr(C)]
pub struct hkpBroadPhaseBorder {
    // hkBaseObject
    pub vfptr: *const hkpBroadPhaseBorder__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldDeletionListener
    pub vfptr_2: *const hkpBroadPhaseBorder__vftable,
    // hkpPhantomOverlapListener
    pub vfptr_3: *const hkpBroadPhaseBorder__vftable,
    // hkpWorldPostSimulationListener
    pub vfptr_4: *const hkpBroadPhaseBorder__vftable,
    // hkpBroadPhaseBorder
    pub m_world: *mut hkpWorld,
    pub m_phantoms: [*mut hkpPhantom; 6],
    pub m_type: hkpWorldCinfo__BroadPhaseBorderBehaviour,
    pub m_postponeAndSortCallbacks: hkBool,
    pub m_entitiesExitingBroadPhase: hkArray_hkpEntity___hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBroadPhaseBorder {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBroadPhaseBorder {}

unsafe impl UpcastTo<hkpWorldDeletionListener> for hkpBroadPhaseBorder {
    fn upcast_to(p: *const Self) -> *const hkpWorldDeletionListener {
        (p as usize + 0x8) as *const _
    }
}

unsafe impl UpcastTo<hkpPhantomOverlapListener> for hkpBroadPhaseBorder {
    fn upcast_to(p: *const Self) -> *const hkpPhantomOverlapListener {
        (p as usize + 0xc) as *const _
    }
}

unsafe impl UpcastTo<hkpWorldPostSimulationListener> for hkpBroadPhaseBorder {
    fn upcast_to(p: *const Self) -> *const hkpWorldPostSimulationListener {
        (p as usize + 0x10) as *const _
    }
}

impl hkpBroadPhaseBorder {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).postSimulationCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inactiveEntityMovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).inactiveEntityMovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn maxPositionExceededCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).maxPositionExceededCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn deactivate(&self) {
        ((*self.vfptr).deactivate)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorder__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder, _: *mut hkpEntity),
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBroadPhaseBorder),
    pub maxPositionExceededCallback:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder, _: *mut hkpEntity),
    pub deactivate: unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder),
}

#[repr(C)]
pub struct hkpMtThreadStructure {
    pub m_world: *mut hkpWorld,
    __pdbindgen_padding: [u8; 12],
    pub m_constraintQueryIn: hkpConstraintQueryIn,
    pub m_collisionInput: hkpProcessCollisionInput,
    pub m_simulation: hkPadSpu_hkpMultiThreadedSimulation___,
    pub m_dynamicsStepInfo: hkPadSpu_hkpWorldDynamicsStepInfo___,
    pub m_tolerance: hkPadSpu_float_,
    pub m_weldingTable: hkPadSpu_void___,
}

#[repr(C)]
pub struct hkPadSpu_hkpMultiThreadedSimulation___ {
    pub m_storage: *mut hkpMultiThreadedSimulation,
}

#[repr(C)]
pub struct hkPadSpu_hkpWorldDynamicsStepInfo___ {
    pub m_storage: *mut hkpWorldDynamicsStepInfo,
}

#[repr(C)]
pub struct hkPadSpu_void___ {
    pub m_storage: *mut (),
}

#[repr(C)]
pub struct hkpProcessCdPoint {
    pub m_contact: hkContactPoint,
    pub m_contactPointId: u32,
    pub m_isShortestPoint: u32,
    pub m_padding: [u32; 2],
}

#[repr(C)]
pub struct hkpProcessCollisionData {
    pub m_firstFreeContactPoint: hkPadSpu_hkpProcessCdPoint___,
    pub m_constraintOwner: hkPadSpu_hkpConstraintOwner___,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_contactPoints: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 12296],
    pub m_toi: hkpProcessCollisionData__ToiInfo,
}

#[repr(C)]
pub struct hkpProcessCollisionData__ToiInfo {
    pub m_contactPoint: hkContactPoint,
    pub m_time: hkPadSpu_float_,
    pub m_seperatingVelocity: hkPadSpu_float_,
    __pdbindgen_padding: [u8; 8],
    pub m_gskCache: hkGskCache16,
    pub m_properties: hkContactPointPropertiesWithExtendedUserData16,
}

#[repr(C)]
pub struct hkpToiEvent {
    pub m_time: f32,
    pub m_seperatingVelocity: f32,
    pub m_useSimpleHandling: hkBool,
    pub m_entities: [*mut hkpEntity; 2],
    pub m_contactMgr: *mut hkpDynamicsContactMgr,
    pub m_properties: hkpContactPointProperties,
    pub m_extendedUserDatas: [u32; 7],
    __pdbindgen_padding: [u8; 8],
    pub m_contactPoint: hkContactPoint,
}

#[repr(C)]
pub struct hkpContinuousSimulation {
    // hkBaseObject
    pub vfptr: *const hkpContinuousSimulation__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpSimulation
    pub m_determinismCheckFrameCounter: u32,
    pub m_world: *mut hkpWorld,
    pub m_lastProcessingStep: hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_,
    pub m_currentTime: f32,
    pub m_currentPsiTime: f32,
    pub m_physicsDeltaTime: f32,
    pub m_simulateUntilTime: f32,
    pub m_frameMarkerPsiSnap: f32,
    pub m_previousStepResult: u32,
    // hkpContinuousSimulation
    __pdbindgen_padding: [u8; 4],
    pub m_toiEvents: hkArray_hkpToiEvent_hkContainerHeapAllocator_,
    pub m_entitiesNeedingPsiCollisionDetection:
        hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    pub m_toiResourceMgr: *mut hkpToiResourceMgr,
    pub m_toiCounter: i32,
}

unsafe impl UpcastToNop<hkpSimulation> for hkpContinuousSimulation {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpContinuousSimulation {}

unsafe impl UpcastToNop<hkBaseObject> for hkpContinuousSimulation {}

impl hkpContinuousSimulation {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepDeltaTime(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).stepDeltaTime)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn integrate(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).integrate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collide(&self) -> hkpStepResult {
        ((*self.vfptr).collide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn advanceTime(&self) -> hkpStepResult {
        ((*self.vfptr).advanceTime)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepBeginSt(
        &self,
        a1: *mut hkJobQueue,
        a2: f32,
    ) -> hkpStepResult {
        ((*self.vfptr).stepBeginSt)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn finishMtStep(
        &self,
        a1: *mut hkJobQueue,
        a2: *mut hkThreadPool,
    ) -> hkpStepResult {
        ((*self.vfptr).finishMtStep)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMultithreadConfig(&self, a1: *mut hkpMultithreadConfig) {
        ((*self.vfptr).getMultithreadConfig)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMultithreadConfig(
        &self,
        a1: *const hkpMultithreadConfig,
        a2: *mut hkJobQueue,
    ) {
        ((*self.vfptr).setMultithreadConfig)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn collideEntitiesDiscrete(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: *const hkStepInfo,
        a5: hkpSimulation__FindContacts,
    ) {
        ((*self.vfptr).collideEntitiesDiscrete)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn resetCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: hkpSimulation__ResetCollisionInformation,
    ) {
        ((*self.vfptr).resetCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
        )
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
        )
    }

    pub unsafe extern "thiscall" fn removeCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).removeCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reintegrateAndRecollideEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: i32,
    ) {
        ((*self.vfptr).reintegrateAndRecollideEntities)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn collideInternal(&self, a1: *const hkStepInfo) {
        ((*self.vfptr).collideInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn warpTime(&self, a1: f32) {
        ((*self.vfptr).warpTime)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
        )
    }

    pub unsafe extern "thiscall" fn collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly(
        &self,
        a1: *const hkpProcessCollisionInput,
        a2: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly)(
            self as *const _ as *mut _,
            a1,
            a2,
        )
    }

    pub unsafe extern "thiscall" fn simulateToi(
        &self,
        a1: *mut hkpWorld,
        a2: *mut hkpToiEvent,
        a3: f32,
        a4: f32,
    ) {
        ((*self.vfptr).simulateToi)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct hkpContinuousSimulation__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpContinuousSimulation) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpContinuousSimulation),
    pub stepDeltaTime:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: f32) -> hkpStepResult,
    pub integrate:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: f32) -> hkpStepResult,
    pub collide: unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation) -> hkpStepResult,
    pub advanceTime:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation) -> hkpStepResult,
    pub stepBeginSt: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut hkJobQueue,
        _: f32,
    ) -> hkpStepResult,
    pub finishMtStep: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut hkJobQueue,
        _: *mut hkThreadPool,
    ) -> hkpStepResult,
    pub getMultithreadConfig: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut hkpMultithreadConfig,
    ),
    pub setMultithreadConfig: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *const hkpMultithreadConfig,
        _: *mut hkJobQueue,
    ),
    pub collideEntitiesDiscrete: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: *const hkStepInfo,
        _: hkpSimulation__FindContacts,
    ),
    pub resetCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: hkpSimulation__ResetCollisionInformation,
    ),
    pub assertThereIsNoCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
    ),
    pub removeCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: *mut hkpAgentNnEntry),
    pub assertThereIsNoCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: *mut hkpAgentNnEntry),
    pub reintegrateAndRecollideEntities: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: i32,
    ),
    pub collideInternal:
        unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: *const hkStepInfo),
    pub warpTime: unsafe extern "thiscall" fn(this: *mut hkpContinuousSimulation, _: f32),
    pub collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *const hkpProcessCollisionInput,
        _: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ),
    pub collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly:
        unsafe extern "thiscall" fn(
            this: *mut hkpContinuousSimulation,
            _: *const hkpProcessCollisionInput,
            _: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
        ),
    pub simulateToi: unsafe extern "thiscall" fn(
        this: *mut hkpContinuousSimulation,
        _: *mut hkpWorld,
        _: *mut hkpToiEvent,
        _: f32,
        _: f32,
    ),
}

#[repr(C)]
pub struct hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_ {
    pub m_map:
        hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkArrayBase_hkpToiEvent_ {
    pub m_data: *mut hkpToiEvent,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpToiEvent_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpToiEvent_
    pub m_data: *mut hkpToiEvent,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpToiEvent_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpToiEvent_>
    for hkArray_hkpToiEvent_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpBodyVelocity {
    pub m_linear: hkVector4f,
    pub m_angular: hkVector4f,
}

#[repr(C)]
pub struct hkpSimpleConstraintInfoInitInput {
    pub m_massRelPos: hkVector4f,
    pub m_invInertia: hkMatrix3f,
    pub m_invMasses: hkVector4f,
    pub m_transform: *const hkTransformf,
    __pdbindgen_padding: [u8; 12],
}

#[repr(C)]
pub struct hkArray_short_hkContainerHeapAllocator_ {
    // hkArrayBase_short_
    pub m_data: *mut i16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_short_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_short_> for hkArray_short_hkContainerHeapAllocator_ {}

#[repr(C)]
pub struct hkArrayBase_short_ {
    pub m_data: *mut i16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpPhantomBroadPhaseListener {
    // hkBaseObject
    pub vfptr: *const hkpPhantomBroadPhaseListener__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBroadPhaseListener
    pub vfptr_2: *const hkpPhantomBroadPhaseListener__vftable,
    // hkpPhantomBroadPhaseListener
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpPhantomBroadPhaseListener {}

unsafe impl UpcastToNop<hkBaseObject> for hkpPhantomBroadPhaseListener {}

unsafe impl UpcastTo<hkpBroadPhaseListener> for hkpPhantomBroadPhaseListener {
    fn upcast_to(p: *const Self) -> *const hkpBroadPhaseListener {
        (p as usize + 0x8) as *const _
    }
}

impl hkpPhantomBroadPhaseListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpPhantomBroadPhaseListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomBroadPhaseListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpPhantomBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpPhantomBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpPhantomBroadPhaseListener),
}

#[repr(C)]
pub struct hkpContactMgr {
    // hkBaseObject
    pub vfptr: *const hkpContactMgr__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpContactMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpContactMgr {}

impl hkpContactMgr {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addContactPointImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: *const hkpGskCache,
        a6: *mut hkContactPoint,
    ) -> u16 {
        ((*self.vfptr).addContactPointImpl)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn reserveContactPointsImpl(
        &self,
        result: *mut hkResult,
        a2: i32,
    ) -> *mut hkResult {
        ((*self.vfptr).reserveContactPointsImpl)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn removeContactPointImpl(
        &self,
        a1: u16,
        a2: *mut hkpConstraintOwner,
    ) {
        ((*self.vfptr).removeContactPointImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processContactImpl(
        &self,
        a1: *const hkpCollidable,
        a2: *const hkpCollidable,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionData,
    ) {
        ((*self.vfptr).processContactImpl)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn addToiImpl(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
        a5: f32,
        a6: *mut hkContactPoint,
        a7: *const hkpGskCache,
        a8: *mut f32,
        a9: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept {
        ((*self.vfptr).addToiImpl)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
        )
    }

    pub unsafe extern "thiscall" fn removeToiImpl(
        &self,
        a1: *mut hkpConstraintOwner,
        a2: *mut hkpContactPointProperties,
    ) {
        ((*self.vfptr).removeToiImpl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn cleanup(&self) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpContactMgr__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpContactMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpContactMgr),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpContactMgr) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpContactMgr),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        result: *mut hkResult,
        _: i32,
    ) -> *mut hkResult,
    pub removeContactPointImpl:
        unsafe extern "thiscall" fn(this: *mut hkpContactMgr, _: u16, _: *mut hkpConstraintOwner),
    pub processContactImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionData,
    ),
    pub addToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: f32,
        _: *mut hkContactPoint,
        _: *const hkpGskCache,
        _: *mut f32,
        _: *mut hkpContactPointProperties,
    ) -> hkpContactMgr__ToiAccept,
    pub removeToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *mut hkpConstraintOwner,
        _: *mut hkpContactPointProperties,
    ),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut hkpContactMgr),
}

#[repr(C)]
pub struct hkpWorldExtension {
    // hkBaseObject
    pub vfptr: *const hkpWorldExtension__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldExtension
    pub m_world: *mut hkpWorld,
    pub m_id: i32,
    pub m_attachmentCount: u16,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpWorldExtension {}

unsafe impl UpcastToNop<hkBaseObject> for hkpWorldExtension {}

impl hkpWorldExtension {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn performAttachments(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).performAttachments)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn performDetachments(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).performDetachments)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpWorldExtension__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpWorldExtension, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpWorldExtension),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpWorldExtension) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpWorldExtension),
    pub performAttachments:
        unsafe extern "thiscall" fn(this: *mut hkpWorldExtension, _: *mut hkpWorld),
    pub performDetachments:
        unsafe extern "thiscall" fn(this: *mut hkpWorldExtension, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkpEntityActivationListener {
    pub vfptr: *const hkpEntityActivationListener__vftable,
}

impl hkpEntityActivationListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityDeactivatedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityDeactivatedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityActivatedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityActivatedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpEntityActivationListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpEntityActivationListener, _: u32) -> *mut (),
    pub entityDeactivatedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityActivationListener, _: *mut hkpEntity),
    pub entityActivatedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityActivationListener, _: *mut hkpEntity),
}

#[repr(C)]
pub struct hkViewPtr_hkpConstraintInstance_ {
    pub m_ptr: *mut hkpConstraintInstance,
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation {
    // hkBaseObject
    pub vfptr: *const hkpMultiThreadedSimulation__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpSimulation
    pub m_determinismCheckFrameCounter: u32,
    pub m_world: *mut hkpWorld,
    pub m_lastProcessingStep: hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_,
    pub m_currentTime: f32,
    pub m_currentPsiTime: f32,
    pub m_physicsDeltaTime: f32,
    pub m_simulateUntilTime: f32,
    pub m_frameMarkerPsiSnap: f32,
    pub m_previousStepResult: u32,
    // hkpContinuousSimulation
    __pdbindgen_padding: [u8; 4],
    pub m_toiEvents: hkArray_hkpToiEvent_hkContainerHeapAllocator_,
    pub m_entitiesNeedingPsiCollisionDetection:
        hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    pub m_toiResourceMgr: *mut hkpToiResourceMgr,
    pub m_toiCounter: i32,
    // hkpMultiThreadedSimulation
    pub m_entityEntityBroadPhaseListener:
        hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener,
    pub m_phantomBroadPhaseListener: hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener,
    pub m_broadPhaseBorderListener: hkpMultiThreadedSimulation__MtBroadPhaseBorderListener,
    pub m_crossIslandPairsCollectingActive: hkBool,
    pub m_addedCrossIslandPairs: hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_,
    pub m_addCrossIslandPairCriticalSection: hkCriticalSection,
    pub m_removedCrossIslandPairs: hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_,
    pub m_removeCrossIslandPairCriticalSection: hkCriticalSection,
    pub m_multithreadConfig: hkpMultithreadConfig,
    pub m_numActiveIslandsAtBeginningOfStep: i32,
    pub m_numInactiveIslandsAtBeginningOfStep: i32,
    pub m_jobQueueHandleForToiSolve: *mut hkJobQueue,
    __pdbindgen_padding_2: [u8; 60],
    pub m_toiQueueCriticalSection: hkCriticalSection,
    __pdbindgen_padding_3: [u8; 40],
    pub m_phantomCriticalSection: hkCriticalSection,
    __pdbindgen_padding_4: [u8; 40],
}

unsafe impl UpcastToNop<hkpContinuousSimulation> for hkpMultiThreadedSimulation {}

unsafe impl UpcastToNop<hkpSimulation> for hkpMultiThreadedSimulation {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpMultiThreadedSimulation {}

unsafe impl UpcastToNop<hkBaseObject> for hkpMultiThreadedSimulation {}

impl hkpMultiThreadedSimulation {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepDeltaTime(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).stepDeltaTime)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn integrate(&self, a1: f32) -> hkpStepResult {
        ((*self.vfptr).integrate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collide(&self) -> hkpStepResult {
        ((*self.vfptr).collide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn advanceTime(&self) -> hkpStepResult {
        ((*self.vfptr).advanceTime)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn stepBeginSt(
        &self,
        a1: *mut hkJobQueue,
        a2: f32,
    ) -> hkpStepResult {
        ((*self.vfptr).stepBeginSt)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn finishMtStep(
        &self,
        a1: *mut hkJobQueue,
        a2: *mut hkThreadPool,
    ) -> hkpStepResult {
        ((*self.vfptr).finishMtStep)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMultithreadConfig(&self, a1: *mut hkpMultithreadConfig) {
        ((*self.vfptr).getMultithreadConfig)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMultithreadConfig(
        &self,
        a1: *const hkpMultithreadConfig,
        a2: *mut hkJobQueue,
    ) {
        ((*self.vfptr).setMultithreadConfig)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn collideEntitiesDiscrete(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: *const hkStepInfo,
        a5: hkpSimulation__FindContacts,
    ) {
        ((*self.vfptr).collideEntitiesDiscrete)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn resetCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: hkpSimulation__ResetCollisionInformation,
    ) {
        ((*self.vfptr).resetCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
        )
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForEntities)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
        )
    }

    pub unsafe extern "thiscall" fn removeCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).removeCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn assertThereIsNoCollisionInformationForAgent(
        &self,
        a1: *mut hkpAgentNnEntry,
    ) {
        ((*self.vfptr).assertThereIsNoCollisionInformationForAgent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reintegrateAndRecollideEntities(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *mut hkpWorld,
        a4: i32,
    ) {
        ((*self.vfptr).reintegrateAndRecollideEntities)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn collideInternal(&self, a1: *const hkStepInfo) {
        ((*self.vfptr).collideInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn warpTime(&self, a1: f32) {
        ((*self.vfptr).warpTime)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly(
        &self,
        a1: *mut *mut hkpEntity,
        a2: i32,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
        )
    }

    pub unsafe extern "thiscall" fn collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly(
        &self,
        a1: *const hkpProcessCollisionInput,
        a2: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly)(
            self as *const _ as *mut _,
            a1,
            a2,
        )
    }

    pub unsafe extern "thiscall" fn simulateToi(
        &self,
        a1: *mut hkpWorld,
        a2: *mut hkpToiEvent,
        a3: f32,
        a4: f32,
    ) {
        ((*self.vfptr).simulateToi)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpMultiThreadedSimulation) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpMultiThreadedSimulation),
    pub stepDeltaTime:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: f32) -> hkpStepResult,
    pub integrate:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: f32) -> hkpStepResult,
    pub collide:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation) -> hkpStepResult,
    pub advanceTime:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation) -> hkpStepResult,
    pub stepBeginSt: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut hkJobQueue,
        _: f32,
    ) -> hkpStepResult,
    pub finishMtStep: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut hkJobQueue,
        _: *mut hkThreadPool,
    ) -> hkpStepResult,
    pub getMultithreadConfig: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut hkpMultithreadConfig,
    ),
    pub setMultithreadConfig: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *const hkpMultithreadConfig,
        _: *mut hkJobQueue,
    ),
    pub collideEntitiesDiscrete: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: *const hkStepInfo,
        _: hkpSimulation__FindContacts,
    ),
    pub resetCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: hkpSimulation__ResetCollisionInformation,
    ),
    pub assertThereIsNoCollisionInformationForEntities: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
    ),
    pub removeCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: *mut hkpAgentNnEntry),
    pub assertThereIsNoCollisionInformationForAgent:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: *mut hkpAgentNnEntry),
    pub reintegrateAndRecollideEntities: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *mut hkpWorld,
        _: i32,
    ),
    pub collideInternal:
        unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: *const hkStepInfo),
    pub warpTime: unsafe extern "thiscall" fn(this: *mut hkpMultiThreadedSimulation, _: f32),
    pub collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut *mut hkpEntity,
        _: i32,
        _: *const hkpProcessCollisionInput,
        _: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    ),
    pub collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly:
        unsafe extern "thiscall" fn(
            this: *mut hkpMultiThreadedSimulation,
            _: *const hkpProcessCollisionInput,
            _: *mut hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
        ),
    pub simulateToi: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation,
        _: *mut hkpWorld,
        _: *mut hkpToiEvent,
        _: f32,
        _: f32,
    ),
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener {
    // hkpBroadPhaseListener
    pub vfptr: *const hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener__vftable,
    // hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener
    pub m_simulation: *mut hkpMultiThreadedSimulation,
}

unsafe impl UpcastToNop<hkpBroadPhaseListener>
    for hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener
{
}

impl hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener,
        _: u32,
    ) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener {
    // hkpBroadPhaseListener
    pub vfptr: *const hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener__vftable,
    // hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener
    pub m_criticalSection: *mut hkCriticalSection,
}

unsafe impl UpcastToNop<hkpBroadPhaseListener>
    for hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener
{
}

impl hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener,
        _: u32,
    ) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtBroadPhaseBorderListener {
    // hkpBroadPhaseListener
    pub vfptr: *const hkpMultiThreadedSimulation__MtBroadPhaseBorderListener__vftable,
    // hkpMultiThreadedSimulation__MtBroadPhaseBorderListener
    pub m_criticalSection: *mut hkCriticalSection,
}

unsafe impl UpcastToNop<hkpBroadPhaseListener>
    for hkpMultiThreadedSimulation__MtBroadPhaseBorderListener
{
}

impl hkpMultiThreadedSimulation__MtBroadPhaseBorderListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtBroadPhaseBorderListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtBroadPhaseBorderListener,
        _: u32,
    ) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtBroadPhaseBorderListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpMultiThreadedSimulation__MtBroadPhaseBorderListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
}

#[repr(C)]
pub struct hkArrayBase_hkpTypedBroadPhaseHandlePair_ {
    pub m_data: *mut hkpTypedBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpTypedBroadPhaseHandlePair_
    pub m_data: *mut hkpTypedBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpTypedBroadPhaseHandlePair_>
    for hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListener {
    pub vfptr: *const hkpContactImpulseLimitBreachedListener__vftable,
}

impl hkpContactImpulseLimitBreachedListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactImpulseLimitBreachedCallback(
        &self,
        a1: *const hkpContactImpulseLimitBreachedListenerInfo,
        a2: i32,
    ) {
        ((*self.vfptr).contactImpulseLimitBreachedCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpContactImpulseLimitBreachedListener,
        _: u32,
    ) -> *mut (),
    pub contactImpulseLimitBreachedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpContactImpulseLimitBreachedListener,
        _: *const hkpContactImpulseLimitBreachedListenerInfo,
        _: i32,
    ),
}

#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListenerInfo {
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_data: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct hkpShapeRayBundleCastOutput {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_outputs: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 320],
}

#[repr(C)]
pub struct hkpWorldMaintenanceMgr {
    // hkBaseObject
    pub vfptr: *const hkpWorldMaintenanceMgr__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldMaintenanceMgr
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpWorldMaintenanceMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpWorldMaintenanceMgr {}

impl hkpWorldMaintenanceMgr {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn init(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).init)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn performMaintenance(
        &self,
        a1: *mut hkpWorld,
        a2: *mut hkStepInfo,
    ) {
        ((*self.vfptr).performMaintenance)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn performMaintenanceNoSplit(
        &self,
        a1: *mut hkpWorld,
        a2: *mut hkStepInfo,
    ) {
        ((*self.vfptr).performMaintenanceNoSplit)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpWorldMaintenanceMgr__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldMaintenanceMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpWorldMaintenanceMgr),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpWorldMaintenanceMgr) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpWorldMaintenanceMgr),
    pub init: unsafe extern "thiscall" fn(this: *mut hkpWorldMaintenanceMgr, _: *mut hkpWorld),
    pub performMaintenance: unsafe extern "thiscall" fn(
        this: *mut hkpWorldMaintenanceMgr,
        _: *mut hkpWorld,
        _: *mut hkStepInfo,
    ),
    pub performMaintenanceNoSplit: unsafe extern "thiscall" fn(
        this: *mut hkpWorldMaintenanceMgr,
        _: *mut hkpWorld,
        _: *mut hkStepInfo,
    ),
}

#[repr(C)]
pub struct hkpEntityEntityBroadPhaseListener {
    // hkBaseObject
    pub vfptr: *const hkpEntityEntityBroadPhaseListener__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBroadPhaseListener
    pub vfptr_2: *const hkpEntityEntityBroadPhaseListener__vftable,
    // hkpEntityEntityBroadPhaseListener
    pub m_world: *mut hkpWorld,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpEntityEntityBroadPhaseListener {}

unsafe impl UpcastToNop<hkBaseObject> for hkpEntityEntityBroadPhaseListener {}

unsafe impl UpcastTo<hkpBroadPhaseListener> for hkpEntityEntityBroadPhaseListener {
    fn upcast_to(p: *const Self) -> *const hkpBroadPhaseListener {
        (p as usize + 0x8) as *const _
    }
}

impl hkpEntityEntityBroadPhaseListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpEntityEntityBroadPhaseListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkpEntityEntityBroadPhaseListener,
        _: u32,
    ) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpEntityEntityBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpEntityEntityBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpEntityEntityBroadPhaseListener),
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener {
    // hkBaseObject
    pub vfptr: *const hkpBroadPhaseBorderListener__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBroadPhaseListener
    pub vfptr_2: *const hkpBroadPhaseBorderListener__vftable,
    // hkpBroadPhaseBorderListener
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBroadPhaseBorderListener {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBroadPhaseBorderListener {}

unsafe impl UpcastTo<hkpBroadPhaseListener> for hkpBroadPhaseBorderListener {
    fn upcast_to(p: *const Self) -> *const hkpBroadPhaseListener {
        (p as usize + 0x8) as *const _
    }
}

impl hkpBroadPhaseBorderListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCollisionPair(&self, a1: *mut hkpTypedBroadPhaseHandlePair) {
        ((*self.vfptr).addCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeCollisionPair(
        &self,
        a1: *mut hkpTypedBroadPhaseHandlePair,
    ) {
        ((*self.vfptr).removeCollisionPair)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorderListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseBorderListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseBorderListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpBroadPhaseBorderListener),
}

#[repr(C)]
pub struct hkpToiResourceMgr {
    // hkBaseObject
    pub vfptr: *const hkpToiResourceMgr__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpToiResourceMgr
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpToiResourceMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpToiResourceMgr {}

impl hkpToiResourceMgr {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn beginToiAndSetupResources(
        &self,
        result: *mut hkResult,
        a2: *const hkpToiEvent,
        a3: *const hkArray_hkpToiEvent_hkContainerHeapAllocator_,
        a4: *mut hkpToiResources,
    ) -> *mut hkResult {
        ((*self.vfptr).beginToiAndSetupResources)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn cannotSolve(
        &self,
        a1: *mut hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_,
    ) -> hkpToiResourceMgrResponse {
        ((*self.vfptr).cannotSolve)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn resourcesDepleted(&self) -> hkpToiResourceMgrResponse {
        ((*self.vfptr).resourcesDepleted)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn endToiAndFreeResources(
        &self,
        a1: *const hkpToiEvent,
        a2: *const hkArray_hkpToiEvent_hkContainerHeapAllocator_,
        a3: *const hkpToiResources,
    ) {
        ((*self.vfptr).endToiAndFreeResources)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getScratchpadCapacity(&self) -> i32 {
        ((*self.vfptr).getScratchpadCapacity)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpToiResourceMgr__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpToiResourceMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpToiResourceMgr),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpToiResourceMgr) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpToiResourceMgr),
    pub beginToiAndSetupResources: unsafe extern "thiscall" fn(
        this: *mut hkpToiResourceMgr,
        result: *mut hkResult,
        _: *const hkpToiEvent,
        _: *const hkArray_hkpToiEvent_hkContainerHeapAllocator_,
        _: *mut hkpToiResources,
    ) -> *mut hkResult,
    pub cannotSolve: unsafe extern "thiscall" fn(
        this: *mut hkpToiResourceMgr,
        _: *mut hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_,
    ) -> hkpToiResourceMgrResponse,
    pub resourcesDepleted:
        unsafe extern "thiscall" fn(this: *mut hkpToiResourceMgr) -> hkpToiResourceMgrResponse,
    pub endToiAndFreeResources: unsafe extern "thiscall" fn(
        this: *mut hkpToiResourceMgr,
        _: *const hkpToiEvent,
        _: *const hkArray_hkpToiEvent_hkContainerHeapAllocator_,
        _: *const hkpToiResources,
    ),
    pub getScratchpadCapacity: unsafe extern "thiscall" fn(this: *mut hkpToiResourceMgr) -> i32,
}

#[repr(C)]
pub struct hkpToiResources {
    pub m_minPriorityToProcess: hkpConstraintInstance__ConstraintPriority,
    pub m_maxNumEntities: i32,
    pub m_maxNumActiveEntities: i32,
    pub m_maxNumConstraints: i32,
    pub m_numToiSolverIterations: i32,
    pub m_numForcedToiFinalSolverIterations: i32,
    pub m_scratchpad: *mut i8,
    pub m_scratchpadSize: i32,
    pub m_priorityClassMap: *const u8,
    pub m_priorityClassRatios: *const f32,
}

#[repr(C)]
pub struct hkpToiResourceMgr__ConstraintViolationInfo {
    pub m_constraint: *mut hkpConstraintInstance,
    pub m_contactPoint: *const hkContactPoint,
    pub m_contactPointProperties: *const hkpContactPointProperties,
}

#[repr(C)]
pub struct hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_
    pub m_data: *mut hkpToiResourceMgr__ConstraintViolationInfo,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_>
    for hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_ {
    pub m_data: *mut hkpToiResourceMgr__ConstraintViolationInfo,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpActionListener {
    pub vfptr: *const hkpActionListener__vftable,
}

impl hkpActionListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn actionAddedCallback(&self, a1: *mut hkpAction) {
        ((*self.vfptr).actionAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn actionRemovedCallback(&self, a1: *mut hkpAction) {
        ((*self.vfptr).actionRemovedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpActionListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: u32) -> *mut (),
    pub actionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: *mut hkpAction),
    pub actionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: *mut hkpAction),
}

#[repr(C)]
pub struct hkpIslandActivationListener {
    pub vfptr: *const hkpIslandActivationListener__vftable,
}

impl hkpIslandActivationListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn islandActivatedCallback(&self, a1: *mut hkpSimulationIsland) {
        ((*self.vfptr).islandActivatedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn islandDeactivatedCallback(&self, a1: *mut hkpSimulationIsland) {
        ((*self.vfptr).islandDeactivatedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpIslandActivationListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpIslandActivationListener, _: u32) -> *mut (),
    pub islandActivatedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpIslandActivationListener,
        _: *mut hkpSimulationIsland,
    ),
    pub islandDeactivatedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpIslandActivationListener,
        _: *mut hkpSimulationIsland,
    ),
}

#[repr(C)]
pub struct hkpWorldPostCollideListener {
    pub vfptr: *const hkpWorldPostCollideListener__vftable,
}

impl hkpWorldPostCollideListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postCollideCallback(
        &self,
        a1: *mut hkpWorld,
        a2: *const hkStepInfo,
    ) {
        ((*self.vfptr).postCollideCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpWorldPostCollideListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostCollideListener, _: u32) -> *mut (),
    pub postCollideCallback: unsafe extern "thiscall" fn(
        this: *mut hkpWorldPostCollideListener,
        _: *mut hkpWorld,
        _: *const hkStepInfo,
    ),
}

#[repr(C)]
pub struct hkpWorldPostIntegrateListener {
    pub vfptr: *const hkpWorldPostIntegrateListener__vftable,
}

impl hkpWorldPostIntegrateListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postIntegrateCallback(
        &self,
        a1: *mut hkpWorld,
        a2: *const hkStepInfo,
    ) {
        ((*self.vfptr).postIntegrateCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpWorldPostIntegrateListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostIntegrateListener, _: u32) -> *mut (),
    pub postIntegrateCallback: unsafe extern "thiscall" fn(
        this: *mut hkpWorldPostIntegrateListener,
        _: *mut hkpWorld,
        _: *const hkStepInfo,
    ),
}

#[repr(C)]
pub struct hkpIslandPostCollideListener {
    pub vfptr: *const hkpIslandPostCollideListener__vftable,
}

impl hkpIslandPostCollideListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postCollideCallback(
        &self,
        a1: *mut hkpSimulationIsland,
        a2: *const hkStepInfo,
    ) {
        ((*self.vfptr).postCollideCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpIslandPostCollideListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpIslandPostCollideListener, _: u32) -> *mut (),
    pub postCollideCallback: unsafe extern "thiscall" fn(
        this: *mut hkpIslandPostCollideListener,
        _: *mut hkpSimulationIsland,
        _: *const hkStepInfo,
    ),
}

#[repr(C)]
pub struct hkpIslandPostIntegrateListener {
    pub vfptr: *const hkpIslandPostIntegrateListener__vftable,
}

impl hkpIslandPostIntegrateListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postIntegrateCallback(
        &self,
        a1: *mut hkpSimulationIsland,
        a2: *const hkStepInfo,
    ) {
        ((*self.vfptr).postIntegrateCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpIslandPostIntegrateListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpIslandPostIntegrateListener, _: u32) -> *mut (),
    pub postIntegrateCallback: unsafe extern "thiscall" fn(
        this: *mut hkpIslandPostIntegrateListener,
        _: *mut hkpSimulationIsland,
        _: *const hkStepInfo,
    ),
}

#[repr(C)]
pub struct hkpEntity {
    // hkBaseObject
    pub vfptr: *const hkpEntity__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldObject
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
    // hkpEntity
    pub m_material: hkpMaterial,
    pub m_limitContactImpulseUtilAndFlag: *mut (),
    pub m_damageMultiplier: f32,
    pub m_breakableBody: *mut hkpBreakableBody,
    pub m_solverData: u32,
    pub m_storageIndex: u16,
    pub m_contactPointCallbackDelay: u16,
    pub m_constraintsMaster: hkSmallArray_hkConstraintInternal_,
    pub m_constraintsSlave: hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_,
    pub m_constraintRuntime: hkArray_unsigned_char_hkContainerHeapAllocator_,
    pub m_simulationIsland: *mut hkpSimulationIsland,
    pub m_autoRemoveLevel: i8,
    pub m_numShapeKeysInContactPointProperties: u8,
    pub m_responseModifierFlags: u8,
    pub m_uid: u32,
    pub m_spuCollisionCallback: hkpEntity__SpuCollisionCallback,
    __pdbindgen_padding: [u8; 4],
    pub m_motion: hkpMaxSizeMotion,
    pub m_contactListeners: hkSmallArray_hkpContactListener___,
    pub m_actions: hkSmallArray_hkpAction___,
    pub m_localFrame: hkRefPtr_hkLocalFrame_,
    pub m_extendedListeners: *mut hkpEntity__ExtendedListeners,
    pub m_npData: u32,
    __pdbindgen_padding_2: [u8; 4],
}

unsafe impl UpcastToNop<hkpWorldObject> for hkpEntity {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpEntity {}

unsafe impl UpcastToNop<hkBaseObject> for hkpEntity {}

impl hkpEntity {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setShape(
        &self,
        a1: *const hkpShape,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).setShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShape(
        &self,
        a1: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).updateShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMotionState(&self) -> *mut hkMotionState {
        ((*self.vfptr).getMotionState)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deallocateInternalArrays(&self) {
        ((*self.vfptr).deallocateInternalArrays)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpEntity__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpEntity, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpEntity),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpEntity) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpEntity),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpEntity,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpEntity,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState: unsafe extern "thiscall" fn(this: *mut hkpEntity) -> *mut hkMotionState,
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpEntity),
}

#[repr(C)]
pub struct hkEnum_enum_hkpMotion__MotionType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpMaterial {
    pub m_responseType: hkEnum_enum_hkpMaterial__ResponseType_signed_char_,
    pub m_rollingFrictionMultiplier: hkHalf,
    pub m_friction: f32,
    pub m_restitution: f32,
}

#[repr(C)]
pub struct hkEnum_enum_hkpConstraintInstance__InstanceType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpConstraintInstance {
    // hkBaseObject
    pub vfptr: *const hkpConstraintInstance__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintInstance
    pub m_owner: *mut hkpConstraintOwner,
    pub m_data: *mut hkpConstraintData,
    pub m_constraintModifiers: *mut hkpModifierConstraintAtom,
    pub m_entities: [*mut hkpEntity; 2],
    pub m_priority: hkEnum_enum_hkpConstraintInstance__ConstraintPriority_unsigned_char_,
    pub m_wantRuntime: hkBool,
    pub m_destructionRemapInfo:
        hkEnum_enum_hkpConstraintInstance__OnDestructionRemapInfo_unsigned_char_,
    pub m_listeners: hkSmallArray_hkpConstraintListener___,
    pub m_name: hkStringPtr,
    pub m_userData: u32,
    pub m_internal: *mut hkConstraintInternal,
    pub m_uid: u32,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConstraintInstance {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConstraintInstance {}

impl hkpConstraintInstance {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn entityAddedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityRemovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityDeletedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityDeletedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpConstraintInstance__InstanceType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpConstraintInstance__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintInstance, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintInstance),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConstraintInstance) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConstraintInstance),
    pub entityAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintInstance, _: *mut hkpEntity),
    pub entityRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintInstance, _: *mut hkpEntity),
    pub entityDeletedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintInstance, _: *mut hkpEntity),
    pub getType: unsafe extern "thiscall" fn(
        this: *const hkpConstraintInstance,
    ) -> hkpConstraintInstance__InstanceType,
}

#[repr(C)]
pub struct hkpShapeBase {
    // hkBaseObject
    pub vfptr: *const hkpShapeBase__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkcdShape
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    // hkpShapeBase
}

unsafe impl UpcastToNop<hkcdShape> for hkpShapeBase {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpShapeBase {}

unsafe impl UpcastToNop<hkBaseObject> for hkpShapeBase {}

impl hkpShapeBase {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isConvex(&self) -> bool {
        ((*self.vfptr).isConvex)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAabb(
        &self,
        a1: *const hkTransformf,
        a2: f32,
        a3: *mut hkAabb,
    ) {
        ((*self.vfptr).getAabb)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRay(
        &self,
        result: *mut hkBool,
        a2: *const hkpShapeRayCastInput,
        a3: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool {
        ((*self.vfptr).castRay)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayWithCollector(
        &self,
        a1: *const hkpShapeRayCastInput,
        a2: *const hkpCdBody,
        a3: *mut hkpRayHitCollector,
    ) {
        ((*self.vfptr).castRayWithCollector)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayBundle(
        &self,
        result: *mut hkVector4fComparison,
        a2: *const hkpShapeRayBundleCastInput,
        a3: *mut hkpShapeRayBundleCastOutput,
        a4: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison {
        ((*self.vfptr).castRayBundle)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSupportingVertex(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkcdVertex,
    ) {
        ((*self.vfptr).getSupportingVertex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertVertexIdsToVertices(
        &self,
        a1: *const u16,
        a2: i32,
        a3: *mut hkcdVertex,
    ) {
        ((*self.vfptr).convertVertexIdsToVertices)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCentre(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getCentre)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumCollisionSpheres(&self) -> i32 {
        ((*self.vfptr).getNumCollisionSpheres)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCollisionSpheres(
        &self,
        a1: *mut hkSphere,
    ) -> *const hkSphere {
        ((*self.vfptr).getCollisionSpheres)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn weldContactPoint(
        &self,
        a1: *mut u16,
        a2: *mut u8,
        a3: *mut hkVector4f,
        a4: *const hkTransformf,
        a5: *const hkpConvexShape,
        a6: *const hkTransformf,
        a7: *mut hkVector4f,
    ) -> i32 {
        ((*self.vfptr).weldContactPoint)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6, a7)
    }
}

#[repr(C)]
pub struct hkpShapeBase__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpShapeBase, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpShapeBase),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpShapeBase),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpShapeBase, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> i32,
    pub getCollisionSpheres:
        unsafe extern "thiscall" fn(this: *const hkpShapeBase, _: *mut hkSphere) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
}

#[repr(C)]
pub struct hkpMotion {
    // hkBaseObject
    pub vfptr: *const hkpMotion__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpMotion
    pub m_type: hkEnum_enum_hkpMotion__MotionType_unsigned_char_,
    pub m_deactivationIntegrateCounter: u8,
    pub m_deactivationNumInactiveFrames: [u16; 2],
    pub m_motionState: hkMotionState,
    pub m_inertiaAndMassInv: hkVector4f,
    pub m_linearVelocity: hkVector4f,
    pub m_angularVelocity: hkVector4f,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_deactivationRefPosition: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 32],
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpMotion {}

unsafe impl UpcastToNop<hkBaseObject> for hkpMotion {}

impl hkpMotion {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setMass(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMass)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMass_2(&self, a1: f32) {
        ((*self.vfptr).setMass_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMassInv)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv_2(&self, a1: f32) {
        ((*self.vfptr).setMassInv_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaInvLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setCenterOfMassInLocal(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setCenterOfMassInLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: *const hkQuaternionf) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPositionAndRotation(
        &self,
        a1: *const hkVector4f,
        a2: *const hkQuaternionf,
    ) {
        ((*self.vfptr).setPositionAndRotation)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setTransform(&self, a1: *const hkTransformf) {
        ((*self.vfptr).setTransform)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLinearVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setLinearVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setAngularVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setAngularVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocity(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut f32,
        a4: *mut f32,
    ) {
        ((*self.vfptr).getProjectedPointVelocity)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocitySimd(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut hkSimdFloat32,
        a4: *mut hkSimdFloat32,
    ) {
        ((*self.vfptr).getProjectedPointVelocitySimd)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn applyLinearImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyLinearImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyPointImpulse(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
    ) {
        ((*self.vfptr).applyPointImpulse)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyAngularImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyAngularImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyForce(
        &self,
        a1: f32,
        a2: *const hkVector4f,
        a3: *const hkVector4f,
    ) {
        ((*self.vfptr).applyForce)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn applyForce_2(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyForce_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyTorque(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyTorque)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMotionStateAndVelocitiesAndDeactivationType(
        &self,
        a1: *mut hkpMotion,
    ) {
        ((*self.vfptr).getMotionStateAndVelocitiesAndDeactivationType)(
            self as *const _ as *mut _,
            a1,
        )
    }
}

#[repr(C)]
pub struct hkpMotion__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpMotion),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpMotion) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpMotion),
    pub setMass: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkSimdFloat32),
    pub setMass_2: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: f32),
    pub setMassInv: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkSimdFloat32),
    pub setMassInv_2: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: f32),
    pub getInertiaLocal: unsafe extern "thiscall" fn(this: *const hkpMotion, _: *mut hkMatrix3f),
    pub getInertiaWorld: unsafe extern "thiscall" fn(this: *const hkpMotion, _: *mut hkMatrix3f),
    pub setInertiaLocal: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkMatrix3f),
    pub setInertiaInvLocal: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkMatrix3f),
    pub getInertiaInvLocal: unsafe extern "thiscall" fn(this: *const hkpMotion, _: *mut hkMatrix3f),
    pub getInertiaInvWorld: unsafe extern "thiscall" fn(this: *const hkpMotion, _: *mut hkMatrix3f),
    pub setCenterOfMassInLocal:
        unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub setPosition: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub setRotation: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkQuaternionf),
    pub setPositionAndRotation: unsafe extern "thiscall" fn(
        this: *mut hkpMotion,
        _: *const hkVector4f,
        _: *const hkQuaternionf,
    ),
    pub setTransform: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkTransformf),
    pub setLinearVelocity: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub setAngularVelocity: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub getProjectedPointVelocity: unsafe extern "thiscall" fn(
        this: *const hkpMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut f32,
        _: *mut f32,
    ),
    pub getProjectedPointVelocitySimd: unsafe extern "thiscall" fn(
        this: *const hkpMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut hkSimdFloat32,
        _: *mut hkSimdFloat32,
    ),
    pub applyLinearImpulse: unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub applyPointImpulse: unsafe extern "thiscall" fn(
        this: *mut hkpMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyAngularImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *const hkVector4f),
    pub applyForce: unsafe extern "thiscall" fn(
        this: *mut hkpMotion,
        _: f32,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyForce_2:
        unsafe extern "thiscall" fn(this: *mut hkpMotion, _: f32, _: *const hkVector4f),
    pub applyTorque:
        unsafe extern "thiscall" fn(this: *mut hkpMotion, _: f32, _: *const hkVector4f),
    pub getMotionStateAndVelocitiesAndDeactivationType:
        unsafe extern "thiscall" fn(this: *mut hkpMotion, _: *mut hkpMotion),
}

#[repr(C)]
pub struct hkEnum_enum_hkpConstraintInstance__OnDestructionRemapInfo_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkEnum_enum_hkpMaterial__ResponseType_signed_char_ {
    pub m_storage: i8,
}

#[repr(C)]
pub struct hkpWorldObject {
    // hkBaseObject
    pub vfptr: *const hkpWorldObject__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldObject
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpWorldObject {}

unsafe impl UpcastToNop<hkBaseObject> for hkpWorldObject {}

impl hkpWorldObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setShape(
        &self,
        a1: *const hkpShape,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).setShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShape(
        &self,
        a1: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result {
        ((*self.vfptr).updateShape)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMotionState(&self) -> *mut hkMotionState {
        ((*self.vfptr).getMotionState)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpWorldObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpWorldObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpWorldObject),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpWorldObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpWorldObject),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpWorldObject,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpWorldObject,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState:
        unsafe extern "thiscall" fn(this: *mut hkpWorldObject) -> *mut hkMotionState,
}

#[repr(C)]
pub struct hkEnum_enum_hkpConstraintInstance__ConstraintPriority_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpShape {
    // hkBaseObject
    pub vfptr: *const hkpShape__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkcdShape
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    // hkpShapeBase
    // hkpShape
    pub m_userData: u32,
}

unsafe impl UpcastToNop<hkpShapeBase> for hkpShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpShape {}

impl hkpShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getClassType(&self) -> *const hkClass {
        ((*self.vfptr).getClassType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deleteThisReferencedObject(&self) {
        ((*self.vfptr).deleteThisReferencedObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isConvex(&self) -> bool {
        ((*self.vfptr).isConvex)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAabb(
        &self,
        a1: *const hkTransformf,
        a2: f32,
        a3: *mut hkAabb,
    ) {
        ((*self.vfptr).getAabb)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRay(
        &self,
        result: *mut hkBool,
        a2: *const hkpShapeRayCastInput,
        a3: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool {
        ((*self.vfptr).castRay)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayWithCollector(
        &self,
        a1: *const hkpShapeRayCastInput,
        a2: *const hkpCdBody,
        a3: *mut hkpRayHitCollector,
    ) {
        ((*self.vfptr).castRayWithCollector)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn castRayBundle(
        &self,
        result: *mut hkVector4fComparison,
        a2: *const hkpShapeRayBundleCastInput,
        a3: *mut hkpShapeRayBundleCastOutput,
        a4: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison {
        ((*self.vfptr).castRayBundle)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSupportingVertex(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkcdVertex,
    ) {
        ((*self.vfptr).getSupportingVertex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertVertexIdsToVertices(
        &self,
        a1: *const u16,
        a2: i32,
        a3: *mut hkcdVertex,
    ) {
        ((*self.vfptr).convertVertexIdsToVertices)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCentre(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getCentre)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumCollisionSpheres(&self) -> i32 {
        ((*self.vfptr).getNumCollisionSpheres)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCollisionSpheres(
        &self,
        a1: *mut hkSphere,
    ) -> *const hkSphere {
        ((*self.vfptr).getCollisionSpheres)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn weldContactPoint(
        &self,
        a1: *mut u16,
        a2: *mut u8,
        a3: *mut hkVector4f,
        a4: *const hkTransformf,
        a5: *const hkpConvexShape,
        a6: *const hkTransformf,
        a7: *mut hkVector4f,
    ) -> i32 {
        ((*self.vfptr).weldContactPoint)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6, a7)
    }

    pub unsafe extern "thiscall" fn getContainer(&self) -> *const hkpShapeContainer {
        ((*self.vfptr).getContainer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaximumProjection(&self, a1: *const hkVector4f) -> f32 {
        ((*self.vfptr).getMaximumProjection)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn calcSizeForSpu(
        &self,
        a1: *const hkpShape__CalcSizeForSpuInput,
        a2: i32,
    ) -> i32 {
        ((*self.vfptr).calcSizeForSpu)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpShape) -> i32,
    pub getCollisionSpheres:
        unsafe extern "thiscall" fn(this: *const hkpShape, _: *mut hkSphere) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
}

pub type keen__FileAccessMode = i32;

pub type keen__ProjectionType = i32;

pub type kaiko__LocalGameSessionInteractionId = i32;

pub type keen__PixelFormat = i32;

pub type keen__IoError = i32;

pub type DXGI_FORMAT = i32;

pub type D3D_NAME = i32;

pub type D3D_REGISTER_COMPONENT_TYPE = i32;

pub type keen__PrimitiveType = i32;

pub type keen__graphics__WindowMode = i32;

pub type keen__SaveData__OperationStatus = i32;

pub type keen__SaveData__OperationResult = i32;

pub type keen__SaveData__CurrentOperation = i32;

pub type keen__SaveData__SaveDataSystemType = i32;

pub type keen__SaveFlowState = i32;

pub type keen__SaveDataHandler__SaveGameClientState = i32;

pub type keen__ControllerType = i32;

pub type keen__ControllerClass = i32;

pub type keen__InputSystemControllerAutoCatchType = i32;

pub type gfc__Stream__StreamType = i32;

pub type gfc__ImageType = i32;

pub type gfc__ImageFormat = i32;

pub type gfc__Texture__Type = i32;

pub type gfc__Mesh__Type = i32;

pub type gfc__Darksiders__LoadState = i32;

pub type gfc__WorldManager__WorldState = i32;

pub type unit4__LocalGameSession__InteractionStarter = i32;

pub type unit4__LocalGameSession__LocalUserState = i32;

pub type hkpCollidableAccept = i32;

pub type hkResultEnum = i32;

pub type hkpContactPointAccept = i32;

pub type hkJobType = i32;

pub type hkpContactMgr__ToiAccept = i32;

pub type hkpContactMgr__Type = i32;

pub type hkpCollisionEvent__CallbackSource = i32;

pub type hkpConvexListFilter__ConvexListCollisionType = i32;

pub type hkpWorldCinfo__BroadPhaseBorderBehaviour = i32;

pub type hkJobQueue__JobCreationStatus = i32;

pub type hkJobQueue__JobPopFuncResult = i32;

pub type hkJobQueue__WaitPolicy = i32;

pub type hkpConstraintAtom__SolvingMethod = i32;

pub type hkpContactPointEvent__Type = i32;

pub type gfc__StateMapValue__ValueMode = i32;

pub type gfc__SceneObject__Type = i32;

pub type hkpConstraintInstance__InstanceType = i32;

pub type hkpConstraintInstance__ConstraintPriority = i32;

pub type hkcdShapeType__ShapeTypeEnum = i32;

pub type hkJobQueueHwSetup__SpuSchedulePolicy = i32;

pub type hkJobQueueHwSetup__CellRules = i32;

pub type hkpConstraintData__UpdateAtomsResult__Enum = i32;

pub type hkpBroadPhase__Capabilities = i32;

pub type hkpBroadPhase__BroadPhaseType = i32;

pub type unit4__RankingListQueryType = i32;

pub type unit4__RankingError = i32;

pub type EResult = i32;

pub type ELeaderboardDataRequest = i32;

pub type ELeaderboardUploadScoreMethod = i32;

pub type unit4__SystemServices__ReceiveRankingStep = i32;

pub type unit4__SystemServices__SendRankingStep = i32;

pub type unit4__LocalPlayerType = i32;

pub type DXGI_MODE_SCANLINE_ORDER = i32;

pub type DXGI_MODE_SCALING = i32;

pub type DXGI_SWAP_EFFECT = i32;

pub type keen__ImmediateCullMode = i32;

pub type keen__ImmediateFillMode = i32;

pub type keen__ScreenCaptureState = i32;

pub type gfc__MoveInput__DirectionHemispheres = i32;

pub type gfc__MoveInput__DirectionQuad = i32;

pub type gfc__Weapon__LoadingState = i32;

pub type hkpPhantomType = i32;

pub type hkpStepResult = i32;

pub type hkWorldOperation__Result = i32;

pub type hkpBodyOperation__ExecutionState = i32;

pub type hkpSimulation__ResetCollisionInformation = i32;

pub type hkpSimulation__FindContacts = i32;

pub type hkpToiResourceMgrResponse = i32;

#[repr(C)]
pub struct keen__LocalPlayerIdStructureType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct gfc__OblivionGameDebug {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkSpuCollisionCallbackUtil {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkdWorld {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hknpWorld {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkSpuThreadPool {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkpPhysicsContext {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkVisualDebugger {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkpConstraintRuntime {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkDefaultTaskQueue {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hkArrayBase_hkTimerData_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ELeaderboardSortMethod {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ELeaderboardDisplayType {
    _opaque: [u8; 0],
}
