#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types::*, types2::*, types3::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

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
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_contactMgrFactory: compile_error!("unimplemented feature: sizeof array 0x0"),
    __pdbindgen_padding: [u8; 260],
    pub m_hasAlternateType: [u32; 35],
    pub m_numAgent2Types: i32,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent2Types: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent2TypesPred: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_agent2Func: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 3732],
    pub m_numAgent3Types: i32,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent3Types: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent3TypesPred: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_agent3Func: compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_collisionQualityTable: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_collisionQualityInfo: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_3: [u8; 4248],
    pub m_collisionAgentRegistered: hkBool,
    pub m_agent3Registered: hkBool,
    pub m_midphaseAgent3Registered: hkBool,
    pub m_checkEnabled: hkBool,
    pub m_shapeInheritance:
        hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_debugAgent2Table: *mut compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_debugAgent2TablePred: *mut compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_debugAgent3Table: *mut compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_debugAgent3TablePred: *mut compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_contactPoints: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_data: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct hkpShapeRayBundleCastOutput {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_outputs: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_deactivationRefPosition: compile_error!("unimplemented feature: class layout 0x0"),
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
