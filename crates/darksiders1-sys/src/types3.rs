#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types::*, types2::*, types4::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

#[repr(C)]
pub struct hkVariant {
    pub m_object: *mut (),
    pub m_class: *const hkClass,
}

#[repr(C)]
pub struct hkArrayBase_hkSimpleProperty_ {
    pub m_data: *mut hkSimpleProperty,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpContactPointProperties {
    // hkpSolverResults
    pub m_impulseApplied: f32,
    pub m_internalSolverData: f32,
    // hkContactPointMaterial
    pub m_userData: u32,
    pub m_friction: hkUFloat8,
    pub m_restitution: u8,
    pub m_maxImpulse: hkUFloat8,
    pub m_flags: u8,
    // hkpContactPointProperties
    pub m_internalDataA: f32,
}

unsafe impl UpcastToNop<hkpSolverResults> for hkpContactPointProperties {}

unsafe impl UpcastTo<hkContactPointMaterial> for hkpContactPointProperties {
    fn upcast_to(p: *const Self) -> *const hkContactPointMaterial {
        (p as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct hkArrayBase_int_ {
    pub m_data: *mut i32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpConstraintData__RuntimeInfo {
    pub m_sizeOfExternalRuntime: i32,
    pub m_numSolverResults: i32,
}

#[repr(C)]
pub struct hkpConstraintData__ConstraintInfo {
    // hkpConstraintInfo
    pub m_maxSizeOfSchema: i32,
    pub m_sizeOfSchemas: i32,
    pub m_numSolverResults: i32,
    pub m_numSolverElemTemps: i32,
    // hkpConstraintData__ConstraintInfo
    pub m_atoms: *mut hkpConstraintAtom,
    pub m_sizeOfAllAtoms: u32,
    pub m_extraSchemaSize: u32,
}

unsafe impl UpcastToNop<hkpConstraintInfo> for hkpConstraintData__ConstraintInfo {}

#[repr(C)]
pub struct hkArrayBase_hkStackTracer__CallTree__Node_ {
    pub m_data: *mut hkStackTracer__CallTree__Node,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkWorldMemoryAvailableWatchDog {
    // hkBaseObject
    pub vfptr: *const hkWorldMemoryAvailableWatchDog__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkWorldMemoryAvailableWatchDog
}

unsafe impl UpcastToNop<hkReferencedObject> for hkWorldMemoryAvailableWatchDog {}

unsafe impl UpcastToNop<hkBaseObject> for hkWorldMemoryAvailableWatchDog {}

impl hkWorldMemoryAvailableWatchDog {
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

    pub unsafe extern "thiscall" fn getAmountOfFreeHeapMemoryRequested(&self) -> i32 {
        ((*self.vfptr).getAmountOfFreeHeapMemoryRequested)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn freeHeapMemoryTillRequestedAmountIsAvailable(
        &self,
        a1: *mut hkpWorld,
    ) {
        ((*self.vfptr).freeHeapMemoryTillRequestedAmountIsAvailable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reduceConstraintsInIsland(
        &self,
        a1: *const hkWorldMemoryAvailableWatchDog__MemUsageInfo,
        a2: i32,
    ) {
        ((*self.vfptr).reduceConstraintsInIsland)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkWorldMemoryAvailableWatchDog__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkWorldMemoryAvailableWatchDog, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkWorldMemoryAvailableWatchDog),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkWorldMemoryAvailableWatchDog) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkWorldMemoryAvailableWatchDog),
    pub getAmountOfFreeHeapMemoryRequested:
        unsafe extern "thiscall" fn(this: *mut hkWorldMemoryAvailableWatchDog) -> i32,
    pub freeHeapMemoryTillRequestedAmountIsAvailable:
        unsafe extern "thiscall" fn(this: *mut hkWorldMemoryAvailableWatchDog, _: *mut hkpWorld),
    pub reduceConstraintsInIsland: unsafe extern "thiscall" fn(
        this: *mut hkWorldMemoryAvailableWatchDog,
        _: *const hkWorldMemoryAvailableWatchDog__MemUsageInfo,
        _: i32,
    ),
}

#[repr(C)]
pub struct hkWorldMemoryAvailableWatchDog__MemUsageInfo {
    pub m_maxRuntimeBlockSize: i32,
    pub m_sumRuntimeBlockSize: i32,
    pub m_largestSimulationIsland: *mut hkpSimulationIsland,
}

#[repr(C)]
pub struct hkMemoryAllocator__ExtendedInterface {
    pub vfptr: *const hkMemoryAllocator__ExtendedInterface__vftable,
}

impl hkMemoryAllocator__ExtendedInterface {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn garbageCollect(&self) {
        ((*self.vfptr).garbageCollect)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn incrementalGarbageCollect(&self, a1: i32) {
        ((*self.vfptr).incrementalGarbageCollect)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMemorySoftLimit(
        &self,
        result: *mut hkResult,
        a2: u32,
    ) -> *mut hkResult {
        ((*self.vfptr).setMemorySoftLimit)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getMemorySoftLimit(&self) -> u32 {
        ((*self.vfptr).getMemorySoftLimit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn canAllocTotal(&self, a1: i32) -> bool {
        ((*self.vfptr).canAllocTotal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn walkMemory(
        &self,
        result: *mut hkResult,
        a2: *mut unsafe extern "C" fn(_: *mut (), _: u32, _: bool, _: i32, _: *mut ()),
        a3: *mut (),
    ) -> *mut hkResult {
        ((*self.vfptr).walkMemory)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn getApproxTotalAllocated(&self) -> u32 {
        ((*self.vfptr).getApproxTotalAllocated)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setScrubValues(&self, a1: u32, a2: u32) {
        ((*self.vfptr).setScrubValues)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn addToSnapshot(
        &self,
        a1: *mut hkMemorySnapshot,
        a2: i32,
    ) -> i32 {
        ((*self.vfptr).addToSnapshot)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkMemoryAllocator__ExtendedInterface__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: u32,
    ) -> *mut (),
    pub garbageCollect:
        unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator__ExtendedInterface),
    pub incrementalGarbageCollect:
        unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator__ExtendedInterface, _: i32),
    pub setMemorySoftLimit: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        result: *mut hkResult,
        _: u32,
    ) -> *mut hkResult,
    pub getMemorySoftLimit:
        unsafe extern "thiscall" fn(this: *const hkMemoryAllocator__ExtendedInterface) -> u32,
    pub canAllocTotal: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: i32,
    ) -> bool,
    pub walkMemory: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        result: *mut hkResult,
        _: *mut unsafe extern "C" fn(_: *mut (), _: u32, _: bool, _: i32, _: *mut ()),
        _: *mut (),
    ) -> *mut hkResult,
    pub getApproxTotalAllocated:
        unsafe extern "thiscall" fn(this: *const hkMemoryAllocator__ExtendedInterface) -> u32,
    pub setScrubValues: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: u32,
        _: u32,
    ),
    pub addToSnapshot: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: *mut hkMemorySnapshot,
        _: i32,
    ) -> i32,
}

#[repr(C)]
pub struct hkArray_hkpAgentNnSector___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpAgentNnSector___
    pub m_data: *mut *mut hkpAgentNnSector,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpAgentNnSector___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpAgentNnSector___>
    for hkArray_hkpAgentNnSector___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpEntityListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpEntityListener___
    pub m_data: *mut *mut hkpEntityListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpEntityListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpEntityListener___>
    for hkArray_hkpEntityListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkLocalFrameGroup {
    // hkBaseObject
    pub vfptr: *const hkLocalFrameGroup__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkLocalFrameGroup
    pub m_name: hkStringPtr,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkLocalFrameGroup {}

unsafe impl UpcastToNop<hkBaseObject> for hkLocalFrameGroup {}

impl hkLocalFrameGroup {
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
pub struct hkLocalFrameGroup__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkLocalFrameGroup, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkLocalFrameGroup),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkLocalFrameGroup) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkLocalFrameGroup),
}

#[repr(C)]
pub struct hkArray_hkpSimulationIsland___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpSimulationIsland___
    pub m_data: *mut *mut hkpSimulationIsland,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpSimulationIsland___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpSimulationIsland___>
    for hkArray_hkpSimulationIsland___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkLocalFrame {
    // hkBaseObject
    pub vfptr: *const hkLocalFrame__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkLocalFrame
}

unsafe impl UpcastToNop<hkReferencedObject> for hkLocalFrame {}

unsafe impl UpcastToNop<hkBaseObject> for hkLocalFrame {}

impl hkLocalFrame {
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

    pub unsafe extern "thiscall" fn getLocalTransform(&self, a1: *mut hkTransformf) {
        ((*self.vfptr).getLocalTransform)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocalTransform(&self, a1: *const hkTransformf) {
        ((*self.vfptr).setLocalTransform)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLocalPosition(&self, a1: *mut hkVector4f) {
        ((*self.vfptr).getLocalPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNearbyFrames(
        &self,
        a1: *const hkVector4f,
        a2: f32,
        a3: *mut hkLocalFrameCollector,
    ) {
        ((*self.vfptr).getNearbyFrames)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const i8 {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getParentFrame(&self) -> *const hkLocalFrame {
        ((*self.vfptr).getParentFrame)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setParentFrame(&self, a1: *const hkLocalFrame) {
        ((*self.vfptr).setParentFrame)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumChildFrames(&self) -> i32 {
        ((*self.vfptr).getNumChildFrames)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getChildFrame(&self, a1: i32) -> *mut hkLocalFrame {
        ((*self.vfptr).getChildFrame)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *const hkLocalFrameGroup {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setGroup(&self, a1: *const hkLocalFrameGroup) {
        ((*self.vfptr).setGroup)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDescendants(
        &self,
        a1: *mut hkArrayBase_hkLocalFrame_const___,
        a2: *mut hkMemoryAllocator,
    ) {
        ((*self.vfptr).getDescendants)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkLocalFrame__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkLocalFrame, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkLocalFrame),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkLocalFrame) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkLocalFrame),
    pub getLocalTransform:
        unsafe extern "thiscall" fn(this: *const hkLocalFrame, _: *mut hkTransformf),
    pub setLocalTransform:
        unsafe extern "thiscall" fn(this: *mut hkLocalFrame, _: *const hkTransformf),
    pub getLocalPosition:
        unsafe extern "thiscall" fn(this: *const hkLocalFrame, _: *mut hkVector4f),
    pub getNearbyFrames: unsafe extern "thiscall" fn(
        this: *const hkLocalFrame,
        _: *const hkVector4f,
        _: f32,
        _: *mut hkLocalFrameCollector,
    ),
    pub getName: unsafe extern "thiscall" fn(this: *const hkLocalFrame) -> *const i8,
    pub getParentFrame:
        unsafe extern "thiscall" fn(this: *const hkLocalFrame) -> *const hkLocalFrame,
    pub setParentFrame:
        unsafe extern "thiscall" fn(this: *mut hkLocalFrame, _: *const hkLocalFrame),
    pub getNumChildFrames: unsafe extern "thiscall" fn(this: *const hkLocalFrame) -> i32,
    pub getChildFrame:
        unsafe extern "thiscall" fn(this: *const hkLocalFrame, _: i32) -> *mut hkLocalFrame,
    pub getGroup:
        unsafe extern "thiscall" fn(this: *const hkLocalFrame) -> *const hkLocalFrameGroup,
    pub setGroup: unsafe extern "thiscall" fn(this: *mut hkLocalFrame, _: *const hkLocalFrameGroup),
    pub getDescendants: unsafe extern "thiscall" fn(
        this: *const hkLocalFrame,
        _: *mut hkArrayBase_hkLocalFrame_const___,
        _: *mut hkMemoryAllocator,
    ),
}

#[repr(C)]
pub struct hkpContactProcessEvent {
    pub m_collidableA: *const hkpCollidable,
    pub m_collidableB: *const hkpCollidable,
    pub m_callbackFiredFrom: *mut hkpEntity,
    pub m_collisionData: *mut hkpProcessCollisionData,
    pub m_contactPointProperties: [*mut hkpContactPointProperties; 256],
    pub m_internalContactMgr: *mut hkpDynamicsContactMgr,
}

#[repr(C)]
pub struct hkpConstraintQueryIn {
    // hkpConstraintQueryStepInfo
    pub m_subStepDeltaTime: hkPadSpu_float_,
    pub m_microStepDeltaTime: hkPadSpu_float_,
    pub m_subStepInvDeltaTime: hkPadSpu_float_,
    pub m_frameDeltaTime: hkPadSpu_float_,
    pub m_frameInvDeltaTime: hkPadSpu_float_,
    pub m_invNumSteps: hkPadSpu_float_,
    pub m_invNumStepsTimesMicroSteps: hkPadSpu_float_,
    pub m_maxConstraintViolationSqrd: hkPadSpu_float_,
    pub m_rhsFactor: hkPadSpu_float_,
    pub m_virtMassFactor: hkPadSpu_float_,
    pub m_frictionRhsFactor: hkPadSpu_float_,
    // hkpConstraintQueryIn
    __pdbindgen_padding: [u8; 4],
    pub m_bodyA: hkPadSpu_hkpVelocityAccumulator_const___,
    pub m_bodyB: hkPadSpu_hkpVelocityAccumulator_const___,
    pub m_transformA: hkPadSpu_hkTransformf_const___,
    pub m_transformB: hkPadSpu_hkTransformf_const___,
    pub m_tau: hkPadSpu_float_,
    pub m_damping: hkPadSpu_float_,
    __pdbindgen_padding_2: [u8; 8],
    pub m_maxConstraintViolationSqrd_2: hkSimdFloat32,
    pub m_constraintInstance: hkPadSpu_hkpConstraintInstance___,
    pub m_violatedConstraints: hkPadSpu_hkpViolatedConstraintArray___,
    pub m_accumulatorAIndex: hkPadSpu_unsigned_int_,
    pub m_accumulatorBIndex: hkPadSpu_unsigned_int_,
    pub m_beginConstraints: *mut unsafe extern "C" fn(
        _: *const hkpConstraintQueryIn,
        _: *mut hkpConstraintQueryOut,
        _: *mut hkpSolverResults,
        _: i32,
    ),
    __pdbindgen_padding_3: [u8; 12],
}

unsafe impl UpcastToNop<hkpConstraintQueryStepInfo> for hkpConstraintQueryIn {}

#[repr(C)]
pub struct hkpShape__CalcSizeForSpuInput {
    pub m_midphaseAgent3Registered: bool,
    pub m_isFixedOrKeyframed: bool,
    pub m_hasDynamicMotionSaved: bool,
}

#[repr(C)]
pub struct hkBaseObject {
    pub vfptr: *const hkBaseObject__vftable,
}

impl hkBaseObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __first_virtual_table_function__(&self) {
        ((*self.vfptr).__first_virtual_table_function__)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkBaseObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Value___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Value_,
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Value___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Value_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Value___>
    for std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value___
{
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct List_gfc__AutoRef_gfc__WorldObject_____ListNode {
    pub next: *mut List_gfc__AutoRef_gfc__WorldObject_____ListNode,
    pub value: gfc__AutoRef_gfc__WorldObject_,
}

#[repr(C)]
pub struct gfc__StaticLightingVisualOpt {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticLightingVisualOpt__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__StaticLightingVisualOpt
    pub mLightTable: *mut gfc__VertexBuffer,
    pub mUVTable: *mut gfc__VertexBuffer,
    pub mColorMap: *mut gfc__Texture,
    pub mDirMap: *mut gfc__Texture,
    pub mAOMap: *mut gfc__Texture,
    pub mColorMapID: u8,
    pub mDirMapID: u8,
    pub mAOMapID: u8,
    pub mUseLightMaps: bool,
    pub mID: i32,
    pub mChecksum: u32,
    pub mLightTableBase: u32,
    pub mUVTableBase: u32,
    pub mColorMapMult: f32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__StaticLightingVisualOpt {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticLightingVisualOpt {}

impl gfc__StaticLightingVisualOpt {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__StaticLightingVisualOpt__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingVisualOpt, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingVisualOpt) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingVisualOpt,
        _: *const gfc__HString,
    ),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingVisualOpt) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingVisualOpt) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingVisualOpt,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingVisualOpt,
    ) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingVisualOpt,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingVisualOpt,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__CollisionObject {
    pub vfptr: *const gfc__CollisionObject__vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
}

impl gfc__CollisionObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getShape(
        &self,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_ {
        ((*self.vfptr).getShape)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getMatrix(&self, a1: *mut gfc__Matrix4) {
        ((*self.vfptr).getMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroupContext(&self) -> *mut () {
        ((*self.vfptr).getGroupContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CollisionObject__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CollisionObject, _: u32) -> *mut (),
    pub getShape: unsafe extern "thiscall" fn(
        this: *const gfc__CollisionObject,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_,
    pub getMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__CollisionObject, _: *mut gfc__Matrix4),
    pub getContext:
        unsafe extern "thiscall" fn(this: *mut gfc__CollisionObject) -> *mut gfc__Object,
    pub getGroupContext: unsafe extern "thiscall" fn(this: *mut gfc__CollisionObject) -> *mut (),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CollisionObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Visual__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Visual_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__StaticLightingObjectOpt {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticLightingObjectOpt__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__StaticLightingObjectOpt
    pub mLayerID: u32,
    pub mObjectID: u32,
    pub mDataElementCount: u32,
    pub mData: *mut *mut gfc__StaticLightingVisualOpt,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__StaticLightingObjectOpt {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticLightingObjectOpt {}

impl gfc__StaticLightingObjectOpt {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__StaticLightingObjectOpt__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingObjectOpt, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingObjectOpt) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingObjectOpt,
        _: *const gfc__HString,
    ),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingObjectOpt) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingObjectOpt) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingObjectOpt,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingObjectOpt,
    ) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingObjectOpt,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingObjectOpt,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Body__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Body_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Body_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ParticleController_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__RegionLayer {
    // gfc__IRefObject
    pub vfptr: *const gfc__RegionLayer__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__RegionLayer
    pub mID: i32,
    pub mName: gfc__HString,
    pub mRoot: gfc__AutoRef_gfc__WorldGroup_,
    pub mPackageIDs: gfc__Vector_int_0_gfc__CAllocator_,
    pub mPackageNamesDeprecated: gfc__Vector_gfc__String_0_gfc__CAllocator_,
    pub mNextObjectID: u32,
    pub mExternallyModified: bool,
    pub mWorld: *mut gfc__World,
    pub mHide: bool,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__RegionLayer {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__RegionLayer {}

impl gfc__RegionLayer {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__RegionLayer__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__RegionLayer, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__RegionLayer) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__RegionLayer, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__RegionLayer) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__RegionLayer) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__RegionLayer,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__RegionLayer) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__RegionLayer, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__RegionLayer,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Object3DCache {
    // gfc__ResourceCache
    pub vfptr: *const gfc__Object3DCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__Object3DCache
}

unsafe impl UpcastToNop<gfc__ResourceCache> for gfc__Object3DCache {}

impl gfc__Object3DCache {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadDefaultResource(&self, a1: gfc__AutoRef_gfc__File_) {
        ((*self.vfptr).loadDefaultResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn initThread(&self) {
        ((*self.vfptr).initThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdownThread(&self) {
        ((*self.vfptr).shutdownThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn analyzeResource(
        &self,
        a1: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool {
        ((*self.vfptr).analyzeResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canCreateBuffersInThread(&self, a1: i32) -> bool {
        ((*self.vfptr).canCreateBuffersInThread)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createBuffers(&self, a1: *mut gfc__ResourceBufferInfo) {
        ((*self.vfptr).createBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeBuffers(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).freeBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadResource(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).loadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canReloadResources(&self) -> bool {
        ((*self.vfptr).canReloadResources)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadsQueued(&self) -> bool {
        ((*self.vfptr).reloadsQueued)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadResource(&self, a1: *mut gfc__ResourceLoadInfo) -> bool {
        ((*self.vfptr).reloadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn needUnlinkResource(&self) -> bool {
        ((*self.vfptr).needUnlinkResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unlinkResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).unlinkResource)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn needUnloadResource(&self) -> bool {
        ((*self.vfptr).needUnloadResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unloadResource(
        &self,
        a1: *mut (),
        a2: *mut gfc__ResourceLoadInfo,
    ) {
        ((*self.vfptr).unloadResource)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn freeResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).freeResource)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__Object3DCache__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3DCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__Object3DCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3DCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__Object3DCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__Object3DCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3DCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__Object3DCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3DCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__Object3DCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3DCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3DCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CShape_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AnimationController_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__IconGizmo {
    // gfc__SceneObject
    pub vfptr: *const gfc__IconGizmo__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__IconGizmo__vftable,
    pub mLocked: bool,
    // gfc__Gizmo
    pub mObject: *mut gfc__WorldObject,
    // gfc__IconGizmo
    pub mMaterial: gfc__AutoRef_gfc__Material_,
}

unsafe impl UpcastToNop<gfc__Gizmo> for gfc__IconGizmo {}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__IconGizmo {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__IconGizmo {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__IconGizmo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn startGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).startGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finishGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).finishGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn prepGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cullAndSubmit(
        &self,
        a1: *const gfc__Clipper,
        a2: *mut gfc__Camera3D,
        a3: u32,
    ) {
        ((*self.vfptr).cullAndSubmit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D, a2: bool, a3: u32) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submitHidden(&self, a1: *mut gfc__Camera3D, a2: u32) {
        ((*self.vfptr).submitHidden)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isHighPriority(&self) -> bool {
        ((*self.vfptr).isHighPriority)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn writeText(&self, a1: *mut gfc__AutoRef_gfc__OutputStream_) {
        ((*self.vfptr).writeText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIsSky(&self, a1: bool) {
        ((*self.vfptr).setIsSky)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getIsSky(&self) -> bool {
        ((*self.vfptr).getIsSky)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__IconGizmo__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden:
        unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: *mut gfc__Camera3D, _: u32),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__IconGizmo) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__IconGizmo) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__IconGizmo,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__IconGizmo) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__IconGizmo) -> bool,
}

#[repr(C)]
pub struct gfc__Object3D {
    // gfc__IRefObject
    pub vfptr: *const gfc__Object3D__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Object3D
    pub mWorld: *mut gfc__World,
    pub mWorldObject: *mut gfc__WorldObject,
    pub mSkeleton: gfc__AutoRef_gfc__Skeleton3D_,
    pub mVisuals: gfc__Vector_gfc__AutoRef_gfc__Visual__0_gfc__CAllocator_,
    pub mBodies: gfc__Vector_gfc__AutoRef_gfc__Body__0_gfc__CAllocator_,
    pub mConstraints: gfc__Vector_gfc__AutoRef_gfc__Constraint__0_gfc__CAllocator_,
    pub mRagdoll: gfc__AutoRef_gfc__RagdollMapper_,
    pub mLODDistances: gfc__Vector_float_0_gfc__CAllocator_,
    pub mLODFloor: u32,
    pub mAnimationController: gfc__AutoRef_gfc__AnimationController_,
    pub mParticleController: gfc__AutoRef_gfc__ParticleController_,
    pub mPackageID: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Object3D {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Object3D {}

impl gfc__Object3D {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Object3D__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object3D) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3D,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3D) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object3D, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object3D,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__ScenePortal {
    pub vfptr: *const gfc__ScenePortal__vftable,
    pub mCellAID: u32,
    pub mCellBID: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCellA: *mut gfc__SceneCell,
    pub mCellB: *mut gfc__SceneCell,
    pub mDrawCounter: u32,
}

impl gfc__ScenePortal {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isVisible(&self, a1: *mut gfc__Clipper) -> bool {
        ((*self.vfptr).isVisible)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addClippingPlanes(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *mut gfc__Clipper,
    ) {
        ((*self.vfptr).addClippingPlanes)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn testCollision(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: f32,
    ) -> bool {
        ((*self.vfptr).testCollision)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__ScenePortal__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal, _: u32) -> *mut (),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ScenePortal,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isVisible:
        unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal, _: *mut gfc__Clipper) -> bool,
    pub addClippingPlanes: unsafe extern "thiscall" fn(
        this: *mut gfc__ScenePortal,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__Clipper,
    ),
    pub testCollision: unsafe extern "thiscall" fn(
        this: *mut gfc__ScenePortal,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__ScenePortal) -> bool,
}

#[repr(C)]
pub struct gfc__StaticObject {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticObject__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__StaticObject
    pub mPackageName: gfc__HString,
    pub mObjectName: gfc__HString,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mScale: gfc__TVector3_float_gfc__FloatMath_,
    pub mAORayLength: i32,
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__StaticObject {}

unsafe impl UpcastToNop<gfc__Object> for gfc__StaticObject {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticObject {}

impl gfc__StaticObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__StaticObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__StaticObject) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__StaticObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__StaticObject),
}

#[repr(C)]
pub struct gfc__RigidBody {
    // gfc__IRefObject
    pub vfptr: *const gfc__RigidBody__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CollisionObject
    pub vfptr_2: *const gfc__RigidBody__vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
    // gfc__Body
    pub mObject: *mut gfc__Object3D,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mNodeName: gfc__HString,
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mElementData: gfc__AutoRef_gfc__Object_,
    pub mElementType: u16,
    pub mAutoActivate: bool,
    pub mBodyType: u8,
    // gfc__RigidBody
    __pdbindgen_padding: [u8; 4],
    pub mRootBody: gfc__AutoRef_gfc__RigidBody_,
    pub mRigidBody: *mut hkpRigidBody,
    pub mCachedKeyframe: *mut gfc__CachedKeyframe,
    pub mMass: f32,
    pub mFriction: f32,
    pub mRestitution: f32,
    pub mVolume: f32,
    pub mVolumeRatio: f32,
    pub mCenterOfMass: gfc__TVector3_float_gfc__FloatMath_,
    pub mInertiaTensor: gfc__Matrix3,
    pub mSystem: u16,
    pub mSubsystem: u8,
    pub mIgnoreSubsys: u8,
    pub mLastNodeVersion: i32,
    pub mHavokShape: *mut hkpShape,
    pub mMotionType: u8,
    pub mIsAdded: bool,
}

unsafe impl UpcastToNop<gfc__Body> for gfc__RigidBody {}

unsafe impl UpcastToNop<gfc__Object> for gfc__RigidBody {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__RigidBody {}

unsafe impl UpcastTo<gfc__CollisionObject> for gfc__RigidBody {
    fn upcast_to(p: *const Self) -> *const gfc__CollisionObject {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__RigidBody {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getShape(
        &self,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_ {
        ((*self.vfptr).getShape)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getMatrix(&self, a1: *mut gfc__Matrix4) {
        ((*self.vfptr).getMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroupContext(&self) -> *mut () {
        ((*self.vfptr).getGroupContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setBodyType(&self, a1: u8) {
        ((*self.vfptr).setBodyType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invalidateNode(&self) {
        ((*self.vfptr).invalidateNode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self, a1: i32) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn buildHavokRigidBody(&self) {
        ((*self.vfptr).buildHavokRigidBody)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__RigidBody__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: u32) -> *mut (),
    pub getShape: unsafe extern "thiscall" fn(
        this: *const gfc__RigidBody,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_,
    pub getMatrix: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: *mut gfc__Matrix4),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody) -> *mut gfc__Object,
    pub getGroupContext: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody) -> *mut (),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__RigidBody,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__RigidBody) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__RigidBody,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setBodyType: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: u8),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
    pub invalidateNode: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody, _: i32),
    pub buildHavokRigidBody: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
}

#[repr(C)]
pub struct gfc__StaticLightingRegionOpt {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticLightingRegionOpt__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__StaticLightingRegionOpt
    pub LightTable: gfc__AutoRef_gfc__VertexBuffer_,
    pub UVTable: gfc__AutoRef_gfc__VertexBuffer_,
    pub mDirty: bool,
    pub mInPlace: bool,
    pub mMapsElementCount: u32,
    pub mObjectsElementCount: u32,
    pub mMaps: *mut gfc__AutoRef_gfc__Texture_,
    pub mObjects: *mut *mut gfc__StaticLightingObjectOpt,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__StaticLightingRegionOpt {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticLightingRegionOpt {}

impl gfc__StaticLightingRegionOpt {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__StaticLightingRegionOpt__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingRegionOpt, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingRegionOpt) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingRegionOpt,
        _: *const gfc__HString,
    ),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__StaticLightingRegionOpt) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticLightingRegionOpt) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingRegionOpt,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingRegionOpt,
    ) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingRegionOpt,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticLightingRegionOpt,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__CShape {
    // gfc__IRefObject
    pub vfptr: *const gfc__CShape__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShape {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShape {}

impl gfc__CShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *const gfc__Matrix4,
        a2: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getBoundingBox_2(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collideRay(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut gfc__CInfo,
    ) -> bool {
        ((*self.vfptr).collideRay)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn isComposite(&self) -> bool {
        ((*self.vfptr).isComposite)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn containsType(&self, a1: i32) -> bool {
        ((*self.vfptr).containsType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryShapes(
        &self,
        a1: *const gfc__TBox_float_gfc__FloatMath_,
        a2: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).queryShapes)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createHavokShape(&self, a1: f32, a2: i32) -> *mut hkpShape {
        ((*self.vfptr).createHavokShape)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setMaterialID(&self, a1: i32) {
        ((*self.vfptr).setMaterialID)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CShape, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShape) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__CShape, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShape) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShape) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShape) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__CShape, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShape) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShape) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShape, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShape,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape:
        unsafe extern "thiscall" fn(this: *mut gfc__CShape, _: f32, _: i32) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShape, _: i32),
}

#[repr(C)]
pub struct gfc__SceneLight {
    pub vfptr: *const gfc__SceneLight__vftable,
    pub mDrawCounter: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mDynamicVis: bool,
    pub mVisible: bool,
}

impl gfc__SceneLight {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLightType(&self) -> u8 {
        ((*self.vfptr).getLightType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getBoundingVolume(&self, a1: *mut gfc__BoundingVolume) -> bool {
        ((*self.vfptr).getBoundingVolume)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVisType(&self) -> i32 {
        ((*self.vfptr).getVisType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVisSphere(
        &self,
        a1: *mut gfc__TSphere_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getVisSphere)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVisFrustum(&self, a1: *mut gfc__Frustum) {
        ((*self.vfptr).getVisFrustum)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__SceneLight__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight, _: u32) -> *mut (),
    pub getLightType: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight) -> u8,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneLight,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneLight,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub submit: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight, _: *mut gfc__Camera3D),
    pub getVisType: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight) -> i32,
    pub getVisSphere: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneLight,
        _: *mut gfc__TSphere_float_gfc__FloatMath_,
    ),
    pub getVisFrustum:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneLight, _: *mut gfc__Frustum),
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__SceneLight) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight) -> bool,
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneLight) -> *mut gfc__Object,
}

#[repr(C)]
pub struct gfc__Body {
    // gfc__IRefObject
    pub vfptr: *const gfc__Body__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CollisionObject
    pub vfptr_2: *const gfc__Body__vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
    // gfc__Body
    pub mObject: *mut gfc__Object3D,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mNodeName: gfc__HString,
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mElementData: gfc__AutoRef_gfc__Object_,
    pub mElementType: u16,
    pub mAutoActivate: bool,
    pub mBodyType: u8,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Body {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Body {}

unsafe impl UpcastTo<gfc__CollisionObject> for gfc__Body {
    fn upcast_to(p: *const Self) -> *const gfc__CollisionObject {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__Body {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getShape(
        &self,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_ {
        ((*self.vfptr).getShape)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getMatrix(&self, a1: *mut gfc__Matrix4) {
        ((*self.vfptr).getMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroupContext(&self) -> *mut () {
        ((*self.vfptr).getGroupContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setBodyType(&self, a1: u8) {
        ((*self.vfptr).setBodyType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invalidateNode(&self) {
        ((*self.vfptr).invalidateNode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self, a1: i32) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Body__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Body, _: u32) -> *mut (),
    pub getShape: unsafe extern "thiscall" fn(
        this: *const gfc__Body,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_,
    pub getMatrix: unsafe extern "thiscall" fn(this: *mut gfc__Body, _: *mut gfc__Matrix4),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__Body) -> *mut gfc__Object,
    pub getGroupContext: unsafe extern "thiscall" fn(this: *mut gfc__Body) -> *mut (),
    pub getBoundingBox:
        unsafe extern "thiscall" fn(this: *mut gfc__Body, _: *mut gfc__TBox_float_gfc__FloatMath_),
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Body) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Body, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Body,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setBodyType: unsafe extern "thiscall" fn(this: *mut gfc__Body, _: u8),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__Body, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__Body),
    pub invalidateNode: unsafe extern "thiscall" fn(this: *mut gfc__Body),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Body),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Body),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Body, _: i32),
}

#[repr(C)]
pub struct gfc__Vector_gfc__SceneObserver___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneObserver,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__WorldObjectGizmo {
    // gfc__SceneObject
    pub vfptr: *const gfc__WorldObjectGizmo__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__WorldObjectGizmo__vftable,
    pub mLocked: bool,
    // gfc__Gizmo
    pub mObject: *mut gfc__WorldObject,
    // gfc__WorldObjectGizmo
    pub mGizmoColor: gfc__TVector4_float_gfc__FloatMath_,
}

unsafe impl UpcastToNop<gfc__Gizmo> for gfc__WorldObjectGizmo {}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__WorldObjectGizmo {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__WorldObjectGizmo {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__WorldObjectGizmo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn startGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).startGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finishGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).finishGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn prepGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cullAndSubmit(
        &self,
        a1: *const gfc__Clipper,
        a2: *mut gfc__Camera3D,
        a3: u32,
    ) {
        ((*self.vfptr).cullAndSubmit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D, a2: bool, a3: u32) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submitHidden(&self, a1: *mut gfc__Camera3D, a2: u32) {
        ((*self.vfptr).submitHidden)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isHighPriority(&self) -> bool {
        ((*self.vfptr).isHighPriority)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn writeText(&self, a1: *mut gfc__AutoRef_gfc__OutputStream_) {
        ((*self.vfptr).writeText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIsSky(&self, a1: bool) {
        ((*self.vfptr).setIsSky)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getIsSky(&self) -> bool {
        ((*self.vfptr).getIsSky)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__WorldObjectGizmo__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub getContext:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__WorldObjectGizmo) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__WorldObjectGizmo) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObjectGizmo,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__WorldObjectGizmo) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObjectGizmo) -> bool,
}

#[repr(C)]
pub struct gfc__PSystem {
    // gfc__IRefObject
    pub vfptr: *const gfc__PSystem__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__PSystem
    pub mName: gfc__HString,
    pub mRefObjectScale: f32,
    pub mRefObject: gfc__HString,
    pub mRefAnimation: gfc__HString,
    pub mCameraNode: gfc__HString,
    pub mObjectNode: gfc__HString,
    pub mBackgroundColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
    pub mOffset: gfc__TVector3_float_gfc__FloatMath_,
    pub mPackageID: i32,
    pub mEmitters: gfc__Vector_gfc__AutoRef_gfc__Emitter__0_gfc__CAllocator_,
    pub mSoundsLookedUp: bool,
    pub mSounds: gfc__AutoRef_gfc__SoundList_,
    pub mSoundDescFile: gfc__String,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__PSystem {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PSystem {}

impl gfc__PSystem {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__PSystem__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__PSystem, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__PSystem) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__PSystem, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__PSystem) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__PSystem) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__PSystem,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__PSystem) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__PSystem, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PSystem,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Skeleton3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Gizmo {
    // gfc__SceneObject
    pub vfptr: *const gfc__Gizmo__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__Gizmo__vftable,
    pub mLocked: bool,
    // gfc__Gizmo
    pub mObject: *mut gfc__WorldObject,
}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__Gizmo {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__Gizmo {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__Gizmo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn startGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).startGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finishGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).finishGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn prepGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cullAndSubmit(
        &self,
        a1: *const gfc__Clipper,
        a2: *mut gfc__Camera3D,
        a3: u32,
    ) {
        ((*self.vfptr).cullAndSubmit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D, a2: bool, a3: u32) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submitHidden(&self, a1: *mut gfc__Camera3D, a2: u32) {
        ((*self.vfptr).submitHidden)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isHighPriority(&self) -> bool {
        ((*self.vfptr).isHighPriority)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn writeText(&self, a1: *mut gfc__AutoRef_gfc__OutputStream_) {
        ((*self.vfptr).writeText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIsSky(&self, a1: bool) {
        ((*self.vfptr).setIsSky)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getIsSky(&self) -> bool {
        ((*self.vfptr).getIsSky)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Gizmo__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Gizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__Gizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__RenderNode),
    pub finishGeometry: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__RenderNode),
    pub prepGeometry: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__Gizmo,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit:
        unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__Camera3D, _: bool, _: u32),
    pub submitHidden:
        unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__Camera3D, _: u32),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Gizmo,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__Gizmo) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__Gizmo) -> bool,
    pub writeText:
        unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: *mut gfc__AutoRef_gfc__OutputStream_),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__Gizmo) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__Gizmo) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Emitter__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Emitter_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RagdollMapper_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__WorldObject,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Constraint__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Constraint_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__SceneCell {
    pub vfptr: *const gfc__SceneCell__vftable,
    pub mObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mLights: gfc__Vector_gfc__SceneLight___0_gfc__CAllocator_,
    pub mPortals: gfc__Vector_gfc__ScenePortal___0_gfc__CAllocator_,
    pub mObservers: gfc__Vector_gfc__SceneObserver___0_gfc__CAllocator_,
    pub m_ID: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mDrawCounter: u32,
    pub mInCell: bool,
}

impl gfc__SceneCell {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pointInCell(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> bool {
        ((*self.vfptr).pointInCell)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn testOverlapSphere(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: f32,
    ) -> bool {
        ((*self.vfptr).testOverlapSphere)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn testOverlapFrustum(&self, a1: *const gfc__Frustum) -> bool {
        ((*self.vfptr).testOverlapFrustum)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn testOverlapAABB(
        &self,
        a1: *const gfc__TBox_float_gfc__FloatMath_,
    ) -> bool {
        ((*self.vfptr).testOverlapAABB)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn testOverlapOBB(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__Matrix4,
    ) -> bool {
        ((*self.vfptr).testOverlapOBB)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__SceneCell__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell) -> i32,
    pub pointInCell: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneCell,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> bool,
    pub testOverlapSphere: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneCell,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ) -> bool,
    pub testOverlapFrustum:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneCell, _: *const gfc__Frustum) -> bool,
    pub testOverlapAABB: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneCell,
        _: *const gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub testOverlapOBB: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneCell,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__Matrix4,
    ) -> bool,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneCell,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell) -> *mut gfc__Object,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__SceneCell) -> bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Visual_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ModuleSystem_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__LayerLoadRequest_0_gfc__CAllocator_ {
    pub mData: *mut gfc__LayerLoadRequest,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TraversalWaypoint_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldQueueItem {
    pub load: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub unload: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub required: bool,
    pub allowGlobal: bool,
}

#[repr(C)]
pub struct gfc__LayerLoadRequest {
    pub LayerID: i32,
    pub Layer: gfc__AutoRef_gfc__RegionLayer_,
}

#[repr(C)]
pub struct gfc__RegionLoadRequest {
    pub RegionData: gfc__AutoRef_gfc__WorldRegionData_,
    pub Region: gfc__AutoRef_gfc__WorldRegion_,
    pub Layers: gfc__Vector_gfc__LayerLoadRequest_0_gfc__CAllocator_,
    pub RegionAlreadyLoaded: bool,
}

#[repr(C)]
pub struct gfc__SceneOccluder {
    pub vfptr: *const gfc__SceneOccluder__vftable,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mDrawCounter: u32,
}

impl gfc__SceneOccluder {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isVisible(&self, a1: *mut gfc__Clipper) -> bool {
        ((*self.vfptr).isVisible)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOccluder(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__Occluder {
        ((*self.vfptr).getOccluder)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__SceneOccluder__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder, _: u32) -> *mut (),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneOccluder,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isVisible:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder, _: *mut gfc__Clipper) -> bool,
    pub getOccluder: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneOccluder,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__Occluder,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__SceneOccluder) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__RegionLoadRequest_0_gfc__CAllocator_ {
    pub mData: *mut gfc__RegionLoadRequest,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__RegionLoadInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TraversalLink_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__TraversalWaypoint___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__TraversalWaypoint,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__TraversalLink__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TraversalLink_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__WorldLoadRequest {
    pub Regions: gfc__Vector_gfc__RegionLoadRequest_0_gfc__CAllocator_,
    pub Marker: gfc__AutoRef_gfc__PackageMarker_,
    pub State: i32,
    pub Required: bool,
}

#[repr(C)]
pub struct hkpRayHitCollector {
    pub vfptr: *const hkpRayHitCollector__vftable,
    pub m_earlyOutHitFraction: f32,
}

impl hkpRayHitCollector {
    pub unsafe extern "thiscall" fn addRayHit(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpShapeRayCastCollectorOutput,
    ) {
        ((*self.vfptr).addRayHit)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpRayHitCollector__vftable {
    pub addRayHit: unsafe extern "thiscall" fn(
        this: *mut hkpRayHitCollector,
        _: *const hkpCdBody,
        _: *const hkpShapeRayCastCollectorOutput,
    ),
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpRayHitCollector, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Constraint_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__RagdollBoneMapping__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__RagdollBoneMapping_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RagdollBoneMapping_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PhantomBody {
    // gfc__IRefObject
    pub vfptr: *const gfc__PhantomBody__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CollisionObject
    pub vfptr_2: *const gfc__PhantomBody__vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
    // gfc__Body
    pub mObject: *mut gfc__Object3D,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mNodeName: gfc__HString,
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mElementData: gfc__AutoRef_gfc__Object_,
    pub mElementType: u16,
    pub mAutoActivate: bool,
    pub mBodyType: u8,
    // gfc__PhantomBody
    __pdbindgen_padding: [u8; 4],
    pub mHavokShape: *mut hkpShape,
    pub mPhantom: *mut hkpShapePhantom,
    pub mLastNodeVersion: i32,
    pub mActivateCount: i32,
    pub mPhantomID: u8,
    pub mPhantomFlags: u8,
    pub mClothCollisionChannels: u32,
}

unsafe impl UpcastToNop<gfc__Body> for gfc__PhantomBody {}

unsafe impl UpcastToNop<gfc__Object> for gfc__PhantomBody {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PhantomBody {}

unsafe impl UpcastTo<gfc__CollisionObject> for gfc__PhantomBody {
    fn upcast_to(p: *const Self) -> *const gfc__CollisionObject {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__PhantomBody {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getShape(
        &self,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_ {
        ((*self.vfptr).getShape)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getMatrix(&self, a1: *mut gfc__Matrix4) {
        ((*self.vfptr).getMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroupContext(&self) -> *mut () {
        ((*self.vfptr).getGroupContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setBodyType(&self, a1: u8) {
        ((*self.vfptr).setBodyType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invalidateNode(&self) {
        ((*self.vfptr).invalidateNode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self, a1: i32) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__PhantomBody__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: u32) -> *mut (),
    pub getShape: unsafe extern "thiscall" fn(
        this: *const gfc__PhantomBody,
        result: *mut gfc__AutoRef_gfc__CShape_,
    ) -> *mut gfc__AutoRef_gfc__CShape_,
    pub getMatrix: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: *mut gfc__Matrix4),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody) -> *mut gfc__Object,
    pub getGroupContext: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody) -> *mut (),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__PhantomBody,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhantomBody,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setBodyType: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: u8),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody),
    pub invalidateNode: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__PhantomBody, _: i32),
}

#[repr(C)]
pub struct gfc__RagdollMapper {
    // gfc__IRefObject
    pub vfptr: *const gfc__RagdollMapper__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__RagdollMapper
    pub mRootBody: gfc__AutoRef_gfc__RigidBody_,
    pub mBones: gfc__Vector_gfc__AutoRef_gfc__RagdollBoneMapping__0_gfc__CAllocator_,
    pub mStabilizer: *mut hkRagdollStabilizer,
    pub mDefaultStepsUntilStabilize: i32,
    pub mStepsUntilStabilize: i32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__RagdollMapper {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__RagdollMapper {}

impl gfc__RagdollMapper {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__RagdollMapper__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__RagdollMapper, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__RagdollMapper) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__RagdollMapper, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__RagdollMapper) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__RagdollMapper) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__RagdollMapper,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__RagdollMapper) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__RagdollMapper,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__RagdollMapper,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Emitter_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct hkArrayBase_hkpCollisionDispatcher__ShapeInheritance_ {
    pub m_data: *mut hkpCollisionDispatcher__ShapeInheritance,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpCollisionDispatcher__ShapeInheritance_
    pub m_data: *mut hkpCollisionDispatcher__ShapeInheritance,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpCollisionDispatcher__ShapeInheritance_>
    for hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkFlags_enum_hkClassMember__FlagValues_unsigned_short_ {
    pub m_storage: u16,
}

#[repr(C)]
pub struct hkpCollisionQualityInfo {
    pub m_keepContact: f32,
    pub m_create4dContact: f32,
    pub m_createContact: f32,
    pub m_manifoldTimDistance: f32,
    pub m_useContinuousPhysics: hkPadSpu_unsigned_int_,
    pub m_useSimpleToiHandling: hkBool,
    pub m_minSeparation: f32,
    pub m_minExtraSeparation: f32,
    pub m_minSafeDeltaTime: f32,
    pub m_minAbsoluteSafeDeltaTime: f32,
    pub m_toiSeparation: f32,
    pub m_toiExtraSeparation: f32,
    pub m_toiAccuracy: f32,
    pub m_maxContraintViolation: f32,
    pub m_minToiDeltaTime: f32,
    pub m_constraintPriority: u16,
    pub m_enableToiWeldRejection: hkBool,
}

#[repr(C)]
pub struct hkPadSpu_hkpCollisionDispatcher___ {
    pub m_storage: *mut hkpCollisionDispatcher,
}

#[repr(C)]
pub struct hkpShapeBufferStorage {
    pub m_storage: [i8; 512],
}

#[repr(C)]
pub struct hkpCollisionAgent {
    // hkBaseObject
    pub vfptr: *const hkpCollisionAgent__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpCollisionAgent
    pub m_contactMgr: *mut hkpContactMgr,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpCollisionAgent {}

unsafe impl UpcastToNop<hkBaseObject> for hkpCollisionAgent {}

impl hkpCollisionAgent {
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

    pub unsafe extern "thiscall" fn getPenetrations(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpCollisionInput,
        a4: *mut hkpCdBodyPairCollector,
    ) {
        ((*self.vfptr).getPenetrations)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getClosestPoints(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpCollisionInput,
        a4: *mut hkpCdPointCollector,
    ) {
        ((*self.vfptr).getClosestPoints)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn linearCast(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpLinearCastCollisionInput,
        a4: *mut hkpCdPointCollector,
        a5: *mut hkpCdPointCollector,
    ) {
        ((*self.vfptr).linearCast)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn processCollision(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpProcessCollisionInput,
        a4: *mut hkpProcessCollisionOutput,
    ) {
        ((*self.vfptr).processCollision)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn cleanup(&self, a1: *mut hkpConstraintOwner) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateShapeCollectionFilter(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpCollisionInput,
        a4: *mut hkpConstraintOwner,
    ) {
        ((*self.vfptr).updateShapeCollectionFilter)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn invalidateTim(&self, a1: *const hkpCollisionInput) {
        ((*self.vfptr).invalidateTim)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn warpTime(
        &self,
        a1: f32,
        a2: f32,
        a3: *const hkpCollisionInput,
    ) {
        ((*self.vfptr).warpTime)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn removePoint(&self, a1: u16) {
        ((*self.vfptr).removePoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn commitPotential(&self, a1: u16) {
        ((*self.vfptr).commitPotential)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createZombie(&self, a1: u16) {
        ((*self.vfptr).createZombie)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpCollisionAgent__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpCollisionAgent) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpCollisionAgent),
    pub getPenetrations: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpCollisionInput,
        _: *mut hkpCdBodyPairCollector,
    ),
    pub getClosestPoints: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpCollisionInput,
        _: *mut hkpCdPointCollector,
    ),
    pub linearCast: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpLinearCastCollisionInput,
        _: *mut hkpCdPointCollector,
        _: *mut hkpCdPointCollector,
    ),
    pub processCollision: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
    ),
    pub cleanup:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: *mut hkpConstraintOwner),
    pub updateShapeCollectionFilter: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpCollisionInput,
        _: *mut hkpConstraintOwner,
    ),
    pub invalidateTim:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: *const hkpCollisionInput),
    pub warpTime: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionAgent,
        _: f32,
        _: f32,
        _: *const hkpCollisionInput,
    ),
    pub removePoint: unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: u16),
    pub commitPotential: unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: u16),
    pub createZombie: unsafe extern "thiscall" fn(this: *mut hkpCollisionAgent, _: u16),
}

#[repr(C)]
pub struct hkArray_unsigned_int_hkContainerHeapAllocator_ {
    // hkArrayBase_unsigned_int_
    pub m_data: *mut u32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_unsigned_int_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_unsigned_int_>
    for hkArray_unsigned_int_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkPadSpu_hkpCollisionQualityInfo___ {
    pub m_storage: *mut hkpCollisionQualityInfo,
}

#[repr(C)]
pub struct hkpProcessCollisionInput {
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
    // hkpProcessCollisionInput
    pub m_stepInfo: hkStepInfo,
    pub m_collisionQualityInfo: hkPadSpu_hkpCollisionQualityInfo___,
    pub m_spareAgentSector: *mut hkpAgent1nSector,
    pub m_dynamicsInfo: *mut (),
    pub m_enableDeprecatedWelding: hkBool,
    pub m_allowToSkipConfirmedCallbacks: hkBool,
    pub m_config: *mut hkpCollisionAgentConfig,
    __pdbindgen_padding_2: [u8; 12],
}

unsafe impl UpcastToNop<hkpCollisionInput> for hkpProcessCollisionInput {}

#[repr(C)]
pub struct hkPadSpu_hkpCollisionFilter_const___ {
    pub m_storage: *const hkpCollisionFilter,
}

#[repr(C)]
pub struct hkFlags_enum_hkClassEnum__FlagValues_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpCdPointCollector {
    pub vfptr: *const hkpCdPointCollector__vftable,
    pub m_earlyOutDistance: f32,
}

impl hkpCdPointCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCdPoint(&self, a1: *const hkpCdPoint) {
        ((*self.vfptr).addCdPoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpCdPointCollector__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCdPointCollector, _: u32) -> *mut (),
    pub addCdPoint:
        unsafe extern "thiscall" fn(this: *mut hkpCdPointCollector, _: *const hkpCdPoint),
    pub reset: unsafe extern "thiscall" fn(this: *mut hkpCdPointCollector),
}

#[repr(C)]
pub struct hkClassEnum__Item {
    pub m_value: i32,
    pub m_name: *const i8,
}

#[repr(C)]
pub struct hkArray_float_hkContainerHeapAllocator_ {
    // hkArrayBase_float_
    pub m_data: *mut f32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_float_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_float_> for hkArray_float_hkContainerHeapAllocator_ {}

#[repr(C)]
pub struct hkpAgent1nSector {
    pub m_bytesAllocated: u32,
    pub m_pad0: u32,
    pub m_pad1: u32,
    pub m_pad2: u32,
    pub m_data: [u8; 496],
}

#[repr(C)]
pub struct hkPadSpu_hkpConvexListFilter_const___ {
    pub m_storage: *const hkpConvexListFilter,
}

#[repr(C)]
pub struct hkpCdBodyPairCollector {
    pub vfptr: *const hkpCdBodyPairCollector__vftable,
    pub m_earlyOut: hkBool,
}

impl hkpCdBodyPairCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCdBodyPair(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
    ) {
        ((*self.vfptr).addCdBodyPair)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpCdBodyPairCollector__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCdBodyPairCollector, _: u32) -> *mut (),
    pub addCdBodyPair: unsafe extern "thiscall" fn(
        this: *mut hkpCdBodyPairCollector,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
    ),
    pub reset: unsafe extern "thiscall" fn(this: *mut hkpCdBodyPairCollector),
}

#[repr(C)]
pub struct hkArrayBase_float_ {
    pub m_data: *mut f32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_unsigned_int_ {
    pub m_data: *mut u32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__SceneObserver {
    pub vfptr: *const gfc__SceneObserver__vftable,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mInvalid: bool,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mWarpDistanceThreshold: f32,
}

impl gfc__SceneObserver {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__SceneObserver__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObserver, _: u32) -> *mut (),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneObserver) -> *mut gfc__Object,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Myval: std__pair_int_const__gfc__AutoRef_gfc__OverrideResources___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___ {
    // std___Pair_base_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___
    pub first: gfc__HString,
    pub second: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    // std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___
}

unsafe impl
    UpcastToNop<std___Pair_base_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___>
    for std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___
{
}

#[repr(C)]
pub struct std__pair_int_const__gfc__AutoRef_gfc__OverrideResources___ {
    // std___Pair_base_int_const__gfc__AutoRef_gfc__OverrideResources___
    pub first: i32,
    pub second: gfc__AutoRef_gfc__OverrideResources_,
    // std__pair_int_const__gfc__AutoRef_gfc__OverrideResources___
}

unsafe impl UpcastToNop<std___Pair_base_int_const__gfc__AutoRef_gfc__OverrideResources___>
    for std__pair_int_const__gfc__AutoRef_gfc__OverrideResources___
{
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__ResourceCache___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__ResourceCache___ {
    pub first: gfc__HString,
    pub second: *mut gfc__ResourceCache,
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__ResourceCache___ {
    // std___Pair_base_gfc__HString_const__gfc__ResourceCache___
    pub first: gfc__HString,
    pub second: *mut gfc__ResourceCache,
    // std__pair_gfc__HString_const__gfc__ResourceCache___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__ResourceCache___>
    for std__pair_gfc__HString_const__gfc__ResourceCache___
{
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator___ {
    pub first: gfc__HString,
    pub second: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
}

#[repr(C)]
pub struct std___Pair_base_int_const__gfc__AutoRef_gfc__OverrideResources___ {
    pub first: i32,
    pub second: gfc__AutoRef_gfc__OverrideResources_,
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: gfc__HString,
    pub mValue: gfc__ResourceManager__ShaderState,
}

#[repr(C)]
pub struct hkArrayBase_hkFourTransposedPointsf_ {
    pub m_data: *mut hkFourTransposedPointsf,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkPadSpu_hkpConstraintRuntime___ {
    pub m_storage: *mut hkpConstraintRuntime,
}

#[repr(C)]
pub struct hkArray_hkFourTransposedPointsf_hkContainerHeapAllocator_ {
    // hkArrayBase_hkFourTransposedPointsf_
    pub m_data: *mut hkFourTransposedPointsf,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkFourTransposedPointsf_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkFourTransposedPointsf_>
    for hkArray_hkFourTransposedPointsf_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpNullBroadPhaseListener {
    // hkBaseObject
    pub vfptr: *const hkpNullBroadPhaseListener__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBroadPhaseListener
    pub vfptr_2: *const hkpNullBroadPhaseListener__vftable,
    // hkpNullBroadPhaseListener
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpNullBroadPhaseListener {}

unsafe impl UpcastToNop<hkBaseObject> for hkpNullBroadPhaseListener {}

unsafe impl UpcastTo<hkpBroadPhaseListener> for hkpNullBroadPhaseListener {
    fn upcast_to(p: *const Self) -> *const hkpBroadPhaseListener {
        (p as usize + 0x8) as *const _
    }
}

impl hkpNullBroadPhaseListener {
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
pub struct hkpNullBroadPhaseListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpNullBroadPhaseListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpNullBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpNullBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpNullBroadPhaseListener),
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Sample___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Sample_,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Myval: *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Sample___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Sample_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Sample___>
    for std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample___
{
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct hkpConvexVerticesShape {
    // hkBaseObject
    pub vfptr: *const hkpConvexVerticesShape__vftable,
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
    // hkpConvexVerticesShape
    __pdbindgen_padding: [u8; 12],
    pub m_aabbHalfExtents: hkVector4f,
    pub m_aabbCenter: hkVector4f,
    pub m_rotatedVertices: hkArray_hkFourTransposedPointsf_hkContainerHeapAllocator_,
    pub m_numVertices: i32,
    pub m_useSpuBuffer: hkBool,
    pub m_planeEquations: hkArray_hkVector4f_hkContainerHeapAllocator_,
    pub m_connectivity: *const hkpConvexVerticesConnectivity,
    __pdbindgen_padding_2: [u8; 12],
}

unsafe impl UpcastToNop<hkpConvexShape> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkpSphereRepShape> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkpShape> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexVerticesShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexVerticesShape {}

impl hkpConvexVerticesShape {
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
pub struct hkpConvexVerticesShape__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesShape),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
    ) -> *const hkpShapeContainer,
    pub getMaximumProjection: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const hkVector4f,
    ) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpConvexVerticesShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub getFirstVertex:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape, _: *mut hkVector4f),
    pub getSize: unsafe extern "thiscall" fn(this: *const hkpConvexVerticesShape) -> i32,
}

#[repr(C)]
pub struct hkpMoppBvTreeShape {
    // hkBaseObject
    pub vfptr: *const hkpMoppBvTreeShape__vftable,
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
    // hkpBvTreeShape
    pub m_bvTreeType: hkEnum_enum_hkpBvTreeShape__BvTreeType_unsigned_char_,
    // hkMoppBvTreeShapeBase
    pub m_code: *const hkpMoppCode,
    pub m_moppData: *const u8,
    pub m_moppDataSize: u32,
    pub m_codeInfoCopy: hkVector4f,
    // hkpMoppBvTreeShape
    pub m_child: hkpSingleShapeContainer,
    pub m_childSize: i32,
    __pdbindgen_padding: [u8; 4],
}

unsafe impl UpcastToNop<hkMoppBvTreeShapeBase> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkpBvTreeShape> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkpShape> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpMoppBvTreeShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpMoppBvTreeShape {}

impl hkpMoppBvTreeShape {
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

    pub unsafe extern "thiscall" fn queryAabb(
        &self,
        a1: *const hkAabb,
        a2: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).queryAabb)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn castAabbImpl(
        &self,
        a1: *const hkAabb,
        a2: *const hkVector4f,
        a3: *mut hkpAabbCastCollector,
    ) {
        ((*self.vfptr).castAabbImpl)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn queryAabbImpl(
        &self,
        a1: *const hkAabb,
        a2: *mut u32,
        a3: i32,
    ) -> u32 {
        ((*self.vfptr).queryAabbImpl)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn queryObb(
        &self,
        a1: *const hkTransformf,
        a2: *const hkVector4f,
        a3: f32,
        a4: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).queryObb)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct hkpMoppBvTreeShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpMoppBvTreeShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpMoppBvTreeShape),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpMoppBvTreeShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub queryAabb: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkAabb,
        _: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ),
    pub castAabbImpl: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkAabb,
        _: *const hkVector4f,
        _: *mut hkpAabbCastCollector,
    ),
    pub queryAabbImpl: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkAabb,
        _: *mut u32,
        _: i32,
    ) -> u32,
    pub queryObb: unsafe extern "thiscall" fn(
        this: *const hkpMoppBvTreeShape,
        _: *const hkTransformf,
        _: *const hkVector4f,
        _: f32,
        _: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ),
}

#[repr(C)]
pub struct hkpAction {
    // hkBaseObject
    pub vfptr: *const hkpAction__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpAction
    pub m_world: *mut hkpWorld,
    pub m_island: *mut hkpSimulationIsland,
    pub m_userData: u32,
    pub m_name: hkStringPtr,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpAction {}

unsafe impl UpcastToNop<hkBaseObject> for hkpAction {}

impl hkpAction {
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

    pub unsafe extern "thiscall" fn applyAction(&self, a1: *const hkStepInfo) {
        ((*self.vfptr).applyAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEntities(
        &self,
        a1: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getEntities)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPhantoms(
        &self,
        a1: *mut hkArray_hkpPhantom___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getPhantoms)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityRemovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clone(
        &self,
        a1: *const hkArray_hkpEntity___hkContainerHeapAllocator_,
        a2: *const hkArray_hkpPhantom___hkContainerHeapAllocator_,
    ) -> *mut hkpAction {
        ((*self.vfptr).clone)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpAction__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpAction, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpAction),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpAction) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpAction),
    pub applyAction: unsafe extern "thiscall" fn(this: *mut hkpAction, _: *const hkStepInfo),
    pub getEntities: unsafe extern "thiscall" fn(
        this: *mut hkpAction,
        _: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ),
    pub getPhantoms: unsafe extern "thiscall" fn(
        this: *mut hkpAction,
        _: *mut hkArray_hkpPhantom___hkContainerHeapAllocator_,
    ),
    pub entityRemovedCallback: unsafe extern "thiscall" fn(this: *mut hkpAction, _: *mut hkpEntity),
    pub clone: unsafe extern "thiscall" fn(
        this: *const hkpAction,
        _: *const hkArray_hkpEntity___hkContainerHeapAllocator_,
        _: *const hkArray_hkpPhantom___hkContainerHeapAllocator_,
    ) -> *mut hkpAction,
}

#[repr(C)]
pub struct hkPadSpu_hkpJacobianSchema___ {
    pub m_storage: *mut hkpJacobianSchema,
}

#[repr(C)]
pub struct hkArrayBase_hkMemorySnapshot__Allocation_ {
    pub m_data: *mut hkMemorySnapshot__Allocation,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkAabb_hkContainerHeapAllocator_ {
    // hkArrayBase_hkAabb_
    pub m_data: *mut hkAabb,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkAabb_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkAabb_> for hkArray_hkAabb_hkContainerHeapAllocator_ {}

#[repr(C)]
pub struct hkMemorySnapshot__Provider {
    pub m_name: [i8; 32],
    pub m_parentIndices: hkArrayBase_int_,
}

#[repr(C)]
pub struct hkMemorySnapshot__Allocation {
    pub m_start: *const (),
    pub m_size: i32,
    pub m_sourceId: i32,
    pub m_traceId: i32,
    pub m_status: hkEnum_enum_hkMemorySnapshot__StatusBits_signed_char_,
}

#[repr(C)]
pub struct hkpWorldDeletionListener {
    pub vfptr: *const hkpWorldDeletionListener__vftable,
}

impl hkpWorldDeletionListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn worldDeletedCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).worldDeletedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn worldRemoveAllCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).worldRemoveAllCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpWorldDeletionListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: u32) -> *mut (),
    pub worldDeletedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: *mut hkpWorld),
    pub worldRemoveAllCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkpExtendedMeshShape {
    // hkBaseObject
    pub vfptr: *const hkpExtendedMeshShape__vftable,
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
    // hkpShapeContainer
    pub vfptr_2: *const hkpExtendedMeshShape__vftable,
    // hkpShapeCollection
    pub m_disableWelding: hkBool,
    pub m_collectionType: hkEnum_enum_hkpShapeCollection__CollectionType_unsigned_char_,
    // hkpExtendedMeshShape
    __pdbindgen_padding: [u8; 10],
    pub m_embeddedTrianglesSubpart: hkpExtendedMeshShape__TrianglesSubpart,
    pub m_aabbHalfExtents: hkVector4f,
    pub m_aabbCenter: hkVector4f,
    pub m_materialClass: *const hkClass,
    pub m_numBitsForSubpartIndex: i32,
    pub m_trianglesSubparts:
        hkArray_hkpExtendedMeshShape__TrianglesSubpart_hkContainerHeapAllocator_,
    pub m_shapesSubparts: hkArray_hkpExtendedMeshShape__ShapesSubpart_hkContainerHeapAllocator_,
    pub m_weldingInfo: hkArray_unsigned_short_hkContainerHeapAllocator_,
    pub m_weldingType: hkEnum_enum_hkpWeldingUtility__WeldingType_unsigned_char_,
    pub m_defaultCollisionFilterInfo: u32,
    pub m_cachedNumChildShapes: i32,
    pub m_triangleRadius: f32,
    pub m_padding: i32,
}

unsafe impl UpcastToNop<hkpShapeCollection> for hkpExtendedMeshShape {}

unsafe impl UpcastToNop<hkpShape> for hkpExtendedMeshShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpExtendedMeshShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpExtendedMeshShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpExtendedMeshShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpExtendedMeshShape {}

unsafe impl UpcastTo<hkpShapeContainer> for hkpExtendedMeshShape {
    fn upcast_to(p: *const Self) -> *const hkpShapeContainer {
        (p as usize + 0x10) as *const _
    }
}

impl hkpExtendedMeshShape {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumChildShapes(&self) -> i32 {
        ((*self.vfptr).getNumChildShapes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFirstKey(&self) -> u32 {
        ((*self.vfptr).getFirstKey)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNextKey(&self, a1: u32) -> u32 {
        ((*self.vfptr).getNextKey)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCollisionFilterInfo(&self, a1: u32) -> u32 {
        ((*self.vfptr).getCollisionFilterInfo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getChildShape(
        &self,
        a1: u32,
        a2: *mut hkpShapeBufferStorage,
    ) -> *const hkpShape {
        ((*self.vfptr).getChildShape)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn isWeldingEnabled(&self) -> bool {
        ((*self.vfptr).isWeldingEnabled)(self as *const _ as *mut _)
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

    pub unsafe extern "thiscall" fn initWeldingInfo(&self, a1: hkpWeldingUtility__WeldingType) {
        ((*self.vfptr).initWeldingInfo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setWeldingInfo(&self, a1: u32, a2: i16) {
        ((*self.vfptr).setWeldingInfo)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn addTrianglesSubpart(
        &self,
        a1: *const hkpExtendedMeshShape__TrianglesSubpart,
    ) {
        ((*self.vfptr).addTrianglesSubpart)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addShapesSubpart(
        &self,
        a1: *const hkpExtendedMeshShape__ShapesSubpart,
    ) -> i32 {
        ((*self.vfptr).addShapesSubpart)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpExtendedMeshShape__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpExtendedMeshShape, _: u32) -> *mut (),
    pub getNumChildShapes: unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape) -> i32,
    pub getFirstKey: unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape) -> u32,
    pub getNextKey: unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape, _: u32) -> u32,
    pub getCollisionFilterInfo:
        unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape, _: u32) -> u32,
    pub getChildShape: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: u32,
        _: *mut hkpShapeBufferStorage,
    ) -> *const hkpShape,
    pub isWeldingEnabled: unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape) -> bool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre:
        unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres:
        unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpExtendedMeshShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpExtendedMeshShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub initWeldingInfo: unsafe extern "thiscall" fn(
        this: *mut hkpExtendedMeshShape,
        _: hkpWeldingUtility__WeldingType,
    ),
    pub setWeldingInfo:
        unsafe extern "thiscall" fn(this: *mut hkpExtendedMeshShape, _: u32, _: i16),
    pub addTrianglesSubpart: unsafe extern "thiscall" fn(
        this: *mut hkpExtendedMeshShape_2,
        _: *const hkpExtendedMeshShape__TrianglesSubpart,
    ),
    pub addShapesSubpart: unsafe extern "thiscall" fn(
        this: *mut hkpExtendedMeshShape_2,
        _: *const hkpExtendedMeshShape__ShapesSubpart,
    ) -> i32,
}

#[repr(C)]
pub struct hkpExtendedMeshShape__ShapesSubpart {
    // hkpExtendedMeshShape__Subpart
    pub m_typeAndFlags: u16,
    pub m_shapeInfo: u16,
    pub m_materialStriding: i16,
    pub m_materialIndexStriding: u16,
    pub m_materialIndexBase: *const (),
    pub m_materialBase: *const hkpMeshMaterial,
    pub m_userData: u32,
    // hkpExtendedMeshShape__ShapesSubpart
    pub m_childShapes: hkArray_hkRefPtr_hkpConvexShape__hkContainerHeapAllocator_,
    pub m_rotation: hkQuaternionf,
    pub m_translation: hkVector4f,
}

unsafe impl UpcastToNop<hkpExtendedMeshShape__Subpart> for hkpExtendedMeshShape__ShapesSubpart {}

#[repr(C)]
pub struct hkArray_hkpExtendedMeshShape__TrianglesSubpart_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpExtendedMeshShape__TrianglesSubpart_
    pub m_data: *mut hkpExtendedMeshShape__TrianglesSubpart,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpExtendedMeshShape__TrianglesSubpart_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpExtendedMeshShape__TrianglesSubpart_>
    for hkArray_hkpExtendedMeshShape__TrianglesSubpart_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkpConstraintInstance___ {
    pub m_data: *mut *mut hkpConstraintInstance,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpEntityListener {
    pub vfptr: *const hkpEntityListener__vftable,
}

impl hkpEntityListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityAddedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityRemovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityShapeSetCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityShapeSetCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entitySetMotionTypeCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entitySetMotionTypeCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn entityDeletedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).entityDeletedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpEntityListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: u32) -> *mut (),
    pub entityAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: *mut hkpEntity),
    pub entityRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: *mut hkpEntity),
    pub entityShapeSetCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: *mut hkpEntity),
    pub entitySetMotionTypeCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: *mut hkpEntity),
    pub entityDeletedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpEntityListener, _: *mut hkpEntity),
}

#[repr(C)]
pub struct hkArrayBase_hkpExtendedMeshShape__ShapesSubpart_ {
    pub m_data: *mut hkpExtendedMeshShape__ShapesSubpart,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkFlags_enum_hkClass__FlagValues_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpBroadPhaseHandlePair {
    pub m_a: *mut hkpBroadPhaseHandle,
    pub m_b: *mut hkpBroadPhaseHandle,
}

#[repr(C)]
pub struct gfc__CInfo {
    pub Type: i32,
    pub Position: gfc__TVector3_float_gfc__FloatMath_,
    pub Normal: gfc__TVector3_float_gfc__FloatMath_,
    pub Depth: f32,
    pub PolyNormal: gfc__TVector3_float_gfc__FloatMath_,
    pub Object1: *mut gfc__CollisionObject,
    pub Object2: *mut gfc__CollisionObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__DebrisManager__Debris_0_gfc__CAllocator_ {
    pub mData: *mut gfc__DebrisManager__Debris,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhysMeshCache__ReloadInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysMeshCache__ReloadInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__PhysMeshCache {
    // gfc__ResourceCache
    pub vfptr: *const gfc__PhysMeshCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__PhysMeshCache
    pub mExportCache: *mut gfc__ExportCache,
    pub mReloadInfo: gfc__Vector_gfc__PhysMeshCache__ReloadInfo_0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__ResourceCache> for gfc__PhysMeshCache {}

impl gfc__PhysMeshCache {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadDefaultResource(&self, a1: gfc__AutoRef_gfc__File_) {
        ((*self.vfptr).loadDefaultResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn initThread(&self) {
        ((*self.vfptr).initThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdownThread(&self) {
        ((*self.vfptr).shutdownThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn analyzeResource(
        &self,
        a1: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool {
        ((*self.vfptr).analyzeResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canCreateBuffersInThread(&self, a1: i32) -> bool {
        ((*self.vfptr).canCreateBuffersInThread)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createBuffers(&self, a1: *mut gfc__ResourceBufferInfo) {
        ((*self.vfptr).createBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeBuffers(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).freeBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadResource(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).loadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canReloadResources(&self) -> bool {
        ((*self.vfptr).canReloadResources)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadsQueued(&self) -> bool {
        ((*self.vfptr).reloadsQueued)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadResource(&self, a1: *mut gfc__ResourceLoadInfo) -> bool {
        ((*self.vfptr).reloadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn needUnlinkResource(&self) -> bool {
        ((*self.vfptr).needUnlinkResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unlinkResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).unlinkResource)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn needUnloadResource(&self) -> bool {
        ((*self.vfptr).needUnloadResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unloadResource(
        &self,
        a1: *mut (),
        a2: *mut gfc__ResourceLoadInfo,
    ) {
        ((*self.vfptr).unloadResource)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn freeResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).freeResource)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__PhysMeshCache__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysMeshCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__PhysMeshCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysMeshCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__PhysMeshCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__PhysMeshCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysMeshCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__PhysMeshCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysMeshCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__PhysMeshCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysMeshCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysMeshCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
}

#[repr(C)]
pub struct gfc__PhysMeshCache__ReloadInfo {
    pub PackageID: i32,
    pub Name: gfc__HString,
    pub Buffer: *mut u8,
    pub Length: u32,
}

#[repr(C)]
pub struct gfc__CachedKeyframe {
    pub Position: hkVector4f,
    pub Rotation: hkQuaternionf,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__StaticObject_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__CShapeBox {
    // gfc__IRefObject
    pub vfptr: *const gfc__CShapeBox__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeBox
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeBox {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeBox {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeBox {}

impl gfc__CShapeBox {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *const gfc__Matrix4,
        a2: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getBoundingBox_2(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collideRay(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut gfc__CInfo,
    ) -> bool {
        ((*self.vfptr).collideRay)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn isComposite(&self) -> bool {
        ((*self.vfptr).isComposite)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn containsType(&self, a1: i32) -> bool {
        ((*self.vfptr).containsType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryShapes(
        &self,
        a1: *const gfc__TBox_float_gfc__FloatMath_,
        a2: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).queryShapes)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createHavokShape(&self, a1: f32, a2: i32) -> *mut hkpShape {
        ((*self.vfptr).createHavokShape)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setMaterialID(&self, a1: i32) {
        ((*self.vfptr).setMaterialID)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CShapeBox__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeBox) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeBox) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeBox) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShapeBox, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeBox,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox, _: f32, _: i32) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeBox, _: i32),
}

#[repr(C)]
pub struct gfc__CShapeMesh {
    // gfc__IRefObject
    pub vfptr: *const gfc__CShapeMesh__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeMesh
    pub mMeshName: gfc__HString,
    pub mMeshID: i32,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeMesh {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeMesh {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeMesh {}

impl gfc__CShapeMesh {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *const gfc__Matrix4,
        a2: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getBoundingBox_2(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collideRay(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut gfc__CInfo,
    ) -> bool {
        ((*self.vfptr).collideRay)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn isComposite(&self) -> bool {
        ((*self.vfptr).isComposite)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn containsType(&self, a1: i32) -> bool {
        ((*self.vfptr).containsType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryShapes(
        &self,
        a1: *const gfc__TBox_float_gfc__FloatMath_,
        a2: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).queryShapes)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createHavokShape(&self, a1: f32, a2: i32) -> *mut hkpShape {
        ((*self.vfptr).createHavokShape)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setMaterialID(&self, a1: i32) {
        ((*self.vfptr).setMaterialID)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CShapeMesh__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeMesh) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeMesh) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeMesh) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShapeMesh, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeMesh,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh, _: f32, _: i32) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeMesh, _: i32),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__StaticObject__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__StaticObject_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ExportCache {
    pub Package: gfc__String,
    pub Names: gfc__Vector_gfc__String_0_gfc__CAllocator_,
    pub Helper: gfc__PhysMeshCache,
}

#[repr(C)]
pub struct gfc__DebrisManager {
    // gfc__ResourceListener
    pub vfptr: *const gfc__DebrisManager__vftable,
    // gfc__DebrisManager
    pub mWorld: *mut gfc__World,
    pub mTime: f32,
    pub mDebrisObjects: gfc__Vector_gfc__AutoRef_gfc__StaticObject__0_gfc__CAllocator_,
    pub mLargeObjects: gfc__Vector_gfc__DebrisManager__Debris_0_gfc__CAllocator_,
    pub mQueuedBodies: gfc__Vector_gfc__AutoRef_gfc__RigidBody__0_gfc__CAllocator_,
    pub mDesc: gfc__AutoRef_gfc__DebrisManagerDesc_,
}

unsafe impl UpcastToNop<gfc__ResourceListener> for gfc__DebrisManager {}

impl gfc__DebrisManager {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn packageUnloading(&self, a1: i32) {
        ((*self.vfptr).packageUnloading)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DebrisManager__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__DebrisManager, _: u32) -> *mut (),
    pub packageUnloading: unsafe extern "thiscall" fn(this: *mut gfc__DebrisManager, _: i32),
}

#[repr(C)]
pub struct gfc__DebrisManager__Debris {
    pub Obj: gfc__AutoRef_gfc__StaticObject_,
    pub RemainingTime: f32,
    pub Fading: bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DebrisManagerDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__RigidBody__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__RigidBody_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct hkRagdollStabilizer {
    pub vfptr: *const hkRagdollStabilizer__vftable,
    pub m_bodies: hkArray_hkpRigidBody___hkContainerHeapAllocator_,
    pub m_constraints: hkArray_hkpConstraintInstance___hkContainerHeapAllocator_,
    pub m_oldImpulses: hkArray_float_hkContainerHeapAllocator_,
    pub m_linearDamping: f32,
    pub m_angularDamping: f32,
    pub m_gain: f32,
    pub m_threshold: f32,
}

impl hkRagdollStabilizer {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkRagdollStabilizer__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkRagdollStabilizer, _: u32) -> *mut (),
}

#[repr(C)]
pub struct hkpConstraintQueryOut {
    pub m_jacobianSchemas: hkPadSpu_hkpJacobianSchema___,
    pub m_constraintRuntime: hkPadSpu_hkpConstraintRuntime___,
    pub m_instanceRuntime: hkPadSpu_hkpConstraintRuntime___,
}

#[repr(C)]
pub struct hkArrayBase_hkpBroadPhaseHandlePair_ {
    pub m_data: *mut hkpBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpBroadPhaseHandlePair_
    pub m_data: *mut hkpBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpBroadPhaseHandlePair_>
    for hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpExtendedMeshShape__ShapesSubpart_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpExtendedMeshShape__ShapesSubpart_
    pub m_data: *mut hkpExtendedMeshShape__ShapesSubpart,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpExtendedMeshShape__ShapesSubpart_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpExtendedMeshShape__ShapesSubpart_>
    for hkArray_hkpExtendedMeshShape__ShapesSubpart_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkpExtendedMeshShape__TrianglesSubpart_ {
    pub m_data: *mut hkpExtendedMeshShape__TrianglesSubpart,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpMeshMaterial {
    pub m_filterInfo: u32,
}

#[repr(C)]
pub struct hkpTypedBroadPhaseDispatcher {
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_broadPhaseListeners: compile_error!("unimplemented sizeof array"),
    __pdbindgen_padding: [u8; 256],
    pub m_nullBroadPhaseListener: hkpNullBroadPhaseListener,
}

#[repr(C)]
pub struct hkRefPtr_hkpConvexShape_ {
    pub m_pntr: *mut hkpConvexShape,
}

#[repr(C)]
pub struct hkArrayBase_hkpBroadPhaseHandle___ {
    pub m_data: *mut *mut hkpBroadPhaseHandle,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpTypedBroadPhaseHandlePair {
    // hkpBroadPhaseHandlePair
    pub m_a: *mut hkpBroadPhaseHandle,
    pub m_b: *mut hkpBroadPhaseHandle,
    // hkpTypedBroadPhaseHandlePair
}

unsafe impl UpcastToNop<hkpBroadPhaseHandlePair> for hkpTypedBroadPhaseHandlePair {}

#[repr(C)]
pub struct hkArray_hkRefPtr_hkpConvexShape__hkContainerHeapAllocator_ {
    // hkArrayBase_hkRefPtr_hkpConvexShape___
    pub m_data: *mut hkRefPtr_hkpConvexShape_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkRefPtr_hkpConvexShape__hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkRefPtr_hkpConvexShape___>
    for hkArray_hkRefPtr_hkpConvexShape__hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpRigidBody___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpRigidBody___
    pub m_data: *mut *mut hkpRigidBody,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpRigidBody___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpRigidBody___>
    for hkArray_hkpRigidBody___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpWorldPostSimulationListener {
    pub vfptr: *const hkpWorldPostSimulationListener__vftable,
}

impl hkpWorldPostSimulationListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).postSimulationCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inactiveEntityMovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).inactiveEntityMovedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpWorldPostSimulationListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpEntity),
}

#[repr(C)]
pub struct hkArrayBase_hkAabb_ {
    pub m_data: *mut hkAabb,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkMemorySnapshot__Provider_ {
    pub m_data: *mut hkMemorySnapshot__Provider,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkRefPtr_hkpConvexShape___ {
    pub m_data: *mut hkRefPtr_hkpConvexShape_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpRigidBody___ {
    pub m_data: *mut *mut hkpRigidBody,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpConstraintInstance___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpConstraintInstance___
    pub m_data: *mut *mut hkpConstraintInstance,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpConstraintInstance___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpConstraintInstance___>
    for hkArray_hkpConstraintInstance___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__BezierCurve_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion {
    // hkpWorldPostSimulationListener
    pub vfptr: *const gfc__PhysicsDetectRegion__vftable,
    // gfc__PhysicsDetectRegion
    pub mPhantomOverlapListenerProxy: gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
    pub mWorld: *mut gfc__World,
    pub mPhantom: *mut hkpPhantom,
    pub mBodies: gfc__Vector_gfc__PhysicsDetectRegion__BodyInfo_0_gfc__CAllocator_,
    pub mNumContainedBodies: i32,
    pub mIsShapeDetector: bool,
    pub mAABBListChanged: bool,
}

unsafe impl UpcastToNop<hkpWorldPostSimulationListener> for gfc__PhysicsDetectRegion {}

impl gfc__PhysicsDetectRegion {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).postSimulationCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inactiveEntityMovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).inactiveEntityMovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn bodyEntered(&self, a1: *mut gfc__Body) {
        ((*self.vfptr).bodyEntered)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn bodyExited(
        &self,
        a1: *mut gfc__Body,
        a2: *mut gfc__WorldObject,
    ) {
        ((*self.vfptr).bodyExited)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback_2(&self) {
        ((*self.vfptr).postSimulationCallback_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn collidableAddedCallback(
        &self,
        a1: *const hkpCollidableAddedEvent,
    ) {
        ((*self.vfptr).collidableAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collidableRemovedCallback(
        &self,
        a1: *const hkpCollidableRemovedEvent,
    ) {
        ((*self.vfptr).collidableRemovedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion, _: *mut hkpEntity),
    pub addToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion, _: *mut gfc__World),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion),
    pub bodyEntered:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion, _: *mut gfc__Body),
    pub bodyExited: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion,
        _: *mut gfc__Body,
        _: *mut gfc__WorldObject,
    ),
    pub postSimulationCallback_2: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsDetectRegion),
    pub collidableAddedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion,
        _: *const hkpCollidableAddedEvent,
    ),
    pub collidableRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion,
        _: *const hkpCollidableRemovedEvent,
    ),
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion__BodyInfo {
    pub mBody: gfc__AutoRef_gfc__Body_,
    pub mObject: gfc__AutoRef_gfc__WorldObject_,
    pub mAABBState: u8,
    pub mShapeState: u8,
    pub mAABBStatePrev: u8,
    pub mShapeStatePrev: u8,
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy {
    // hkpPhantomOverlapListener
    pub vfptr: *const gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy__vftable,
    // gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy
}

unsafe impl UpcastToNop<hkpPhantomOverlapListener>
    for gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy
{
}

impl gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy {
    pub unsafe extern "thiscall" fn collidableAddedCallback(
        &self,
        a1: *const hkpCollidableAddedEvent,
    ) {
        ((*self.vfptr).collidableAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collidableRemovedCallback(
        &self,
        a1: *const hkpCollidableRemovedEvent,
    ) {
        ((*self.vfptr).collidableRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy__vftable {
    pub collidableAddedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
        _: *const hkpCollidableAddedEvent,
    ),
    pub collidableRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
        _: *const hkpCollidableRemovedEvent,
    ),
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
        _: u32,
    ) -> *mut (),
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhysicsDetectRegion__BodyInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysicsDetectRegion__BodyInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Actor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Actor_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__TraversalWaypoint__UnreachableWaypoint_0_gfc__CAllocator_ {
    pub mData: *mut gfc__TraversalWaypoint__UnreachableWaypoint,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TraversalWaypoint {
    // gfc__IRefObject
    pub vfptr: *const gfc__TraversalWaypoint__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__TraversalWaypoint
    pub mSeed: i32,
    pub mWaypointType: i32,
    pub mOpenSpaceID: i32,
    pub mLinks: gfc__Vector_gfc__AutoRef_gfc__TraversalLink__0_gfc__CAllocator_,
    pub mUnreachableWaypoints:
        gfc__Vector_gfc__TraversalWaypoint__UnreachableWaypoint_0_gfc__CAllocator_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mUpVector: gfc__TVector3_float_gfc__FloatMath_,
    pub mInvTransform: gfc__Matrix4,
    pub mBoundingBox: gfc__TBox_float_gfc__FloatMath_,
    pub mScale: f32,
    pub mGizmo: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__TraversalWaypoint {}

unsafe impl UpcastToNop<gfc__Object> for gfc__TraversalWaypoint {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__TraversalWaypoint {}

impl gfc__TraversalWaypoint {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__TraversalWaypoint) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__TraversalWaypoint) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__TraversalWaypoint,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub removeFromPathFinding: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__TraversalWaypoint,
    ),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__TraversalWaypoint),
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__UnreachableWaypoint {
    pub waypoint: gfc__AutoRef_gfc__TraversalWaypoint_,
    pub pathFlags: u32,
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__TraversalWaypointGizmo {
    // gfc__SceneObject
    pub vfptr: *const gfc__TraversalWaypoint__TraversalWaypointGizmo__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__TraversalWaypoint__TraversalWaypointGizmo__vftable,
    pub mLocked: bool,
    // gfc__Gizmo
    pub mObject: *mut gfc__WorldObject,
    // gfc__WorldObjectGizmo
    pub mGizmoColor: gfc__TVector4_float_gfc__FloatMath_,
    // gfc__TraversalWaypoint__TraversalWaypointGizmo
}

unsafe impl UpcastToNop<gfc__WorldObjectGizmo> for gfc__TraversalWaypoint__TraversalWaypointGizmo {}

unsafe impl UpcastToNop<gfc__Gizmo> for gfc__TraversalWaypoint__TraversalWaypointGizmo {}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__TraversalWaypoint__TraversalWaypointGizmo {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__TraversalWaypoint__TraversalWaypointGizmo {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__TraversalWaypoint__TraversalWaypointGizmo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn startGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).startGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finishGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).finishGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn prepGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cullAndSubmit(
        &self,
        a1: *const gfc__Clipper,
        a2: *mut gfc__Camera3D,
        a3: u32,
    ) {
        ((*self.vfptr).cullAndSubmit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D, a2: bool, a3: u32) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submitHidden(&self, a1: *mut gfc__Camera3D, a2: u32) {
        ((*self.vfptr).submitHidden)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isHighPriority(&self) -> bool {
        ((*self.vfptr).isHighPriority)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn writeText(&self, a1: *mut gfc__AutoRef_gfc__OutputStream_) {
        ((*self.vfptr).writeText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIsSky(&self, a1: bool) {
        ((*self.vfptr).setIsSky)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getIsSky(&self) -> bool {
        ((*self.vfptr).getIsSky)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__TraversalWaypointGizmo__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: u32,
    ) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__RenderNode,
    ),
    pub finishGeometry: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__RenderNode,
    ),
    pub prepGeometry: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__RenderNode,
    ),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub getContext: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(
        this: *const gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(
        this: *const gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
        _: bool,
    ),
    pub getIsSky: unsafe extern "thiscall" fn(
        this: *const gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
    pub getHide: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(
        this: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
    ) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__CShape___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__CShape,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__CollisionObject,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__CollisionManager {
    // gfc__IRefObject
    pub vfptr: *const gfc__CollisionManager__vftable,
    pub ReferenceCount: i32,
    // gfc__CollisionManager
    pub mObjects: gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_,
    pub mQueryResult: gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CollisionManager {}

impl gfc__CollisionManager {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CollisionManager__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CollisionManager, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CameraCinematicGroup_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__FullScreenFXGroup_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CinematicGroup_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct std___Pair_base_gfc__String_const__gfc__StateMapValue_ {
    pub first: gfc__String,
    pub second: gfc__StateMapValue,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Myval: std__pair_gfc__String_const__gfc__StateMapValue_,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__StateMapValue_ {
    // std___Pair_base_gfc__String_const__gfc__StateMapValue_
    pub first: gfc__String,
    pub second: gfc__StateMapValue,
    // std__pair_gfc__String_const__gfc__StateMapValue_
}

unsafe impl UpcastToNop<std___Pair_base_gfc__String_const__gfc__StateMapValue_>
    for std__pair_gfc__String_const__gfc__StateMapValue_
{
}

#[repr(C)]
pub struct gfc__ModSysContainerModule {
    // gfc__IRefObject
    pub vfptr: *const gfc__ModSysContainerModule__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__VisScriptEntity
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    // gfc__VisScriptModule
    __pdbindgen_padding: [u8; 4],
    pub mEventLinks: gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_,
    pub mInputLinks: gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_,
    pub mVariableLinks: gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_,
    // gfc__ModSysContainerModule
    __pdbindgen_padding_2: [u8; 4],
    pub mEnable: bool,
    pub mIncludeName: gfc__HString,
    pub mInternalModuleSystem: gfc__AutoRef_gfc__ModuleSystem_,
    pub mInputModule: gfc__AutoRef_gfc__InputModule_,
    pub mActionNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mEventNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mVariableConnections:
        gfc__Vector_gfc__AutoRef_gfc__VariableConnectionInfo__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__VisScriptModule> for gfc__ModSysContainerModule {}

unsafe impl UpcastToNop<gfc__VisScriptEntity> for gfc__ModSysContainerModule {}

unsafe impl UpcastToNop<gfc__Object> for gfc__ModSysContainerModule {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ModSysContainerModule {}

impl gfc__ModSysContainerModule {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getUILabel(&self) -> *const i8 {
        ((*self.vfptr).getUILabel)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn compile(&self, a1: *mut gfc__ModuleSystem) {
        ((*self.vfptr).compile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn begin(&self, a1: *mut gfc__Object) {
        ((*self.vfptr).begin)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn end(&self) {
        ((*self.vfptr).end)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clearDeadLinks(&self, a1: *mut gfc__ModuleSystem) {
        ((*self.vfptr).clearDeadLinks)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCategory(&self) -> i32 {
        ((*self.vfptr).getCategory)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNumActions(&self) -> i32 {
        ((*self.vfptr).getNumActions)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActionID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getActionID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getActionName(&self, a1: i32) -> *const i8 {
        ((*self.vfptr).getActionName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumEvents(&self) -> i32 {
        ((*self.vfptr).getNumEvents)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getEventID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getEventID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEventName(&self, a1: i32) -> *const i8 {
        ((*self.vfptr).getEventName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumVariableConnections(&self) -> i32 {
        ((*self.vfptr).getNumVariableConnections)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVariableConnectionID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getVariableConnectionID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVariableConnectionInfo(
        &self,
        result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_ {
        ((*self.vfptr).getVariableConnectionInfo)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn doEvent(&self, a1: u32) {
        ((*self.vfptr).doEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn execute(&self, a1: u32) {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVariableValue(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: u32,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getVariableValue)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setVariableValue(&self, a1: u32, a2: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setVariableValue)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn tryAgain(&self) {
        ((*self.vfptr).tryAgain)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVariablesIn(
        &self,
        result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
        a2: u32,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_ {
        ((*self.vfptr).getVariablesIn)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getVariablesOut(
        &self,
        result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
        a2: u32,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_ {
        ((*self.vfptr).getVariablesOut)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn executeInternal(&self, a1: u32) {
        ((*self.vfptr).executeInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn hasVariableIn(&self, a1: u32) -> bool {
        ((*self.vfptr).hasVariableIn)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn hasVariableOut(&self, a1: u32) -> bool {
        ((*self.vfptr).hasVariableOut)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__ModSysContainerModule__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, result: *mut gfc__HString) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *mut gfc__ObjectCloner, _: gfc__AutoRef_gfc__Object_),
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> *const i8,
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule),
    pub clearDeadLinks: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: *mut gfc__ModuleSystem),
    pub getCategory: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> i32,
    pub getNumActions: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> i32,
    pub getActionID: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, _: i32) -> u32,
    pub getActionName: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, _: i32) -> *const i8,
    pub getNumEvents: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> i32,
    pub getEventID: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, _: i32) -> u32,
    pub getEventName: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, _: i32) -> *const i8,
    pub getNumVariableConnections: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule) -> i32,
    pub getVariableConnectionID: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, _: i32) -> u32,
    pub getVariableConnectionInfo: unsafe extern "thiscall" fn(this: *const gfc__ModSysContainerModule, result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_, _: i32) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub doEvent: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32),
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32),
    pub getVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, result: *mut gfc__AutoRef_gfc__Value_, _: u32) -> *mut gfc__AutoRef_gfc__Value_,
    pub setVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32, _: gfc__AutoRef_gfc__Value_),
    pub tryAgain: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule),
    pub getVariablesIn: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub getVariablesOut: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub executeInternal: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32),
    pub hasVariableIn: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32) -> bool,
    pub hasVariableOut: unsafe extern "thiscall" fn(this: *mut gfc__ModSysContainerModule, _: u32) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__VisScriptVariable_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__InputModule_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DebugOutModule {
    // gfc__IRefObject
    pub vfptr: *const gfc__DebugOutModule__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__VisScriptEntity
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    // gfc__VisScriptModule
    __pdbindgen_padding: [u8; 4],
    pub mEventLinks: gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_,
    pub mInputLinks: gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_,
    pub mVariableLinks: gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_,
    // gfc__DebugOutModule
    pub mMessage: gfc__HString,
}

unsafe impl UpcastToNop<gfc__VisScriptModule> for gfc__DebugOutModule {}

unsafe impl UpcastToNop<gfc__VisScriptEntity> for gfc__DebugOutModule {}

unsafe impl UpcastToNop<gfc__Object> for gfc__DebugOutModule {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__DebugOutModule {}

impl gfc__DebugOutModule {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getUILabel(&self) -> *const i8 {
        ((*self.vfptr).getUILabel)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn compile(&self, a1: *mut gfc__ModuleSystem) {
        ((*self.vfptr).compile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn begin(&self, a1: *mut gfc__Object) {
        ((*self.vfptr).begin)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn end(&self) {
        ((*self.vfptr).end)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clearDeadLinks(&self, a1: *mut gfc__ModuleSystem) {
        ((*self.vfptr).clearDeadLinks)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCategory(&self) -> i32 {
        ((*self.vfptr).getCategory)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNumActions(&self) -> i32 {
        ((*self.vfptr).getNumActions)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActionID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getActionID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getActionName(&self, a1: i32) -> *const i8 {
        ((*self.vfptr).getActionName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumEvents(&self) -> i32 {
        ((*self.vfptr).getNumEvents)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getEventID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getEventID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEventName(&self, a1: i32) -> *const i8 {
        ((*self.vfptr).getEventName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNumVariableConnections(&self) -> i32 {
        ((*self.vfptr).getNumVariableConnections)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVariableConnectionID(&self, a1: i32) -> u32 {
        ((*self.vfptr).getVariableConnectionID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVariableConnectionInfo(
        &self,
        result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_ {
        ((*self.vfptr).getVariableConnectionInfo)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn doEvent(&self, a1: u32) {
        ((*self.vfptr).doEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn execute(&self, a1: u32) {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVariableValue(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: u32,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getVariableValue)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setVariableValue(&self, a1: u32, a2: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setVariableValue)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn tryAgain(&self) {
        ((*self.vfptr).tryAgain)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVariablesIn(
        &self,
        result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
        a2: u32,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_ {
        ((*self.vfptr).getVariablesIn)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getVariablesOut(
        &self,
        result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
        a2: u32,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_ {
        ((*self.vfptr).getVariablesOut)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn executeInternal(&self, a1: u32) {
        ((*self.vfptr).executeInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn hasVariableIn(&self, a1: u32) -> bool {
        ((*self.vfptr).hasVariableIn)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn hasVariableOut(&self, a1: u32) -> bool {
        ((*self.vfptr).hasVariableOut)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DebugOutModule__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, result: *mut gfc__HString) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *mut gfc__ObjectCloner, _: gfc__AutoRef_gfc__Object_),
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> *const i8,
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule),
    pub clearDeadLinks: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: *mut gfc__ModuleSystem),
    pub getCategory: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> i32,
    pub getNumActions: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> i32,
    pub getActionID: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, _: i32) -> u32,
    pub getActionName: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, _: i32) -> *const i8,
    pub getNumEvents: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> i32,
    pub getEventID: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, _: i32) -> u32,
    pub getEventName: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, _: i32) -> *const i8,
    pub getNumVariableConnections: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule) -> i32,
    pub getVariableConnectionID: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, _: i32) -> u32,
    pub getVariableConnectionInfo: unsafe extern "thiscall" fn(this: *const gfc__DebugOutModule, result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_, _: i32) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub doEvent: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32),
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32),
    pub getVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, result: *mut gfc__AutoRef_gfc__Value_, _: u32) -> *mut gfc__AutoRef_gfc__Value_,
    pub setVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32, _: gfc__AutoRef_gfc__Value_),
    pub tryAgain: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule),
    pub getVariablesIn: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub getVariablesOut: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub executeInternal: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32),
    pub hasVariableIn: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32) -> bool,
    pub hasVariableOut: unsafe extern "thiscall" fn(this: *mut gfc__DebugOutModule, _: u32) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__VariableConnectionInfo__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VisScriptVariable_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__ScriptState_,
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__ScriptState_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___>
    for std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState___
{
}

#[repr(C)]
pub struct gfc__Vector_gfc__InsRun__CachedEnv_0_gfc__CAllocator_ {
    pub mData: *mut gfc__InsRun__CachedEnv,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__InsExecutor {
    pub vfptr: *const gfc__InsExecutor__vftable,
    pub mFunctions: gfc__AutoRef_gfc__ScriptFunctions_,
    pub mCurrentFunction: gfc__AutoRef_gfc__ScriptFunction_,
    pub mBuf: *mut u8,
    pub mPC: i32,
    pub mLineNumber: i32,
    pub mNeedsSwap: bool,
    pub mResult: bool,
    pub mReturn: bool,
    pub mMaxInstructions: u32,
}

impl gfc__InsExecutor {
    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn run(&self, a1: *const gfc__HString) -> bool {
        ((*self.vfptr).run)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn execute(&self, a1: gfc__Instruction) -> bool {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAdd(&self) -> bool {
        ((*self.vfptr).doAdd)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doSubtract(&self) -> bool {
        ((*self.vfptr).doSubtract)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMultiply(&self) -> bool {
        ((*self.vfptr).doMultiply)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDivide(&self) -> bool {
        ((*self.vfptr).doDivide)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCat(&self) -> bool {
        ((*self.vfptr).doCat)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAnd(&self) -> bool {
        ((*self.vfptr).doAnd)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doOr(&self) -> bool {
        ((*self.vfptr).doOr)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doModulo(&self) -> bool {
        ((*self.vfptr).doModulo)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBranchFalse(&self) -> bool {
        ((*self.vfptr).doBranchFalse)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBranchTrue(&self) -> bool {
        ((*self.vfptr).doBranchTrue)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doVar(&self) -> bool {
        ((*self.vfptr).doVar)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doVarAssign(&self) -> bool {
        ((*self.vfptr).doVarAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doParamAssign(&self) -> bool {
        ((*self.vfptr).doParamAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAssign(&self) -> bool {
        ((*self.vfptr).doAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGoto(&self) -> bool {
        ((*self.vfptr).doGoto)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIncScope(&self) -> bool {
        ((*self.vfptr).doIncScope)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDecScope(&self) -> bool {
        ((*self.vfptr).doDecScope)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doFloat(&self) -> bool {
        ((*self.vfptr).doFloat)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doInt(&self) -> bool {
        ((*self.vfptr).doInt)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doString(&self) -> bool {
        ((*self.vfptr).doString)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBool(&self) -> bool {
        ((*self.vfptr).doBool)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNullObject(&self) -> bool {
        ((*self.vfptr).doNullObject)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRand(&self) -> bool {
        ((*self.vfptr).doRand)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRandRange(&self) -> bool {
        ((*self.vfptr).doRandRange)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIdentifier(&self) -> bool {
        ((*self.vfptr).doIdentifier)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLabel(&self) -> bool {
        ((*self.vfptr).doLabel)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doReturn(&self) -> bool {
        ((*self.vfptr).doReturn)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doReturnNull(&self) -> bool {
        ((*self.vfptr).doReturnNull)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doPrint(&self) -> bool {
        ((*self.vfptr).doPrint)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLess(&self) -> bool {
        ((*self.vfptr).doLess)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGreater(&self) -> bool {
        ((*self.vfptr).doGreater)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doEqual(&self) -> bool {
        ((*self.vfptr).doEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNotEqual(&self) -> bool {
        ((*self.vfptr).doNotEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGreaterEqual(&self) -> bool {
        ((*self.vfptr).doGreaterEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLessEqual(&self) -> bool {
        ((*self.vfptr).doLessEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNot(&self) -> bool {
        ((*self.vfptr).doNot)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNegate(&self) -> bool {
        ((*self.vfptr).doNegate)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIntCast(&self) -> bool {
        ((*self.vfptr).doIntCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doFloatCast(&self) -> bool {
        ((*self.vfptr).doFloatCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBoolCast(&self) -> bool {
        ((*self.vfptr).doBoolCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doStringCast(&self) -> bool {
        ((*self.vfptr).doStringCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCall(&self) -> bool {
        ((*self.vfptr).doCall)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCallMethod(&self) -> bool {
        ((*self.vfptr).doCallMethod)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMethod(&self) -> bool {
        ((*self.vfptr).doMethod)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doPop(&self) -> bool {
        ((*self.vfptr).doPop)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNew(&self) -> bool {
        ((*self.vfptr).doNew)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNewValue(&self) -> bool {
        ((*self.vfptr).doNewValue)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNop(&self) -> bool {
        ((*self.vfptr).doNop)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doInc(&self) -> bool {
        ((*self.vfptr).doInc)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDec(&self) -> bool {
        ((*self.vfptr).doDec)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAddAssign(&self) -> bool {
        ((*self.vfptr).doAddAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doSubAssign(&self) -> bool {
        ((*self.vfptr).doSubAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMulAssign(&self) -> bool {
        ((*self.vfptr).doMulAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDivAssign(&self) -> bool {
        ((*self.vfptr).doDivAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doModAssign(&self) -> bool {
        ((*self.vfptr).doModAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMethodChain(&self) -> bool {
        ((*self.vfptr).doMethodChain)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doProperty(&self) -> bool {
        ((*self.vfptr).doProperty)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLine(&self) -> bool {
        ((*self.vfptr).doLine)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doArray(&self) -> bool {
        ((*self.vfptr).doArray)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doElement(&self) -> bool {
        ((*self.vfptr).doElement)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doContains(&self) -> bool {
        ((*self.vfptr).doContains)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRemove(&self) -> bool {
        ((*self.vfptr).doRemove)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIterator(&self) -> bool {
        ((*self.vfptr).doIterator)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIteratorTest(&self) -> bool {
        ((*self.vfptr).doIteratorTest)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIteratorAssign(&self) -> bool {
        ((*self.vfptr).doIteratorAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn validateParamCount(
        &self,
        a1: i32,
        a2: *const gfc__HString,
    ) -> bool {
        ((*self.vfptr).validateParamCount)(self as *const _ as *mut _, a1, a2)
    }

    #[cfg(pdb_issue = "malformed PDB: type index 0")]
    pub unsafe extern "C" fn scriptError(
        &self,
        a1: *const i8,
        a2: compile_error!("malformed PDB: type index 0"),
    ) {
        ((*self.vfptr).scriptError)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__InsExecutor__vftable {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field scriptError")]
#[repr(C)]
pub struct gfc__InsExecutor__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor, _: u32) -> *mut (),
    pub run:
        unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor, _: *const gfc__HString) -> bool,
    pub execute:
        unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor, _: gfc__Instruction) -> bool,
    pub doAdd: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doSubtract: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doMultiply: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doDivide: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doCat: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doAnd: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doOr: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doModulo: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doBranchFalse: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doBranchTrue: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doVar: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doVarAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doParamAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doGoto: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIncScope: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doDecScope: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doFloat: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doInt: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doString: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doBool: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNullObject: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doRand: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doRandRange: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIdentifier: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doLabel: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doReturn: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doReturnNull: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doPrint: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doLess: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doGreater: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNotEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doGreaterEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doLessEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNot: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNegate: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIntCast: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doFloatCast: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doBoolCast: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doStringCast: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doCall: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doCallMethod: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doMethod: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doPop: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNew: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNewValue: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doNop: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doInc: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doDec: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doAddAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doSubAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doMulAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doDivAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doModAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doMethodChain: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doProperty: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doLine: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doArray: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doElement: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doContains: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doRemove: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIterator: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIteratorTest: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub doIteratorAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsExecutor) -> bool,
    pub validateParamCount: unsafe extern "thiscall" fn(
        this: *mut gfc__InsExecutor,
        _: i32,
        _: *const gfc__HString,
    ) -> bool,
    #[cfg(pdb_issue = "malformed PDB: type index 0")]
    pub scriptError: unsafe extern "C" fn(
        this: *mut gfc__InsExecutor,
        _: *const i8,
        _: compile_error!("malformed PDB: type index 0"),
    ),
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__InsRun__CachedEnv_20_0_gfc__CAllocator_ {
    // gfc__Vector_gfc__InsRun__CachedEnv_0_gfc__CAllocator_
    pub mData: *mut gfc__InsRun__CachedEnv,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__InsRun__CachedEnv_20_0_gfc__CAllocator_
    pub mFixedData: [u8; 160],
}

unsafe impl UpcastToNop<gfc__Vector_gfc__InsRun__CachedEnv_0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__InsRun__CachedEnv_20_0_gfc__CAllocator_
{
}

#[repr(C)]
pub struct gfc__InsRun {
    // gfc__InsExecutor
    pub vfptr: *const gfc__InsRun__vftable,
    pub mFunctions: gfc__AutoRef_gfc__ScriptFunctions_,
    pub mCurrentFunction: gfc__AutoRef_gfc__ScriptFunction_,
    pub mBuf: *mut u8,
    pub mPC: i32,
    pub mLineNumber: i32,
    pub mNeedsSwap: bool,
    pub mResult: bool,
    pub mReturn: bool,
    pub mMaxInstructions: u32,
    // gfc__InsRun
    pub mSourceName: gfc__HString,
    pub mStack: *mut gfc__ValueStack,
    pub mEnvCurrent: gfc__AutoRef_gfc__Environment_,
    pub mEnvBase: gfc__AutoRef_gfc__Environment_,
    pub mEnvCache: gfc__FixedVector_gfc__InsRun__CachedEnv_20_0_gfc__CAllocator_,
    pub mMethodCallValue: gfc__AutoRef_gfc__Value_,
    pub mMethodCallName: gfc__AutoRef_gfc__Value_,
}

unsafe impl UpcastToNop<gfc__InsExecutor> for gfc__InsRun {}

impl gfc__InsRun {
    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn run(&self, a1: *const gfc__HString) -> bool {
        ((*self.vfptr).run)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn execute(&self, a1: gfc__Instruction) -> bool {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAdd(&self) -> bool {
        ((*self.vfptr).doAdd)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doSubtract(&self) -> bool {
        ((*self.vfptr).doSubtract)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMultiply(&self) -> bool {
        ((*self.vfptr).doMultiply)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDivide(&self) -> bool {
        ((*self.vfptr).doDivide)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCat(&self) -> bool {
        ((*self.vfptr).doCat)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAnd(&self) -> bool {
        ((*self.vfptr).doAnd)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doOr(&self) -> bool {
        ((*self.vfptr).doOr)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doModulo(&self) -> bool {
        ((*self.vfptr).doModulo)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBranchFalse(&self) -> bool {
        ((*self.vfptr).doBranchFalse)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBranchTrue(&self) -> bool {
        ((*self.vfptr).doBranchTrue)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doVar(&self) -> bool {
        ((*self.vfptr).doVar)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doVarAssign(&self) -> bool {
        ((*self.vfptr).doVarAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doParamAssign(&self) -> bool {
        ((*self.vfptr).doParamAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAssign(&self) -> bool {
        ((*self.vfptr).doAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGoto(&self) -> bool {
        ((*self.vfptr).doGoto)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIncScope(&self) -> bool {
        ((*self.vfptr).doIncScope)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDecScope(&self) -> bool {
        ((*self.vfptr).doDecScope)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doFloat(&self) -> bool {
        ((*self.vfptr).doFloat)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doInt(&self) -> bool {
        ((*self.vfptr).doInt)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doString(&self) -> bool {
        ((*self.vfptr).doString)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBool(&self) -> bool {
        ((*self.vfptr).doBool)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNullObject(&self) -> bool {
        ((*self.vfptr).doNullObject)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRand(&self) -> bool {
        ((*self.vfptr).doRand)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRandRange(&self) -> bool {
        ((*self.vfptr).doRandRange)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIdentifier(&self) -> bool {
        ((*self.vfptr).doIdentifier)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLabel(&self) -> bool {
        ((*self.vfptr).doLabel)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doReturn(&self) -> bool {
        ((*self.vfptr).doReturn)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doReturnNull(&self) -> bool {
        ((*self.vfptr).doReturnNull)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doPrint(&self) -> bool {
        ((*self.vfptr).doPrint)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLess(&self) -> bool {
        ((*self.vfptr).doLess)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGreater(&self) -> bool {
        ((*self.vfptr).doGreater)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doEqual(&self) -> bool {
        ((*self.vfptr).doEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNotEqual(&self) -> bool {
        ((*self.vfptr).doNotEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doGreaterEqual(&self) -> bool {
        ((*self.vfptr).doGreaterEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLessEqual(&self) -> bool {
        ((*self.vfptr).doLessEqual)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNot(&self) -> bool {
        ((*self.vfptr).doNot)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNegate(&self) -> bool {
        ((*self.vfptr).doNegate)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIntCast(&self) -> bool {
        ((*self.vfptr).doIntCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doFloatCast(&self) -> bool {
        ((*self.vfptr).doFloatCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doBoolCast(&self) -> bool {
        ((*self.vfptr).doBoolCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doStringCast(&self) -> bool {
        ((*self.vfptr).doStringCast)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCall(&self) -> bool {
        ((*self.vfptr).doCall)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doCallMethod(&self) -> bool {
        ((*self.vfptr).doCallMethod)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMethod(&self) -> bool {
        ((*self.vfptr).doMethod)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doPop(&self) -> bool {
        ((*self.vfptr).doPop)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNew(&self) -> bool {
        ((*self.vfptr).doNew)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNewValue(&self) -> bool {
        ((*self.vfptr).doNewValue)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doNop(&self) -> bool {
        ((*self.vfptr).doNop)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doInc(&self) -> bool {
        ((*self.vfptr).doInc)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDec(&self) -> bool {
        ((*self.vfptr).doDec)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doAddAssign(&self) -> bool {
        ((*self.vfptr).doAddAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doSubAssign(&self) -> bool {
        ((*self.vfptr).doSubAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMulAssign(&self) -> bool {
        ((*self.vfptr).doMulAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doDivAssign(&self) -> bool {
        ((*self.vfptr).doDivAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doModAssign(&self) -> bool {
        ((*self.vfptr).doModAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doMethodChain(&self) -> bool {
        ((*self.vfptr).doMethodChain)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doProperty(&self) -> bool {
        ((*self.vfptr).doProperty)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doLine(&self) -> bool {
        ((*self.vfptr).doLine)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doArray(&self) -> bool {
        ((*self.vfptr).doArray)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doElement(&self) -> bool {
        ((*self.vfptr).doElement)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doContains(&self) -> bool {
        ((*self.vfptr).doContains)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doRemove(&self) -> bool {
        ((*self.vfptr).doRemove)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIterator(&self) -> bool {
        ((*self.vfptr).doIterator)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIteratorTest(&self) -> bool {
        ((*self.vfptr).doIteratorTest)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn doIteratorAssign(&self) -> bool {
        ((*self.vfptr).doIteratorAssign)(self as *const _ as *mut _)
    }

    #[cfg(pdb_issue = "error in field scriptError")]
    pub unsafe extern "thiscall" fn validateParamCount(
        &self,
        a1: i32,
        a2: *const gfc__HString,
    ) -> bool {
        ((*self.vfptr).validateParamCount)(self as *const _ as *mut _, a1, a2)
    }

    #[cfg(pdb_issue = "malformed PDB: type index 0")]
    pub unsafe extern "C" fn scriptError(
        &self,
        a1: *const i8,
        a2: compile_error!("malformed PDB: type index 0"),
    ) {
        ((*self.vfptr).scriptError)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__InsRun__vftable {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field scriptError")]
#[repr(C)]
pub struct gfc__InsRun__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__InsRun, _: u32) -> *mut (),
    pub run: unsafe extern "thiscall" fn(this: *mut gfc__InsRun, _: *const gfc__HString) -> bool,
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__InsRun, _: gfc__Instruction) -> bool,
    pub doAdd: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doSubtract: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doMultiply: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doDivide: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doCat: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doAnd: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doOr: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doModulo: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doBranchFalse: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doBranchTrue: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doVar: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doVarAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doParamAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doGoto: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIncScope: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doDecScope: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doFloat: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doInt: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doString: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doBool: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNullObject: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doRand: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doRandRange: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIdentifier: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doLabel: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doReturn: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doReturnNull: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doPrint: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doLess: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doGreater: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNotEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doGreaterEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doLessEqual: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNot: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNegate: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIntCast: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doFloatCast: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doBoolCast: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doStringCast: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doCall: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doCallMethod: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doMethod: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doPop: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNew: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNewValue: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doNop: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doInc: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doDec: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doAddAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doSubAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doMulAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doDivAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doModAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doMethodChain: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doProperty: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doLine: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doArray: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doElement: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doContains: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doRemove: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIterator: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIteratorTest: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub doIteratorAssign: unsafe extern "thiscall" fn(this: *mut gfc__InsRun) -> bool,
    pub validateParamCount:
        unsafe extern "thiscall" fn(this: *mut gfc__InsRun, _: i32, _: *const gfc__HString) -> bool,
    #[cfg(pdb_issue = "malformed PDB: type index 0")]
    pub scriptError: unsafe extern "C" fn(
        this: *mut gfc__InsRun,
        _: *const i8,
        _: compile_error!("malformed PDB: type index 0"),
    ),
}

#[repr(C)]
pub struct gfc__InsRun__CachedEnv {
    pub PC: i32,
    pub Env: gfc__AutoRef_gfc__Environment_,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__HStringManager__StringRef_gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: u64,
    pub mValue: gfc__HStringManager__StringRef,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: u64,
    pub mValue: gfc__AutoRef_gfc__Variable_,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: u64,
    pub mValue: gfc__AutoRef_gfc__Property_,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: u64,
    pub mValue: gfc__AutoRef_gfc__Method_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ClassLoader_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__OOObjectWriter {
    // gfc__IRefObject
    pub vfptr: *const gfc__OOObjectWriter__vftable,
    pub ReferenceCount: i32,
    // gfc__ObjectWriter
    // gfc__OOObjectWriter
    pub mObjectDatabase: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mWriteDefaults: bool,
}

unsafe impl UpcastToNop<gfc__ObjectWriter> for gfc__OOObjectWriter {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__OOObjectWriter {}

impl gfc__OOObjectWriter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__OOObjectWriter__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__OOObjectWriter, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: gfc__HString,
    pub mValue: gfc__AutoRef_gfc__Class_,
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_ {
    // CCallbackBase
    pub vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_24_
    // CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_
    pub m_pObj: *mut keen__ISteamStatsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsReceived_t,
    ),
}

unsafe impl UpcastToNop<CCallbackImpl_24_>
    for CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_
{
}

unsafe impl UpcastToNop<CCallbackBase>
    for CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_
{
}

impl CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0___vftable {
    pub Run: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
        _: *mut (),
        _: bool,
        _: u64,
    ),
    pub Run_2: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
        _: *mut (),
    ),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    ) -> i32,
}

#[repr(C)]
pub struct UserStatsReceived_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
    pub m_steamIDUser: CSteamID,
}

#[repr(C)]
pub struct CCallbackImpl_16_ {
    // CCallbackBase
    pub vfptr: *const CCallbackImpl_16___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_16_
}

unsafe impl UpcastToNop<CCallbackBase> for CCallbackImpl_16_ {}

impl CCallbackImpl_16_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallbackImpl_16___vftable {
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_16_, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_16_, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_16_) -> i32,
}

#[repr(C)]
pub struct UserAchievementStored_t {
    pub m_nGameID: u64,
    pub m_bGroupAchievement: bool,
    pub m_rgchAchievementName: [i8; 128],
    pub m_nCurProgress: u32,
    pub m_nMaxProgress: u32,
}

#[repr(C)]
pub struct unit4__SystemServicesUserId {
    pub providerData: [i8; 36],
    pub guestIndex: u8,
    pub isValid: bool,
}

#[repr(C)]
pub struct unit4__RankingReceiveData {
    pub cachedBoards: keen__Array_unit4__RankingBoardCacheEntry_,
    pub servingFromCache: bool,
    pub downloadInProgress: bool,
    pub tableId: u16,
    pub user: keen__UserAccount,
    pub r#type: unit4__RankingListQueryType,
    pub offset: u32,
    pub entriesToQuery: u32,
    pub cacheToUseForResult: u32,
}

#[repr(C)]
pub struct unit4__SystemServicesUserInfo {
    pub id: unit4__SystemServicesUserId,
    pub localAccountId: keen__UserAccountId,
    pub displayName: [i8; 64],
}

#[repr(C)]
pub struct unit4__RankingListEntry {
    pub userInfo: unit4__SystemServicesUserInfo,
    pub rank: u32,
    pub score: u32,
}

#[repr(C)]
pub struct unit4__SystemServicesInteractionData {
    pub id: u32,
    pub responseOptions: [u32; 2],
    pub responseOptionCount: u32,
    pub user: keen__UserAccountId,
}

#[repr(C)]
pub struct unit4__SystemServicesBase {
    pub currentTimeInMs: u32,
    pub rankingSendData: unit4__RankingSendData,
    pub rankingReceiveData: unit4__RankingReceiveData,
    pub currentRankingError: unit4__RankingError,
    pub rankingSendInteraction: unit4__SystemServicesInteractionData,
    pub onlineInteraction: unit4__SystemServicesInteractionData,
}

#[repr(C)]
pub struct unit4__RankingSendData {
    pub sendInPogress: bool,
    pub tableId: u16,
    pub user: keen__UserAccount,
    pub score: u32,
}

#[repr(C)]
pub struct unit4__SystemServices {
    // unit4__SystemServicesBase
    pub currentTimeInMs: u32,
    pub rankingSendData: unit4__RankingSendData,
    pub rankingReceiveData: unit4__RankingReceiveData,
    pub currentRankingError: unit4__RankingError,
    pub rankingSendInteraction: unit4__SystemServicesInteractionData,
    pub onlineInteraction: unit4__SystemServicesInteractionData,
    // unit4__SystemServices
    pub steamAchievements: keen__SteamAchievements,
    pub steamStats: keen__SteamStats,
    pub pPresenceStrings: [*const i8; 64],
    pub presenceStringCount: u32,
    pub sendRankingStep: unit4__SystemServices__SendRankingStep,
    pub sendRankingCall: u64,
    pub receiveRankingStep: unit4__SystemServices__ReceiveRankingStep,
    pub receiveRankingCall: u64,
}

unsafe impl UpcastToNop<unit4__SystemServicesBase> for unit4__SystemServices {}

#[repr(C)]
pub struct unit4__RankingBoardCacheEntry {
    pub tableId: u16,
    pub user: keen__UserAccount,
    pub r#type: unit4__RankingListQueryType,
    pub entries: keen__SizedArray_unit4__RankingListEntry_,
    pub totalEntriesOnBoard: u32,
    pub downloadTimeInMs: u32,
}

#[repr(C)]
pub struct keen__ISteamAchievementsCallback {
    pub vfptr: *const keen__ISteamAchievementsCallback__vftable,
    pub m_onUserAchievementStored:
        CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
}

impl keen__ISteamAchievementsCallback {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserAchievementStored(
        &self,
        a1: *mut UserAchievementStored_t,
    ) {
        ((*self.vfptr).onUserAchievementStored)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct keen__ISteamAchievementsCallback__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__ISteamAchievementsCallback, _: u32) -> *mut (),
    pub onUserAchievementStored: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamAchievementsCallback,
        _: *mut UserAchievementStored_t,
    ),
}

#[repr(C)]
pub struct keen__SteamStats {
    // keen__ISteamStatsCallback
    pub vfptr: *const keen__SteamStats__vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
    // keen__SteamStats
    pub m_pSteamUserStats: *mut ISteamUserStats,
    pub m_gameSteamID64: u64,
    pub m_pStatUpdateMap: *const unit4__StatUpdateData,
    pub m_statUpdateCount: u32,
    pub m_requestingUserStats: bool,
    pub m_statsAvailable: bool,
    pub m_storingStats: bool,
    pub m_hasUnstoredStats: bool,
    pub m_secondsSinceLastUpdate: f32,
    pub m_hasCached: bool,
    pub m_cachedAdds: [f64; 64],
}

unsafe impl UpcastToNop<keen__ISteamStatsCallback> for keen__SteamStats {}

impl keen__SteamStats {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserStatsReceived(&self, a1: *mut UserStatsReceived_t) {
        ((*self.vfptr).onUserStatsReceived)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserStatsStored(&self, a1: *mut UserStatsStored_t) {
        ((*self.vfptr).onUserStatsStored)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct keen__SteamStats__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut keen__SteamStats, _: u32) -> *mut (),
    pub onUserStatsReceived:
        unsafe extern "thiscall" fn(this: *mut keen__SteamStats, _: *mut UserStatsReceived_t),
    pub onUserStatsStored:
        unsafe extern "thiscall" fn(this: *mut keen__SteamStats, _: *mut UserStatsStored_t),
}

#[repr(C)]
pub struct keen__Array_unit4__RankingBoardCacheEntry_ {
    pub m_pData: *mut unit4__RankingBoardCacheEntry,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__SteamAchievements {
    // keen__ISteamAchievementsCallback
    pub vfptr: *const keen__SteamAchievements__vftable,
    pub m_onUserAchievementStored:
        CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
    // keen__ISteamStatsCallback
    pub vfptr_2: *const keen__SteamAchievements__vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
    // keen__SteamAchievements
    pub m_pSteamUserStats: *mut ISteamUserStats,
    pub m_gameSteamID64: u64,
    pub m_pAchievementIdMap: *const u32,
    pub m_pAchievementUnlockCountMap: *const u32,
    pub m_achievementCount: u32,
    pub m_platinumAchievementId: u32,
    pub m_platinumAchievementUnlocked: bool,
}

unsafe impl UpcastToNop<keen__ISteamAchievementsCallback> for keen__SteamAchievements {}

unsafe impl UpcastTo<keen__ISteamStatsCallback> for keen__SteamAchievements {
    fn upcast_to(p: *const Self) -> *const keen__ISteamStatsCallback {
        (p as usize + 0x18) as *const _
    }
}

impl keen__SteamAchievements {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserStatsReceived(&self, a1: *mut UserStatsReceived_t) {
        ((*self.vfptr).onUserStatsReceived)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct keen__SteamAchievements__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__SteamAchievements, _: u32) -> *mut (),
    pub onUserStatsReceived: unsafe extern "thiscall" fn(
        this: *mut keen__SteamAchievements,
        _: *mut UserStatsReceived_t,
    ),
}

#[repr(C)]
pub struct keen__SizedArray_unit4__RankingListEntry_ {
    pub m_pData: *mut unit4__RankingListEntry,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__ISteamStatsCallback {
    pub vfptr: *const keen__ISteamStatsCallback__vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
}

impl keen__ISteamStatsCallback {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserStatsReceived(&self, a1: *mut UserStatsReceived_t) {
        ((*self.vfptr).onUserStatsReceived)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUserStatsStored(&self, a1: *mut UserStatsStored_t) {
        ((*self.vfptr).onUserStatsStored)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct keen__ISteamStatsCallback__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__ISteamStatsCallback, _: u32) -> *mut (),
    pub onUserStatsReceived: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsReceived_t,
    ),
    pub onUserStatsStored: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsStored_t,
    ),
}

#[repr(C)]
pub struct UserStatsStored_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
}

#[repr(C)]
pub struct CCallbackImpl_152_ {
    // CCallbackBase
    pub vfptr: *const CCallbackImpl_152___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_152_
}

unsafe impl UpcastToNop<CCallbackBase> for CCallbackImpl_152_ {}

impl CCallbackImpl_152_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallbackImpl_152___vftable {
    pub Run:
        unsafe extern "thiscall" fn(this: *mut CCallbackImpl_152_, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_152_, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_152_) -> i32,
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_ {
    // CCallbackBase
    pub vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_16_
    // CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_
    pub m_pObj: *mut keen__ISteamStatsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsStored_t,
    ),
}

unsafe impl UpcastToNop<CCallbackImpl_16_>
    for CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_
{
}

unsafe impl UpcastToNop<CCallbackBase>
    for CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_
{
}

impl CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0___vftable {
    pub Run: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
        _: *mut (),
        _: bool,
        _: u64,
    ),
    pub Run_2: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
        _: *mut (),
    ),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
    ) -> i32,
}

#[repr(C)]
pub struct CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_ {
    // CCallbackBase
    pub vfptr:
        *const CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_152_
    // CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_
    pub m_pObj: *mut keen__ISteamAchievementsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamAchievementsCallback,
        _: *mut UserAchievementStored_t,
    ),
}

unsafe impl UpcastToNop<CCallbackImpl_152_>
    for CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_
{
}

unsafe impl UpcastToNop<CCallbackBase>
    for CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_
{
}

impl CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0___vftable {
    pub Run: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
        _: *mut (),
        _: bool,
        _: u64,
    ),
    pub Run_2: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
        _: *mut (),
    ),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(
        this: *mut CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
    ) -> i32,
}

#[repr(C)]
pub struct CCallbackImpl_24_ {
    // CCallbackBase
    pub vfptr: *const CCallbackImpl_24___vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    // CCallbackImpl_24_
}

unsafe impl UpcastToNop<CCallbackBase> for CCallbackImpl_24_ {}

impl CCallbackImpl_24_ {
    pub unsafe extern "thiscall" fn Run(&self, a1: *mut (), a2: bool, a3: u64) {
        ((*self.vfptr).Run)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn Run_2(&self, a1: *mut ()) {
        ((*self.vfptr).Run_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetCallbackSizeBytes(&self) -> i32 {
        ((*self.vfptr).GetCallbackSizeBytes)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct CCallbackImpl_24___vftable {
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_24_, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_24_, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackImpl_24_) -> i32,
}

#[repr(C)]
pub struct ID3D11DeviceContext {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11DeviceContext */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11DeviceContext {}

unsafe impl UpcastToNop<IUnknown> for ID3D11DeviceContext {}

#[repr(C)]
pub struct IDXGIDeviceSubObject {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* IDXGIObject
     * IDXGIDeviceSubObject */
}

unsafe impl UpcastToNop<IDXGIObject> for IDXGIDeviceSubObject {}

unsafe impl UpcastToNop<IUnknown> for IDXGIDeviceSubObject {}

#[repr(C)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}

#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}

#[repr(C)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: u32,
    pub BufferCount: u32,
    pub OutputWindow: *mut HWND__,
    pub Windowed: i32,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}

#[repr(C)]
pub struct keen__ScreenCapture {
    pub pGraphicsSystem: *mut keen__GraphicsSystem,
    pub pFileSystem: *mut keen__FileSystem,
    pub width: u32,
    pub height: u32,
    pub captureTargetData: keen__TextureData,
    pub captureCopyData: keen__TextureData,
    pub pCaptureTarget: *const keen__RenderTarget,
    pub pNoGammaCaptureTarget: *const keen__RenderTarget,
    pub frameBuffer: keen__Array_unsigned_char___,
    pub frameBufferWriteIndex: u32,
    pub tempBuffer: keen__Array_unsigned_char___,
    pub tempBufferWriteIndex: u32,
    pub tempBufferReadIndex: u32,
    pub immediateRenderer: keen__ImmediateRenderer,
    pub basePath: [i8; 128],
    pub subPath: [i8; 128],
    pub filenamePrefix: [i8; 128],
    pub thread: keen__Thread,
    pub threadStarted: bool,
    pub tempBufferDataCount: keen__Semaphore,
    pub tempBufferFreeCount: keen__Semaphore,
    pub captureCount: u32,
    pub saveFrameIndex: u32,
    pub state: keen__ScreenCaptureState,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__SamplerState_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__RasterizerState_ {
    pub m_allocator: keen__PoolAllocator_keen__RasterizerState_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__RasterizerState_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__TextureData {
    pub description: keen__TextureDescription,
    pub pTexture: *mut ID3D11Resource,
    pub pShaderView: *mut ID3D11ShaderResourceView,
    pub d3dFormat: DXGI_FORMAT,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__DepthStencilState_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__GraphicsSystemBase {
    pub pShaderFactory: *mut keen__ShaderFactory,
    pub pSystemAllocator: *mut keen__MemoryAllocator,
    pub stateObjectPool: keen__CombinedGraphicsStateObjectPool,
    pub stockObjects: keen__StockObjects,
}

#[repr(C)]
pub struct keen__ImmediateRenderer {
    pub m_pCommandWriter: *mut keen__GraphicsCommandWriter,
    pub m_pVertexData: *mut (),
    pub m_pGraphicsSystem: *mut keen__GraphicsSystem,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_pRasterizerStates: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_pBlendStates: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_pDepthStencilState: compile_error!("unimplemented sizeof array"),
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_pSamplerState: compile_error!("unimplemented sizeof array"),
    __pdbindgen_padding: [u8; 172],
    pub m_pShaderParameterBuffer: *mut keen__DynamicConstantBuffer,
    pub m_pFragmentParameterBuffer: *mut keen__DynamicConstantBuffer,
    pub m_pShaderPipeline: *const keen__ShaderPipeline,
    pub m_pShaderPipelineVolumeSlice: *const keen__ShaderPipeline,
    pub m_pVertexFormat: *const keen__VertexFormat,
    pub m_pVertexInputBinding: *const keen__VertexInputBinding,
    pub m_pWhiteTexture: *const keen__TextureData,
    pub m_viewProjectionMatrix: keen__Matrix44,
    pub m_worldMatrix: keen__Matrix43,
    pub m_screenWidth: u32,
    pub m_screenHeight: u32,
    pub m_mvpMatrixDirty: bool,
    pub m_endFinalRenderPass: bool,
    pub m_cullMode: keen__ImmediateCullMode,
    pub m_fillMode: keen__ImmediateFillMode,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_renderPassCameraData: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 912],
    pub m_renderPassCameras: [*const keen__Camera; 4],
    pub m_currentRenderPassStackSize: u32,
}

#[repr(C)]
pub struct keen__Array_unsigned_char___ {
    pub m_pData: *mut *mut u8,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__VertexInputBinding_ {
    pub m_allocator: keen__PoolAllocator_keen__VertexInputBinding_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__DownsampleDepthContext {
    pub shaders: keen__DownsampleDepthVariants,
    pub copyDepthShader: keen__CopyDepthVariants,
    pub pConstantBuffer: *mut keen__DynamicConstantBuffer,
    pub pRasterizerState: *const keen__RasterizerState,
    pub pDepthBufferSamplerState: *const keen__SamplerState,
    pub pVertexInputBinding: *const keen__VertexInputBinding,
    pub pCopyVertexInputBinding: *const keen__VertexInputBinding,
    pub pDepthBlendState: *const keen__BlendState,
    pub pDepthDepthStencilState: *const keen__DepthStencilState,
    pub pStencilBlendState: *const keen__BlendState,
    pub pStencilDepthStencilState: *const keen__DepthStencilState,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub vertexData: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 24],
}

#[repr(C)]
pub struct keen__CombinedGraphicsStateObjectPool {
    pub m_blendStatePool: keen__GraphicsStateObjectPool_keen__BlendState_,
    pub m_rasterizerStatePool: keen__GraphicsStateObjectPool_keen__RasterizerState_,
    pub m_samplerStatePool: keen__GraphicsStateObjectPool_keen__SamplerState_,
    pub m_depthStencilStatePool: keen__GraphicsStateObjectPool_keen__DepthStencilState_,
    pub m_vertexFormatPool: keen__GraphicsStateObjectPool_keen__VertexFormat_,
    pub m_vertexInputBindingPool: keen__GraphicsStateObjectPool_keen__VertexInputBinding_,
}

#[repr(C)]
pub struct keen__GraphicsSystem {
    // keen__GraphicsSystemBase
    pub pShaderFactory: *mut keen__ShaderFactory,
    pub pSystemAllocator: *mut keen__MemoryAllocator,
    pub stateObjectPool: keen__CombinedGraphicsStateObjectPool,
    pub stockObjects: keen__StockObjects,
    // keen__GraphicsSystem
    pub pDevice: *mut ID3D11Device,
    pub pImmediateContext: *mut ID3D11DeviceContext,
    pub pDeferredContext: *mut ID3D11DeviceContext,
    pub screenAspectRatio: f32,
    pub createdWindowHandle: *mut HWND__,
    pub isWindowDestroyed: bool,
    pub skinningBuffer: keen__SkinningD3D11,
    pub ownerThreadId: u32,
    pub frontThreadId: u32,
    pub immediateCommandBuffer: keen__GraphicsCommandBuffer,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub deferredCommandBuffer: keen__GraphicsCommandBuffer,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub emptyFragmentShader: keen__FragmentShader,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub pDefaultSwapChain: *mut keen__RenderSwapChain,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub pCurrentSwapChain: *mut keen__RenderSwapChain,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub currentFrameNumber: u32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub pScreenCapture: *mut keen__ScreenCapture,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub previousFullscreenMode: keen__graphics__WindowMode,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub fullscreenMode: keen__graphics__WindowMode,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub exclusiveModeWidth: u32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub exclusiveModeHeight: u32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub exclusiveModeRefreshRateNumerator: u32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub exclusiveModeRefreshRateDenominator: u32,
}

unsafe impl UpcastToNop<keen__GraphicsSystemBase> for keen__GraphicsSystem {}

#[repr(C)]
pub struct keen__PoolAllocator_keen__VertexFormat_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator
{
    // keen__InternalListBase__IteratorBase
    pub m_pCurrent: *mut keen__InternalListBase__Listable,
    /* keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator */
}

unsafe impl UpcastToNop<keen__InternalListBase__IteratorBase> for keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator {}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__DepthStencilState_ {
    pub m_allocator: keen__PoolAllocator_keen__DepthStencilState_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__RenderSwapChain {
    pub pGraphicsSystem: *mut keen__GraphicsSystem,
    pub depthBufferFormat: keen__PixelFormat,
    pub windowHandle: *mut HWND__,
    pub backBufferColorData: keen__TextureData,
    pub backBufferDepthData: keen__TextureData,
    pub backBufferRenderTarget: keen__RenderTarget,
    pub noGammaBackBufferRenderTarget: keen__RenderTarget,
    pub pSwapChain: *mut IDXGISwapChain,
    pub swapChainDescription: DXGI_SWAP_CHAIN_DESC,
    pub pBackBufferRenderTargetView: *mut ID3D11RenderTargetView,
    pub pNoGammaBackBufferRenderTargetView: *mut ID3D11RenderTargetView,
    pub pBackBufferDepthView: *mut ID3D11DepthStencilView,
    pub windowWidth: u32,
    pub windowHeight: u32,
    pub presentationInterval: u32,
}

#[repr(C)]
pub struct keen__DownsampleDepthVariants {
    pub m_fragmentShaders: [*const keen__FragmentShader; 2],
    pub m_vertexShaders: [*const keen__VertexShader; 1],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_pipelines: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__BlendState_ {
    pub m_allocator: keen__PoolAllocator_keen__BlendState_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__VertexFormat_ {
    pub m_allocator: keen__PoolAllocator_keen__VertexFormat_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__CopyDepthVariants {
    pub m_fragmentShaders: [*const keen__FragmentShader; 1],
    pub m_vertexShaders: [*const keen__VertexShader; 1],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_pipelines: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__BlendState_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__ShaderFactory {
    pub pShaderFileSystem: *mut keen__FileSystem,
    pub pGraphicsSystem: *mut keen__GraphicsSystem,
    pub scratchAllocator: keen__TlsfMemoryAllocator,
    pub shaderObjectAllocator: keen__TlsfMemoryAllocator,
    pub shaderProgramCodeAllocator: keen__TlsfMemoryAllocator,
    pub pShaderProgramCodeAllocator: *mut keen__MemoryAllocator,
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__SamplerState_ {
    pub m_allocator: keen__PoolAllocator_keen__SamplerState_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__VertexInputBinding_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__StockObjects {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_shaderPipelines: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 24],
    pub m_pStockVertexShaders: [*const keen__VertexShader; 2],
    pub m_pStockFragmentShaders: [*const keen__FragmentShader; 3],
    pub m_pFormats: [*const keen__VertexFormat; 3],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_textures: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 32],
}

#[repr(C)]
pub struct IDXGIObject {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // IDXGIObject
}

unsafe impl UpcastToNop<IUnknown> for IDXGIObject {}

#[repr(C)]
pub struct IDXGISwapChain {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* IDXGIObject
     * IDXGIDeviceSubObject
     * IDXGISwapChain */
}

unsafe impl UpcastToNop<IDXGIDeviceSubObject> for IDXGISwapChain {}

unsafe impl UpcastToNop<IDXGIObject> for IDXGISwapChain {}

unsafe impl UpcastToNop<IUnknown> for IDXGISwapChain {}

#[repr(C)]
pub struct List_gfc__KinematicActor__KAnimation_ {
    pub mList: *mut List_gfc__KinematicActor__KAnimation___ListNode,
    pub mTail: *mut List_gfc__KinematicActor__KAnimation___ListNode,
    pub mSize: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__GrabbableActor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__ChannelAccessor_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CharacterClass_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Character {
    // gfc__IRefObject
    pub vfptr: *const gfc__Character__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
    __pdbindgen_padding_2: [u8; 4],
    pub mHitPoints: f32,
    pub mMoveDir: f32,
    pub mSpeed: f32,
    pub mStrength: f32,
    pub mLastWaypointPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mLastWaypointUp: gfc__TVector3_float_gfc__FloatMath_,
    pub mVariableBlendValue: f32,
    pub mTimeUntilDecayOverride: f32,
    pub mDecayAnimIdOverride: i32,
    pub mMobID: i32,
    pub mFootstepMaterialOverlay: i32,
    pub mDeathInteractiveID: i32,
    pub mCharacterFlags: gfc__TFlags_unsigned_long_,
    pub mCharacterProxy: gfc__AutoRef_gfc__CharacterProxy_,
    pub mCurrentMoveState: gfc__AutoRef_gfc__CharacterMoveState_,
    pub mInteractiveContextNode: *mut gfc__Node3D,
    pub mCenterPositionNode: *mut gfc__Node3D,
    pub mCenterOffset: gfc__TVector3_float_gfc__FloatMath_,
    pub mInRangeRegion: *mut gfc__PhysicsDetectRegion,
    pub mAttributes: [i32; 64],
    pub mInventory: gfc__AutoRef_gfc__Inventory_,
    pub mVisuals: gfc__Vector_gfc__Character__CVisual_0_gfc__CAllocator_,
    pub mLookAtSolver: gfc__AutoRef_gfc__CharacterLookAtSolver_,
    pub mCurrentBody: gfc__AutoRef_gfc__Body_,
    pub mLastRefPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mFlinchCounter: f32,
    pub mFlinchChannel: i32,
    pub mEnabledInteractive: i32,
    pub mAutoJumpSolvers: gfc__Vector_gfc__AutoRef_gfc__AutoJumpSolver__0_gfc__CAllocator_,
    pub mAutoJumpCooldown: f32,
    pub mIKSolver: gfc__AutoRef_gfc__IKSolver_,
    pub mPActors: gfc__Vector_gfc__Character__PActor_0_gfc__CAllocator_,
    pub mPersonalTimeStamp: f32,
    pub mDetectors: gfc__Vector_gfc__AutoRef_gfc__DetectorObject__0_gfc__CAllocator_,
    pub mHitLog: gfc__Vector_gfc__HitRecord_0_gfc__CAllocator_,
    pub mDamageThresholds: gfc__Vector_gfc__DamageThresholdCallback_0_gfc__CAllocator_,
    pub mNextThresholdID: u32,
    pub mCurrentEffectID: i32,
    pub mLastSurfaceType: i32,
    pub mGravity: f32,
    pub mPreviousGravity: f32,
    pub mPlatformVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mExternalVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentWaypoint: *mut gfc__TraversalWaypoint,
    pub mNearestWaypoint: gfc__Vector_gfc__TraversalWaypoint___0_gfc__CAllocator_,
    pub mNearestWaypointExpire: gfc__Vector_float_0_gfc__CAllocator_,
    pub mNearestWaypointTTL: f32,
    pub mRagdollMap: gfc__AutoRef_gfc__RagdollBoneMapping_,
    pub mRagdollParentNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollRoot: gfc__AutoRef_gfc__Node3D_,
    pub mVictimNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollMapper: gfc__AutoRef_gfc__RagdollMapper_,
    pub mRagdoll_2: gfc__AutoRef_gfc__Object3D_,
    pub mOldMotionType: i32,
    pub mCharacterClass: gfc__AutoRef_gfc__CharacterClass_,
    pub mBodyRelativeVersion: i32,
    __pdbindgen_padding_3: [u8; 4],
    pub mOrientation: gfc__Matrix4,
    pub mOrientationInv: gfc__Matrix4,
    pub mLastBodyMatrix: gfc__Matrix4,
    pub mBodyRelativeMatrix: gfc__Matrix4,
    pub mOldGrabMat: gfc__Matrix4,
    pub mCombatState: i32,
    pub mCombatPosition: i32,
    pub mAttackState: i32,
    pub mAttackRadius: f32,
    pub mAttackHeight: f32,
    pub mDissolveElapsedTime: f32,
    pub mDissolveDuration: f32,
    pub mBrightness: f32,
    pub mDissolveIn: bool,
    pub mDissolveActive: bool,
    pub mTexturePackage: gfc__HString,
    pub mDissolveTexture: gfc__HString,
    pub mDissolvePatternTexture: gfc__HString,
}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__Character {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__Character {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Character {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Character {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Character {}

impl gfc__Character {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupMovement(
        &self,
        a1: f32,
        a2: *mut gfc__CharMoveVars,
    ) -> bool {
        ((*self.vfptr).setupMovement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyMovement(&self, a1: f32) {
        ((*self.vfptr).applyMovement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateAttributes(&self) {
        ((*self.vfptr).invalidateAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isAttributeValid(&self, a1: i32) -> bool {
        ((*self.vfptr).isAttributeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAttribute(&self, a1: i32) -> *mut i32 {
        ((*self.vfptr).getAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptAttribute(&self, a1: i32) -> i32 {
        ((*self.vfptr).getScriptAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateAttributes(&self, a1: f32) {
        ((*self.vfptr).updateAttributes)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKillingBlow(&self) {
        ((*self.vfptr).doKillingBlow)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isFlying(&self) -> bool {
        ((*self.vfptr).isFlying)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn validateInteractive(&self, a1: u32) -> bool {
        ((*self.vfptr).validateInteractive)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doInteractive(
        &self,
        a1: *mut gfc__CharacterDoInteractiveDesc,
        a2: gfc__AutoRef_gfc__Character_,
        a3: bool,
    ) {
        ((*self.vfptr).doInteractive)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onInterrupt(&self) {
        ((*self.vfptr).onInterrupt)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn grab(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).grab)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn throww(
        &self,
        a1: *mut gfc__Character,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).throww)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drop(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).drop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGrabNode(
        &self,
        result: *mut gfc__HString,
        a2: *mut gfc__Character,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getGrabNode)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn onGrabbableWeaponized(
        &self,
        a1: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).onGrabbableWeaponized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pickupDraggable(&self, a1: *mut gfc__DraggableActor) {
        ((*self.vfptr).pickupDraggable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doMounted(&self, a1: *mut gfc__Character, a2: i32) {
        ((*self.vfptr).doMounted)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn findBestTargetInRange(&self, a1: f32) -> *mut gfc__Character {
        ((*self.vfptr).findBestTargetInRange)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findBestTarget(&self) -> *mut gfc__Character {
        ((*self.vfptr).findBestTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn distanceTo(&self, a1: *mut gfc__Character) -> f32 {
        ((*self.vfptr).distanceTo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findGrabbable(&self) -> *mut gfc__Actor {
        ((*self.vfptr).findGrabbable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRotationOnly(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotationOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHeadingOnly(&self, a1: f32) {
        ((*self.vfptr).setHeadingOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCenterOffset(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setBody(&self) {
        ((*self.vfptr).setBody)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupCharacterProxy(&self) {
        ((*self.vfptr).setupCharacterProxy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActualVelocity(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getActualVelocity)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn updateLastGroundMaterial(&self) {
        ((*self.vfptr).updateLastGroundMaterial)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doHitPause(&self, a1: f32) {
        ((*self.vfptr).doHitPause)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Character__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Character) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Character) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Character,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Character) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Character,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Character) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Character) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Character) -> bool,
    pub enteredDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub exitedDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__HitInfo),
    pub queryHit:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub setupMovement: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: f32,
        _: *mut gfc__CharMoveVars,
    ) -> bool,
    pub applyMovement: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub invalidateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub isAttributeValid: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: i32) -> bool,
    pub getAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: i32) -> *mut i32,
    pub getScriptAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: i32) -> i32,
    pub updateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub doKillingBlow: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub isFlying: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> bool,
    pub validateInteractive: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: u32) -> bool,
    pub doInteractive: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__CharacterDoInteractiveDesc,
        _: gfc__AutoRef_gfc__Character_,
        _: bool,
    ),
    pub onInterrupt: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub grab: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Character),
    pub throww: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drop: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Character),
    pub getGrabNode: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__HString,
        _: *mut gfc__Character,
    ) -> *mut gfc__HString,
    pub onGrabbableWeaponized: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ),
    pub pickupDraggable:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__DraggableActor),
    pub doMounted:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Character, _: i32),
    pub findBestTargetInRange:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32) -> *mut gfc__Character,
    pub findBestTarget:
        unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut gfc__Character,
    pub distanceTo:
        unsafe extern "thiscall" fn(this: *mut gfc__Character, _: *mut gfc__Character) -> f32,
    pub findGrabbable: unsafe extern "thiscall" fn(this: *mut gfc__Character) -> *mut gfc__Actor,
    pub setRotationOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHeadingOnly: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
    pub getCenterOffset: unsafe extern "thiscall" fn(
        this: *const gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setBody: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
}

#[repr(C)]
pub struct gfc__Character__CVisual {
    pub Name: gfc__HString,
    pub Type: i32,
    pub Object: gfc__AutoRef_gfc__Object3D_,
    pub Spawned: bool,
}

#[repr(C)]
pub struct gfc__Character__PActor {
    pub Node: *mut gfc__Node3D,
    pub Actor: gfc__AutoRef_gfc__GrabbableActor_,
}

#[repr(C)]
pub struct gfc__Actor {
    // gfc__IRefObject
    pub vfptr: *const gfc__Actor__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Actor {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Actor {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Actor {}

impl gfc__Actor {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Actor__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__TBox_float_gfc__FloatMath_),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Actor,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Actor,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Actor) -> bool,
    pub enteredDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: gfc__AutoRef_gfc__DetectorObject_),
    pub exitedDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: gfc__AutoRef_gfc__DetectorObject_),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__HitInfo),
    pub queryHit: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ActorDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__GameCamera {
    // gfc__IRefObject
    pub vfptr: *const gfc__GameCamera__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__ISynchronized
    pub mLock: gfc__Lock,
    // gfc__GameCamera
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__GameCamera {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__GameCamera {}

unsafe impl UpcastTo<gfc__ISynchronized> for gfc__GameCamera {
    fn upcast_to(p: *const Self) -> *const gfc__ISynchronized {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__GameCamera {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn begin(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).begin)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn end(&self) {
        ((*self.vfptr).end)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *const gfc__Matrix4,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMatrix(
        &self,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4 {
        ((*self.vfptr).getMatrix)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getCameraDirection(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCameraDirection)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getCameraPos(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCameraPos)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getCameraDirectionOfInterest(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCameraDirectionOfInterest)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getFOV(&self) -> f32 {
        ((*self.vfptr).getFOV)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFocusActor(&self, a1: gfc__AutoRef_gfc__Actor_) {
        ((*self.vfptr).setFocusActor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFocusActor(
        &self,
        result: *mut gfc__AutoRef_gfc__Actor_,
    ) -> *mut gfc__AutoRef_gfc__Actor_ {
        ((*self.vfptr).getFocusActor)(self as *const _ as *mut _, result)
    }
}

#[repr(C)]
pub struct gfc__GameCamera__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__GameCamera) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__GameCamera) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__GameCamera) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub begin:
        unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: gfc__AutoRef_gfc__WorldObject_),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        _: *mut gfc__UIRenderer,
        _: *const gfc__Matrix4,
    ),
    pub getMatrix: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4,
    pub getCameraDirection: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getCameraPos: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getCameraDirectionOfInterest:
        unsafe extern "thiscall" fn(
            this: *mut gfc__GameCamera,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getFOV: unsafe extern "thiscall" fn(this: *mut gfc__GameCamera) -> f32,
    pub setFocusActor:
        unsafe extern "thiscall" fn(this: *mut gfc__GameCamera, _: gfc__AutoRef_gfc__Actor_),
    pub getFocusActor: unsafe extern "thiscall" fn(
        this: *mut gfc__GameCamera,
        result: *mut gfc__AutoRef_gfc__Actor_,
    ) -> *mut gfc__AutoRef_gfc__Actor_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Item_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TriggerRegion {
    // gfc__IRefObject
    pub vfptr: *const gfc__TriggerRegion__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__PhysicsShapeObject
    pub mShape: i32,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
    pub mBounds: gfc__TBox_float_gfc__FloatMath_,
    pub mGizmo: *mut gfc__PhysicsShapeGizmo,
    // gfc__DetectorObject
    __pdbindgen_padding: [u8; 4],
    pub mBodyType: u8,
    pub mRegion: *mut gfc__DetectorRegion,
    pub mEnabled: bool,
    // gfc__TriggerRegion
    __pdbindgen_padding_2: [u8; 7],
    pub mFilterList: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mDefaultFilterList: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mNameFilter: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mOnEnterList: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mOnExitList: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mOneShot: bool,
    pub mEnable: bool,
    pub mExclude: bool,
    pub mRequireInWorld: bool,
    pub mEnterHasFired: bool,
    pub mExitHasFired: bool,
    pub mRegionDesc: gfc__AutoRef_gfc__TriggerRegionDesc_,
}

unsafe impl UpcastToNop<gfc__DetectorObject> for gfc__TriggerRegion {}

unsafe impl UpcastToNop<gfc__PhysicsShapeObject> for gfc__TriggerRegion {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__TriggerRegion {}

unsafe impl UpcastToNop<gfc__Object> for gfc__TriggerRegion {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__TriggerRegion {}

impl gfc__TriggerRegion {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGizmoColor(
        &self,
    ) -> *const gfc__TVector4_float_gfc__FloatMath_ {
        ((*self.vfptr).getGizmoColor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGizmoIcon(&self) -> *const gfc__HString {
        ((*self.vfptr).getGizmoIcon)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPhantomBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getPhantomBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onEnter(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).onEnter)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onExit(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).onExit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doWork(
        &self,
        a1: *mut gfc__Actor,
        a2: *mut bool,
        a3: *mut gfc__Vector_gfc__HString_0_gfc__CAllocator_,
        a4: *const gfc__HString,
    ) {
        ((*self.vfptr).doWork)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct gfc__TriggerRegion__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__TriggerRegion) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__TriggerRegion) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__TriggerRegion,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion),
    pub getGizmoColor: unsafe extern "thiscall" fn(
        this: *const gfc__TriggerRegion,
    )
        -> *const gfc__TVector4_float_gfc__FloatMath_,
    pub getGizmoIcon:
        unsafe extern "thiscall" fn(this: *const gfc__TriggerRegion) -> *const gfc__HString,
    pub getPhantomBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub onEnter: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__Actor),
    pub onExit: unsafe extern "thiscall" fn(this: *mut gfc__TriggerRegion, _: *mut gfc__Actor),
    pub doWork: unsafe extern "thiscall" fn(
        this: *mut gfc__TriggerRegion,
        _: *mut gfc__Actor,
        _: *mut bool,
        _: *mut gfc__Vector_gfc__HString_0_gfc__CAllocator_,
        _: *const gfc__HString,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ButtonMashPoint_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PlayerSaveData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__FrenzyItem_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SaveValue_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DetectorObject_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__DetectorObject__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__DetectorObject_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Player {
    // gfc__IRefObject
    pub vfptr: *const gfc__Player__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
    __pdbindgen_padding_2: [u8; 4],
    pub mHitPoints: f32,
    pub mMoveDir: f32,
    pub mSpeed: f32,
    pub mStrength: f32,
    pub mLastWaypointPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mLastWaypointUp: gfc__TVector3_float_gfc__FloatMath_,
    pub mVariableBlendValue: f32,
    pub mTimeUntilDecayOverride: f32,
    pub mDecayAnimIdOverride: i32,
    pub mMobID: i32,
    pub mFootstepMaterialOverlay: i32,
    pub mDeathInteractiveID: i32,
    pub mCharacterFlags: gfc__TFlags_unsigned_long_,
    pub mCharacterProxy: gfc__AutoRef_gfc__CharacterProxy_,
    pub mCurrentMoveState: gfc__AutoRef_gfc__CharacterMoveState_,
    pub mInteractiveContextNode: *mut gfc__Node3D,
    pub mCenterPositionNode: *mut gfc__Node3D,
    pub mCenterOffset: gfc__TVector3_float_gfc__FloatMath_,
    pub mInRangeRegion: *mut gfc__PhysicsDetectRegion,
    pub mAttributes: [i32; 64],
    pub mInventory: gfc__AutoRef_gfc__Inventory_,
    pub mVisuals: gfc__Vector_gfc__Character__CVisual_0_gfc__CAllocator_,
    pub mLookAtSolver: gfc__AutoRef_gfc__CharacterLookAtSolver_,
    pub mCurrentBody: gfc__AutoRef_gfc__Body_,
    pub mLastRefPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mFlinchCounter: f32,
    pub mFlinchChannel: i32,
    pub mEnabledInteractive: i32,
    pub mAutoJumpSolvers: gfc__Vector_gfc__AutoRef_gfc__AutoJumpSolver__0_gfc__CAllocator_,
    pub mAutoJumpCooldown: f32,
    pub mIKSolver: gfc__AutoRef_gfc__IKSolver_,
    pub mPActors: gfc__Vector_gfc__Character__PActor_0_gfc__CAllocator_,
    pub mPersonalTimeStamp: f32,
    pub mDetectors: gfc__Vector_gfc__AutoRef_gfc__DetectorObject__0_gfc__CAllocator_,
    pub mHitLog: gfc__Vector_gfc__HitRecord_0_gfc__CAllocator_,
    pub mDamageThresholds: gfc__Vector_gfc__DamageThresholdCallback_0_gfc__CAllocator_,
    pub mNextThresholdID: u32,
    pub mCurrentEffectID: i32,
    pub mLastSurfaceType: i32,
    pub mGravity: f32,
    pub mPreviousGravity: f32,
    pub mPlatformVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mExternalVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentWaypoint: *mut gfc__TraversalWaypoint,
    pub mNearestWaypoint: gfc__Vector_gfc__TraversalWaypoint___0_gfc__CAllocator_,
    pub mNearestWaypointExpire: gfc__Vector_float_0_gfc__CAllocator_,
    pub mNearestWaypointTTL: f32,
    pub mRagdollMap: gfc__AutoRef_gfc__RagdollBoneMapping_,
    pub mRagdollParentNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollRoot: gfc__AutoRef_gfc__Node3D_,
    pub mVictimNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollMapper: gfc__AutoRef_gfc__RagdollMapper_,
    pub mRagdoll_2: gfc__AutoRef_gfc__Object3D_,
    pub mOldMotionType: i32,
    pub mCharacterClass: gfc__AutoRef_gfc__CharacterClass_,
    pub mBodyRelativeVersion: i32,
    __pdbindgen_padding_3: [u8; 4],
    pub mOrientation: gfc__Matrix4,
    pub mOrientationInv: gfc__Matrix4,
    pub mLastBodyMatrix: gfc__Matrix4,
    pub mBodyRelativeMatrix: gfc__Matrix4,
    pub mOldGrabMat: gfc__Matrix4,
    pub mCombatState: i32,
    pub mCombatPosition: i32,
    pub mAttackState: i32,
    pub mAttackRadius: f32,
    pub mAttackHeight: f32,
    pub mDissolveElapsedTime: f32,
    pub mDissolveDuration: f32,
    pub mBrightness: f32,
    pub mDissolveIn: bool,
    pub mDissolveActive: bool,
    pub mTexturePackage: gfc__HString,
    pub mDissolveTexture: gfc__HString,
    pub mDissolvePatternTexture: gfc__HString,
    // gfc__ResourceListener
    pub vfptr_2: *const gfc__Player__vftable,
    // gfc__Player
    pub mDoReinitTransitions: bool,
    pub mFrenzyItem: gfc__AutoRef_gfc__FrenzyItem_,
    pub mSoulBridgeItem: gfc__AutoRef_gfc__Item_,
    pub mMountSpawnable: bool,
    pub mMountAttackPower: f32,
    pub mShouldDismount: bool,
    pub mLastEquippableHotKeyAssigned: i32,
    pub mIsWalkingWithToggle: bool,
    pub mIsDraggingWithToggle: bool,
    pub mKillRegionRespawnPoint: gfc__HString,
    pub mSpawnWorld: gfc__HString,
    pub mSpawnRegion: gfc__HString,
    pub mSpawnPoint: gfc__HString,
    pub mSpawnLoadRegion: gfc__HString,
    pub mSplineCheckpoint: i32,
    pub mSplineName: gfc__HString,
    pub mChronicleLastTabID: i32,
    pub mCharacterLastTabID: i32,
    pub mVulgrimLastTabID: i32,
    pub mFromSave: bool,
    pub mMerchantInventory: gfc__AutoRef_gfc__Object_,
    pub mBoughtScythe: bool,
    pub mBoughtHarvester: bool,
    pub mActiveScythe: i32,
    pub mScytheExp: f32,
    pub mShownScytheTip: bool,
    pub mShownHarvesterTip: bool,
    pub mStatTracker: gfc__AutoRef_gfc__PlayerStatTracker_,
    pub mEnemyHPDifficultyBonus: gfc__Vector_int_0_gfc__CAllocator_,
    pub mEnemyDamageDifficultyBonus: gfc__Vector_int_0_gfc__CAllocator_,
    pub mIsInHavocForm: bool,
    pub mHavocFormIsEligible: bool,
    pub mHavocFormEnabled: bool,
    pub mHavocFormDecayRate: i32,
    pub mDelayedSwapDirection: i32,
    __pdbindgen_padding_4: [u8; 4],
    pub mMoveInput: gfc__MoveInput,
    pub mSaveData_2: gfc__AutoRef_gfc__PlayerSaveData_,
    pub mSaveDataStored: gfc__AutoRef_gfc__PlayerSaveData_,
    pub mMount: *mut gfc__Mount,
    pub mIsFullyMounted: bool,
    pub mWrathMan: *mut gfc__WrathMan,
    pub mPlayerContextMan: gfc__AutoRef_gfc__PlayerContextMan_,
    pub mSouls: f32,
    pub mSoulGatherCooldown: f32,
    pub mLastGhostHookPoint: gfc__AutoRef_gfc__Body_,
    pub mGhostHookEffectManager: *mut gfc__GhostHookEffectManager,
    pub mInTortureCoilDetectRegion: i32,
    pub mInTortureCoilSafeRegion: i32,
    pub mPeekContext: *mut gfc__PhysicsDetectRegion,
    pub mPreviousPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mInCombatTimer: f32,
    pub mComboTimer: f32,
    pub mCombatTimerDirtyFlag: bool,
    pub mSoftTarget: gfc__ActorTarget,
    pub mSoftTargetTimer: f32,
    pub mSoftTargetPeriod: f32,
    pub mSoftTargetIsDirty: bool,
    pub mButtonUsedToActivateWrath: i32,
    pub mHavocFormTimer: f32,
    pub mHavocSavedGear: gfc__AutoRef_gfc__EquippableItem_,
    pub mHavocSavedPrimaryWeapon: gfc__AutoRef_gfc__EquippableItem_,
    pub mHavocPreviousVisualID: i32,
    pub mPreviousMoveWeight: f32,
    pub mPreviousSize: i32,
    pub mHavocEffectID: i32,
    pub mHavocEffectID2: i32,
    pub mOnBird: bool,
    pub mVelocityRecord: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub mVelocityAverage: gfc__TVector3_float_gfc__FloatMath_,
    pub mNumVelocityEnteries: i32,
    pub mVelocityTimer: f32,
    pub mItemType: gfc__HString,
    pub mDraggableToIgnore: gfc__AutoRef_gfc__DraggableActor_,
    pub mHairLastPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mHair: gfc__Vector_gfc__AutoRef_gfc__Player__HairSim__0_gfc__CAllocator_,
    pub mWaterSurfaceNode: *mut gfc__Node3D,
    pub mWaterSurfacePSystem: *mut gfc__PSystem,
    pub mWaterSurfaceFXChannel: i32,
    pub mNumSecondaryToggleQueued: i32,
    pub mLastEquippedTime: f32,
    pub mHitPauseTime: f32,
    __pdbindgen_padding_5: [u8; 12],
}

unsafe impl UpcastToNop<gfc__Character> for gfc__Player {}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__Player {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__Player {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Player {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Player {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Player {}

unsafe impl UpcastTo<gfc__ResourceListener> for gfc__Player {
    fn upcast_to(p: *const Self) -> *const gfc__ResourceListener {
        (p as usize + 0x4d0) as *const _
    }
}

impl gfc__Player {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn packageUnloading(&self, a1: i32) {
        ((*self.vfptr).packageUnloading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupMovement(
        &self,
        a1: f32,
        a2: *mut gfc__CharMoveVars,
    ) -> bool {
        ((*self.vfptr).setupMovement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyMovement(&self, a1: f32) {
        ((*self.vfptr).applyMovement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateAttributes(&self) {
        ((*self.vfptr).invalidateAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isAttributeValid(&self, a1: i32) -> bool {
        ((*self.vfptr).isAttributeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAttribute(&self, a1: i32) -> *mut i32 {
        ((*self.vfptr).getAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptAttribute(&self, a1: i32) -> i32 {
        ((*self.vfptr).getScriptAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateAttributes(&self, a1: f32) {
        ((*self.vfptr).updateAttributes)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKillingBlow(&self) {
        ((*self.vfptr).doKillingBlow)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isFlying(&self) -> bool {
        ((*self.vfptr).isFlying)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn validateInteractive(&self, a1: u32) -> bool {
        ((*self.vfptr).validateInteractive)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doInteractive(
        &self,
        a1: *mut gfc__CharacterDoInteractiveDesc,
        a2: gfc__AutoRef_gfc__Character_,
        a3: bool,
    ) {
        ((*self.vfptr).doInteractive)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onInterrupt(&self) {
        ((*self.vfptr).onInterrupt)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn grab(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).grab)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn throww(
        &self,
        a1: *mut gfc__Character,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).throww)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drop(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).drop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGrabNode(
        &self,
        result: *mut gfc__HString,
        a2: *mut gfc__Character,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getGrabNode)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn onGrabbableWeaponized(
        &self,
        a1: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).onGrabbableWeaponized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pickupDraggable(&self, a1: *mut gfc__DraggableActor) {
        ((*self.vfptr).pickupDraggable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doMounted(&self, a1: *mut gfc__Character, a2: i32) {
        ((*self.vfptr).doMounted)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn findBestTargetInRange(&self, a1: f32) -> *mut gfc__Character {
        ((*self.vfptr).findBestTargetInRange)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findBestTarget(&self) -> *mut gfc__Character {
        ((*self.vfptr).findBestTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn distanceTo(&self, a1: *mut gfc__Character) -> f32 {
        ((*self.vfptr).distanceTo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findGrabbable(&self) -> *mut gfc__Actor {
        ((*self.vfptr).findGrabbable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRotationOnly(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotationOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHeadingOnly(&self, a1: f32) {
        ((*self.vfptr).setHeadingOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCenterOffset(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setBody(&self) {
        ((*self.vfptr).setBody)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupCharacterProxy(&self) {
        ((*self.vfptr).setupCharacterProxy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActualVelocity(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getActualVelocity)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn updateLastGroundMaterial(&self) {
        ((*self.vfptr).updateLastGroundMaterial)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doHitPause(&self, a1: f32) {
        ((*self.vfptr).doHitPause)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Player__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: u32) -> *mut (),
    pub packageUnloading: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32),
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Player) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Player,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Player) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Player,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Player) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Player) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Player) -> bool,
    pub enteredDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__DetectorObject_),
    pub exitedDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: gfc__AutoRef_gfc__DetectorObject_),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__HitInfo),
    pub queryHit: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub setupMovement: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: f32,
        _: *mut gfc__CharMoveVars,
    ) -> bool,
    pub applyMovement: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub invalidateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub isAttributeValid: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32) -> bool,
    pub getAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32) -> *mut i32,
    pub getScriptAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: i32) -> i32,
    pub updateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub doKillingBlow: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub isFlying: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> bool,
    pub validateInteractive: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: u32) -> bool,
    pub doInteractive: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__CharacterDoInteractiveDesc,
        _: gfc__AutoRef_gfc__Character_,
        _: bool,
    ),
    pub onInterrupt: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub grab: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Character),
    pub throww: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drop: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Character),
    pub getGrabNode: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__HString,
        _: *mut gfc__Character,
    ) -> *mut gfc__HString,
    pub onGrabbableWeaponized: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ),
    pub pickupDraggable:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__DraggableActor),
    pub doMounted:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Character, _: i32),
    pub findBestTargetInRange:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32) -> *mut gfc__Character,
    pub findBestTarget: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut gfc__Character,
    pub distanceTo:
        unsafe extern "thiscall" fn(this: *mut gfc__Player, _: *mut gfc__Character) -> f32,
    pub findGrabbable: unsafe extern "thiscall" fn(this: *mut gfc__Player) -> *mut gfc__Actor,
    pub setRotationOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHeadingOnly: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
    pub getCenterOffset: unsafe extern "thiscall" fn(
        this: *const gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setBody: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Player,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Player),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__Player, _: f32),
}

#[repr(C)]
pub struct gfc__CharacterDoInteractiveDesc {
    // gfc__IRefObject
    pub vfptr: *const gfc__CharacterDoInteractiveDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CharMoveStateDesc
    pub mID: u32,
    pub mName: gfc__HString,
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mTransitions: gfc__Vector_gfc__AutoRef_gfc__CharTransitionDesc__0_gfc__CAllocator_,
    pub mCameraStateDesc: gfc__AutoRef_gfc__CameraStateDesc_,
    pub mOnActionComplete: i32,
    pub mOnBlock: i32,
    pub mOnFall: i32,
    pub mOnIdle: i32,
    pub mOnJump: i32,
    pub mOnMove: i32,
    pub mOnSwim: i32,
    pub mOnImpulseImpact: i32,
    pub mAcceleration: gfc__Half,
    pub mDeceleration: gfc__Half,
    pub mGravityScale: gfc__Half,
    pub mMaxSpeed: gfc__Half,
    pub mMoveStrength: gfc__Half,
    pub mMoveWeight: gfc__Half,
    pub mTurnSpeed: gfc__Half,
    pub mOnLeaveSpeedScale: gfc__Half,
    pub mImpulseDamage: u16,
    pub mImpulsePeriod: gfc__Half,
    pub mDamageMitigation: i8,
    pub mPowerResistance: i8,
    pub mAutoJumpSolverID: u8,
    pub mCharacterFlags: gfc__TFlags_unsigned_short_,
    // gfc__CharacterDoInteractiveDesc
    pub mTargetType: gfc__HString,
    pub mTargetInAirOrOnGround: u8,
    pub mSourceInAirOrOnGround: u8,
    pub mTargetProne: u8,
    pub mInitiator: i32,
    pub mAutoface: bool,
    pub mValidateFromPivot: bool,
    pub mMaxErrorDistance: f32,
    pub mArc: f32,
    pub mIgnoreTime: f32,
    pub mDoCollision: bool,
    pub mDoCollisionOther: bool,
    pub mDoCameraCollision: bool,
    pub mConstrainToGroundMaxDistance: f32,
    pub mMounted: bool,
    pub mMountedRequired: bool,
    pub mValidStates: gfc__Vector_int_0_gfc__CAllocator_,
    pub mDoRaycast: bool,
    pub mAirGravityScale: f32,
    pub mValidStateDescs: gfc__Vector_int_0_gfc__CAllocator_,
    pub mPriority: u32,
    pub mUseCameraNode: bool,
    pub mValidateCameraCollision: bool,
    pub mHideUI: bool,
    pub mShowWorldContext: bool,
    pub mClampWorldContextDistance: f32,
    pub mCameraNode: gfc__HString,
    pub mCameraCollisionNode: gfc__HString,
    pub mCameraTransitionTime: f32,
    pub mOutCameraTransitionTime: f32,
    pub mCameraFrames: i32,
    pub mShowCinematicBars: bool,
    pub mFreezeCameraOnExit: bool,
    pub mCanPlayDuringCinematic: bool,
    pub mDestroyOtherOnCinematic: bool,
    pub mOnImpact: i32,
    pub mOnImpactOther: i32,
    pub mInteractiveType: i32,
    pub mAnchor: u8,
    pub mConvergeTime: f32,
    pub mUseAnchorNode: bool,
    pub mAnchorNodeName: gfc__HString,
    pub mUseWeapon: i32,
    pub mUseGun: bool,
    pub mImmediateInterrupt: bool,
    pub mLoopCount: i32,
    pub mUseTargetRotation: i32,
    pub mValidateProperty: gfc__HString,
    pub mCanBeAttacked: bool,
    pub mDropCarriedActors: bool,
    pub mButtonMash: bool,
    pub mButtonMashButton: u8,
    pub mInitialMashValue: f32,
    pub mMashSuccessValue: f32,
    pub mMashIncrement: f32,
    pub mMashDecayArray: gfc__Vector_gfc__AutoRef_gfc__ButtonMashPoint__0_gfc__CAllocator_,
    pub mMashAdvances: bool,
    pub mAllowControl: bool,
    pub mAnimationID: i32,
    pub mTargetAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimationSource: u8,
    pub mMoveStateSource: u8,
    pub mOnComplete: i32,
    pub mOnCompleteOther: i32,
    pub mOnMashSuccess: i32,
    pub mOnMashSuccessOther: i32,
    pub mOnMashFailure: i32,
    pub mOnMashFailureOther: i32,
    pub mOnDeath: i32,
    pub mOnCompleteMount: i32,
    pub mOnPathBlocked: i32,
    pub mForceDisperseMount: bool,
    pub mOnDisperseMountState: i32,
    pub mMoveTransitionFrame: i32,
    pub mOnMoveTransition: i32,
    pub mOverridesHavocForm: bool,
    pub mTransitions_2: gfc__Vector_gfc__AutoRef_gfc__InteractiveTransition__0_gfc__CAllocator_,
    pub mInited: i32,
    pub mInitialPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mInitialHeading: f32,
    pub mTargetInitialPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mTargetInitialHeading: f32,
    pub mCachedProperty: gfc__AutoRef_gfc__Property_,
}

unsafe impl UpcastToNop<gfc__CharMoveStateDesc> for gfc__CharacterDoInteractiveDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CharacterDoInteractiveDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CharacterDoInteractiveDesc {}

impl gfc__CharacterDoInteractiveDesc {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createMoveState(
        &self,
        result: *mut gfc__AutoRef_gfc__CharacterMoveState_,
        a2: *mut gfc__Character,
    ) -> *mut gfc__AutoRef_gfc__CharacterMoveState_ {
        ((*self.vfptr).createMoveState)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn validate(
        &self,
        a1: i32,
        a2: *mut gfc__Character,
        a3: *mut gfc__Character,
        a4: *mut gfc__PlayerContextMan__DistanceMetrics,
        a5: bool,
        a6: bool,
    ) -> bool {
        ((*self.vfptr).validate)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn validatePosition(
        &self,
        a1: i32,
        a2: *mut gfc__Character,
        a3: *mut gfc__Character,
    ) -> bool {
        ((*self.vfptr).validatePosition)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn validateDistance(
        &self,
        a1: *mut gfc__Character,
        a2: *mut gfc__Character,
    ) -> bool {
        ((*self.vfptr).validateDistance)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn interactiveDone(
        &self,
        a1: *mut gfc__Player,
        a2: *mut gfc__Character,
    ) {
        ((*self.vfptr).interactiveDone)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createPlayerMoveState(
        &self,
        result: *mut gfc__AutoRef_gfc__PlayerDoInteractive_,
        a2: *mut gfc__Player,
    ) -> *mut gfc__AutoRef_gfc__PlayerDoInteractive_ {
        ((*self.vfptr).createPlayerMoveState)(self as *const _ as *mut _, result, a2)
    }
}

#[repr(C)]
pub struct gfc__CharacterDoInteractiveDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CharacterDoInteractiveDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(
        this: *const gfc__CharacterDoInteractiveDesc,
    ) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: *const gfc__HString,
    ),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__CharacterDoInteractiveDesc) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__CharacterDoInteractiveDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
    ) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub createMoveState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        result: *mut gfc__AutoRef_gfc__CharacterMoveState_,
        _: *mut gfc__Character,
    )
        -> *mut gfc__AutoRef_gfc__CharacterMoveState_,
    pub validate: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: i32,
        _: *mut gfc__Character,
        _: *mut gfc__Character,
        _: *mut gfc__PlayerContextMan__DistanceMetrics,
        _: bool,
        _: bool,
    ) -> bool,
    pub validatePosition: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: i32,
        _: *mut gfc__Character,
        _: *mut gfc__Character,
    ) -> bool,
    pub validateDistance: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: *mut gfc__Character,
        _: *mut gfc__Character,
    ) -> bool,
    pub interactiveDone: unsafe extern "thiscall" fn(
        this: *mut gfc__CharacterDoInteractiveDesc,
        _: *mut gfc__Player,
        _: *mut gfc__Character,
    ),
    pub createPlayerMoveState:
        unsafe extern "thiscall" fn(
            this: *mut gfc__CharacterDoInteractiveDesc,
            result: *mut gfc__AutoRef_gfc__PlayerDoInteractive_,
            _: *mut gfc__Player,
        ) -> *mut gfc__AutoRef_gfc__PlayerDoInteractive_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__HavokWeaponProxy_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Mount {
    // gfc__IRefObject
    pub vfptr: *const gfc__Mount__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
    __pdbindgen_padding_2: [u8; 4],
    pub mHitPoints: f32,
    pub mMoveDir: f32,
    pub mSpeed: f32,
    pub mStrength: f32,
    pub mLastWaypointPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mLastWaypointUp: gfc__TVector3_float_gfc__FloatMath_,
    pub mVariableBlendValue: f32,
    pub mTimeUntilDecayOverride: f32,
    pub mDecayAnimIdOverride: i32,
    pub mMobID: i32,
    pub mFootstepMaterialOverlay: i32,
    pub mDeathInteractiveID: i32,
    pub mCharacterFlags: gfc__TFlags_unsigned_long_,
    pub mCharacterProxy: gfc__AutoRef_gfc__CharacterProxy_,
    pub mCurrentMoveState: gfc__AutoRef_gfc__CharacterMoveState_,
    pub mInteractiveContextNode: *mut gfc__Node3D,
    pub mCenterPositionNode: *mut gfc__Node3D,
    pub mCenterOffset: gfc__TVector3_float_gfc__FloatMath_,
    pub mInRangeRegion: *mut gfc__PhysicsDetectRegion,
    pub mAttributes: [i32; 64],
    pub mInventory: gfc__AutoRef_gfc__Inventory_,
    pub mVisuals: gfc__Vector_gfc__Character__CVisual_0_gfc__CAllocator_,
    pub mLookAtSolver: gfc__AutoRef_gfc__CharacterLookAtSolver_,
    pub mCurrentBody: gfc__AutoRef_gfc__Body_,
    pub mLastRefPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mFlinchCounter: f32,
    pub mFlinchChannel: i32,
    pub mEnabledInteractive: i32,
    pub mAutoJumpSolvers: gfc__Vector_gfc__AutoRef_gfc__AutoJumpSolver__0_gfc__CAllocator_,
    pub mAutoJumpCooldown: f32,
    pub mIKSolver: gfc__AutoRef_gfc__IKSolver_,
    pub mPActors: gfc__Vector_gfc__Character__PActor_0_gfc__CAllocator_,
    pub mPersonalTimeStamp: f32,
    pub mDetectors: gfc__Vector_gfc__AutoRef_gfc__DetectorObject__0_gfc__CAllocator_,
    pub mHitLog: gfc__Vector_gfc__HitRecord_0_gfc__CAllocator_,
    pub mDamageThresholds: gfc__Vector_gfc__DamageThresholdCallback_0_gfc__CAllocator_,
    pub mNextThresholdID: u32,
    pub mCurrentEffectID: i32,
    pub mLastSurfaceType: i32,
    pub mGravity: f32,
    pub mPreviousGravity: f32,
    pub mPlatformVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mExternalVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentWaypoint: *mut gfc__TraversalWaypoint,
    pub mNearestWaypoint: gfc__Vector_gfc__TraversalWaypoint___0_gfc__CAllocator_,
    pub mNearestWaypointExpire: gfc__Vector_float_0_gfc__CAllocator_,
    pub mNearestWaypointTTL: f32,
    pub mRagdollMap: gfc__AutoRef_gfc__RagdollBoneMapping_,
    pub mRagdollParentNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollRoot: gfc__AutoRef_gfc__Node3D_,
    pub mVictimNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollMapper: gfc__AutoRef_gfc__RagdollMapper_,
    pub mRagdoll_2: gfc__AutoRef_gfc__Object3D_,
    pub mOldMotionType: i32,
    pub mCharacterClass: gfc__AutoRef_gfc__CharacterClass_,
    pub mBodyRelativeVersion: i32,
    __pdbindgen_padding_3: [u8; 4],
    pub mOrientation: gfc__Matrix4,
    pub mOrientationInv: gfc__Matrix4,
    pub mLastBodyMatrix: gfc__Matrix4,
    pub mBodyRelativeMatrix: gfc__Matrix4,
    pub mOldGrabMat: gfc__Matrix4,
    pub mCombatState: i32,
    pub mCombatPosition: i32,
    pub mAttackState: i32,
    pub mAttackRadius: f32,
    pub mAttackHeight: f32,
    pub mDissolveElapsedTime: f32,
    pub mDissolveDuration: f32,
    pub mBrightness: f32,
    pub mDissolveIn: bool,
    pub mDissolveActive: bool,
    pub mTexturePackage: gfc__HString,
    pub mDissolveTexture: gfc__HString,
    pub mDissolvePatternTexture: gfc__HString,
    // gfc__Mount
    pub mPlayer: gfc__AutoRef_gfc__Player_,
    __pdbindgen_padding_4: [u8; 12],
}

unsafe impl UpcastToNop<gfc__Character> for gfc__Mount {}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__Mount {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__Mount {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Mount {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Mount {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Mount {}

impl gfc__Mount {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupMovement(
        &self,
        a1: f32,
        a2: *mut gfc__CharMoveVars,
    ) -> bool {
        ((*self.vfptr).setupMovement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyMovement(&self, a1: f32) {
        ((*self.vfptr).applyMovement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateAttributes(&self) {
        ((*self.vfptr).invalidateAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isAttributeValid(&self, a1: i32) -> bool {
        ((*self.vfptr).isAttributeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAttribute(&self, a1: i32) -> *mut i32 {
        ((*self.vfptr).getAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptAttribute(&self, a1: i32) -> i32 {
        ((*self.vfptr).getScriptAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateAttributes(&self, a1: f32) {
        ((*self.vfptr).updateAttributes)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKillingBlow(&self) {
        ((*self.vfptr).doKillingBlow)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isFlying(&self) -> bool {
        ((*self.vfptr).isFlying)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn validateInteractive(&self, a1: u32) -> bool {
        ((*self.vfptr).validateInteractive)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doInteractive(
        &self,
        a1: *mut gfc__CharacterDoInteractiveDesc,
        a2: gfc__AutoRef_gfc__Character_,
        a3: bool,
    ) {
        ((*self.vfptr).doInteractive)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onInterrupt(&self) {
        ((*self.vfptr).onInterrupt)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn grab(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).grab)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn throww(
        &self,
        a1: *mut gfc__Character,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).throww)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drop(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).drop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGrabNode(
        &self,
        result: *mut gfc__HString,
        a2: *mut gfc__Character,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getGrabNode)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn onGrabbableWeaponized(
        &self,
        a1: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).onGrabbableWeaponized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pickupDraggable(&self, a1: *mut gfc__DraggableActor) {
        ((*self.vfptr).pickupDraggable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doMounted(&self, a1: *mut gfc__Character, a2: i32) {
        ((*self.vfptr).doMounted)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn findBestTargetInRange(&self, a1: f32) -> *mut gfc__Character {
        ((*self.vfptr).findBestTargetInRange)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findBestTarget(&self) -> *mut gfc__Character {
        ((*self.vfptr).findBestTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn distanceTo(&self, a1: *mut gfc__Character) -> f32 {
        ((*self.vfptr).distanceTo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findGrabbable(&self) -> *mut gfc__Actor {
        ((*self.vfptr).findGrabbable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRotationOnly(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotationOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHeadingOnly(&self, a1: f32) {
        ((*self.vfptr).setHeadingOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCenterOffset(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setBody(&self) {
        ((*self.vfptr).setBody)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupCharacterProxy(&self) {
        ((*self.vfptr).setupCharacterProxy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActualVelocity(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getActualVelocity)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn updateLastGroundMaterial(&self) {
        ((*self.vfptr).updateLastGroundMaterial)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doHitPause(&self, a1: f32) {
        ((*self.vfptr).doHitPause)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doMounted_2(&self, a1: gfc__AutoRef_gfc__Player_, a2: i32) {
        ((*self.vfptr).doMounted_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRiderDeath(&self) {
        ((*self.vfptr).onRiderDeath)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDismountState(&self) -> i32 {
        ((*self.vfptr).getDismountState)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn disperseMount(&self, a1: i32) {
        ((*self.vfptr).disperseMount)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onCinematicIdle(&self) -> bool {
        ((*self.vfptr).onCinematicIdle)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Mount__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__TBox_float_gfc__FloatMath_),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Mount,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Mount,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Mount) -> bool,
    pub enteredDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: gfc__AutoRef_gfc__DetectorObject_),
    pub exitedDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: gfc__AutoRef_gfc__DetectorObject_),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__HitInfo),
    pub queryHit: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub setupMovement: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: f32,
        _: *mut gfc__CharMoveVars,
    ) -> bool,
    pub applyMovement: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub invalidateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub isAttributeValid: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32) -> bool,
    pub getAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32) -> *mut i32,
    pub getScriptAttribute: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32) -> i32,
    pub updateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub doKillingBlow: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub isFlying: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
    pub validateInteractive: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: u32) -> bool,
    pub doInteractive: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__CharacterDoInteractiveDesc,
        _: gfc__AutoRef_gfc__Character_,
        _: bool,
    ),
    pub onInterrupt: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub grab: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Character),
    pub throww: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drop: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Character),
    pub getGrabNode: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__HString,
        _: *mut gfc__Character,
    ) -> *mut gfc__HString,
    pub onGrabbableWeaponized: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ),
    pub pickupDraggable:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__DraggableActor),
    pub doMounted:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Character, _: i32),
    pub findBestTargetInRange:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32) -> *mut gfc__Character,
    pub findBestTarget: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut gfc__Character,
    pub distanceTo:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: *mut gfc__Character) -> f32,
    pub findGrabbable: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> *mut gfc__Actor,
    pub setRotationOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHeadingOnly: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub getCenterOffset: unsafe extern "thiscall" fn(
        this: *const gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setBody: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Mount,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: f32),
    pub doMounted_2:
        unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: gfc__AutoRef_gfc__Player_, _: i32),
    pub onRiderDeath: unsafe extern "thiscall" fn(this: *mut gfc__Mount),
    pub getDismountState: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> i32,
    pub disperseMount: unsafe extern "thiscall" fn(this: *mut gfc__Mount, _: i32),
    pub onCinematicIdle: unsafe extern "thiscall" fn(this: *mut gfc__Mount) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HitRecord_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HitRecord,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__SaveValue__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__SaveValue_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__CharMoveVars {
    pub vfptr: *const gfc__CharMoveVars__vftable,
}

impl gfc__CharMoveVars {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setOnGround(&self) {
        ((*self.vfptr).setOnGround)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setInAir(&self) {
        ((*self.vfptr).setInAir)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn forceCollisionCheck(&self) {
        ((*self.vfptr).forceCollisionCheck)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setUseObstacleAvoidance(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setUseObstacleAvoidance)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn setMovementVelocity(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setMovementVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setExternalVelocity(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setExternalVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setGravity(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setGravity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCharacter(&self) -> *mut gfc__Character {
        ((*self.vfptr).getCharacter)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__CharMoveVars__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CharMoveVars, _: u32) -> *mut (),
    pub setOnGround: unsafe extern "thiscall" fn(this: *mut gfc__CharMoveVars),
    pub setInAir: unsafe extern "thiscall" fn(this: *mut gfc__CharMoveVars),
    pub forceCollisionCheck: unsafe extern "thiscall" fn(this: *mut gfc__CharMoveVars),
    pub setUseObstacleAvoidance:
        unsafe extern "thiscall" fn(this: *mut gfc__CharMoveVars, _: f32, _: f32, _: f32),
    pub setMovementVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveVars,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setExternalVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveVars,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setGravity: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveVars,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getCharacter:
        unsafe extern "thiscall" fn(this: *const gfc__CharMoveVars) -> *mut gfc__Character,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__EquippableItem_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__LiquidRegion_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__IKSolver_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Character_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__LiquidRegion_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldObjectData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Inventory_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Player__HairSim__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Player__HairSim_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__KinematicActor {
    // gfc__IRefObject
    pub vfptr: *const gfc__KinematicActor__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
}

unsafe impl UpcastToNop<gfc__Actor> for gfc__KinematicActor {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__KinematicActor {}

unsafe impl UpcastToNop<gfc__Object> for gfc__KinematicActor {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__KinematicActor {}

impl gfc__KinematicActor {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__KinematicActor__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__KinematicActor,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
    pub setScale_2:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__KinematicActor,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__KinematicActor) -> bool,
    pub enteredDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub exitedDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub queryStrike:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__HitInfo),
    pub queryHit:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__HitInfo),
    pub recordHit:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__KinematicActor,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
}

#[repr(C)]
pub struct gfc__KinematicActor__KAnimation {
    pub ID: i32,
    pub Elapsed: f32,
    pub Controller: gfc__AutoRef_gfc__AnimController_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__AutoJumpSolver__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__AutoJumpSolver_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__CharMoveStateDesc {
    // gfc__IRefObject
    pub vfptr: *const gfc__CharMoveStateDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CharMoveStateDesc
    pub mID: u32,
    pub mName: gfc__HString,
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mTransitions: gfc__Vector_gfc__AutoRef_gfc__CharTransitionDesc__0_gfc__CAllocator_,
    pub mCameraStateDesc: gfc__AutoRef_gfc__CameraStateDesc_,
    pub mOnActionComplete: i32,
    pub mOnBlock: i32,
    pub mOnFall: i32,
    pub mOnIdle: i32,
    pub mOnJump: i32,
    pub mOnMove: i32,
    pub mOnSwim: i32,
    pub mOnImpulseImpact: i32,
    pub mAcceleration: gfc__Half,
    pub mDeceleration: gfc__Half,
    pub mGravityScale: gfc__Half,
    pub mMaxSpeed: gfc__Half,
    pub mMoveStrength: gfc__Half,
    pub mMoveWeight: gfc__Half,
    pub mTurnSpeed: gfc__Half,
    pub mOnLeaveSpeedScale: gfc__Half,
    pub mImpulseDamage: u16,
    pub mImpulsePeriod: gfc__Half,
    pub mDamageMitigation: i8,
    pub mPowerResistance: i8,
    pub mAutoJumpSolverID: u8,
    pub mCharacterFlags: gfc__TFlags_unsigned_short_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__CharMoveStateDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CharMoveStateDesc {}

impl gfc__CharMoveStateDesc {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn createMoveState(
        &self,
        result: *mut gfc__AutoRef_gfc__CharacterMoveState_,
        a2: *mut gfc__Character,
    ) -> *mut gfc__AutoRef_gfc__CharacterMoveState_ {
        ((*self.vfptr).createMoveState)(self as *const _ as *mut _, result, a2)
    }
}

#[repr(C)]
pub struct gfc__CharMoveStateDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CharMoveStateDesc, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__CharMoveStateDesc) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CharMoveStateDesc, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__CharMoveStateDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CharMoveStateDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveStateDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CharMoveStateDesc) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveStateDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveStateDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub createMoveState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveStateDesc,
        result: *mut gfc__AutoRef_gfc__CharacterMoveState_,
        _: *mut gfc__Character,
    )
        -> *mut gfc__AutoRef_gfc__CharacterMoveState_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TriggerRegionDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CharacterProxy_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__ButtonMashPoint__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__ButtonMashPoint_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DraggableActorInfo_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__Character__CVisual_0_gfc__CAllocator_ {
    pub mData: *mut gfc__Character__CVisual,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__Character__PActor_0_gfc__CAllocator_ {
    pub mData: *mut gfc__Character__PActor,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AnimController_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__HitInfo {
    // gfc__IRefObject
    pub vfptr: *const gfc__HitInfo__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__HitInfo
    pub mID: u32,
    pub mName: gfc__HString,
    pub mHitLocation: u8,
    pub mWType: u8,
    pub mHType: u8,
    pub mStrikeDirection: u8,
    pub mPower: i32,
    pub mDamage: f32,
    pub mKnockBackPower: f32,
    pub mKnockUpPower: f32,
    pub mKnockBackPowerVariance: f32,
    pub mKnockUpPowerVariance: f32,
    pub mEnvPowerScale: f32,
    pub mEnvDamageScale: f32,
    pub mComboLinkTime: f32,
    pub mDamageMod: i32,
    pub mMeleeDamageMod: i32,
    pub mRangedDamageMod: i32,
    pub mWrathDamageMod: i32,
    pub mGrabbableDamageMod: i32,
    pub mKnockBackMod: i32,
    pub mChaosMod: i32,
    pub mWrathMod: i32,
    pub mWrathBonus: i32,
    pub mWrathBonusOnDeath: i32,
    pub mSoulsMod: i32,
    pub mHealthSteal: i32,
    pub mWeaponXPMod: i32,
    pub mOnHitAnimOverlayID: i32,
    pub mOnBlockedAnimOverlayID: i32,
    pub mCombatEffect: gfc__HString,
    pub mHitMaterial: gfc__HString,
    pub mImpactFX: gfc__HString,
    pub mKillFX: gfc__HString,
    pub mBlockFX: gfc__HString,
    pub mImpactPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mImpactDir: gfc__TVector3_float_gfc__FloatMath_,
    pub mHitMaterialID: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mRuntimeFlags: gfc__TFlags_unsigned_char_,
    pub mSource: *mut gfc__Actor,
    pub mCause: *mut gfc__Actor,
    pub mTarget: *mut gfc__Actor,
    pub mHitPauseTime: f32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__HitInfo {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__HitInfo {}

impl gfc__HitInfo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__HitInfo__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__HitInfo, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__HitInfo) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__HitInfo, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__HitInfo) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__HitInfo) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__HitInfo,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__HitInfo) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__HitInfo, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__HitInfo,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__PlayerContextMan__DistanceMetrics {
    pub mInteractiveDetected: bool,
    pub mInteractivePosition: gfc__TVector3_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__ActorTarget {
    pub Actor: gfc__AutoRef_gfc__Actor_,
    pub Body: gfc__AutoRef_gfc__Body_,
    pub WorldObject: gfc__AutoRef_gfc__WorldObject_,
    pub Position: gfc__TVector3_float_gfc__FloatMath_,
    pub Normal: gfc__TVector3_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CameraStateDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__MoveInput {
    pub MoveLR: f32,
    pub MoveUD: f32,
    pub MoveLRRaw: f32,
    pub MoveUDRaw: f32,
    pub MoveDirection: f32,
    pub MoveSpeed: f32,
    __pdbindgen_padding: [u8; 8],
    pub CameraMatrix: gfc__Matrix4,
    pub CameraYaw: f32,
    pub CameraPitch: f32,
    pub CameraYawRaw: f32,
    pub CameraPitchRaw: f32,
    pub TargetYaw: f32,
    pub TargetPitch: f32,
    pub MoveDirectionChanged: bool,
    pub Strafe: bool,
    pub Move: bool,
    pub MoveDown: bool,
    pub WalkToggleDown: bool,
    pub Block: bool,
    pub BlockDown: bool,
    pub BlockUp: bool,
    pub BlockHeld: bool,
    pub Jump: bool,
    pub JumpDown: bool,
    pub Context: bool,
    pub ContextDown: bool,
    pub DraggableToggle: bool,
    pub Release: bool,
    pub ReleaseDown: bool,
    pub Attack: bool,
    pub AttackDown: bool,
    pub Activate: bool,
    pub ActivateDown: bool,
    pub PowerAttack: bool,
    pub PowerAttackDown: bool,
    pub WeaponFire: bool,
    pub WeaponFireDown: bool,
    pub WeaponFireSecondary: bool,
    pub WeaponFireSecondaryDown: bool,
    pub WeaponFireTertiary: bool,
    pub WeaponFireTertiaryDown: bool,
    pub SwimUp: bool,
    pub SwimUpDown: bool,
    pub SwimDown: bool,
    pub SwimDownDown: bool,
    pub Focus: bool,
    pub FocusDown: bool,
    pub Wrath: bool,
    pub WrathDown: bool,
    pub LThumb: bool,
    pub LThumbDown: bool,
    pub RThumb: bool,
    pub RThumbDown: bool,
    pub DPadUp: bool,
    pub DPadUpDown: bool,
    pub DPadLeft: bool,
    pub DPadLeftDown: bool,
    pub DPadDown: bool,
    pub DPadDownDown: bool,
    pub DPadRight: bool,
    pub DPadRightDown: bool,
    pub GearScrollUp: bool,
    pub GearScrollUpDown: bool,
    pub GearScrollDown: bool,
    pub GearScrollDownDown: bool,
    pub HasTarget: bool,
    pub Target: gfc__ActorTarget,
    pub MoveDirectionQuad: gfc__MoveInput__DirectionQuad,
    pub MoveFBHemisphere: gfc__MoveInput__DirectionHemispheres,
    pub MoveLRHemisphere: gfc__MoveInput__DirectionHemispheres,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PlayerContextMan_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__HitRecord {
    pub mDamage: f32,
    pub mTimeStamp: f32,
    pub mSource: *mut gfc__Actor,
}

#[repr(C)]
pub struct gfc__SaveData {
    // gfc__IRefObject
    pub vfptr: *const gfc__SaveData__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__SaveData
    pub mValues: gfc__Vector_gfc__AutoRef_gfc__SaveValue__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__SaveData {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__SaveData {}

impl gfc__SaveData {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__SaveData__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SaveData, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__SaveData) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__SaveData, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__SaveData) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__SaveData) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__SaveData,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__SaveData) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__SaveData, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__SaveData,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__DamageThresholdCallback_0_gfc__CAllocator_ {
    pub mData: *mut gfc__DamageThresholdCallback,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__HavokWeaponProxy_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LoadRegion {
    // gfc__IRefObject
    pub vfptr: *const gfc__LoadRegion__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__PhysicsShapeObject
    pub mShape: i32,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
    pub mBounds: gfc__TBox_float_gfc__FloatMath_,
    pub mGizmo: *mut gfc__PhysicsShapeGizmo,
    // gfc__DetectorObject
    __pdbindgen_padding: [u8; 4],
    pub mBodyType: u8,
    pub mRegion: *mut gfc__DetectorRegion,
    pub mEnabled: bool,
    // gfc__LoadRegion
    __pdbindgen_padding_2: [u8; 7],
    pub mLoadOnEnter: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub mUnloadOnEnter: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub mLoadOnExit: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub mUnloadOnExit: gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
    pub mRequired: bool,
}

unsafe impl UpcastToNop<gfc__DetectorObject> for gfc__LoadRegion {}

unsafe impl UpcastToNop<gfc__PhysicsShapeObject> for gfc__LoadRegion {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__LoadRegion {}

unsafe impl UpcastToNop<gfc__Object> for gfc__LoadRegion {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__LoadRegion {}

impl gfc__LoadRegion {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGizmoColor(
        &self,
    ) -> *const gfc__TVector4_float_gfc__FloatMath_ {
        ((*self.vfptr).getGizmoColor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGizmoIcon(&self) -> *const gfc__HString {
        ((*self.vfptr).getGizmoIcon)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPhantomBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getPhantomBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onEnter(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).onEnter)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onExit(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).onExit)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__LoadRegion__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__LoadRegion) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__LoadRegion) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__LoadRegion,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion),
    pub getGizmoColor: unsafe extern "thiscall" fn(
        this: *const gfc__LoadRegion,
    )
        -> *const gfc__TVector4_float_gfc__FloatMath_,
    pub getGizmoIcon:
        unsafe extern "thiscall" fn(this: *const gfc__LoadRegion) -> *const gfc__HString,
    pub getPhantomBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadRegion,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub onEnter: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__Actor),
    pub onExit: unsafe extern "thiscall" fn(this: *mut gfc__LoadRegion, _: *mut gfc__Actor),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CharacterMoveState_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__HitInfo_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DraggableActor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ChannelAccessor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DamageThresholdCallback {
    pub mDamage: f32,
    pub mDuration: f32,
    pub mStartTime: f32,
    pub mID: u32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CharacterLookAtSolver_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__CharTransitionDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CharTransitionDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PlayerStatTracker_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Object3D_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CollisionPhantom_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DetectorRegion {
    // hkpWorldPostSimulationListener
    pub vfptr: *const gfc__DetectorRegion__vftable,
    // gfc__PhysicsDetectRegion
    pub mPhantomOverlapListenerProxy: gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
    pub mWorld: *mut gfc__World,
    pub mPhantom: *mut hkpPhantom,
    pub mBodies: gfc__Vector_gfc__PhysicsDetectRegion__BodyInfo_0_gfc__CAllocator_,
    pub mNumContainedBodies: i32,
    pub mIsShapeDetector: bool,
    pub mAABBListChanged: bool,
    // gfc__DetectorRegion
    pub mOwner: *mut gfc__DetectorObject,
    pub mActors: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mActorRefs: gfc__Vector_int_0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__PhysicsDetectRegion> for gfc__DetectorRegion {}

unsafe impl UpcastToNop<hkpWorldPostSimulationListener> for gfc__DetectorRegion {}

impl gfc__DetectorRegion {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).postSimulationCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inactiveEntityMovedCallback(&self, a1: *mut hkpEntity) {
        ((*self.vfptr).inactiveEntityMovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn bodyEntered(&self, a1: *mut gfc__Body) {
        ((*self.vfptr).bodyEntered)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn bodyExited(
        &self,
        a1: *mut gfc__Body,
        a2: *mut gfc__WorldObject,
    ) {
        ((*self.vfptr).bodyExited)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn postSimulationCallback_2(&self) {
        ((*self.vfptr).postSimulationCallback_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn collidableAddedCallback(
        &self,
        a1: *const hkpCollidableAddedEvent,
    ) {
        ((*self.vfptr).collidableAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collidableRemovedCallback(
        &self,
        a1: *const hkpCollidableRemovedEvent,
    ) {
        ((*self.vfptr).collidableRemovedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DetectorRegion__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut hkpEntity),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut gfc__World),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion),
    pub bodyEntered: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion, _: *mut gfc__Body),
    pub bodyExited: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorRegion,
        _: *mut gfc__Body,
        _: *mut gfc__WorldObject,
    ),
    pub postSimulationCallback_2: unsafe extern "thiscall" fn(this: *mut gfc__DetectorRegion),
    pub collidableAddedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorRegion,
        _: *const hkpCollidableAddedEvent,
    ),
    pub collidableRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorRegion,
        _: *const hkpCollidableRemovedEvent,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CharTransitionDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__InteractiveTransition__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__InteractiveTransition_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Inventory {
    // gfc__IRefObject
    pub vfptr: *const gfc__Inventory__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Inventory
    pub mOwner: *mut gfc__Character,
    pub mContainers: gfc__Vector_gfc__AutoRef_gfc__Container__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Inventory {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Inventory {}

impl gfc__Inventory {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Inventory__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Inventory, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Inventory) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Inventory, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Inventory) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Inventory) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Inventory,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Inventory) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Inventory, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Inventory,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Container {
    // gfc__IRefObject
    pub vfptr: *const gfc__Container__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Container
    pub mName: gfc__HString,
    pub mNumSlots: i32,
    pub mOwner: *mut gfc__Character,
    pub mItems: gfc__Vector_gfc__AutoRef_gfc__Item__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Container {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Container {}

impl gfc__Container {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Container__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Container, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Container) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Container, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Container) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Container) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Container,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Container) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Container, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Container,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Container__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Container_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ChaosLevel_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PlayerStatTracker {
    // gfc__IRefObject
    pub vfptr: *const gfc__PlayerStatTracker__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__PlayerStatTracker
    pub mNumberOfKills: u32,
    pub mNumberOfKillsMounted: u32,
    pub mNumberOfKillsOnRuin: u32,
    pub mNumberOfKillsOnBird: u32,
    pub mNumberOfKillsOnFoot: u32,
    pub mNumberOfDemonsKilled: u32,
    pub mNumberOfDemonsKilledOnHorseback: u32,
    pub mNumberOfEnemiesKilledWithEnvironment: u32,
    pub mMostNumberOfKillsInSingleAttack: u32,
    pub mMostNumberOfKillsInSingleAttackWithWrath: u32,
    pub mNumberOfDeaths: u32,
    pub mDamageDealt: f32,
    pub mDamageReceived: f32,
    pub mDamageDealtWarSword: f32,
    pub mDamageDealtScythe: f32,
    pub mDamageDealtTremorGauntlet: f32,
    pub mHealthCollected: f32,
    pub mDistanceTraveled: f32,
    pub mDistanceTraveledMounted: f32,
    pub mDistanceTraveledOnRuin: f32,
    pub mDistanceTraveledOnBird: f32,
    pub mDistanceTraveledOnFoot: f32,
    pub mNumberOfDashes: u32,
    pub mNumberOfObjectsPickedUp: u32,
    pub mNumberOfEnvWeaponsPickedUp: u32,
    pub mNumberOfFocusUses: u32,
    pub mSoulsCollected: u32,
    pub mTotalGameTime: u32,
    pub mTotalCombatTime: f32,
    pub mCurrentComboCount: u32,
    pub mHighestComboCount: u32,
    pub mNumberOfChestsOpened: u32,
    pub mMostBatsKilledWithoutTouchingGround: u32,
    pub mNumberOfArtifactsCollected: u32,
    pub mDeltaTime: f32,
    pub mCurrentNumberOfKillsInAttack: u32,
    pub mCurrentNumberOfKillsInAttackWithWrath: u32,
    pub mNumberOfBatsKilledWithoutTouchingGround: u32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__PlayerStatTracker {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PlayerStatTracker {}

impl gfc__PlayerStatTracker {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__PlayerStatTracker__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PlayerStatTracker, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__PlayerStatTracker) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__PlayerStatTracker, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__PlayerStatTracker) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__PlayerStatTracker) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__PlayerStatTracker,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__PlayerStatTracker) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__PlayerStatTracker,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PlayerStatTracker,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Item__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Item_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__WindowHelper {
    // gfc__IRefObject
    pub vfptr: *const gfc__WindowHelper__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Helper
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    // gfc__WindowHelper
    pub mWindowStack: gfc__Vector_gfc__AutoRef_gfc___UIControl__0_gfc__CAllocator_,
    pub mOverlayWindow: gfc__AutoRef_gfc___UIControl_,
    pub mEvenOverlayederWindow: gfc__AutoRef_gfc___UIControl_,
    pub mWindow: gfc__AutoRef_gfc___UIControl_,
    pub mRootWindow: gfc__AutoRef_gfc___UIControl_,
}

unsafe impl UpcastToNop<gfc__Helper> for gfc__WindowHelper {}

unsafe impl UpcastToNop<gfc__Object> for gfc__WindowHelper {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WindowHelper {}

impl gfc__WindowHelper {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn init(&self) {
        ((*self.vfptr).init)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdown(&self) {
        ((*self.vfptr).shutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__WindowHelper__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WindowHelper) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WindowHelper) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WindowHelper,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__WindowHelper,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WindowHelper,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__WindowHelper),
}

#[repr(C)]
pub struct gfc__EquippableItem {
    // gfc__IRefObject
    pub vfptr: *const gfc__EquippableItem__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Item
    __pdbindgen_padding_2: [u8; 4],
    pub mValid: bool,
    pub mAutoPersist: bool,
    pub mInfoID: u32,
    pub mContainer: *mut gfc__Container,
    pub mParticleSystem: i32,
    // gfc__EquippableItem
    pub mEquipped: bool,
    pub mDirectionSlot: i32,
    pub mMustReequip: bool,
    pub mHotKey: i32,
    pub mVisualAttached: bool,
    pub mMountPoint: i32,
    pub mQueuedMountPoint: i32,
    pub mLoadState: i32,
    pub mLoadMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mMountNode: *mut gfc__Node3D,
    pub mQueuePreload: bool,
    pub mQueueLoadVisuals: bool,
    pub mQueueShow: bool,
    pub mWasVisualAttached: bool,
    pub mIsVisible: bool,
    pub mPreviousMountPoint: i32,
}

unsafe impl UpcastToNop<gfc__Item> for gfc__EquippableItem {}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__EquippableItem {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__EquippableItem {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__EquippableItem {}

unsafe impl UpcastToNop<gfc__Object> for gfc__EquippableItem {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__EquippableItem {}

impl gfc__EquippableItem {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPickup(&self) {
        ((*self.vfptr).onPickup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDrop(&self) {
        ((*self.vfptr).onDrop)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onChanged(&self) {
        ((*self.vfptr).onChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Container) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadVisuals(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).loadVisuals)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unloadVisuals(&self) {
        ((*self.vfptr).unloadVisuals)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoadSaveData(&self) {
        ((*self.vfptr).onLoadSaveData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invokeOnChanged(&self) {
        ((*self.vfptr).invokeOnChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDescription(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getDescription)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn doKilled(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKilled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn load(&self) {
        ((*self.vfptr).load)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unload(&self) {
        ((*self.vfptr).unload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isLoaded(&self) -> bool {
        ((*self.vfptr).isLoaded)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isEquipped(&self) -> bool {
        ((*self.vfptr).isEquipped)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attachRef(&self) {
        ((*self.vfptr).attachRef)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn detachRef(&self) {
        ((*self.vfptr).detachRef)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAimOffset(
        &self,
        a1: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getAimOffset)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHotKey(&self, a1: i32) {
        ((*self.vfptr).setHotKey)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn equipExclusive(&self) {
        ((*self.vfptr).equipExclusive)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn equip(&self) {
        ((*self.vfptr).equip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unequip(&self) {
        ((*self.vfptr).unequip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn show(&self) {
        ((*self.vfptr).show)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn hide(&self) {
        ((*self.vfptr).hide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoaded(&self) {
        ((*self.vfptr).onLoaded)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isBusy(&self) -> bool {
        ((*self.vfptr).isBusy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn render_2(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *const gfc__Matrix4,
    ) {
        ((*self.vfptr).render_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn isReady(&self) -> bool {
        ((*self.vfptr).isReady)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn autoEquip(&self) {
        ((*self.vfptr).autoEquip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn equipAfterLoad(&self) {
        ((*self.vfptr).equipAfterLoad)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__EquippableItem__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__EquippableItem,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: f32),
    pub setScale_2:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__EquippableItem,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__EquippableItem) -> bool,
    pub enteredDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub exitedDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub queryStrike:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo),
    pub queryHit:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo),
    pub recordHit:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: f32),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onPickup: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onDrop: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onChanged: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__Container),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub loadVisuals:
        unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__Actor),
    pub unloadVisuals: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onLoadSaveData: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub invokeOnChanged: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getDescription: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub doKilled: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: *mut gfc__HitInfo),
    pub load: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub isLoaded: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub isEquipped: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub attachRef: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub detachRef: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub getAimOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHotKey: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem, _: i32),
    pub equipExclusive: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub equip: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub unequip: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub show: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub hide: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub onLoaded: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub isBusy: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub render_2: unsafe extern "thiscall" fn(
        this: *mut gfc__EquippableItem,
        _: *mut gfc__UIRenderer,
        _: *const gfc__Matrix4,
    ),
    pub isReady: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem) -> bool,
    pub autoEquip: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
    pub equipAfterLoad: unsafe extern "thiscall" fn(this: *mut gfc__EquippableItem),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc___UIControl__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc___UIControl_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TeleportHelper {
    // gfc__IRefObject
    pub vfptr: *const gfc__TeleportHelper__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Helper
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    // gfc__TeleportHelper
    pub mTeleportTransition: bool,
}

unsafe impl UpcastToNop<gfc__Helper> for gfc__TeleportHelper {}

unsafe impl UpcastToNop<gfc__Object> for gfc__TeleportHelper {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__TeleportHelper {}

impl gfc__TeleportHelper {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn init(&self) {
        ((*self.vfptr).init)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdown(&self) {
        ((*self.vfptr).shutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__TeleportHelper__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__TeleportHelper) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__TeleportHelper) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__TeleportHelper,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__TeleportHelper,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__TeleportHelper,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__TeleportHelper),
}

#[repr(C)]
pub struct gfc__Hud {
    // gfc__IRefObject
    pub vfptr: *const gfc__Hud__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc___UIControl_
    pub vfptr_2: *const gfc__Hud__vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
    // gfc___UIControl
    pub mID: i32,
    pub mName: gfc__HString,
    pub mOpacity: f32,
    pub mContext: i32,
    pub mWindowSize: i32,
    pub mVisual: gfc__AutoRef_gfc___UIVisual_,
    pub mClipMask: gfc__HString,
    pub mIgnoreHideUI: bool,
    pub mAnchorPoint: u8,
    pub mAnchorRelativePoint: u8,
    pub mAnchorRelativeCtrl: gfc__HString,
    pub mAnchorRelSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorRelOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub OnEvent: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnAction: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnMouse: gfc__TUIEventBroadcaster_gfc__MouseEvent_,
    pub OnKeyboard: gfc__TUIEventBroadcaster_gfc__KeyboardEvent_,
    pub OnFocus: gfc__TUIEventBroadcaster_gfc__FocusEvent_,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mLastVisibilityState: bool,
    pub mPackageID: i32,
    pub mIndents: gfc__TRect_float_,
    pub mPosition: gfc__TVector2_float_gfc__FloatMath_,
    pub mSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub mRotation: f32,
    pub mScale: gfc__TVector2_float_gfc__FloatMath_,
    pub mLayoutManager: gfc__AutoRef_gfc__UILayoutManager_,
    pub mLayoutHint: f32,
    pub mCurrentActions: gfc__Vector_gfc__AutoRef_gfc___UIAction__0_gfc__CAllocator_,
    pub mClipMaterial: gfc__AutoRef_gfc__Material_,
    pub mOnInitCalledAlready: bool,
    pub mFirstDraw: bool,
    pub mLastDialogResult: gfc__AutoRef_gfc__Value_,
    // gfc__UIScriptControl
    // gfc__Hud
    __pdbindgen_padding: [u8; 4],
    pub mHudGearID: i32,
    pub mHudSoulsID: i32,
    pub mHudTextID: i32,
    pub mHudTimerID: i32,
    pub mHudComboID: i32,
    pub mHudChallengeCounterID: i32,
    pub mHudWrathID: i32,
    pub mHudBossHealthID: i32,
    pub mHudSmallIconID: i32,
    pub mHudActiveGearID: i32,
    pub mHudScreenFadeID: i32,
    pub mHudCharacterTitleID: i32,
    pub mHudEnvironmentTitleID: i32,
}

unsafe impl UpcastToNop<gfc__UIScriptControl> for gfc__Hud {}

unsafe impl UpcastToNop<gfc___UIControl> for gfc__Hud {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Hud {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Hud {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc___UIControl_> for gfc__Hud {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc___UIControl_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__Hud {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addFront(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addFront)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBack(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addBack)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn add(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).add)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn remove(&self) {
        ((*self.vfptr).remove)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn remove_2(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).remove_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn added(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).added)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removed(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).removed)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateHierarchy(&self) {
        ((*self.vfptr).invalidateHierarchy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getEnabled(&self) -> bool {
        ((*self.vfptr).getEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setVisible(&self, a1: bool) {
        ((*self.vfptr).setVisible)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVisible(&self) -> bool {
        ((*self.vfptr).getVisible)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFocusTraversable(&self, a1: bool) {
        ((*self.vfptr).setFocusTraversable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFocusTraversable(&self) -> bool {
        ((*self.vfptr).getFocusTraversable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setMouseEnabled(&self, a1: bool) {
        ((*self.vfptr).setMouseEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMouseEnabled(&self) -> bool {
        ((*self.vfptr).getMouseEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setLayoutEnabled(&self, a1: bool) {
        ((*self.vfptr).setLayoutEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutEnabled(&self) -> bool {
        ((*self.vfptr).getLayoutEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setClipChildren(&self, a1: bool) {
        ((*self.vfptr).setClipChildren)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClipChildren(&self) -> bool {
        ((*self.vfptr).getClipChildren)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRegisterControl(&self, a1: bool) {
        ((*self.vfptr).setRegisterControl)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRegisterControl(&self) -> bool {
        ((*self.vfptr).getRegisterControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setText(&self, a1: *const gfc__WString) {
        ((*self.vfptr).setText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSize(&self, a1: *const gfc__TVector2_float_gfc__FloatMath_) {
        ((*self.vfptr).setSize)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSizeValid(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setSizeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPreferredSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPreferredSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getAbsolutePosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAbsolutePosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: f32) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(&self) -> f32 {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutManager(
        &self,
        a1: gfc__AutoRef_gfc__UILayoutManager_,
    ) {
        ((*self.vfptr).setLayoutManager)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutManager(
        &self,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    ) -> *mut gfc__AutoRef_gfc__UILayoutManager_ {
        ((*self.vfptr).getLayoutManager)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutHint(&self, a1: f32) {
        ((*self.vfptr).setLayoutHint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutHint(&self) -> f32 {
        ((*self.vfptr).getLayoutHint)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).addAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).removeAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearAllActions(&self) {
        ((*self.vfptr).clearAllActions)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateActions(&self, a1: f32) {
        ((*self.vfptr).updateActions)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateLayout(&self) {
        ((*self.vfptr).invalidateLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doAnchorLayout(&self) {
        ((*self.vfptr).doAnchorLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doLayout(&self) {
        ((*self.vfptr).doLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAnchorOffset(
        &self,
        a1: gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setAnchorOffset)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorOffset(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn draw(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *mut gfc__TRect_long_,
    ) {
        ((*self.vfptr).draw)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pick(
        &self,
        a1: gfc__TVector2_int_gfc__FloatMath_,
        a2: bool,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).pick)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getControlByID(&self, a1: i32) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getControlByName(
        &self,
        a1: *const gfc__HString,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setControlText(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__WString,
    ) -> bool {
        ((*self.vfptr).setControlText)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlTextA(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__String,
    ) -> bool {
        ((*self.vfptr).setControlTextA)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlVisible(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlVisible)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlEnabled(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlEnabled)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processMouseEvent(&self, a1: *mut gfc__MouseEvent) {
        ((*self.vfptr).processMouseEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processKeyboardEvent(&self, a1: *mut gfc__KeyboardEvent) {
        ((*self.vfptr).processKeyboardEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processFocusEvent(&self, a1: *mut gfc__FocusEvent) {
        ((*self.vfptr).processFocusEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onReInit(&self) {
        ((*self.vfptr).onReInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDeInit(&self) {
        ((*self.vfptr).onDeInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onVisibilityLost(&self) {
        ((*self.vfptr).onVisibilityLost)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDialogResults(&self, a1: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setDialogResults)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLastDialogResult(
        &self,
        a1: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool {
        ((*self.vfptr).getLastDialogResult)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unregisterToolTipEvent(&self) {
        ((*self.vfptr).unregisterToolTipEvent)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getInputListener(
        &self,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_ {
        ((*self.vfptr).getInputListener)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn initControl(&self) {
        ((*self.vfptr).initControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn postInitControl(&self) {
        ((*self.vfptr).postInitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deinitControl(&self) {
        ((*self.vfptr).deinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reinitControl(&self) {
        ((*self.vfptr).reinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doInit(&self) {
        ((*self.vfptr).doInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn drawInternal(&self, a1: *mut gfc__UIRenderer) {
        ((*self.vfptr).drawInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        a2: *mut gfc___UIControl,
        a3: u8,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorPosition)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn getGlobalScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getGlobalScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getParentSize(
        &self,
        a1: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getParentSize)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Hud__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: u32) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub addBack: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub remove_2: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub added: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub removed: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc___UIControl),
    pub invalidateHierarchy: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub getEnabled: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setVisible: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getVisible: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: bool),
    pub getRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> bool,
    pub setText: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *const gfc__WString),
    pub setSize: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub setSizeValid: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getSize: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getPreferredSize: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getAbsolutePosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Hud,
            result: *mut gfc__TVector2_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: f32),
    pub getRotation: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> f32,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setLayoutManager:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: gfc__AutoRef_gfc__UILayoutManager_),
    pub getLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    )
        -> *mut gfc__AutoRef_gfc__UILayoutManager_,
    pub setLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: f32),
    pub getLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__Hud) -> f32,
    pub addAction:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: gfc__AutoRef_gfc___UIAction_),
    pub removeAction:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: gfc__AutoRef_gfc___UIAction_),
    pub clearAllActions: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub updateActions: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: f32),
    pub invalidateLayout: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub doAnchorLayout: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub doLayout: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub setAnchorOffset:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: gfc__TVector2_float_gfc__FloatMath_),
    pub getAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub draw: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *mut gfc__UIRenderer,
        _: *mut gfc__TRect_long_,
    ),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: f32),
    pub pick: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: gfc__TVector2_int_gfc__FloatMath_,
        _: bool,
    ) -> *mut gfc___UIControl,
    pub getControlByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: i32) -> *mut gfc___UIControl,
    pub getControlByName: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__HString,
    ) -> *mut gfc___UIControl,
    pub setControlText: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__HString,
        _: *const gfc__WString,
    ) -> bool,
    pub setControlTextA: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        _: *const gfc__HString,
        _: *const gfc__String,
    ) -> bool,
    pub setControlVisible:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *const gfc__HString, _: bool) -> bool,
    pub setControlEnabled:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *const gfc__HString, _: bool) -> bool,
    pub processMouseEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc__MouseEvent),
    pub processKeyboardEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc__KeyboardEvent),
    pub processFocusEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc__FocusEvent),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub onReInit: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub onDeInit: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub onVisibilityLost: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub setDialogResults:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: gfc__AutoRef_gfc__Value_),
    pub getLastDialogResult:
        unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc__AutoRef_gfc__Value_) -> bool,
    pub unregisterToolTipEvent: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub getInputListener: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_,
    pub initControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub postInitControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub deinitControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub reinitControl: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub doInit: unsafe extern "thiscall" fn(this: *mut gfc__Hud),
    pub drawInternal: unsafe extern "thiscall" fn(this: *mut gfc__Hud, _: *mut gfc__UIRenderer),
    pub getAnchorPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: *mut gfc___UIControl,
        _: u8,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getGlobalScale: unsafe extern "thiscall" fn(
        this: *const gfc__Hud,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getParentSize: unsafe extern "thiscall" fn(
        this: *const gfc__Hud,
        _: *mut gfc__TVector2_float_gfc__FloatMath_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__HudChallengeCounterData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DraggableActor {
    // gfc__IRefObject
    pub vfptr: *const gfc__DraggableActor__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
    __pdbindgen_padding_2: [u8; 4],
    pub mHitPoints: f32,
    pub mMoveDir: f32,
    pub mSpeed: f32,
    pub mStrength: f32,
    pub mLastWaypointPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mLastWaypointUp: gfc__TVector3_float_gfc__FloatMath_,
    pub mVariableBlendValue: f32,
    pub mTimeUntilDecayOverride: f32,
    pub mDecayAnimIdOverride: i32,
    pub mMobID: i32,
    pub mFootstepMaterialOverlay: i32,
    pub mDeathInteractiveID: i32,
    pub mCharacterFlags: gfc__TFlags_unsigned_long_,
    pub mCharacterProxy: gfc__AutoRef_gfc__CharacterProxy_,
    pub mCurrentMoveState: gfc__AutoRef_gfc__CharacterMoveState_,
    pub mInteractiveContextNode: *mut gfc__Node3D,
    pub mCenterPositionNode: *mut gfc__Node3D,
    pub mCenterOffset: gfc__TVector3_float_gfc__FloatMath_,
    pub mInRangeRegion: *mut gfc__PhysicsDetectRegion,
    pub mAttributes: [i32; 64],
    pub mInventory: gfc__AutoRef_gfc__Inventory_,
    pub mVisuals: gfc__Vector_gfc__Character__CVisual_0_gfc__CAllocator_,
    pub mLookAtSolver: gfc__AutoRef_gfc__CharacterLookAtSolver_,
    pub mCurrentBody: gfc__AutoRef_gfc__Body_,
    pub mLastRefPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mFlinchCounter: f32,
    pub mFlinchChannel: i32,
    pub mEnabledInteractive: i32,
    pub mAutoJumpSolvers: gfc__Vector_gfc__AutoRef_gfc__AutoJumpSolver__0_gfc__CAllocator_,
    pub mAutoJumpCooldown: f32,
    pub mIKSolver: gfc__AutoRef_gfc__IKSolver_,
    pub mPActors: gfc__Vector_gfc__Character__PActor_0_gfc__CAllocator_,
    pub mPersonalTimeStamp: f32,
    pub mDetectors: gfc__Vector_gfc__AutoRef_gfc__DetectorObject__0_gfc__CAllocator_,
    pub mHitLog: gfc__Vector_gfc__HitRecord_0_gfc__CAllocator_,
    pub mDamageThresholds: gfc__Vector_gfc__DamageThresholdCallback_0_gfc__CAllocator_,
    pub mNextThresholdID: u32,
    pub mCurrentEffectID: i32,
    pub mLastSurfaceType: i32,
    pub mGravity: f32,
    pub mPreviousGravity: f32,
    pub mPlatformVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mExternalVelocity: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentWaypoint: *mut gfc__TraversalWaypoint,
    pub mNearestWaypoint: gfc__Vector_gfc__TraversalWaypoint___0_gfc__CAllocator_,
    pub mNearestWaypointExpire: gfc__Vector_float_0_gfc__CAllocator_,
    pub mNearestWaypointTTL: f32,
    pub mRagdollMap: gfc__AutoRef_gfc__RagdollBoneMapping_,
    pub mRagdollParentNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollRoot: gfc__AutoRef_gfc__Node3D_,
    pub mVictimNode: gfc__AutoRef_gfc__Node3D_,
    pub mRagdollMapper: gfc__AutoRef_gfc__RagdollMapper_,
    pub mRagdoll_2: gfc__AutoRef_gfc__Object3D_,
    pub mOldMotionType: i32,
    pub mCharacterClass: gfc__AutoRef_gfc__CharacterClass_,
    pub mBodyRelativeVersion: i32,
    __pdbindgen_padding_3: [u8; 4],
    pub mOrientation: gfc__Matrix4,
    pub mOrientationInv: gfc__Matrix4,
    pub mLastBodyMatrix: gfc__Matrix4,
    pub mBodyRelativeMatrix: gfc__Matrix4,
    pub mOldGrabMat: gfc__Matrix4,
    pub mCombatState: i32,
    pub mCombatPosition: i32,
    pub mAttackState: i32,
    pub mAttackRadius: f32,
    pub mAttackHeight: f32,
    pub mDissolveElapsedTime: f32,
    pub mDissolveDuration: f32,
    pub mBrightness: f32,
    pub mDissolveIn: bool,
    pub mDissolveActive: bool,
    pub mTexturePackage: gfc__HString,
    pub mDissolveTexture: gfc__HString,
    pub mDissolvePatternTexture: gfc__HString,
    // gfc__DraggableActor
    pub mShoveMultiplier: f32,
    pub mGhosthookShoveMultiplier: f32,
    pub mCharacterDetector: gfc__AutoRef_gfc__CollisionPhantom_,
    pub mDraggableFlags: gfc__TFlags_unsigned_long_,
    pub mFXState: i32,
    pub mPortalEjectTimer: f32,
    pub mDraggingCharacter: gfc__AutoRef_gfc__Character_,
    pub mPickupNode: gfc__AutoRef_gfc__Node3D_,
    pub mOldRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mOldPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mGlobalDragInfo: gfc__AutoRef_gfc__DraggableActorInfo_,
    pub mGrabNodes: gfc__Vector_gfc__Node3D___0_gfc__CAllocator_,
    pub mVfxNodes: gfc__Vector_gfc__Node3D___0_gfc__CAllocator_,
    __pdbindgen_padding_4: [u8; 12],
}

unsafe impl UpcastToNop<gfc__Character> for gfc__DraggableActor {}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__DraggableActor {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__DraggableActor {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__DraggableActor {}

unsafe impl UpcastToNop<gfc__Object> for gfc__DraggableActor {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__DraggableActor {}

impl gfc__DraggableActor {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupMovement(
        &self,
        a1: f32,
        a2: *mut gfc__CharMoveVars,
    ) -> bool {
        ((*self.vfptr).setupMovement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyMovement(&self, a1: f32) {
        ((*self.vfptr).applyMovement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateAttributes(&self) {
        ((*self.vfptr).invalidateAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isAttributeValid(&self, a1: i32) -> bool {
        ((*self.vfptr).isAttributeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAttribute(&self, a1: i32) -> *mut i32 {
        ((*self.vfptr).getAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptAttribute(&self, a1: i32) -> i32 {
        ((*self.vfptr).getScriptAttribute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateAttributes(&self, a1: f32) {
        ((*self.vfptr).updateAttributes)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKillingBlow(&self) {
        ((*self.vfptr).doKillingBlow)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isFlying(&self) -> bool {
        ((*self.vfptr).isFlying)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn validateInteractive(&self, a1: u32) -> bool {
        ((*self.vfptr).validateInteractive)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doInteractive(
        &self,
        a1: *mut gfc__CharacterDoInteractiveDesc,
        a2: gfc__AutoRef_gfc__Character_,
        a3: bool,
    ) {
        ((*self.vfptr).doInteractive)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onInterrupt(&self) {
        ((*self.vfptr).onInterrupt)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn grab(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).grab)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn throww(
        &self,
        a1: *mut gfc__Character,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).throww)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drop(&self, a1: *mut gfc__Character) {
        ((*self.vfptr).drop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGrabNode(
        &self,
        result: *mut gfc__HString,
        a2: *mut gfc__Character,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getGrabNode)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn onGrabbableWeaponized(
        &self,
        a1: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).onGrabbableWeaponized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pickupDraggable(&self, a1: *mut gfc__DraggableActor) {
        ((*self.vfptr).pickupDraggable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doMounted(&self, a1: *mut gfc__Character, a2: i32) {
        ((*self.vfptr).doMounted)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn findBestTargetInRange(&self, a1: f32) -> *mut gfc__Character {
        ((*self.vfptr).findBestTargetInRange)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findBestTarget(&self) -> *mut gfc__Character {
        ((*self.vfptr).findBestTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn distanceTo(&self, a1: *mut gfc__Character) -> f32 {
        ((*self.vfptr).distanceTo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn findGrabbable(&self) -> *mut gfc__Actor {
        ((*self.vfptr).findGrabbable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRotationOnly(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotationOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHeadingOnly(&self, a1: f32) {
        ((*self.vfptr).setHeadingOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCenterOffset(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setBody(&self) {
        ((*self.vfptr).setBody)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupCharacterProxy(&self) {
        ((*self.vfptr).setupCharacterProxy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActualVelocity(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getActualVelocity)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn updateLastGroundMaterial(&self) {
        ((*self.vfptr).updateLastGroundMaterial)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doHitPause(&self, a1: f32) {
        ((*self.vfptr).doHitPause)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DraggableActor__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__DraggableActor,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub setScale_2:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__DraggableActor,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__DraggableActor) -> bool,
    pub enteredDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub exitedDetectorObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: gfc__AutoRef_gfc__DetectorObject_,
    ),
    pub queryStrike:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__HitInfo),
    pub queryHit:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__HitInfo),
    pub recordHit:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub setupMovement: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: f32,
        _: *mut gfc__CharMoveVars,
    ) -> bool,
    pub applyMovement: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub invalidateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub isAttributeValid:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: i32) -> bool,
    pub getAttribute:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: i32) -> *mut i32,
    pub getScriptAttribute:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: i32) -> i32,
    pub updateAttributes: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub doKillingBlow: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub isFlying: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> bool,
    pub validateInteractive:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: u32) -> bool,
    pub doInteractive: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__CharacterDoInteractiveDesc,
        _: gfc__AutoRef_gfc__Character_,
        _: bool,
    ),
    pub onInterrupt: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub grab: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Character),
    pub throww: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__Character,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drop: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Character),
    pub getGrabNode: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__HString,
        _: *mut gfc__Character,
    ) -> *mut gfc__HString,
    pub onGrabbableWeaponized: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *mut gfc__Vector_gfc__WorldObject___0_gfc__CAllocator_,
    ),
    pub pickupDraggable:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__DraggableActor),
    pub doMounted:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Character, _: i32),
    pub findBestTargetInRange:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32) -> *mut gfc__Character,
    pub findBestTarget:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut gfc__Character,
    pub distanceTo:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: *mut gfc__Character) -> f32,
    pub findGrabbable:
        unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor) -> *mut gfc__Actor,
    pub setRotationOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHeadingOnly: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
    pub getCenterOffset: unsafe extern "thiscall" fn(
        this: *const gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setBody: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__DraggableActor,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__DraggableActor, _: f32),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ItemActor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__UICategories_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__HudTimerData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WeaponEnhancement_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__UICategoryPackages_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__DSSaveGameManager__NotifyDelegate_0_gfc__CAllocator_ {
    pub mData: *mut gfc__DSSaveGameManager__NotifyDelegate,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ISaveGameDelegate_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__WeaponEnhancement__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__WeaponEnhancement_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__DSUIManager {
    // gfc___UIManager
    pub vfptr: *const gfc__DSUIManager__vftable,
    __pdbindgen_padding: [u8; 12],
    pub mUIRenderer: gfc__UIRenderer,
    pub mDimensions: gfc__TVector2_int_gfc__FloatMath_,
    pub mHideUI: bool,
    pub mHideUIFrameCount: i32,
    pub mMousePos: gfc__TVector2_int_gfc__FloatMath_,
    pub mLastMouseMove: gfc__AutoRef_gfc___UIControl_,
    pub mLastMouseClicked: gfc__AutoRef_gfc___UIControl_,
    pub mFocusedControl: gfc__AutoRef_gfc___UIControl_,
    pub mCaptureControl: gfc__AutoRef_gfc___UIControl_,
    pub mControls: List_gfc__AutoRef_gfc___UIControl___,
    pub mSystemControl: gfc__AutoRef_gfc___UIControl_,
    pub mLastMouseBtnClicked: [i32; 5],
    pub mRemoveList: List_gfc__AutoRef_gfc___UIControl___,
    pub mAddList: List_gfc__AutoRef_gfc___UIControl___,
    pub mFonts: gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___,
    pub mGlobalControls: gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___,
    pub mUpdateList: List_gfc__AutoRef_gfc___UIControl___,
    pub mUpdateRemovalList: List_gfc__AutoRef_gfc___UIControl___,
    pub mLastDeltaT: f32,
    pub mScriptEnvironment: gfc__AutoRef_gfc__Environment_,
    pub mTimer: gfc__AutoRef_gfc__UITimer_,
    pub mGlobalScale: gfc__TVector2_float_gfc__FloatMath_,
    pub mCrosshairScale: gfc__TVector2_float_gfc__FloatMath_,
    pub mIsIterating: bool,
    pub mResized: bool,
    // gfc__DSUIManager
    __pdbindgen_padding_2: [u8; 6],
    pub mSounds: gfc__AutoRef_gfc__SoundList_,
    pub mHUD: *mut gfc__Hud,
    pub mHiddenCount: i32,
    pub mCurrentCategory: gfc__AutoRef_gfc__UICategoryPackages_,
    pub mUICategories: gfc__AutoRef_gfc__UICategories_,
    pub mTimerData: gfc__AutoRef_gfc__HudTimerData_,
    pub mCounterData: gfc__AutoRef_gfc__HudChallengeCounterData_,
    pub mLoadMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mCategoryListeners: List_gfc__AutoRef_gfc__Object___,
    pub mHideMinimap: bool,
}

unsafe impl UpcastToNop<gfc___UIManager> for gfc__DSUIManager {}

impl gfc__DSUIManager {
    pub unsafe extern "thiscall" fn init(&self) {
        ((*self.vfptr).init)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdown(&self) {
        ((*self.vfptr).shutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoss(&self) {
        ((*self.vfptr).onLoss)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onRecovery(&self) {
        ((*self.vfptr).onRecovery)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addControl(&self, a1: *mut gfc___UIControl, a2: bool) {
        ((*self.vfptr).addControl)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn removeControl(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).removeControl)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound(&self, a1: *const gfc__String) {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHideUI(&self, a1: bool) {
        ((*self.vfptr).setHideUI)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isUIHidden(&self) -> bool {
        ((*self.vfptr).isUIHidden)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn drawControlsInternal(&self, a1: *mut gfc__UIRenderer) {
        ((*self.vfptr).drawControlsInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateInternal(&self, a1: f32) {
        ((*self.vfptr).updateInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn initSounds(&self, a1: *const gfc__HString) {
        ((*self.vfptr).initSounds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn dispose(&self) {
        ((*self.vfptr).dispose)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn changeUICategory(&self, a1: *const gfc__HString) -> bool {
        ((*self.vfptr).changeUICategory)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DSUIManager__vftable {
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub onLoss: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub onRecovery: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub addControl:
        unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *mut gfc___UIControl, _: bool),
    pub removeControl:
        unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *mut gfc___UIControl),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: f32),
    pub playSound: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *const gfc__String),
    pub setHideUI: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: bool),
    pub isUIHidden: unsafe extern "thiscall" fn(this: *const gfc__DSUIManager) -> bool,
    pub drawControlsInternal:
        unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *mut gfc__UIRenderer),
    pub updateInternal: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: f32),
    pub initSounds:
        unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *const gfc__HString),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub dispose: unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager),
    pub changeUICategory:
        unsafe extern "thiscall" fn(this: *mut gfc__DSUIManager, _: *const gfc__HString) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__DSSaveGameManager__NotifySection_0_gfc__CAllocator_ {
    pub mData: *mut gfc__DSSaveGameManager__NotifySection,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Weapon {
    // gfc__IRefObject
    pub vfptr: *const gfc__Weapon__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Item
    __pdbindgen_padding_2: [u8; 4],
    pub mValid: bool,
    pub mAutoPersist: bool,
    pub mInfoID: u32,
    pub mContainer: *mut gfc__Container,
    pub mParticleSystem: i32,
    // gfc__EquippableItem
    pub mEquipped: bool,
    pub mDirectionSlot: i32,
    pub mMustReequip: bool,
    pub mHotKey: i32,
    pub mVisualAttached: bool,
    pub mMountPoint: i32,
    pub mQueuedMountPoint: i32,
    pub mLoadState: i32,
    pub mLoadMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mMountNode: *mut gfc__Node3D,
    pub mQueuePreload: bool,
    pub mQueueLoadVisuals: bool,
    pub mQueueShow: bool,
    pub mWasVisualAttached: bool,
    pub mIsVisible: bool,
    pub mPreviousMountPoint: i32,
    // gfc__Weapon
    __pdbindgen_padding_3: [u8; 4],
    pub mExperience: f32,
    pub mLevel: i32,
    pub mAllowEnhancements: bool,
    pub mEnhancements: gfc__Vector_gfc__AutoRef_gfc__WeaponEnhancement__0_gfc__CAllocator_,
    pub mNextLevelObjectName: gfc__String,
    pub mCurrentVisualPermID: i32,
    pub mRunTimeID: i32,
    pub mQueueOnActivate: bool,
    pub mLoadingVisualState: gfc__Weapon__LoadingState,
    pub mVisualLoadMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mSupressPopup: bool,
}

unsafe impl UpcastToNop<gfc__EquippableItem> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__Item> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Weapon {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Weapon {}

impl gfc__Weapon {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPickup(&self) {
        ((*self.vfptr).onPickup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDrop(&self) {
        ((*self.vfptr).onDrop)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onChanged(&self) {
        ((*self.vfptr).onChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Container) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadVisuals(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).loadVisuals)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unloadVisuals(&self) {
        ((*self.vfptr).unloadVisuals)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoadSaveData(&self) {
        ((*self.vfptr).onLoadSaveData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invokeOnChanged(&self) {
        ((*self.vfptr).invokeOnChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDescription(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getDescription)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn doKilled(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKilled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn load(&self) {
        ((*self.vfptr).load)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unload(&self) {
        ((*self.vfptr).unload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isLoaded(&self) -> bool {
        ((*self.vfptr).isLoaded)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isEquipped(&self) -> bool {
        ((*self.vfptr).isEquipped)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attachRef(&self) {
        ((*self.vfptr).attachRef)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn detachRef(&self) {
        ((*self.vfptr).detachRef)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAimOffset(
        &self,
        a1: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getAimOffset)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHotKey(&self, a1: i32) {
        ((*self.vfptr).setHotKey)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn equipExclusive(&self) {
        ((*self.vfptr).equipExclusive)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn equip(&self) {
        ((*self.vfptr).equip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unequip(&self) {
        ((*self.vfptr).unequip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn show(&self) {
        ((*self.vfptr).show)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn hide(&self) {
        ((*self.vfptr).hide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoaded(&self) {
        ((*self.vfptr).onLoaded)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isBusy(&self) -> bool {
        ((*self.vfptr).isBusy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn render_2(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *const gfc__Matrix4,
    ) {
        ((*self.vfptr).render_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn isReady(&self) -> bool {
        ((*self.vfptr).isReady)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn autoEquip(&self) {
        ((*self.vfptr).autoEquip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn equipAfterLoad(&self) {
        ((*self.vfptr).equipAfterLoad)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn canEquipEnhancement(
        &self,
        a1: i32,
        a2: gfc__AutoRef_gfc__WeaponEnhancement_,
    ) -> bool {
        ((*self.vfptr).canEquipEnhancement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn equipEnhancement(
        &self,
        a1: i32,
        a2: gfc__AutoRef_gfc__WeaponEnhancement_,
    ) {
        ((*self.vfptr).equipEnhancement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn unequipEnhancement(
        &self,
        a1: gfc__AutoRef_gfc__WeaponEnhancement_,
    ) {
        ((*self.vfptr).unequipEnhancement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEnhancement(
        &self,
        result: *mut gfc__AutoRef_gfc__WeaponEnhancement_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__WeaponEnhancement_ {
        ((*self.vfptr).getEnhancement)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn beginAttack(&self, a1: *const gfc__MoveInput) {
        ((*self.vfptr).beginAttack)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn endAttack(&self) {
        ((*self.vfptr).endAttack)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onTriggerFrame(&self) {
        ((*self.vfptr).onTriggerFrame)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onShootTriggerFrame(&self) {
        ((*self.vfptr).onShootTriggerFrame)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onMoveStateChanged(&self) {
        ((*self.vfptr).onMoveStateChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isAttacking(&self) -> bool {
        ((*self.vfptr).isAttacking)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn leaveTargetCamera(&self) {
        ((*self.vfptr).leaveTargetCamera)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Weapon__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Weapon,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Weapon,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> bool,
    pub enteredDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: gfc__AutoRef_gfc__DetectorObject_),
    pub exitedDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: gfc__AutoRef_gfc__DetectorObject_),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo),
    pub queryHit: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__WorldObject,
        _: f32,
    ) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: f32),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onPickup: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onDrop: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onChanged: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__Container),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub loadVisuals: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__Actor),
    pub unloadVisuals: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onLoadSaveData: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub invokeOnChanged: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getDescription: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub doKilled: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *mut gfc__HitInfo),
    pub load: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub isLoaded: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub isEquipped: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub attachRef: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub detachRef: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub getAimOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub setHotKey: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: i32),
    pub equipExclusive: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub equip: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub unequip: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub show: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub hide: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onLoaded: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub isBusy: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub render_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: *mut gfc__UIRenderer,
        _: *const gfc__Matrix4,
    ),
    pub isReady: unsafe extern "thiscall" fn(this: *mut gfc__Weapon) -> bool,
    pub autoEquip: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub equipAfterLoad: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub canEquipEnhancement: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: i32,
        _: gfc__AutoRef_gfc__WeaponEnhancement_,
    ) -> bool,
    pub equipEnhancement: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: i32,
        _: gfc__AutoRef_gfc__WeaponEnhancement_,
    ),
    pub unequipEnhancement: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        _: gfc__AutoRef_gfc__WeaponEnhancement_,
    ),
    pub getEnhancement: unsafe extern "thiscall" fn(
        this: *mut gfc__Weapon,
        result: *mut gfc__AutoRef_gfc__WeaponEnhancement_,
        _: i32,
    )
        -> *mut gfc__AutoRef_gfc__WeaponEnhancement_,
    pub beginAttack: unsafe extern "thiscall" fn(this: *mut gfc__Weapon, _: *const gfc__MoveInput),
    pub endAttack: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onTriggerFrame: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onShootTriggerFrame: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub onMoveStateChanged: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
    pub isAttacking: unsafe extern "thiscall" fn(this: *const gfc__Weapon) -> bool,
    pub leaveTargetCamera: unsafe extern "thiscall" fn(this: *mut gfc__Weapon),
}

#[repr(C)]
pub struct gfc__Item {
    // gfc__IRefObject
    pub vfptr: *const gfc__Item__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldObject
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    pub mEventGroupID: i32,
    pub mWorld: *mut gfc__World,
    pub mGroup: *mut gfc__WorldObject,
    pub mLightGroup: u32,
    pub mEventHandlers: *mut gfc__WorldObject__EventHandlerNode,
    pub mEventHandlerLocks: i32,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    // gfc__Actor
    pub mFactionOverrideID: i32,
    pub mMaskOfShadowsState: i32,
    pub mInActiveMaskRealm: bool,
    pub mRepulsedThisFrame: bool,
    pub mPortalsEnabled: bool,
    pub mInsidePortal: bool,
    pub mLastHit: gfc__AutoRef_gfc__HitInfo_,
    pub mLastHitInstigatorPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCurrentVisualID: i32,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mRagdoll: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mHeading: f32,
    pub mActorDesc: gfc__AutoRef_gfc__ActorDesc_,
    pub mLastTouchList: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mSoundChannels: gfc__Vector_gfc__AutoRef_gfc__ChannelAccessor__0_gfc__CAllocator_,
    pub mUpdatePhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mWeaponPhantoms: gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_,
    pub mAttached3DObjects: gfc__Vector_gfc__AutoRef_gfc__Object3D__0_gfc__CAllocator_,
    pub mLiquidRegions: gfc__Vector_gfc__AutoRef_gfc__LiquidRegion__0_gfc__CAllocator_,
    pub mFocusNode: *mut gfc__Node3D,
    pub mSaveData: gfc__AutoRef_gfc__WorldObjectData_,
    pub mPreloaded: bool,
    pub mIsEthereal: bool,
    pub mIsOutOfPhase: bool,
    pub mChildActorsDeprecated: gfc__Vector_gfc__AutoRef_gfc__Actor__0_gfc__CAllocator_,
    pub mPreviousFactionID: i32,
    // gfc__KinematicActor
    __pdbindgen_padding: [u8; 4],
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Item
    __pdbindgen_padding_2: [u8; 4],
    pub mValid: bool,
    pub mAutoPersist: bool,
    pub mInfoID: u32,
    pub mContainer: *mut gfc__Container,
    pub mParticleSystem: i32,
}

unsafe impl UpcastToNop<gfc__KinematicActor> for gfc__Item {}

unsafe impl UpcastToNop<gfc__Actor> for gfc__Item {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Item {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Item {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Item {}

impl gfc__Item {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setState(&self, a1: *const gfc__HString) {
        ((*self.vfptr).setState)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScriptData(&self) -> *const () {
        ((*self.vfptr).getScriptData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptData_2(&self) -> *mut () {
        ((*self.vfptr).getScriptData_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getScriptState(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getScriptState)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getScriptEnvironment(&self) -> *mut gfc__Environment {
        ((*self.vfptr).getScriptEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cloneObject(
        &self,
        a1: *mut gfc__ObjectCloner,
        a2: gfc__AutoRef_gfc__Object_,
    ) {
        ((*self.vfptr).cloneObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
        a1: *mut gfc__TBox_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doAddToWorld(&self) {
        ((*self.vfptr).doAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doRemoveFromWorld(&self) {
        ((*self.vfptr).doRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn placedInEditor(&self) {
        ((*self.vfptr).placedInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn droppedToGroundInEditor(&self) {
        ((*self.vfptr).droppedToGroundInEditor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self) {
        ((*self.vfptr).preload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin(&self) {
        ((*self.vfptr).begin)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updatePost(&self, a1: f32) {
        ((*self.vfptr).updatePost)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Renderer,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawDebug(&self) {
        ((*self.vfptr).drawDebug)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPackageID(&self) -> i32 {
        ((*self.vfptr).getPackageID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn playSound(
        &self,
        a1: *mut gfc__SoundDesc,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSound_2(
        &self,
        a1: i32,
        a2: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopSound(&self, a1: i32) {
        ((*self.vfptr).stopSound)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFreeze(&self, a1: bool) {
        ((*self.vfptr).setFreeze)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLocked(&self, a1: bool) {
        ((*self.vfptr).setLocked)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSelected(&self, a1: bool) {
        ((*self.vfptr).setSelected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisabled(&self, a1: bool) {
        ((*self.vfptr).setDisabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDisplayNames(&self, a1: bool) {
        ((*self.vfptr).setDisplayNames)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setError(&self, a1: bool) {
        ((*self.vfptr).setError)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSettled(&self, a1: bool) {
        ((*self.vfptr).setSettled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isGroup(&self) -> bool {
        ((*self.vfptr).isGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeObject(&self, a1: gfc__AutoRef_gfc__WorldObject_) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn inGroup(&self) -> bool {
        ((*self.vfptr).inGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isRoot(&self) -> bool {
        ((*self.vfptr).isRoot)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromGroup(&self) {
        ((*self.vfptr).removeFromGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self) -> *mut gfc__WorldObject {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getObject(&self) -> *mut gfc__Object3D {
        ((*self.vfptr).getObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObject(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).setObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnimation(
        &self,
        result: *mut gfc__AutoRef_gfc__Animation_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_ {
        ((*self.vfptr).getAnimation)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn addObjectToWorld(&self, a1: *mut gfc__World) {
        ((*self.vfptr).addObjectToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addToLayer(&self) {
        ((*self.vfptr).addToLayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn removeFromPathFinding(&self, a1: *mut gfc__TraversalWaypoint) {
        ((*self.vfptr).removeFromPathFinding)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detachFromObject(&self) {
        ((*self.vfptr).detachFromObject)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onAttachedToObject(
        &self,
        a1: *mut gfc__WorldObject,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).onAttachedToObject)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onChildDetachedFromObject(&self, a1: *mut gfc__WorldObject) {
        ((*self.vfptr).onChildDetachedFromObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn overrideHitEffect(&self, a1: f32, a2: *mut gfc__Body) -> bool {
        ((*self.vfptr).overrideHitEffect)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn supportsStaticLighting(&self) -> bool {
        ((*self.vfptr).supportsStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn staticLightingIsDynamic(&self) -> bool {
        ((*self.vfptr).staticLightingIsDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAORayLength(&self) -> i32 {
        ((*self.vfptr).getAORayLength)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initStaticLighting(
        &self,
        a1: *mut gfc__StaticLightingObjectOpt,
    ) -> bool {
        ((*self.vfptr).initStaticLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearStaticLighting(&self) {
        ((*self.vfptr).clearStaticLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getActorType(&self) -> i32 {
        ((*self.vfptr).getActorType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTriggerType(&self) -> u32 {
        ((*self.vfptr).getTriggerType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeading(&self) -> f32 {
        ((*self.vfptr).getHeading)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHeading(&self, a1: f32) {
        ((*self.vfptr).setHeading)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setScale_2(&self, a1: f32, a2: f32, a3: f32) {
        ((*self.vfptr).setScale_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getTopCenterPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getTopCenterPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setEthereal(&self, a1: bool) {
        ((*self.vfptr).setEthereal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getEthereal(&self) -> bool {
        ((*self.vfptr).getEthereal)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setOutOfPhase(&self, a1: bool) {
        ((*self.vfptr).setOutOfPhase)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOutOfPhase(&self) -> bool {
        ((*self.vfptr).getOutOfPhase)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDynamic(&self) -> bool {
        ((*self.vfptr).isDynamic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isDead(&self) -> bool {
        ((*self.vfptr).isDead)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn IsPlayer(&self) -> bool {
        ((*self.vfptr).IsPlayer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn enteredDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).enteredDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn exitedDetectorObject(
        &self,
        a1: gfc__AutoRef_gfc__DetectorObject_,
    ) {
        ((*self.vfptr).exitedDetectorObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryStrike(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).queryStrike)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn queryHit(&self, a1: *mut gfc__HitInfo) -> bool {
        ((*self.vfptr).queryHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn recordHit(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).recordHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn doKill(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKill)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onMoveCollide(&self, a1: *mut gfc__Actor, a2: f32) {
        ((*self.vfptr).onMoveCollide)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onRepulse(&self) {
        ((*self.vfptr).onRepulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doTouch(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).doTouch)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn playSound_3(
        &self,
        a1: i32,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32 {
        ((*self.vfptr).playSound_3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn playSoundEx(&self, a1: *mut gfc__SoundDesc) -> i32 {
        ((*self.vfptr).playSoundEx)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isSoundPlaying(&self, a1: i32) -> bool {
        ((*self.vfptr).isSoundPlaying)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn visualChanged(&self) {
        ((*self.vfptr).visualChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMoveWeight(&self) -> f32 {
        ((*self.vfptr).getMoveWeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onEnterLiquidRegion(&self) {
        ((*self.vfptr).onEnterLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onExitLiquidRegion(&self) {
        ((*self.vfptr).onExitLiquidRegion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCurrentSpline(
        &self,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        a2: *mut f32,
        a3: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_ {
        ((*self.vfptr).getCurrentSpline)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn inArc2D(&self, a1: *mut gfc__WorldObject, a2: f32) -> bool {
        ((*self.vfptr).inArc2D)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateAnimation(&self, a1: f32) {
        ((*self.vfptr).updateAnimation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPickup(&self) {
        ((*self.vfptr).onPickup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDrop(&self) {
        ((*self.vfptr).onDrop)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onChanged(&self) {
        ((*self.vfptr).onChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Container) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeAttributes(&self) {
        ((*self.vfptr).computeAttributes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadVisuals(&self, a1: *mut gfc__Actor) {
        ((*self.vfptr).loadVisuals)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unloadVisuals(&self) {
        ((*self.vfptr).unloadVisuals)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoadSaveData(&self) {
        ((*self.vfptr).onLoadSaveData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn invokeOnChanged(&self) {
        ((*self.vfptr).invokeOnChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDescription(
        &self,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString {
        ((*self.vfptr).getDescription)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn doKilled(&self, a1: *mut gfc__HitInfo) {
        ((*self.vfptr).doKilled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn load(&self) {
        ((*self.vfptr).load)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unload(&self) {
        ((*self.vfptr).unload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isLoaded(&self) -> bool {
        ((*self.vfptr).isLoaded)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isEquipped(&self) -> bool {
        ((*self.vfptr).isEquipped)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attachRef(&self) {
        ((*self.vfptr).attachRef)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn detachRef(&self) {
        ((*self.vfptr).detachRef)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Item__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Item) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Item) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Item) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__TBox_float_gfc__FloatMath_),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Item,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getActorType: unsafe extern "thiscall" fn(this: *const gfc__Item) -> i32,
    pub getTriggerType: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> u32,
    pub getHeading: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> f32,
    pub setHeading: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32),
    pub setScale_2: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32, _: f32, _: f32),
    pub getCenterPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Item,
            result: *mut gfc__TVector3_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setEthereal: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub getEthereal: unsafe extern "thiscall" fn(this: *const gfc__Item) -> bool,
    pub setOutOfPhase: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: bool),
    pub getOutOfPhase: unsafe extern "thiscall" fn(this: *const gfc__Item) -> bool,
    pub isDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub isDead: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub IsPlayer: unsafe extern "thiscall" fn(this: *const gfc__Item) -> bool,
    pub enteredDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: gfc__AutoRef_gfc__DetectorObject_),
    pub exitedDetectorObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: gfc__AutoRef_gfc__DetectorObject_),
    pub queryStrike: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo),
    pub queryHit: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo) -> bool,
    pub doHit: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo),
    pub recordHit: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo),
    pub doKill: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo),
    pub onMoveCollide:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__Actor, _: f32),
    pub onRepulse: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub doTouch: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__Actor),
    pub playSound_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        _: i32,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__AutoRef_gfc__BezierCurve_,
        _: *mut f32,
        _: *mut f32,
    ) -> *mut gfc__AutoRef_gfc__BezierCurve_,
    pub inArc2D:
        unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__WorldObject, _: f32) -> bool,
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: f32),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onPickup: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onDrop: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onChanged: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__Container),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub computeAttributes: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub loadVisuals: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__Actor),
    pub unloadVisuals: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub onLoadSaveData: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub invokeOnChanged: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub getDescription: unsafe extern "thiscall" fn(
        this: *mut gfc__Item,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub doKilled: unsafe extern "thiscall" fn(this: *mut gfc__Item, _: *mut gfc__HitInfo),
    pub load: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub isLoaded: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub isEquipped: unsafe extern "thiscall" fn(this: *mut gfc__Item) -> bool,
    pub attachRef: unsafe extern "thiscall" fn(this: *mut gfc__Item),
    pub detachRef: unsafe extern "thiscall" fn(this: *mut gfc__Item),
}

#[repr(C)]
pub struct gfc__DSSaveGameManager {
    pub mThread: gfc__Thread,
    pub mThreadWake: gfc__Event,
    pub mThreadState: i32,
    pub mLoadSlot: i32,
    pub mLoadDone: gfc__Event,
    pub mSaveSlot: i32,
    pub mDeleteSlot: i32,
    pub mIOBuffer: *mut u8,
    pub mCurIOBufferLen: i32,
    pub mNotifyBuffer: gfc__Vector_gfc__DSSaveGameManager__NotifySection_0_gfc__CAllocator_,
    pub mNotifyBufferLock: gfc__Mutex,
    pub mDelegateList: gfc__Vector_gfc__DSSaveGameManager__NotifyDelegate_0_gfc__CAllocator_,
    pub mDelegateUID: i32,
    pub mCommonStrings: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mInitCalled: bool,
    pub mThreadShutdown: bool,
    pub mSavingEnabled: bool,
}

#[repr(C)]
pub struct gfc__DSSaveGameManager__NotifySection {
    pub mMode: i32,
    pub mThreadStatus: i32,
}

#[repr(C)]
pub struct gfc__DSSaveGameManager__NotifyDelegate {
    pub id: i32,
    pub aDelegate: gfc__AutoRef_gfc__ISaveGameDelegate_,
}

#[repr(C)]
pub struct gfc__UIScriptControl {
    // gfc__IRefObject
    pub vfptr: *const gfc__UIScriptControl__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc___UIControl_
    pub vfptr_2: *const gfc__UIScriptControl__vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
    // gfc___UIControl
    pub mID: i32,
    pub mName: gfc__HString,
    pub mOpacity: f32,
    pub mContext: i32,
    pub mWindowSize: i32,
    pub mVisual: gfc__AutoRef_gfc___UIVisual_,
    pub mClipMask: gfc__HString,
    pub mIgnoreHideUI: bool,
    pub mAnchorPoint: u8,
    pub mAnchorRelativePoint: u8,
    pub mAnchorRelativeCtrl: gfc__HString,
    pub mAnchorRelSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorRelOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub OnEvent: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnAction: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnMouse: gfc__TUIEventBroadcaster_gfc__MouseEvent_,
    pub OnKeyboard: gfc__TUIEventBroadcaster_gfc__KeyboardEvent_,
    pub OnFocus: gfc__TUIEventBroadcaster_gfc__FocusEvent_,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mLastVisibilityState: bool,
    pub mPackageID: i32,
    pub mIndents: gfc__TRect_float_,
    pub mPosition: gfc__TVector2_float_gfc__FloatMath_,
    pub mSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub mRotation: f32,
    pub mScale: gfc__TVector2_float_gfc__FloatMath_,
    pub mLayoutManager: gfc__AutoRef_gfc__UILayoutManager_,
    pub mLayoutHint: f32,
    pub mCurrentActions: gfc__Vector_gfc__AutoRef_gfc___UIAction__0_gfc__CAllocator_,
    pub mClipMaterial: gfc__AutoRef_gfc__Material_,
    pub mOnInitCalledAlready: bool,
    pub mFirstDraw: bool,
    pub mLastDialogResult: gfc__AutoRef_gfc__Value_,
    // gfc__UIScriptControl
}

unsafe impl UpcastToNop<gfc___UIControl> for gfc__UIScriptControl {}

unsafe impl UpcastToNop<gfc__Object> for gfc__UIScriptControl {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__UIScriptControl {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc___UIControl_> for gfc__UIScriptControl {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc___UIControl_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__UIScriptControl {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addFront(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addFront)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBack(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addBack)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn add(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).add)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn remove(&self) {
        ((*self.vfptr).remove)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn remove_2(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).remove_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn added(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).added)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removed(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).removed)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateHierarchy(&self) {
        ((*self.vfptr).invalidateHierarchy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getEnabled(&self) -> bool {
        ((*self.vfptr).getEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setVisible(&self, a1: bool) {
        ((*self.vfptr).setVisible)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVisible(&self) -> bool {
        ((*self.vfptr).getVisible)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFocusTraversable(&self, a1: bool) {
        ((*self.vfptr).setFocusTraversable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFocusTraversable(&self) -> bool {
        ((*self.vfptr).getFocusTraversable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setMouseEnabled(&self, a1: bool) {
        ((*self.vfptr).setMouseEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMouseEnabled(&self) -> bool {
        ((*self.vfptr).getMouseEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setLayoutEnabled(&self, a1: bool) {
        ((*self.vfptr).setLayoutEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutEnabled(&self) -> bool {
        ((*self.vfptr).getLayoutEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setClipChildren(&self, a1: bool) {
        ((*self.vfptr).setClipChildren)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClipChildren(&self) -> bool {
        ((*self.vfptr).getClipChildren)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRegisterControl(&self, a1: bool) {
        ((*self.vfptr).setRegisterControl)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRegisterControl(&self) -> bool {
        ((*self.vfptr).getRegisterControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setText(&self, a1: *const gfc__WString) {
        ((*self.vfptr).setText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSize(&self, a1: *const gfc__TVector2_float_gfc__FloatMath_) {
        ((*self.vfptr).setSize)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSizeValid(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setSizeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPreferredSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPreferredSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getAbsolutePosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAbsolutePosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: f32) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(&self) -> f32 {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutManager(
        &self,
        a1: gfc__AutoRef_gfc__UILayoutManager_,
    ) {
        ((*self.vfptr).setLayoutManager)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutManager(
        &self,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    ) -> *mut gfc__AutoRef_gfc__UILayoutManager_ {
        ((*self.vfptr).getLayoutManager)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutHint(&self, a1: f32) {
        ((*self.vfptr).setLayoutHint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutHint(&self) -> f32 {
        ((*self.vfptr).getLayoutHint)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).addAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).removeAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearAllActions(&self) {
        ((*self.vfptr).clearAllActions)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateActions(&self, a1: f32) {
        ((*self.vfptr).updateActions)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateLayout(&self) {
        ((*self.vfptr).invalidateLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doAnchorLayout(&self) {
        ((*self.vfptr).doAnchorLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doLayout(&self) {
        ((*self.vfptr).doLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAnchorOffset(
        &self,
        a1: gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setAnchorOffset)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorOffset(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn draw(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *mut gfc__TRect_long_,
    ) {
        ((*self.vfptr).draw)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pick(
        &self,
        a1: gfc__TVector2_int_gfc__FloatMath_,
        a2: bool,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).pick)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getControlByID(&self, a1: i32) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getControlByName(
        &self,
        a1: *const gfc__HString,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setControlText(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__WString,
    ) -> bool {
        ((*self.vfptr).setControlText)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlTextA(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__String,
    ) -> bool {
        ((*self.vfptr).setControlTextA)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlVisible(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlVisible)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlEnabled(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlEnabled)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processMouseEvent(&self, a1: *mut gfc__MouseEvent) {
        ((*self.vfptr).processMouseEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processKeyboardEvent(&self, a1: *mut gfc__KeyboardEvent) {
        ((*self.vfptr).processKeyboardEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processFocusEvent(&self, a1: *mut gfc__FocusEvent) {
        ((*self.vfptr).processFocusEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onReInit(&self) {
        ((*self.vfptr).onReInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDeInit(&self) {
        ((*self.vfptr).onDeInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onVisibilityLost(&self) {
        ((*self.vfptr).onVisibilityLost)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDialogResults(&self, a1: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setDialogResults)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLastDialogResult(
        &self,
        a1: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool {
        ((*self.vfptr).getLastDialogResult)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unregisterToolTipEvent(&self) {
        ((*self.vfptr).unregisterToolTipEvent)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getInputListener(
        &self,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_ {
        ((*self.vfptr).getInputListener)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn initControl(&self) {
        ((*self.vfptr).initControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn postInitControl(&self) {
        ((*self.vfptr).postInitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deinitControl(&self) {
        ((*self.vfptr).deinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reinitControl(&self) {
        ((*self.vfptr).reinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doInit(&self) {
        ((*self.vfptr).doInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn drawInternal(&self, a1: *mut gfc__UIRenderer) {
        ((*self.vfptr).drawInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        a2: *mut gfc___UIControl,
        a3: u8,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorPosition)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn getGlobalScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getGlobalScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getParentSize(
        &self,
        a1: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getParentSize)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__UIScriptControl__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: u32) -> *mut (),
    pub addFront:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub addBack:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub remove_2:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub added:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub removed:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc___UIControl),
    pub invalidateHierarchy: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub getEnabled: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setVisible: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getVisible: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: bool),
    pub getRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> bool,
    pub setText:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *const gfc__WString),
    pub setSize: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub setSizeValid: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getSize: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getPreferredSize: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getAbsolutePosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__UIScriptControl,
            result: *mut gfc__TVector2_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: f32),
    pub getRotation: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> f32,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: gfc__AutoRef_gfc__UILayoutManager_,
    ),
    pub getLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    )
        -> *mut gfc__AutoRef_gfc__UILayoutManager_,
    pub setLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: f32),
    pub getLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl) -> f32,
    pub addAction: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: gfc__AutoRef_gfc___UIAction_,
    ),
    pub removeAction: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: gfc__AutoRef_gfc___UIAction_,
    ),
    pub clearAllActions: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub updateActions: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: f32),
    pub invalidateLayout: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub doAnchorLayout: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub doLayout: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub setAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub draw: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *mut gfc__UIRenderer,
        _: *mut gfc__TRect_long_,
    ),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: f32),
    pub pick: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: gfc__TVector2_int_gfc__FloatMath_,
        _: bool,
    ) -> *mut gfc___UIControl,
    pub getControlByID: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: i32,
    ) -> *mut gfc___UIControl,
    pub getControlByName: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__HString,
    ) -> *mut gfc___UIControl,
    pub setControlText: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__HString,
        _: *const gfc__WString,
    ) -> bool,
    pub setControlTextA: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__HString,
        _: *const gfc__String,
    ) -> bool,
    pub setControlVisible: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub setControlEnabled: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub processMouseEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc__MouseEvent),
    pub processKeyboardEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc__KeyboardEvent),
    pub processFocusEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc__FocusEvent),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub onReInit: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub onDeInit: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub onVisibilityLost: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub setDialogResults:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: gfc__AutoRef_gfc__Value_),
    pub getLastDialogResult: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        _: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool,
    pub unregisterToolTipEvent: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub getInputListener: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_,
    pub initControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub postInitControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub deinitControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub reinitControl: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub doInit: unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl),
    pub drawInternal:
        unsafe extern "thiscall" fn(this: *mut gfc__UIScriptControl, _: *mut gfc__UIRenderer),
    pub getAnchorPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: *mut gfc___UIControl,
        _: u8,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getGlobalScale: unsafe extern "thiscall" fn(
        this: *const gfc__UIScriptControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getParentSize: unsafe extern "thiscall" fn(
        this: *const gfc__UIScriptControl,
        _: *mut gfc__TVector2_float_gfc__FloatMath_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__LiquidRegionDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PhysicsShapeGizmo {
    // gfc__SceneObject
    pub vfptr: *const gfc__PhysicsShapeGizmo__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__PhysicsShapeGizmo__vftable,
    pub mLocked: bool,
    // gfc__PhysicsShapeGizmo
    pub mObject: *mut gfc__PhysicsShapeObject,
    pub mMaterial: gfc__AutoRef_gfc__Material_,
}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__PhysicsShapeGizmo {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__PhysicsShapeGizmo {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__PhysicsShapeGizmo {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(
        &self,
        a1: *mut gfc__Camera3D,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn startGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).startGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finishGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).finishGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn prepGeometry(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cullAndSubmit(
        &self,
        a1: *const gfc__Clipper,
        a2: *mut gfc__Camera3D,
        a3: u32,
    ) {
        ((*self.vfptr).cullAndSubmit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submit(&self, a1: *mut gfc__Camera3D, a2: bool, a3: u32) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn submitHidden(&self, a1: *mut gfc__Camera3D, a2: u32) {
        ((*self.vfptr).submitHidden)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getContext(&self) -> *mut gfc__Object {
        ((*self.vfptr).getContext)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn pickObject(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
        a4: bool,
    ) -> bool {
        ((*self.vfptr).pickObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isHighPriority(&self) -> bool {
        ((*self.vfptr).isHighPriority)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn writeText(&self, a1: *mut gfc__AutoRef_gfc__OutputStream_) {
        ((*self.vfptr).writeText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIsSky(&self, a1: bool) {
        ((*self.vfptr).setIsSky)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getIsSky(&self) -> bool {
        ((*self.vfptr).getIsSky)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHide(&self) -> bool {
        ((*self.vfptr).getHide)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFreeze(&self) -> bool {
        ((*self.vfptr).getFreeze)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLocked(&self) -> bool {
        ((*self.vfptr).getLocked)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__PhysicsShapeGizmo__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub getContext:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeGizmo) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeGizmo) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeGizmo,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeGizmo) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeGizmo) -> bool,
}

#[repr(C)]
pub struct hkArrayBase_hkpCollidable___ {
    pub m_data: *mut *mut hkpCollidable,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct List_gfc__KinematicActor__KAnimation___ListNode {
    pub next: *mut List_gfc__KinematicActor__KAnimation___ListNode,
    pub value: gfc__KinematicActor__KAnimation,
}

#[repr(C)]
pub struct hkpPhantomListener {
    pub vfptr: *const hkpPhantomListener__vftable,
}

impl hkpPhantomListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn phantomAddedCallback(&self, a1: *mut hkpPhantom) {
        ((*self.vfptr).phantomAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn phantomRemovedCallback(&self, a1: *mut hkpPhantom) {
        ((*self.vfptr).phantomRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn phantomShapeSetCallback(&self, a1: *mut hkpPhantom) {
        ((*self.vfptr).phantomShapeSetCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn phantomDeletedCallback(&self, a1: *mut hkpPhantom) {
        ((*self.vfptr).phantomDeletedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpPhantomListener__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpPhantomListener, _: u32) -> *mut (),
    pub phantomAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomListener, _: *mut hkpPhantom),
    pub phantomRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomListener, _: *mut hkpPhantom),
    pub phantomShapeSetCallback:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomListener, _: *mut hkpPhantom),
    pub phantomDeletedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomListener, _: *mut hkpPhantom),
}

#[repr(C)]
pub struct gfc__GhostHookEffectManager {
    pub mEffects: gfc__Vector_gfc__AutoRef_gfc__GhostHookEffect__0_gfc__CAllocator_,
    pub mActiveEffect: gfc__AutoRef_gfc__GhostHookEffect_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__GhostHookEffect_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WrathMan {
    pub mWrathLevel: i32,
    pub mWrath: f32,
    pub mChaos: f32,
    pub mPlayer: *mut gfc__Player,
    pub mChaosOutOfCombatTimer: f32,
    pub mChaosLevelIDX: i32,
    pub mChaosLevel: gfc__AutoRef_gfc__ChaosLevel_,
    pub mEffectID: i32,
    pub mWarSwordInUse: bool,
    pub mStartFull: bool,
    pub mWrathStones: i32,
    pub mMaxWrathPerStone: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Player__HairSim_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Container_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__GhostHookEffect__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__GhostHookEffect_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PlayerDoInteractive_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AutoJumpSolver_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__InteractiveTransition_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LoadMapMenu {
    // gfc__IRefObject
    pub vfptr: *const gfc__LoadMapMenu__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc___UIControl_
    pub vfptr_2: *const gfc__LoadMapMenu__vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
    // gfc___UIControl
    pub mID: i32,
    pub mName: gfc__HString,
    pub mOpacity: f32,
    pub mContext: i32,
    pub mWindowSize: i32,
    pub mVisual: gfc__AutoRef_gfc___UIVisual_,
    pub mClipMask: gfc__HString,
    pub mIgnoreHideUI: bool,
    pub mAnchorPoint: u8,
    pub mAnchorRelativePoint: u8,
    pub mAnchorRelativeCtrl: gfc__HString,
    pub mAnchorRelSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorRelOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub OnEvent: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnAction: gfc__TUIEventBroadcaster_gfc___UIEvent_,
    pub OnMouse: gfc__TUIEventBroadcaster_gfc__MouseEvent_,
    pub OnKeyboard: gfc__TUIEventBroadcaster_gfc__KeyboardEvent_,
    pub OnFocus: gfc__TUIEventBroadcaster_gfc__FocusEvent_,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mLastVisibilityState: bool,
    pub mPackageID: i32,
    pub mIndents: gfc__TRect_float_,
    pub mPosition: gfc__TVector2_float_gfc__FloatMath_,
    pub mSize: gfc__TVector2_float_gfc__FloatMath_,
    pub mAnchorOffset: gfc__TVector2_float_gfc__FloatMath_,
    pub mRotation: f32,
    pub mScale: gfc__TVector2_float_gfc__FloatMath_,
    pub mLayoutManager: gfc__AutoRef_gfc__UILayoutManager_,
    pub mLayoutHint: f32,
    pub mCurrentActions: gfc__Vector_gfc__AutoRef_gfc___UIAction__0_gfc__CAllocator_,
    pub mClipMaterial: gfc__AutoRef_gfc__Material_,
    pub mOnInitCalledAlready: bool,
    pub mFirstDraw: bool,
    pub mLastDialogResult: gfc__AutoRef_gfc__Value_,
    // gfc__LoadMapMenu
    __pdbindgen_padding: [u8; 4],
    pub mListItems: gfc__Vector_gfc__String_0_gfc__CAllocator_,
    pub mMaps: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mSelectedMapType: i32,
    pub mSelectedMapIdx: i32,
    pub mLastMapName: gfc__String,
    pub mLastMapRegion: gfc__String,
    pub mHasLastMap: bool,
}

unsafe impl UpcastToNop<gfc___UIControl> for gfc__LoadMapMenu {}

unsafe impl UpcastToNop<gfc__Object> for gfc__LoadMapMenu {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__LoadMapMenu {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc___UIControl_> for gfc__LoadMapMenu {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc___UIControl_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__LoadMapMenu {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addFront(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addFront)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBack(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).addBack)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn add(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).add)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn remove(&self) {
        ((*self.vfptr).remove)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn remove_2(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).remove_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn added(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).added)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removed(&self, a1: *mut gfc___UIControl) {
        ((*self.vfptr).removed)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateHierarchy(&self) {
        ((*self.vfptr).invalidateHierarchy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getEnabled(&self) -> bool {
        ((*self.vfptr).getEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setVisible(&self, a1: bool) {
        ((*self.vfptr).setVisible)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVisible(&self) -> bool {
        ((*self.vfptr).getVisible)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFocusTraversable(&self, a1: bool) {
        ((*self.vfptr).setFocusTraversable)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFocusTraversable(&self) -> bool {
        ((*self.vfptr).getFocusTraversable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setMouseEnabled(&self, a1: bool) {
        ((*self.vfptr).setMouseEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMouseEnabled(&self) -> bool {
        ((*self.vfptr).getMouseEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setLayoutEnabled(&self, a1: bool) {
        ((*self.vfptr).setLayoutEnabled)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutEnabled(&self) -> bool {
        ((*self.vfptr).getLayoutEnabled)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setClipChildren(&self, a1: bool) {
        ((*self.vfptr).setClipChildren)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getClipChildren(&self) -> bool {
        ((*self.vfptr).getClipChildren)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setRegisterControl(&self, a1: bool) {
        ((*self.vfptr).setRegisterControl)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRegisterControl(&self) -> bool {
        ((*self.vfptr).getRegisterControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setText(&self, a1: *const gfc__WString) {
        ((*self.vfptr).setText)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSize(&self, a1: *const gfc__TVector2_float_gfc__FloatMath_) {
        ((*self.vfptr).setSize)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSizeValid(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setSizeValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPreferredSize(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPreferredSize)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setPosition(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getAbsolutePosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAbsolutePosition)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: f32) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRotation(&self) -> f32 {
        ((*self.vfptr).getRotation)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setScale(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setScale)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutManager(
        &self,
        a1: gfc__AutoRef_gfc__UILayoutManager_,
    ) {
        ((*self.vfptr).setLayoutManager)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutManager(
        &self,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    ) -> *mut gfc__AutoRef_gfc__UILayoutManager_ {
        ((*self.vfptr).getLayoutManager)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setLayoutHint(&self, a1: f32) {
        ((*self.vfptr).setLayoutHint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLayoutHint(&self) -> f32 {
        ((*self.vfptr).getLayoutHint)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).addAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeAction(&self, a1: gfc__AutoRef_gfc___UIAction_) {
        ((*self.vfptr).removeAction)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clearAllActions(&self) {
        ((*self.vfptr).clearAllActions)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateActions(&self, a1: f32) {
        ((*self.vfptr).updateActions)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateLayout(&self) {
        ((*self.vfptr).invalidateLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doAnchorLayout(&self) {
        ((*self.vfptr).doAnchorLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doLayout(&self) {
        ((*self.vfptr).doLayout)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAnchorOffset(
        &self,
        a1: gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setAnchorOffset)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorOffset(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorOffset)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn draw(
        &self,
        a1: *mut gfc__UIRenderer,
        a2: *mut gfc__TRect_long_,
    ) {
        ((*self.vfptr).draw)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn update(&self, a1: f32) {
        ((*self.vfptr).update)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pick(
        &self,
        a1: gfc__TVector2_int_gfc__FloatMath_,
        a2: bool,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).pick)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getControlByID(&self, a1: i32) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByID)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getControlByName(
        &self,
        a1: *const gfc__HString,
    ) -> *mut gfc___UIControl {
        ((*self.vfptr).getControlByName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setControlText(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__WString,
    ) -> bool {
        ((*self.vfptr).setControlText)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlTextA(
        &self,
        a1: *const gfc__HString,
        a2: *const gfc__String,
    ) -> bool {
        ((*self.vfptr).setControlTextA)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlVisible(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlVisible)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setControlEnabled(
        &self,
        a1: *const gfc__HString,
        a2: bool,
    ) -> bool {
        ((*self.vfptr).setControlEnabled)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processMouseEvent(&self, a1: *mut gfc__MouseEvent) {
        ((*self.vfptr).processMouseEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processKeyboardEvent(&self, a1: *mut gfc__KeyboardEvent) {
        ((*self.vfptr).processKeyboardEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn processFocusEvent(&self, a1: *mut gfc__FocusEvent) {
        ((*self.vfptr).processFocusEvent)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onInit(&self) {
        ((*self.vfptr).onInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onReInit(&self) {
        ((*self.vfptr).onReInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDeInit(&self) {
        ((*self.vfptr).onDeInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onVisibilityLost(&self) {
        ((*self.vfptr).onVisibilityLost)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDialogResults(&self, a1: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setDialogResults)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getLastDialogResult(
        &self,
        a1: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool {
        ((*self.vfptr).getLastDialogResult)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unregisterToolTipEvent(&self) {
        ((*self.vfptr).unregisterToolTipEvent)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getInputListener(
        &self,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_ {
        ((*self.vfptr).getInputListener)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn initControl(&self) {
        ((*self.vfptr).initControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn postInitControl(&self) {
        ((*self.vfptr).postInitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn deinitControl(&self) {
        ((*self.vfptr).deinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reinitControl(&self) {
        ((*self.vfptr).reinitControl)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn doInit(&self) {
        ((*self.vfptr).doInit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn drawInternal(&self, a1: *mut gfc__UIRenderer) {
        ((*self.vfptr).drawInternal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAnchorPosition(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        a2: *mut gfc___UIControl,
        a3: u8,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getAnchorPosition)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn getGlobalScale(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getGlobalScale)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getParentSize(
        &self,
        a1: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).getParentSize)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__LoadMapMenu__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: u32) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub addBack: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub remove_2: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub added: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub removed: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc___UIControl),
    pub invalidateHierarchy: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub getEnabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setVisible: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getVisible: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getClipChildren: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: bool),
    pub getRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> bool,
    pub setText: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *const gfc__WString),
    pub setSize: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub setSizeValid: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getSize: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getPreferredSize: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getAbsolutePosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc__LoadMapMenu,
            result: *mut gfc__TVector2_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: f32),
    pub getRotation: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> f32,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: gfc__AutoRef_gfc__UILayoutManager_,
    ),
    pub getLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    )
        -> *mut gfc__AutoRef_gfc__UILayoutManager_,
    pub setLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: f32),
    pub getLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu) -> f32,
    pub addAction:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: gfc__AutoRef_gfc___UIAction_),
    pub removeAction:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: gfc__AutoRef_gfc___UIAction_),
    pub clearAllActions: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub updateActions: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: f32),
    pub invalidateLayout: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub doAnchorLayout: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub doLayout: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub setAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub draw: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *mut gfc__UIRenderer,
        _: *mut gfc__TRect_long_,
    ),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: f32),
    pub pick: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: gfc__TVector2_int_gfc__FloatMath_,
        _: bool,
    ) -> *mut gfc___UIControl,
    pub getControlByID:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: i32) -> *mut gfc___UIControl,
    pub getControlByName: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__HString,
    ) -> *mut gfc___UIControl,
    pub setControlText: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__HString,
        _: *const gfc__WString,
    ) -> bool,
    pub setControlTextA: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__HString,
        _: *const gfc__String,
    ) -> bool,
    pub setControlVisible: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub setControlEnabled: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub processMouseEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc__MouseEvent),
    pub processKeyboardEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc__KeyboardEvent),
    pub processFocusEvent:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc__FocusEvent),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub onReInit: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub onDeInit: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub onVisibilityLost: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub setDialogResults:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: gfc__AutoRef_gfc__Value_),
    pub getLastDialogResult: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        _: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool,
    pub unregisterToolTipEvent: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub getInputListener: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_,
    pub initControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub postInitControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub deinitControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub reinitControl: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub doInit: unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu),
    pub drawInternal:
        unsafe extern "thiscall" fn(this: *mut gfc__LoadMapMenu, _: *mut gfc__UIRenderer),
    pub getAnchorPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: *mut gfc___UIControl,
        _: u8,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getGlobalScale: unsafe extern "thiscall" fn(
        this: *const gfc__LoadMapMenu,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getParentSize: unsafe extern "thiscall" fn(
        this: *const gfc__LoadMapMenu,
        _: *mut gfc__TVector2_float_gfc__FloatMath_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VertexDeclaration_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Query_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__KGStaticMesh {
    // gfc__IRefObject
    pub vfptr: *const gfc__KGStaticMesh__vftable,
    pub ReferenceCount: i32,
    // gfc__Mesh
    // gfc__StaticMesh
    // gfc__KGStaticMesh
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_renderGeometries: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 88],
    pub m_pLoadBuffers: [*mut (); 2],
    pub mGraphics: *mut gfc__KGGraphics,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mBoundingBox: gfc__TBox_float_gfc__FloatMath_,
    pub mGroups: *mut gfc__Mesh__Group,
    pub mGroupCount: i32,
}

unsafe impl UpcastToNop<gfc__StaticMesh> for gfc__KGStaticMesh {}

unsafe impl UpcastToNop<gfc__Mesh> for gfc__KGStaticMesh {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__KGStaticMesh {}

impl gfc__KGStaticMesh {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Mesh__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isCompressed(&self) -> bool {
        ((*self.vfptr).isCompressed)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroup(&self, a1: i32) -> *mut gfc__Mesh__Group {
        ((*self.vfptr).getGroup)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGroupCount(&self) -> i32 {
        ((*self.vfptr).getGroupCount)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getGroupIDAt(&self, a1: i32) -> i32 {
        ((*self.vfptr).getGroupIDAt)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getGroupNameAt(&self, a1: i32) -> *const gfc__String {
        ((*self.vfptr).getGroupNameAt)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn beginCreate(&self, a1: *mut gfc__MeshBuilder) {
        ((*self.vfptr).beginCreate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn create(&self, a1: *mut gfc__MeshBuilder) {
        ((*self.vfptr).create)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn endCreate(&self) {
        ((*self.vfptr).endCreate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBoundingBox(
        &self,
    ) -> *const gfc__TBox_float_gfc__FloatMath_ {
        ((*self.vfptr).getBoundingBox)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVertexBuffer(
        &self,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_ {
        ((*self.vfptr).getVertexBuffer)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getIndexBuffer(
        &self,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_ {
        ((*self.vfptr).getIndexBuffer)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn updateAddress(&self, a1: *mut (), a2: *mut ()) {
        ((*self.vfptr).updateAddress)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn isLightmapped(&self) -> bool {
        ((*self.vfptr).isLightmapped)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn testCollision(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
        a3: *mut f32,
    ) -> bool {
        ((*self.vfptr).testCollision)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn testCollision_2(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> bool {
        ((*self.vfptr).testCollision_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn render(&self, a1: *const gfc__RenderNode) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn renderGroup(&self, a1: *const gfc__RenderNode, a2: i32) {
        ((*self.vfptr).renderGroup)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderDepthOnly(&self, a1: *const gfc__RenderNode) {
        ((*self.vfptr).renderDepthOnly)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn renderGroupDepthOnly(
        &self,
        a1: *const gfc__RenderNode,
        a2: i32,
    ) {
        ((*self.vfptr).renderGroupDepthOnly)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn renderLitGroup(
        &self,
        a1: *const gfc__RenderNode,
        a2: i32,
        a3: u32,
        a4: gfc__AutoRef_gfc__VertexBuffer_,
        a5: *mut gfc__Texture,
        a6: *mut gfc__Texture,
        a7: *mut gfc__Texture,
        a8: *mut gfc__Texture,
        a9: f32,
    ) {
        ((*self.vfptr).renderLitGroup)(
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

    pub unsafe extern "thiscall" fn renderLitGroup_2(
        &self,
        a1: *const gfc__RenderNode,
        a2: i32,
        a3: u32,
        a4: *const gfc__AutoRef_gfc__VertexBuffer_,
    ) {
        ((*self.vfptr).renderLitGroup_2)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createRenderCallback(
        &self,
        a1: i32,
    ) -> *mut gfc__IRenderCallback {
        ((*self.vfptr).createRenderCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createRenderCallback_2(&self) -> *mut gfc__IRenderCallback {
        ((*self.vfptr).createRenderCallback_2)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn createLitRenderCallback(
        &self,
        a1: i32,
        a2: u32,
        a3: gfc__AutoRef_gfc__VertexBuffer_,
        a4: *mut gfc__Texture,
        a5: *mut gfc__Texture,
        a6: *mut gfc__Texture,
        a7: *mut gfc__Texture,
        a8: f32,
    ) -> *mut gfc__IRenderCallback {
        ((*self.vfptr).createLitRenderCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
        )
    }

    pub unsafe extern "thiscall" fn createLitRenderCallback_2(
        &self,
        a1: i32,
        a2: u32,
        a3: *const gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__IRenderCallback {
        ((*self.vfptr).createLitRenderCallback_2)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__KGStaticMesh__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh) -> gfc__Mesh__Type,
    pub isCompressed: unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh) -> bool,
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: i32) -> *mut gfc__Mesh__Group,
    pub getGroupCount: unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh) -> i32,
    pub getGroupIDAt: unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh, _: i32) -> i32,
    pub getGroupNameAt:
        unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh, _: i32) -> *const gfc__String,
    pub beginCreate:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: *mut gfc__MeshBuilder),
    pub create: unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: *mut gfc__MeshBuilder),
    pub endCreate: unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *const gfc__KGStaticMesh,
    ) -> *const gfc__TBox_float_gfc__FloatMath_,
    pub getVertexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__KGStaticMesh,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub getIndexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__KGStaticMesh,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub updateAddress:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: *mut (), _: *mut ()),
    pub isLightmapped: unsafe extern "thiscall" fn(this: *const gfc__KGStaticMesh) -> bool,
    pub testCollision: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
    ) -> bool,
    pub testCollision_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> bool,
    pub render:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: *const gfc__RenderNode),
    pub renderGroup: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__RenderNode,
        _: i32,
    ),
    pub renderDepthOnly:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh, _: *const gfc__RenderNode),
    pub renderGroupDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__RenderNode,
        _: i32,
    ),
    pub renderLitGroup: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__RenderNode,
        _: i32,
        _: u32,
        _: gfc__AutoRef_gfc__VertexBuffer_,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: f32,
    ),
    pub renderLitGroup_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: *const gfc__RenderNode,
        _: i32,
        _: u32,
        _: *const gfc__AutoRef_gfc__VertexBuffer_,
    ),
    pub createRenderCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: i32,
    ) -> *mut gfc__IRenderCallback,
    pub createRenderCallback_2:
        unsafe extern "thiscall" fn(this: *mut gfc__KGStaticMesh) -> *mut gfc__IRenderCallback,
    pub createLitRenderCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: i32,
        _: u32,
        _: gfc__AutoRef_gfc__VertexBuffer_,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: f32,
    ) -> *mut gfc__IRenderCallback,
    pub createLitRenderCallback_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGStaticMesh,
        _: i32,
        _: u32,
        _: *const gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__IRenderCallback,
}

#[repr(C)]
pub struct gfc__KGMeshCache {
    // gfc__ResourceCache
    pub vfptr: *const gfc__KGMeshCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__MeshCache
    pub mReloadInfo: gfc__Vector_gfc__MeshCache__ReloadInfo_0_gfc__CAllocator_,
    // gfc__KGMeshCache
    pub mOurExt: i32,
    pub mOurLegacyExt: i32,
}

unsafe impl UpcastToNop<gfc__MeshCache> for gfc__KGMeshCache {}

unsafe impl UpcastToNop<gfc__ResourceCache> for gfc__KGMeshCache {}

impl gfc__KGMeshCache {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadDefaultResource(&self, a1: gfc__AutoRef_gfc__File_) {
        ((*self.vfptr).loadDefaultResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn initThread(&self) {
        ((*self.vfptr).initThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn shutdownThread(&self) {
        ((*self.vfptr).shutdownThread)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn analyzeResource(
        &self,
        a1: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool {
        ((*self.vfptr).analyzeResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canCreateBuffersInThread(&self, a1: i32) -> bool {
        ((*self.vfptr).canCreateBuffersInThread)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createBuffers(&self, a1: *mut gfc__ResourceBufferInfo) {
        ((*self.vfptr).createBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeBuffers(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).freeBuffers)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn loadResource(&self, a1: *mut gfc__ResourceLoadInfo) {
        ((*self.vfptr).loadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn canReloadResources(&self) -> bool {
        ((*self.vfptr).canReloadResources)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadsQueued(&self) -> bool {
        ((*self.vfptr).reloadsQueued)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reloadResource(&self, a1: *mut gfc__ResourceLoadInfo) -> bool {
        ((*self.vfptr).reloadResource)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn needUnlinkResource(&self) -> bool {
        ((*self.vfptr).needUnlinkResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unlinkResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).unlinkResource)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn needUnloadResource(&self) -> bool {
        ((*self.vfptr).needUnloadResource)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unloadResource(
        &self,
        a1: *mut (),
        a2: *mut gfc__ResourceLoadInfo,
    ) {
        ((*self.vfptr).unloadResource)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn freeResource(
        &self,
        a1: *mut (),
        a2: *const gfc__HString,
        a3: *const gfc__HString,
    ) {
        ((*self.vfptr).freeResource)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__KGMeshCache__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__KGMeshCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__KGMeshCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__KGMeshCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__KGMeshCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__KGMeshCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__KGMeshCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__KGMeshCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__KGMeshCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__KGMeshCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__KGMeshCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__KGMeshCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
}

#[repr(C)]
pub struct keen__SteamworksSystem {
    pub m_appId: u32,
}

#[repr(C)]
pub struct IUnknownVtbl {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field QueryInterface")]
#[repr(C)]
pub struct IUnknownVtbl {
    #[cfg(pdb_issue = "unimplemented primitive kind")]
    pub QueryInterface:
        *mut unsafe extern "stdcall" fn(
            _: *mut IUnknown,
            _: *const _GUID,
            _: *mut *mut (),
        ) -> compile_error!("unimplemented primitive kind"),
    pub AddRef: *mut unsafe extern "stdcall" fn(_: *mut IUnknown) -> u32,
    pub Release: *mut unsafe extern "stdcall" fn(_: *mut IUnknown) -> u32,
}

#[repr(C)]
pub struct IUnknown {
    pub lpVtbl: *mut IUnknownVtbl,
}

#[repr(C)]
pub struct hkContactPoint {
    pub m_position: hkVector4f,
    pub m_separatingNormal: hkVector4f,
}

#[repr(C)]
pub struct hkClassMember {
    pub m_name: *const i8,
    pub m_class: *const hkClass,
    pub m_enum: *const hkClassEnum,
    pub m_type: hkEnum_enum_hkClassMember__Type_unsigned_char_,
    pub m_subtype: hkEnum_enum_hkClassMember__Type_unsigned_char_,
    pub m_cArraySize: i16,
    pub m_flags: hkFlags_enum_hkClassMember__FlagValues_unsigned_short_,
    pub m_offset: u16,
    pub m_attributes: *const hkCustomAttributes,
}

#[repr(C)]
pub struct hkClassEnum {
    pub m_name: *const i8,
    pub m_items: *const hkClassEnum__Item,
    pub m_numItems: i32,
    pub m_attributes: *mut hkCustomAttributes,
    pub m_flags: hkFlags_enum_hkClassEnum__FlagValues_unsigned_int_,
}

#[repr(C)]
pub struct hkEnum_enum_hkClassMember__Type_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkClass {
    pub m_name: *const i8,
    pub m_parent: *const hkClass,
    pub m_objectSize: i32,
    pub m_numImplementedInterfaces: i32,
    pub m_declaredEnums: *const hkClassEnum,
    pub m_numDeclaredEnums: i32,
    pub m_declaredMembers: *const hkClassMember,
    pub m_numDeclaredMembers: i32,
    pub m_defaults: *const (),
    pub m_attributes: *const hkCustomAttributes,
    pub m_flags: hkFlags_enum_hkClass__FlagValues_unsigned_int_,
    pub m_describedVersion: i32,
}

#[repr(C)]
pub struct hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_
{
    // hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___
    pub m_elem: *mut hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long_____Pair,
    pub m_numElems: i32,
    pub m_hashMod: i32,
    // hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___>
    for hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___ {
    pub m_elem: *mut hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long_____Pair,
    pub m_numElems: i32,
    pub m_hashMod: i32,
}

#[repr(C)]
pub struct hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long_____Pair {
    pub key: u32,
    pub val: u32,
}

#[repr(C)]
pub struct hkAabb {
    pub m_min: hkVector4f,
    pub m_max: hkVector4f,
}

#[repr(C)]
pub struct hkMultiThreadCheck {
    pub m_threadId: u32,
    pub m_stackTraceId: i32,
    pub m_markCount: u16,
    pub m_markBitStack: u16,
}

#[repr(C)]
pub struct hkStringBuf {
    pub m_string: hkInplaceArray_char_128_hkContainerTempAllocator_,
}

#[repr(C)]
pub struct hkResult {
    pub m_enum: hkResultEnum,
}

#[repr(C)]
pub struct hkMemorySnapshot {
    pub m_mem: *mut hkMemoryAllocator,
    pub m_allocations: hkArrayBase_hkMemorySnapshot__Allocation_,
    pub m_providers: hkArrayBase_hkMemorySnapshot__Provider_,
    pub m_routerWiring: [i32; 5],
    pub m_callTree: hkStackTracer__CallTree,
}

#[repr(C)]
pub struct hkEnum_enum_hkMemorySnapshot__StatusBits_signed_char_ {
    pub m_storage: i8,
}

#[repr(C)]
pub struct hkJob {
    pub m_jobSubType: u8,
    pub m_jobType: hkEnum_enum_hkJobType_unsigned_char_,
    pub m_jobSpuType: hkEnum_enum_hkJobSpuType_unsigned_char_,
    pub m_size: u16,
    pub m_threadAffinity: i16,
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct hkEnum_enum_hkJobType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkEnum_enum_hkJobSpuType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkExternalJobProfiler {
    pub vfptr: *const hkExternalJobProfiler__vftable,
}

impl hkExternalJobProfiler {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onStartJob(&self, a1: hkJobType, a2: u32) {
        ((*self.vfptr).onStartJob)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onEndJob(&self, a1: hkJobType) {
        ((*self.vfptr).onEndJob)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkExternalJobProfiler__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkExternalJobProfiler, _: u32) -> *mut (),
    pub onStartJob:
        unsafe extern "thiscall" fn(this: *mut hkExternalJobProfiler, _: hkJobType, _: u32),
    pub onEndJob: unsafe extern "thiscall" fn(this: *mut hkExternalJobProfiler, _: hkJobType),
}

#[repr(C)]
pub struct hkJobQueueHwSetup {
    pub m_cellRules: hkJobQueueHwSetup__CellRules,
    pub m_noSpu: hkBool,
    pub m_spuSchedulePolicy: hkJobQueueHwSetup__SpuSchedulePolicy,
    pub m_numCpuThreads: i32,
    pub m_threadIdsSharingCaches:
        hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkJobQueue {
    pub m_criticalSection: hkCriticalSection,
    pub m_data: *mut hkJobQueue__DynamicData,
    pub m_popJobFunc: *mut unsafe extern "C" fn(
        _: *mut hkJobQueue,
        _: *mut hkJobQueue__DynamicData,
        _: *mut hkJobQueue__JobQueueEntry,
        _: *mut hkJobQueue__JobQueueEntry,
    ) -> hkJobQueue__JobPopFuncResult,
    pub m_finishJobFunc: *mut unsafe extern "C" fn(
        _: *mut hkJobQueue,
        _: *mut hkJobQueue__DynamicData,
        _: *const hkJobQueue__JobQueueEntry,
        _: *mut hkJobQueue__JobQueueEntryInput,
    ) -> hkJobQueue__JobCreationStatus,
    pub m_numJobTypes: i32,
    pub m_cpuCacheQueuesBegin: i32,
    pub m_cpuCustomQueuesBegin: i32,
    pub m_cpuTypesQueuesBegin: i32,
    pub m_numJobQueues: i32,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_customJobs: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 16],
    pub m_numCustomJobs: i32,
    pub m_cpuSemaphoreBegin: i32,
    pub m_directMapSemaphoreEnd: i32,
    pub m_masterThreadQueue: i32,
    pub m_hwSetup: hkJobQueueHwSetup,
    pub m_queryRulesAreUpdated: hkBool,
    pub m_queueSemaphores: [*mut hkSemaphore; 5],
    pub m_numQueueSemaphores: i32,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m_nextQueueToGet: compile_error!("unimplemented sizeof array"),
    __pdbindgen_padding_2: [u8; 110],
    pub m_cpuThreadIndexToSemaphoreIndex: [i8; 12],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_jobFuncs: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_3: [u8; 242],
    pub m_threadPool: *mut hkSpuThreadPool,
    pub m_customJobSetup: hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_,
    pub m_externalJobProfiler: *mut hkExternalJobProfiler,
}

#[repr(C)]
pub struct hkJobQueue__DynamicData {
    pub m_numActiveJobs: [i16; 15],
    pub m_masterThreadFinishingFlags: i32,
    pub m_waitPolicy: hkJobQueue__WaitPolicy,
    pub m_outOfMemory: hkBool,
    pub m_numThreadsWaiting: [u16; 5],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_jobQueue: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 412],
}
