#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{
    types::*,
    types10::*,
    types11::*,
    types12::*,
    types13::*,
    types2::*,
    types4::*,
    types5::*,
    types6::*,
    types7::*,
    types8::*,
    types9::*,
};

#[repr(C)]
pub struct hkpBroadPhase__hkpCastAabbInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_from")]
#[repr(C)]
pub struct hkpBroadPhase__hkpCastAabbInput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_from: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_to: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_halfExtents: hkVector4f,
    pub m_aabbCacheInfo: *const i8,
}

#[repr(C)]
pub struct hkpCollisionFilter {
    pub __vfptr: *const hkpCollisionFilter____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpCollisionFilter____vftable,
    pub __vfptr_3: *const hkpCollisionFilter____vftable,
    pub __vfptr_4: *const hkpCollisionFilter____vftable,
    pub __vfptr_5: *const hkpCollisionFilter____vftable,
    pub m_prepad: [u32; 2],
    pub m_type: hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_,
    pub m_postpad: [u32; 3],
}

impl hkpCollisionFilter {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }

    pub fn as_hkpCollidableCollidableFilter_ptr(&self) -> *const hkpCollidableCollidableFilter {
        self as *const _ as _
    }

    pub fn as_hkpCollidableCollidableFilter_mut_ptr(
        &mut self,
    ) -> *mut hkpCollidableCollidableFilter {
        self as *mut _ as _
    }

    pub fn as_hkpShapeCollectionFilter_ptr(&self) -> *const hkpShapeCollectionFilter {
        self as *const _ as _
    }

    pub fn as_hkpShapeCollectionFilter_mut_ptr(&mut self) -> *mut hkpShapeCollectionFilter {
        self as *mut _ as _
    }

    pub fn as_hkpRayShapeCollectionFilter_ptr(&self) -> *const hkpRayShapeCollectionFilter {
        self as *const _ as _
    }

    pub fn as_hkpRayShapeCollectionFilter_mut_ptr(&mut self) -> *mut hkpRayShapeCollectionFilter {
        self as *mut _ as _
    }

    pub fn as_hkpRayCollidableFilter_ptr(&self) -> *const hkpRayCollidableFilter {
        self as *const _ as _
    }

    pub fn as_hkpRayCollidableFilter_mut_ptr(&mut self) -> *mut hkpRayCollidableFilter {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollisionFilter____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub init: unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpCdPoint {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_unweldedNormal")]
#[repr(C)]
pub struct hkpCdPoint {
    pub m_contact: hkContactPoint,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_unweldedNormal: hkVector4f,
    pub m_cdBodyA: *const hkpCdBody,
    pub m_cdBodyB: *const hkpCdBody,
}

#[repr(C)]
pub struct hkpSphereRepShape {
    pub __vfptr: *const hkpSphereRepShape____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    pub m_userData: u32,
}

impl hkpSphereRepShape {
    pub fn as_hkpShape_ptr(&self) -> *const hkpShape {
        self as *const _ as _
    }

    pub fn as_hkpShape_mut_ptr(&mut self) -> *mut hkpShape {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSphereRepShape____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> hkVector4fComparison,
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

#[repr(C)]
pub struct hkpConvexShape {
    pub __vfptr: *const hkpConvexShape____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    pub m_userData: u32,
    pub m_radius: f32,
}

impl hkpConvexShape {
    pub fn as_hkpSphereRepShape_ptr(&self) -> *const hkpSphereRepShape {
        self as *const _ as _
    }

    pub fn as_hkpSphereRepShape_mut_ptr(&mut self) -> *mut hkpSphereRepShape {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpConvexShape____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> hkVector4fComparison,
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
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub getFirstVertex:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *mut hkVector4f),
    pub getSize: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> i32,
}

#[repr(C)]
pub struct hkpShapeRayBundleCastInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_from")]
#[repr(C)]
pub struct hkpShapeRayBundleCastInput {
    #[cfg(pdb_issue = "error in hkFourTransposedPointsf")]
    pub m_from: hkFourTransposedPointsf,
    #[cfg(pdb_issue = "error in hkFourTransposedPointsf")]
    pub m_to: hkFourTransposedPointsf,
    pub m_filterInfo: u32,
    pub m_userData: u32,
    pub m_rayShapeCollectionFilter: *const hkpRayShapeCollectionFilter,
}

#[repr(C)]
pub struct hkpAabbCastCollector {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_earlyOutFraction")]
#[repr(C)]
pub struct hkpAabbCastCollector {
    pub __vfptr: *const hkpAabbCastCollector____vftable,
    #[cfg(pdb_issue = "error in hkSimdFloat32")]
    pub m_earlyOutFraction: hkSimdFloat32,
}

#[repr(C)]
pub struct hkpAabbCastCollector____vftable {
    pub addHit: unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32) -> *mut (),
}

#[repr(C)]
pub struct hkpConvexListFilter {
    pub __vfptr: *const hkpConvexListFilter____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkpConvexListFilter {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpConvexListFilter____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpPhantom____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
    pub m_overlapListeners: hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_,
    pub m_phantomListeners: hkArray_hkpPhantomListener___hkContainerHeapAllocator_,
}

impl hkpPhantom {
    pub fn as_hkpWorldObject_ptr(&self) -> *const hkpWorldObject {
        self as *const _ as _
    }

    pub fn as_hkpWorldObject_mut_ptr(&mut self) -> *mut hkpWorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpPhantom____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub getType: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> hkpPhantomType,
    pub calcAabb: unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkAabb),
    pub addOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkpCollidable),
    pub isOverlappingCollidableAdded:
        unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *const hkpCollidable) -> hkBool,
    pub removeOverlappingCollidable:
        unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: *mut hkpCollidable),
    pub ensureDeterministicOrder: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub clone: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> *mut hkpPhantom,
    pub updateShapeCollectionFilter: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
}

#[repr(C)]
pub struct hkpWorld {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_gravity")]
#[repr(C)]
pub struct hkpWorld {
    pub __vfptr: *const hkpWorld____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_simulation: *mut hkpSimulation,
    #[cfg(pdb_issue = "error in hkVector4f")]
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
    pub m_dynamicsStepInfo: hkpWorldDynamicsStepInfo,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_broadPhaseExtents: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_broadPhaseNumMarkers: i32,
    pub m_sizeOfToiEventQueue: i32,
    pub m_broadPhaseQuerySize: i32,
    pub m_broadPhaseUpdateSize: i32,
    pub m_contactPointGeneration: hkEnum_enum_hkpWorldCinfo__ContactPointGeneration_signed_char_,
    pub m_useCompoundSpuElf: hkBool,
}

impl hkpWorld {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpWorld____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpWorldDynamicsStepInfo {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_solverInfo")]
#[repr(C)]
pub struct hkpWorldDynamicsStepInfo {
    pub m_stepInfo: hkStepInfo,
    #[cfg(pdb_issue = "error in hkpSolverInfo")]
    pub m_solverInfo: hkpSolverInfo,
}

#[repr(C)]
pub struct hkEnum_enum_hkpWorldCinfo__SimulationType_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpRigidBody {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_motion")]
#[repr(C)]
pub struct hkpRigidBody {
    pub __vfptr: *const hkpRigidBody____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
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
    #[cfg(pdb_issue = "error in hkpMaxSizeMotion")]
    pub m_motion: hkpMaxSizeMotion,
    pub m_contactListeners: hkSmallArray_hkpContactListener___,
    pub m_actions: hkSmallArray_hkpAction___,
    pub m_localFrame: hkRefPtr_hkLocalFrame_,
    pub m_extendedListeners: *mut hkpEntity__ExtendedListeners,
    pub m_npData: u32,
}

impl hkpRigidBody {
    pub fn as_hkpEntity_ptr(&self) -> *const hkpEntity {
        self as *const _ as _
    }

    pub fn as_hkpEntity_mut_ptr(&mut self) -> *mut hkpEntity {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpRigidBody____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpEntity),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_splitCheckRequested")]
#[repr(C)]
pub struct hkpSimulationIsland {
    pub __vfptr: *const hkpSimulationIsland____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_constraintInfo: hkpConstraintInfo,
    pub m_world: *mut hkpWorld,
    pub m_numConstraints: i32,
    pub m_storageIndex: u16,
    pub m_dirtyListIndex: u16,
    pub m_splitCheckFrameCounter: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_splitCheckRequested: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_isSparse: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_actionListCleanupNeeded: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_allowIslandLocking: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_isInActiveIslandsArray: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_activeMark: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_tryToIncreaseIslandSizeMark: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_inIntegrateJob: compile_error!("unimplemented feature: type kind 0x1205"),
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_timeSinceLastHighFrequencyCheck: f32,
    pub m_timeSinceLastLowFrequencyCheck: f32,
    pub m_actions: hkArray_hkpAction___hkContainerHeapAllocator_,
    pub m_timeOfDeactivation: f32,
    pub m_entities: hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_,
    pub m_midphaseAgentTrack: hkpAgentNnTrack,
    pub m_narrowphaseAgentTrack: hkpAgentNnTrack,
}

impl hkpSimulationIsland {
    pub fn as_hkpConstraintOwner_ptr(&self) -> *const hkpConstraintOwner {
        self as *const _ as _
    }

    pub fn as_hkpConstraintOwner_mut_ptr(&mut self) -> *mut hkpConstraintOwner {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimulationIsland____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub addConstraintToCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintOwner, _: *mut hkpConstraintInstance),
    pub removeConstraintFromCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintOwner, _: *mut hkpConstraintInstance),
    pub addCallbackRequest: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintOwner,
        _: *mut hkpConstraintInstance,
        _: i32,
    ),
    pub checkAccessRw: unsafe extern "thiscall" fn(this: *mut hkpConstraintOwner),
}

#[repr(C)]
pub struct hkpCollisionEvent {
    pub m_source: hkpCollisionEvent__CallbackSource,
    pub m_bodies: [*mut hkpRigidBody; 2],
    pub m_contactMgr: *mut hkpSimpleConstraintContactMgr,
}

#[repr(C)]
pub struct hkpContactPointEvent {
    pub m_source: hkpCollisionEvent__CallbackSource,
    pub m_bodies: [*mut hkpRigidBody; 2],
    pub m_contactMgr: *mut hkpSimpleConstraintContactMgr,
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

impl hkpContactPointEvent {
    pub fn as_hkpCollisionEvent_ptr(&self) -> *const hkpCollisionEvent {
        self as *const _ as _
    }

    pub fn as_hkpCollisionEvent_mut_ptr(&mut self) -> *mut hkpCollisionEvent {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpDynamicsContactMgr {
    pub __vfptr: *const hkpDynamicsContactMgr____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkpContactMgr__Type,
    pub m_world: *mut hkpWorld,
}

impl hkpDynamicsContactMgr {
    pub fn as_hkpContactMgr_ptr(&self) -> *const hkpContactMgr {
        self as *const _ as _
    }

    pub fn as_hkpContactMgr_mut_ptr(&mut self) -> *mut hkpContactMgr {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpDynamicsContactMgr____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl:
        unsafe extern "thiscall" fn(this: *mut hkpContactMgr, _: i32) -> hkResult,
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
        _: *mut hkpToiEvent,
        _: *mut f32,
    ) -> hkBool,
    pub confirmToi: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *mut hkpToiEvent,
        _: f32,
        _: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ),
}

#[repr(C)]
pub struct hkpModifierConstraintAtom {
    pub m_type: hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_,
    pub m_modifierAtomSize: u16,
    pub m_childSize: u16,
    pub m_child: *mut hkpConstraintAtom,
    pub m_pad: [u32; 2],
}

impl hkpModifierConstraintAtom {
    pub fn as_hkpConstraintAtom_ptr(&self) -> *const hkpConstraintAtom {
        self as *const _ as _
    }

    pub fn as_hkpConstraintAtom_mut_ptr(&mut self) -> *mut hkpConstraintAtom {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr {
    pub __vfptr: *const hkpSimpleConstraintContactMgr____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkpContactMgr__Type,
    pub m_world: *mut hkpWorld,
    pub m_reservedContactPoints: u16,
    pub m_contactPointCallbackDelay: u16,
    pub m_contactConstraintData: hkpSimpleContactConstraintData,
    pub m_constraint: hkpConstraintInstance,
    pub m_pad: [u32; 1],
}

impl hkpSimpleConstraintContactMgr {
    pub fn as_hkpDynamicsContactMgr_ptr(&self) -> *const hkpDynamicsContactMgr {
        self as *const _ as _
    }

    pub fn as_hkpDynamicsContactMgr_mut_ptr(&mut self) -> *mut hkpDynamicsContactMgr {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl:
        unsafe extern "thiscall" fn(this: *mut hkpContactMgr, _: i32) -> hkResult,
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
        _: *mut hkpToiEvent,
        _: *mut f32,
    ) -> hkBool,
    pub confirmToi: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *mut hkpToiEvent,
        _: f32,
        _: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ),
    pub __: *const (),
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
pub struct hkEnum_enum_hkpContactPointAddedEvent__Type_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpCollisionAgentConfig {
    pub m_iterativeLinearCastEarlyOutDistance: f32,
    pub m_iterativeLinearCastMaxIterations: i32,
}

#[repr(C)]
pub struct hkpLinearCastCollisionInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_path")]
#[repr(C)]
pub struct hkpLinearCastCollisionInput {
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_path: hkVector4f,
    pub m_maxExtraPenetration: f32,
    pub m_cachedPathLength: f32,
    pub m_config: *mut hkpCollisionAgentConfig,
}

impl hkpLinearCastCollisionInput {
    pub fn as_hkpCollisionInput_ptr(&self) -> *const hkpCollisionInput {
        self as *const _ as _
    }

    pub fn as_hkpCollisionInput_mut_ptr(&mut self) -> *mut hkpCollisionInput {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpShapeRayCastOutput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_normal")]
#[repr(C)]
pub struct hkpShapeRayCastOutput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_normal: hkVector4f,
    pub m_hitFraction: f32,
    pub m_extraInfo: i32,
    pub m_pad: [i32; 2],
    pub m_shapeKeys: [u32; 8],
    pub m_shapeKeyIndex: i32,
}

impl hkpShapeRayCastOutput {
    pub fn as_hkpShapeRayCastCollectorOutput_ptr(&self) -> *const hkpShapeRayCastCollectorOutput {
        self as *const _ as _
    }

    pub fn as_hkpShapeRayCastCollectorOutput_mut_ptr(
        &mut self,
    ) -> *mut hkpShapeRayCastCollectorOutput {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollisionInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_aabb32Info")]
#[repr(C)]
pub struct hkpCollisionInput {
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    #[cfg(pdb_issue = "error in hkpCollisionInput__Aabb32Info")]
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
}

#[repr(C)]
pub struct hkThreadPool {
    pub __vfptr: *const hkThreadPool____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkThreadPool {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkThreadPool____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_contactPoints")]
#[repr(C)]
pub struct hkpProcessCollisionOutput {
    pub m_firstFreeContactPoint: hkPadSpu_hkpProcessCdPoint___,
    pub m_constraintOwner: hkPadSpu_hkpConstraintOwner___,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_contactPoints: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_toi: hkpProcessCollisionData__ToiInfo,
    pub m_potentialContacts: hkPadSpu_hkpProcessCollisionOutput__PotentialInfo___,
}

impl hkpProcessCollisionOutput {
    pub fn as_hkpProcessCollisionData_ptr(&self) -> *const hkpProcessCollisionData {
        self as *const _ as _
    }

    pub fn as_hkpProcessCollisionData_mut_ptr(&mut self) -> *mut hkpProcessCollisionData {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpGskCache {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_maxDimA")]
#[repr(C)]
pub struct hkpGskCache {
    pub m_vertices: [u16; 4],
    pub m_dimA: u8,
    pub m_dimB: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimA: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimB: compile_error!("unimplemented feature: type kind 0x1205"),
    pub m_gskFlags: u8,
}

#[repr(C)]
pub struct hkpWorldRayCastInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_from")]
#[repr(C)]
pub struct hkpWorldRayCastInput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_from: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_to: hkVector4f,
    pub m_enableShapeCollectionFilter: hkBool,
    pub m_filterInfo: u32,
    pub m_userData: u32,
}

#[repr(C)]
pub struct hkpShapeRayCastCollectorOutput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_normal")]
#[repr(C)]
pub struct hkpShapeRayCastCollectorOutput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_normal: hkVector4f,
    pub m_hitFraction: f32,
    pub m_extraInfo: i32,
    pub m_pad: [i32; 2],
}

#[repr(C)]
pub struct hkpCollidable {
    pub m_shape: *const hkpShape,
    pub m_shapeKey: u32,
    pub m_motion: *const (),
    pub m_parent: *const hkpCdBody,
    pub m_ownerOffset: i8,
    pub m_forceCollideOntoPpu: u8,
    pub m_shapeSizeOnSpu: u16,
    pub m_broadPhaseHandle: hkpTypedBroadPhaseHandle,
    pub m_boundingVolumeData: hkpCollidable__BoundingVolumeData,
    pub m_allowedPenetrationDepth: f32,
}

impl hkpCollidable {
    pub fn as_hkpCdBody_ptr(&self) -> *const hkpCdBody {
        self as *const _ as _
    }

    pub fn as_hkpCdBody_mut_ptr(&mut self) -> *mut hkpCdBody {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollisionDispatcher__ShapeInheritance {
    pub m_primaryType: hkcdShapeType__ShapeTypeEnum,
    pub m_alternateType: hkcdShapeType__ShapeTypeEnum,
}

#[repr(C)]
pub struct hkpBreakableMaterial {
    pub __vfptr: *const hkpBreakableMaterial____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_strength: f32,
    pub m_typeAndFlags: i32,
    pub m_properties: *mut hkRefCountedProperties,
}

impl hkpBreakableMaterial {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBreakableMaterial____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpBreakableMaterial__ShapeKeyCollector____vftable,
}

#[repr(C)]
pub struct hkpBreakableMaterial__ShapeKeyCollector____vftable {
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
    pub __vfptr: *const hkpBreakableShape____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_physicsShape: hkRefPtr_hkcdShape_const__,
    pub m_material: hkRefPtr_hkpBreakableMaterial_,
}

impl hkpBreakableShape {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBreakableShape____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkRefPtr_hkcdShape_const__ {
    pub m_pntr: *const hkcdShape,
}

#[repr(C)]
pub struct hkpBreakableBody {
    pub __vfptr: *const hkpBreakableBody____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_controller: hkRefPtr_hkpBreakableBody__Controller_,
    pub m_breakableShape: hkRefPtr_hkpBreakableShape_const__,
    pub m_bodyTypeAndFlags: u8,
    pub m_constraintStrength: hkHalf,
}

impl hkpBreakableBody {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBreakableBody____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub cloneBreakableBody: unsafe extern "thiscall" fn(
        this: *const hkpBreakableBody,
        _: *mut hkpRigidBody,
    ) -> *mut hkpBreakableBody,
}

#[repr(C)]
pub struct hkpBreakableBody__Controller {
    pub __vfptr: *const hkpBreakableBody__Controller____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_breakingImpulse: f32,
}

impl hkpBreakableBody__Controller {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBreakableBody__Controller____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub m_data: *mut hkpBodyOperationEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpBodyOperationEntry__ptr(
        &self,
    ) -> *const hkArrayBase_hkpBodyOperationEntry_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpBodyOperationEntry__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpBodyOperationEntry_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBodyOperation__UpdateInfo {
    pub m_bodyIsDeleted: hkBool,
    pub m_bodyIsInWorld: hkBool,
}

#[repr(C)]
pub struct hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_ {
    pub m_data: *mut hkWorldOperation__BiggestOperation,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkWorldOperation__BiggestOperation__ptr(
        &self,
    ) -> *const hkArrayBase_hkWorldOperation__BiggestOperation_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkWorldOperation__BiggestOperation__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkWorldOperation__BiggestOperation_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkWorldOperation__BiggestOperation {
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
    pub dummy: [u32; 7],
}

impl hkWorldOperation__BiggestOperation {
    pub fn as_hkWorldOperation__BaseOperation_ptr(&self) -> *const hkWorldOperation__BaseOperation {
        self as *const _ as _
    }

    pub fn as_hkWorldOperation__BaseOperation_mut_ptr(
        &mut self,
    ) -> *mut hkWorldOperation__BaseOperation {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkWorldOperation__BaseOperation {
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
}

#[repr(C)]
pub struct hkpBodyOperation {
    pub __vfptr: *const hkpBodyOperation____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkpBodyOperation {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBodyOperation____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub execute: unsafe extern "thiscall" fn(
        this: *mut hkpBodyOperation,
        _: *mut hkpRigidBody,
        _: *mut hkpBodyOperation__UpdateInfo,
    ),
}

#[repr(C)]
pub struct hkpSimulation {
    pub __vfptr: *const hkpSimulation____vftable,
    pub m_memSizeAndRefCount: u32,
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

impl hkpSimulation {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimulation____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpConstraintTrackerData____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkpConstraintTrackerData {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpConstraintTrackerData____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpConstraintListener____vftable,
}

#[repr(C)]
pub struct hkpConstraintListener____vftable {
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
pub struct hkpCollisionInput__Aabb32Info {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_bitOffsetLow")]
#[repr(C)]
pub struct hkpCollisionInput__Aabb32Info {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_bitOffsetLow: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_bitOffsetHigh: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_bitScale: hkVector4f,
}

#[repr(C)]
pub struct hkpCollisionDispatcher {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_contactMgrFactory")]
#[repr(C)]
pub struct hkpCollisionDispatcher {
    pub __vfptr: *const hkpCollisionDispatcher____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_defaultCollisionAgent: *mut unsafe extern "C" fn(
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpCollisionInput,
        _: *mut hkpContactMgr,
    ) -> *mut hkpCollisionAgent,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_contactMgrFactory: compile_error!("unimplemented feature: sizeof array 0x0"),
    pub m_hasAlternateType: [u32; 35],
    pub m_numAgent2Types: i32,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent2Types: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_agent2TypesPred: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_agent2Func: compile_error!("unimplemented feature: class layout 0x0"),
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
}

impl hkpCollisionDispatcher {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollisionDispatcher____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpBroadPhaseBorder {
    pub __vfptr: *const hkpBroadPhaseBorder____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpBroadPhaseBorder____vftable,
    pub __vfptr_3: *const hkpBroadPhaseBorder____vftable,
    pub __vfptr_4: *const hkpBroadPhaseBorder____vftable,
    pub m_world: *mut hkpWorld,
    pub m_phantoms: [*mut hkpPhantom; 6],
    pub m_type: hkpWorldCinfo__BroadPhaseBorderBehaviour,
    pub m_postponeAndSortCallbacks: hkBool,
    pub m_entitiesExitingBroadPhase: hkArray_hkpEntity___hkContainerHeapAllocator_,
}

impl hkpBroadPhaseBorder {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }

    pub fn as_hkpWorldDeletionListener_ptr(&self) -> *const hkpWorldDeletionListener {
        self as *const _ as _
    }

    pub fn as_hkpWorldDeletionListener_mut_ptr(&mut self) -> *mut hkpWorldDeletionListener {
        self as *mut _ as _
    }

    pub fn as_hkpPhantomOverlapListener_ptr(&self) -> *const hkpPhantomOverlapListener {
        self as *const _ as _
    }

    pub fn as_hkpPhantomOverlapListener_mut_ptr(&mut self) -> *mut hkpPhantomOverlapListener {
        self as *mut _ as _
    }

    pub fn as_hkpWorldPostSimulationListener_ptr(&self) -> *const hkpWorldPostSimulationListener {
        self as *const _ as _
    }

    pub fn as_hkpWorldPostSimulationListener_mut_ptr(
        &mut self,
    ) -> *mut hkpWorldPostSimulationListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorder____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub maxPositionExceededCallback:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder, _: *mut hkpEntity),
    pub deactivate: unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseBorder),
}

#[repr(C)]
pub struct hkpMtThreadStructure {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_constraintQueryIn")]
#[repr(C)]
pub struct hkpMtThreadStructure {
    pub m_world: *mut hkpWorld,
    #[cfg(pdb_issue = "error in hkpConstraintQueryIn")]
    pub m_constraintQueryIn: hkpConstraintQueryIn,
    #[cfg(pdb_issue = "error in hkpProcessCollisionInput")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_contact")]
#[repr(C)]
pub struct hkpProcessCdPoint {
    #[cfg(pdb_issue = "error in hkContactPoint")]
    pub m_contact: hkContactPoint,
    pub m_contactPointId: u32,
    pub m_isShortestPoint: u32,
    pub m_padding: [u32; 2],
}

#[repr(C)]
pub struct hkpProcessCollisionData {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_contactPoints")]
#[repr(C)]
pub struct hkpProcessCollisionData {
    pub m_firstFreeContactPoint: hkPadSpu_hkpProcessCdPoint___,
    pub m_constraintOwner: hkPadSpu_hkpConstraintOwner___,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_contactPoints: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_toi: hkpProcessCollisionData__ToiInfo,
}

#[repr(C)]
pub struct hkpProcessCollisionData__ToiInfo {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_gskCache")]
#[repr(C)]
pub struct hkpProcessCollisionData__ToiInfo {
    pub m_contactPoint: hkContactPoint,
    pub m_time: hkPadSpu_float_,
    pub m_seperatingVelocity: hkPadSpu_float_,
    #[cfg(pdb_issue = "error in hkGskCache16")]
    pub m_gskCache: hkGskCache16,
    pub m_properties: hkContactPointPropertiesWithExtendedUserData16,
}

#[repr(C)]
pub struct hkpToiEvent {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_contactPoint")]
#[repr(C)]
pub struct hkpToiEvent {
    pub m_time: f32,
    pub m_seperatingVelocity: f32,
    pub m_useSimpleHandling: hkBool,
    pub m_entities: [*mut hkpEntity; 2],
    pub m_contactMgr: *mut hkpDynamicsContactMgr,
    pub m_properties: hkpContactPointProperties,
    pub m_extendedUserDatas: [u32; 7],
    #[cfg(pdb_issue = "error in hkContactPoint")]
    pub m_contactPoint: hkContactPoint,
}

#[repr(C)]
pub struct hkpContinuousSimulation {
    pub __vfptr: *const hkpContinuousSimulation____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_determinismCheckFrameCounter: u32,
    pub m_world: *mut hkpWorld,
    pub m_lastProcessingStep: hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_,
    pub m_currentTime: f32,
    pub m_currentPsiTime: f32,
    pub m_physicsDeltaTime: f32,
    pub m_simulateUntilTime: f32,
    pub m_frameMarkerPsiSnap: f32,
    pub m_previousStepResult: u32,
    pub m_toiEvents: hkArray_hkpToiEvent_hkContainerHeapAllocator_,
    pub m_entitiesNeedingPsiCollisionDetection:
        hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    pub m_toiResourceMgr: *mut hkpToiResourceMgr,
    pub m_toiCounter: i32,
}

impl hkpContinuousSimulation {
    pub fn as_hkpSimulation_ptr(&self) -> *const hkpSimulation {
        self as *const _ as _
    }

    pub fn as_hkpSimulation_mut_ptr(&mut self) -> *mut hkpSimulation {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpContinuousSimulation____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub m_data: *mut hkpToiEvent,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpToiEvent_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpToiEvent__ptr(&self) -> *const hkArrayBase_hkpToiEvent_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpToiEvent__mut_ptr(&mut self) -> *mut hkArrayBase_hkpToiEvent_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBodyVelocity {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_linear")]
#[repr(C)]
pub struct hkpBodyVelocity {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_linear: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_angular: hkVector4f,
}

#[repr(C)]
pub struct hkpSimpleConstraintInfoInitInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_massRelPos")]
#[repr(C)]
pub struct hkpSimpleConstraintInfoInitInput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_massRelPos: hkVector4f,
    #[cfg(pdb_issue = "error in hkMatrix3f")]
    pub m_invInertia: hkMatrix3f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_invMasses: hkVector4f,
    pub m_transform: *const hkTransformf,
}

#[repr(C)]
pub struct hkArray_short_hkContainerHeapAllocator_ {
    pub m_data: *mut i16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_short_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_short__ptr(&self) -> *const hkArrayBase_short_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_short__mut_ptr(&mut self) -> *mut hkArrayBase_short_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_short_ {
    pub m_data: *mut i16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpPhantomBroadPhaseListener {
    pub __vfptr: *const hkpPhantomBroadPhaseListener____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpPhantomBroadPhaseListener____vftable,
}

impl hkpPhantomBroadPhaseListener {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }

    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpPhantomBroadPhaseListener____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpContactMgr {
    pub __vfptr: *const hkpContactMgr____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkpContactMgr__Type,
}

impl hkpContactMgr {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpContactMgr____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl:
        unsafe extern "thiscall" fn(this: *mut hkpContactMgr, _: i32) -> hkResult,
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
    pub __vfptr: *const hkpWorldExtension____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_id: i32,
    pub m_attachmentCount: u16,
}

impl hkpWorldExtension {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpWorldExtension____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub performAttachments:
        unsafe extern "thiscall" fn(this: *mut hkpWorldExtension, _: *mut hkpWorld),
    pub performDetachments:
        unsafe extern "thiscall" fn(this: *mut hkpWorldExtension, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkpEntityActivationListener {
    pub __vfptr: *const hkpEntityActivationListener____vftable,
}

#[repr(C)]
pub struct hkpEntityActivationListener____vftable {
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
    pub __vfptr: *const hkpMultiThreadedSimulation____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_determinismCheckFrameCounter: u32,
    pub m_world: *mut hkpWorld,
    pub m_lastProcessingStep: hkEnum_enum_hkpSimulation__LastProcessingStep_unsigned_char_,
    pub m_currentTime: f32,
    pub m_currentPsiTime: f32,
    pub m_physicsDeltaTime: f32,
    pub m_simulateUntilTime: f32,
    pub m_frameMarkerPsiSnap: f32,
    pub m_previousStepResult: u32,
    pub m_toiEvents: hkArray_hkpToiEvent_hkContainerHeapAllocator_,
    pub m_entitiesNeedingPsiCollisionDetection:
        hkPointerMap_unsigned_int_hkpEntity___hkContainerHeapAllocator_,
    pub m_toiResourceMgr: *mut hkpToiResourceMgr,
    pub m_toiCounter: i32,
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
    pub m_toiQueueCriticalSection: hkCriticalSection,
    pub m_phantomCriticalSection: hkCriticalSection,
}

impl hkpMultiThreadedSimulation {
    pub fn as_hkpContinuousSimulation_ptr(&self) -> *const hkpContinuousSimulation {
        self as *const _ as _
    }

    pub fn as_hkpContinuousSimulation_mut_ptr(&mut self) -> *mut hkpContinuousSimulation {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
pub struct hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener {
    pub __vfptr: *const hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener____vftable,
    pub m_simulation: *mut hkpMultiThreadedSimulation,
}

impl hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener {
    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener {
    pub __vfptr: *const hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener____vftable,
    pub m_criticalSection: *mut hkCriticalSection,
}

impl hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener {
    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtBroadPhaseBorderListener {
    pub __vfptr: *const hkpMultiThreadedSimulation__MtBroadPhaseBorderListener____vftable,
    pub m_criticalSection: *mut hkCriticalSection,
}

impl hkpMultiThreadedSimulation__MtBroadPhaseBorderListener {
    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMultiThreadedSimulation__MtBroadPhaseBorderListener____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBroadPhaseListener, _: u32) -> *mut (),
    pub addCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
        _: *mut hkpTypedBroadPhaseHandlePair,
    ),
    pub removeCollisionPair: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhaseListener,
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
    pub m_data: *mut hkpTypedBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpTypedBroadPhaseHandlePair__ptr(
        &self,
    ) -> *const hkArrayBase_hkpTypedBroadPhaseHandlePair_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpTypedBroadPhaseHandlePair__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpTypedBroadPhaseHandlePair_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListener {
    pub __vfptr: *const hkpContactImpulseLimitBreachedListener____vftable,
}

#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListener____vftable {
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_data")]
#[repr(C)]
pub struct hkpContactImpulseLimitBreachedListenerInfo {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_data: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct hkpShapeRayBundleCastOutput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_outputs")]
#[repr(C)]
pub struct hkpShapeRayBundleCastOutput {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_outputs: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct hkpWorldMaintenanceMgr {
    pub __vfptr: *const hkpWorldMaintenanceMgr____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkpWorldMaintenanceMgr {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpWorldMaintenanceMgr____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpEntityEntityBroadPhaseListener____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpEntityEntityBroadPhaseListener____vftable,
    pub m_world: *mut hkpWorld,
}

impl hkpEntityEntityBroadPhaseListener {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }

    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpEntityEntityBroadPhaseListener____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener {
    pub __vfptr: *const hkpBroadPhaseBorderListener____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpBroadPhaseBorderListener____vftable,
}

impl hkpBroadPhaseBorderListener {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }

    pub fn as_hkpBroadPhaseListener_ptr(&self) -> *const hkpBroadPhaseListener {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseListener_mut_ptr(&mut self) -> *mut hkpBroadPhaseListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpToiResourceMgr {
    pub __vfptr: *const hkpToiResourceMgr____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkpToiResourceMgr {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpToiResourceMgr____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub beginToiAndSetupResources: unsafe extern "thiscall" fn(
        this: *mut hkpToiResourceMgr,
        _: *const hkpToiEvent,
        _: *const hkArray_hkpToiEvent_hkContainerHeapAllocator_,
        _: *mut hkpToiResources,
    ) -> hkResult,
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
    pub m_data: *mut hkpToiResourceMgr__ConstraintViolationInfo,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo__ptr(
        &self,
    ) -> *const hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_ {
    pub m_data: *mut hkpToiResourceMgr__ConstraintViolationInfo,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpActionListener {
    pub __vfptr: *const hkpActionListener____vftable,
}

#[repr(C)]
pub struct hkpActionListener____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: u32) -> *mut (),
    pub actionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: *mut hkpAction),
    pub actionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpActionListener, _: *mut hkpAction),
}

#[repr(C)]
pub struct hkpIslandActivationListener {
    pub __vfptr: *const hkpIslandActivationListener____vftable,
}

#[repr(C)]
pub struct hkpIslandActivationListener____vftable {
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
    pub __vfptr: *const hkpWorldPostCollideListener____vftable,
}

#[repr(C)]
pub struct hkpWorldPostCollideListener____vftable {
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
    pub __vfptr: *const hkpWorldPostIntegrateListener____vftable,
}

#[repr(C)]
pub struct hkpWorldPostIntegrateListener____vftable {
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
    pub __vfptr: *const hkpIslandPostCollideListener____vftable,
}

#[repr(C)]
pub struct hkpIslandPostCollideListener____vftable {
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
    pub __vfptr: *const hkpIslandPostIntegrateListener____vftable,
}

#[repr(C)]
pub struct hkpIslandPostIntegrateListener____vftable {
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_motion")]
#[repr(C)]
pub struct hkpEntity {
    pub __vfptr: *const hkpEntity____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
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
    #[cfg(pdb_issue = "error in hkpMaxSizeMotion")]
    pub m_motion: hkpMaxSizeMotion,
    pub m_contactListeners: hkSmallArray_hkpContactListener___,
    pub m_actions: hkSmallArray_hkpAction___,
    pub m_localFrame: hkRefPtr_hkLocalFrame_,
    pub m_extendedListeners: *mut hkpEntity__ExtendedListeners,
    pub m_npData: u32,
}

impl hkpEntity {
    pub fn as_hkpWorldObject_ptr(&self) -> *const hkpWorldObject {
        self as *const _ as _
    }

    pub fn as_hkpWorldObject_mut_ptr(&mut self) -> *mut hkpWorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpEntity____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpConstraintInstance____vftable,
    pub m_memSizeAndRefCount: u32,
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

impl hkpConstraintInstance {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpConstraintInstance____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpShapeBase____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
}

impl hkpShapeBase {
    pub fn as_hkcdShape_ptr(&self) -> *const hkcdShape {
        self as *const _ as _
    }

    pub fn as_hkcdShape_mut_ptr(&mut self) -> *mut hkcdShape {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpShapeBase____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> hkVector4fComparison,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_inertiaAndMassInv")]
#[repr(C)]
pub struct hkpMotion {
    pub __vfptr: *const hkpMotion____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkpMotion__MotionType_unsigned_char_,
    pub m_deactivationIntegrateCounter: u8,
    pub m_deactivationNumInactiveFrames: [u16; 2],
    pub m_motionState: hkMotionState,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_inertiaAndMassInv: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_linearVelocity: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_angularVelocity: hkVector4f,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_deactivationRefPosition: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
}

impl hkpMotion {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMotion____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub __: *const (),
    pub ___2: *const (),
    pub ___3: *const (),
    pub ___4: *const (),
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
    pub ___5: *const (),
    pub ___6: *const (),
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
    pub __vfptr: *const hkpWorldObject____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_userData: u32,
    pub m_collidable: hkpLinkedCollidable,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_name: hkStringPtr,
    pub m_properties: hkArray_hkSimpleProperty_hkContainerHeapAllocator_,
}

impl hkpWorldObject {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpWorldObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpShape____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    pub m_userData: u32,
}

impl hkpShape {
    pub fn as_hkpShapeBase_ptr(&self) -> *const hkpShapeBase {
        self as *const _ as _
    }

    pub fn as_hkpShapeBase_mut_ptr(&mut self) -> *mut hkpShapeBase {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpShape____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpShapeBase) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpShapeBase,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> hkVector4fComparison,
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

pub type kaiko__LocalGameSessionInteractionId = i32;

pub type keen__SaveData__OperationStatus = i32;

pub type keen__SaveData__OperationResult = i32;

pub type keen__SaveData__CurrentOperation = i32;

pub type keen__SaveData__SaveDataSystemType = i32;

pub type keen__SaveFlowState = i32;

pub type keen__SaveDataHandler__SaveGameClientState = i32;

pub type keen__ControllerType = i32;

pub type keen__ControllerClass = i32;

pub type keen__InputSystemControllerAutoCatchType = i32;

pub type gfc__ImageFormat = i32;

pub type gfc__WorldObject__Flags = i32;

pub type gfc__Texture__Type = i32;

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

pub type gfc__StaticObject__Flags = i32;

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

pub type gfc__MoveInput__DirectionHemispheres = i32;

pub type gfc__MoveInput__DirectionQuad = i32;

pub type hkpPhantomType = i32;

pub type hkpStepResult = i32;

pub type hkWorldOperation__Result = i32;

pub type hkpBodyOperation__ExecutionState = i32;

pub type hkpSimulation__ResetCollisionInformation = i32;

pub type hkpSimulation__FindContacts = i32;

pub type hkpToiResourceMgrResponse = i32;

#[repr(C)]
pub struct threadmbcinfostruct {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct __lc_time_data {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__ResourceFindDebugDataResult {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__InternalList_keen__ResourceRequest___ConstIterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__ResourceContextDescriptor_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__BlobResourceHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _TP_POOL {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _TP_CLEANUP_GROUP {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _ACTIVATION_CONTEXT {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _TP_CALLBACK_INSTANCE {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ISteamGameServer {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ISteamGameServerStats {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _IMAGELIST {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _DSA {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__LocalPlayerIdStructureType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct _DPA {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__Task___ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__WorkerThreadContext_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice__D3D11_SIGNATURE_PARAMETER_DESC_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__RenderEffectSlot_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__TextureHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__Slice_keen__RenderCommand___ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator_
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__ModelHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__MaterialHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__profiler__ZoneVisit {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_float_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__Matrix43_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_unsigned_char_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__AnimationJoint_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__VoiceSound_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__TemporarySoundDefinition_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__ParameterAutomation_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__SoundTriggerMemory_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__SoundSystem__System {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__SingleLinkedInternalList_keen__SoundDeviceChangedCallback_keen__SoundDeviceChangedCallback___4___ConstIterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__SoundVoiceXAudio2_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SoundSystem__BaseSound_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__JointAnimationHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__LveAnimationHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__MorphAnimationHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__AnimationHandleType {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__AnimationCommandBuffer_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_SOUND {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_SYSTEM {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_DSP {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_CHANNEL {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_SOUNDGROUP {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__VideoSampleData_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__AudioFrameData_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IAudioRenderClient {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FMOD_SYNCPOINT {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SaveData__SaveDataProviderWin32Steam__SaveDataFormatSteam__Segment_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SaveData__SaveDataProviderWin32Steam__SaveDataFormatSteam_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__SaveDataHandler__SaveGameClientData_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__SingleLinkedInternalList_keen__TerminationHandler_keen__TerminationHandler___4___ConstIterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__PakFileDevice__PakFileStream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__PakfileEntry_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__MemoryFileDevice__FileEntry_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_unsigned_int_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__NativeFileDevice__ReadThreadContext_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWaitable__HandlerStruct_std__garray_GWaitable__HandlerStruct_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWaitable__HandlerStruct_std__garray_GWaitable__HandlerStruct_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GColor_std__garray_GColor_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GColor_std__garray_GColor_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GThread___GThreadList__ThreadHashOp_GThreadList__ThreadHashOp_std__ghashset_cached_entry_GThread___GThreadList__ThreadHashOp_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GThread___GThreadList__ThreadHashOp_GThreadList__ThreadHashOp_std__ghashset_cached_entry_GThread___GThreadList__ThreadHashOp_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASArraySortFunctor_std__garray_GASArraySortFunctor_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASArraySortFunctor_std__garray_GASArraySortFunctor_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASWithStackEntry_std__garray_GASWithStackEntry_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASWithStackEntry_std__garray_GASWithStackEntry_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxLineStyle_std__garray_GFxLineStyle_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxLineStyle_std__garray_GFxLineStyle_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonCharacterDef__GFxSoundEnvelope_std__garray_GFxButtonCharacterDef__GFxSoundEnvelope_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonCharacterDef__GFxSoundEnvelope_std__garray_GFxButtonCharacterDef__GFxSoundEnvelope_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_short_std__garray_unsigned_short_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_short_std__garray_unsigned_short_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxImportData__Symbol_std__garray_GFxImportData__Symbol_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxImportData__Symbol_std__garray_GFxImportData__Symbol_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GMatrix2D_std__garray_GMatrix2D_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GMatrix2D_std__garray_GMatrix2D_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontData__AdvanceEntry_std__garray_GFxFontData__AdvanceEntry_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontData__AdvanceEntry_std__garray_GFxFontData__AdvanceEntry_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxMovieDefImpl__std__garray_GPtr_GFxMovieDefImpl_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxMovieDefImpl__std__garray_GPtr_GFxMovieDefImpl_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextDocView__ImageSubstitutor__Element_std__garray_GFxTextDocView__ImageSubstitutor__Element_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextDocView__ImageSubstitutor__Element_std__garray_GFxTextDocView__ImageSubstitutor__Element_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMorphCharacterDef__Path_std__garray_GFxMorphCharacterDef__Path_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMorphCharacterDef__Path_std__garray_GFxMorphCharacterDef__Path_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTexture9Grid_std__garray_GFxTexture9Grid_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTexture9Grid_std__garray_GFxTexture9Grid_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_std__ghashset_cached_entry_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_std__ghashset_cached_entry_GFxFontCompactor__GlyphKeyType_GFxFontCompactor__GlyphKeyType_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMovieRoot__LevelInfo_std__garray_GFxMovieRoot__LevelInfo_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMovieRoot__LevelInfo_std__garray_GFxMovieRoot__LevelInfo_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxTextFormatPtrWrapper_GFxTextParagraphFormat__GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_std__ghashset_cached_entry_GFxTextFormatPtrWrapper_GFxTextParagraphFormat__GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxTextFormatPtrWrapper_GFxTextParagraphFormat__GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_std__ghashset_cached_entry_GFxTextFormatPtrWrapper_GFxTextParagraphFormat__GFxTextFormatPtrWrapper_GFxTextParagraphFormat___HashFunctor_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASObjectInterface___std__garray_GASObjectInterface_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASObjectInterface___std__garray_GASObjectInterface_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASPagedStack_GPtr_GASFunctionObject__32___Page___std__garray_GASPagedStack_GPtr_GASFunctionObject__32___Page_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASPagedStack_GPtr_GASFunctionObject__32___Page___std__garray_GASPagedStack_GPtr_GASFunctionObject__32___Page_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextHighlightDesc_std__garray_GFxTextHighlightDesc_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextHighlightDesc_std__garray_GFxTextHighlightDesc_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextureGlyph_std__garray_GFxTextureGlyph_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextureGlyph_std__garray_GFxTextureGlyph_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASExecuteTag___std__garray_GASExecuteTag_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASExecuteTag___std__garray_GASExecuteTag_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMorphCharacterDef__Edge_std__garray_GFxMorphCharacterDef__Edge_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMorphCharacterDef__Edge_std__garray_GFxMorphCharacterDef__Edge_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStyledText__HTMLImageTagInfo_std__garray_GFxStyledText__HTMLImageTagInfo_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStyledText__HTMLImageTagInfo_std__garray_GFxStyledText__HTMLImageTagInfo_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxCachedStroke_std__garray_GFxCachedStroke_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxCachedStroke_std__garray_GFxCachedStroke_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextKeyMap__KeyMapEntry_std__garray_GFxTextKeyMap__KeyMapEntry_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextKeyMap__KeyMapEntry_std__garray_GFxTextKeyMap__KeyMapEntry_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_unsigned_short_std__gfixed_size_hash_unsigned_short__std__gfixed_size_hash_unsigned_short__std__ghashset_cached_entry_unsigned_short_std__gfixed_size_hash_unsigned_short_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_unsigned_short_std__gfixed_size_hash_unsigned_short__std__gfixed_size_hash_unsigned_short__std__ghashset_cached_entry_unsigned_short_std__gfixed_size_hash_unsigned_short_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxDisplayList__DisplayEntry_std__garray_GFxDisplayList__DisplayEntry_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxDisplayList__DisplayEntry_std__garray_GFxDisplayList__DisplayEntry_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_char_std__garray_unsigned_char_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_char_std__garray_unsigned_char_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_bool_std__garray_bool_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_bool_std__garray_bool_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonRecord_std__garray_GFxButtonRecord_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonRecord_std__garray_GFxButtonRecord_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GFxKeyboardState__IListener__std__garray_GWeakPtr_GFxKeyboardState__IListener_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GFxKeyboardState__IListener__std__garray_GWeakPtr_GFxKeyboardState__IListener_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxSprite__std__garray_GPtr_GFxSprite_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxSprite__std__garray_GPtr_GFxSprite_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTask___std__garray_GFxTask_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTask___std__garray_GFxTask_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxTextureGlyph__std__garray_GPtr_GFxTextureGlyph_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxTextureGlyph__std__garray_GPtr_GFxTextureGlyph_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxShapeBase__std__garray_GPtr_GFxShapeBase_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxShapeBase__std__garray_GPtr_GFxShapeBase_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASIntervalTimer___std__garray_GASIntervalTimer_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASIntervalTimer___std__garray_GASIntervalTimer_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRangeData_GPtr_GFxTextFormat____std__garray_GRangeData_GPtr_GFxTextFormat_________iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRangeData_GPtr_GFxTextFormat____std__garray_GRangeData_GPtr_GFxTextFormat_________const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTaskThreadInPool___std__garray_GFxTaskThreadInPool_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTaskThreadInPool___std__garray_GFxTaskThreadInPool_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASPagedStack_GASValue_32___Page___std__garray_GASPagedStack_GASValue_32___Page_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASPagedStack_GASValue_32___Page___std__garray_GASPagedStack_GASValue_32___Page_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GRefCountBaseImpl__std__garray_GWeakPtr_GRefCountBaseImpl_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GRefCountBaseImpl__std__garray_GWeakPtr_GRefCountBaseImpl_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxTask__std__garray_GPtr_GFxTask_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxTask__std__garray_GPtr_GFxTask_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextLineBuffer__Line___std__garray_GFxTextLineBuffer__Line_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextLineBuffer__Line___std__garray_GFxTextLineBuffer__Line_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GASStringNode___GASStringNodeHashFunc_GASStringNode____GASStringNodeHashFunc_GASStringNode____std__ghashset_entry_GASStringNode___GASStringNodeHashFunc_GASStringNode_________const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GASStringNode___GASStringNodeHashFunc_GASStringNode____GASStringNodeHashFunc_GASStringNode____std__ghashset_entry_GASStringNode___GASStringNodeHashFunc_GASStringNode_________iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFillStyle_std__garray_GFxFillStyle_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFillStyle_std__garray_GFxFillStyle_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASValue_std__garray_GASValue_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASValue_std__garray_GASValue_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxTextFormatPtrWrapper_GFxTextFormat__GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_std__ghashset_cached_entry_GFxTextFormatPtrWrapper_GFxTextFormat__GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxTextFormatPtrWrapper_GFxTextFormat__GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_std__ghashset_cached_entry_GFxTextFormatPtrWrapper_GFxTextFormat__GFxTextFormatPtrWrapper_GFxTextFormat___HashFunctor_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASAsFunctionObject__ArgSpec_std__garray_cc_GASAsFunctionObject__ArgSpec_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASAsFunctionObject__ArgSpec_std__garray_cc_GASAsFunctionObject__ArgSpec_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_node_GFxResourceId_GFxResourcePtr_GFxImageResource__std__gfixed_size_hash_GFxResourceId_____node_althashf
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASAsBroadcaster__ListenerObject_std__garray_GASAsBroadcaster__ListenerObject_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASAsBroadcaster__ListenerObject_std__garray_GASAsBroadcaster__ListenerObject_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPoint_float__std__garray_GPoint_float_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPoint_float__std__garray_GPoint_float_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStaticTextRecord___std__garray_GFxStaticTextRecord_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStaticTextRecord___std__garray_GFxStaticTextRecord_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASString_std__garray_cc_GASString_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASString_std__garray_cc_GASString_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASEnvironment__TryDescr_std__garray_GASEnvironment__TryDescr_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASEnvironment__TryDescr_std__garray_GASEnvironment__TryDescr_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStaticTextRecord__GlyphEntry_std__garray_GFxStaticTextRecord__GlyphEntry_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxStaticTextRecord__GlyphEntry_std__garray_GFxStaticTextRecord__GlyphEntry_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GASObject__std__garray_GWeakPtr_GASObject_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GWeakPtr_GASObject__std__garray_GWeakPtr_GASObject_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSwfEvent___std__garray_GFxSwfEvent_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSwfEvent___std__garray_GFxSwfEvent_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMeshSet__MeshSubShape_std__garray_GFxMeshSet__MeshSubShape_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMeshSet__MeshSubShape_std__garray_GFxMeshSet__MeshSubShape_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonAction_std__garray_GFxButtonAction_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxButtonAction_std__garray_GFxButtonAction_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASActionBufferData___std__garray_GASActionBufferData_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASActionBufferData___std__garray_GASActionBufferData_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASValue___std__garray_GASValue_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GASValue___std__garray_GASValue_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_char_std__garray_char_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_char_std__garray_char_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxCharacter__std__garray_GPtr_GFxCharacter_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxCharacter__std__garray_GPtr_GFxCharacter_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontResource___std__garray_GFxFontResource_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontResource___std__garray_GFxFontResource_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_int_std__garray_unsigned_int_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_unsigned_int_std__garray_unsigned_int_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTimelineDef__Frame_std__garray_GFxTimelineDef__Frame_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTimelineDef__Frame_std__garray_GFxTimelineDef__Frame_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxPathPacker__Edge_std__garray_GFxPathPacker__Edge_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxPathPacker__Edge_std__garray_GFxPathPacker__Edge_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone__std__garray_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone__std__garray_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_short_std__garray_short_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_short_std__garray_short_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_std__ghashset_cached_entry_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__ghash_set_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_std__ghashset_cached_entry_GFxFontCompactor__ContourKeyType_GFxFontCompactor__ContourKeyType_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxASCharacter__std__garray_GPtr_GFxASCharacter_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxASCharacter__std__garray_GPtr_GFxASCharacter_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxScale9GridInfo__ImgAdjust_std__garray_GFxScale9GridInfo__ImgAdjust_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxScale9GridInfo__ImgAdjust_std__garray_GFxScale9GridInfo__ImgAdjust_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMeshSet___std__garray_GFxMeshSet_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMeshSet___std__garray_GFxMeshSet_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GASLocalFrame__std__garray_GPtr_GASLocalFrame_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GASLocalFrame__std__garray_GPtr_GASLocalFrame_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_std__ghash_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const____std__ghash_node_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const______std__ghashset_cached_entry_void_const___std__gidentity_hash_void_const________std__garray_std__ghash_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const____std__ghash_node_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const______std__ghashset_cached_entry_void_const___std__gidentity_hash_void_const_____________iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_std__ghash_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const____std__ghash_node_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const______std__ghashset_cached_entry_void_const___std__gidentity_hash_void_const________std__garray_std__ghash_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const____std__ghash_node_void_const___GASEnvironment__RecursionDescr_std__gidentity_hash_void_const______std__ghashset_cached_entry_void_const___std__gidentity_hash_void_const_____________const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMesh_std__garray_GFxMesh_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxMesh_std__garray_GFxMesh_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GRangeDataArray_GPtr_GFxTextFormat__std__garray_GRangeData_GPtr_GFxTextFormat_________ConstPositionIterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GFxSoundSampleImpl {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GRangeDataArray_GFxEditTextCharacter__CSSHolder__UrlZone_std__garray_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone_______ConstIterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GRangeDataArray_GFxEditTextCharacter__CSSHolder__UrlZone_std__garray_GRangeData_GFxEditTextCharacter__CSSHolder__UrlZone_______ConstPositionIterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GFxTextCompositionString {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GPtr_GFxIMEManager_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GFxXMLSupportBase {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GPtr_GFxXMLSupportBase_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRenderer__BitmapDesc_std__garray_GRenderer__BitmapDesc_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GRenderer__BitmapDesc_std__garray_GRenderer__BitmapDesc_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontGlyphPacker__GlyphInfo_std__garray_GFxFontGlyphPacker__GlyphInfo_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontGlyphPacker__GlyphInfo_std__garray_GFxFontGlyphPacker__GlyphInfo_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontResource__DisposeHandler___std__garray_GFxFontResource__DisposeHandler_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxFontResource__DisposeHandler___std__garray_GFxFontResource__DisposeHandler_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_int_std__garray_int_____iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_int_std__garray_int_____const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxMovieDataDef__std__garray_GPtr_GFxMovieDataDef_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GPtr_GFxMovieDataDef__std__garray_GPtr_GFxMovieDataDef_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base__anonymous_namespace___Range_std__garray__anonymous_namespace___Range_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base__anonymous_namespace___Range_std__garray__anonymous_namespace___Range_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxBatchPackageData__GlyphVerifier_std__garray_GFxBatchPackageData__GlyphVerifier_____iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxBatchPackageData__GlyphVerifier_std__garray_GFxBatchPackageData__GlyphVerifier_____const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct GSize_short_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSGMLStackElemDesc_char__std__garray_GFxSGMLStackElemDesc_char_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSGMLStackElemDesc_char__std__garray_GFxSGMLStackElemDesc_char_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_CSSToken_char__std__garray_CSSToken_char_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_CSSToken_char__std__garray_CSSToken_char_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSGMLStackElemDesc_wchar_t__std__garray_GFxSGMLStackElemDesc_wchar_t_______iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxSGMLStackElemDesc_wchar_t__std__garray_GFxSGMLStackElemDesc_wchar_t_______const_iterator
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextStyle___std__garray_GFxTextStyle_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_GFxTextStyle___std__garray_GFxTextStyle_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_CSSToken_wchar_t__std__garray_CSSToken_wchar_t_______iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__garray_base_CSSToken_wchar_t__std__garray_CSSToken_wchar_t_______const_iterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__InputEvent_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__GameInputMapper__AxisData_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__Slice_keen__GameInputMapper__Input_ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct keen__RingBuffer_keen__GestureHelper__MoveData___ReverseConstIterator {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__pair_gfc__AutoRef_gfc__Value__gfc__AutoRef_gfc__Value___ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__AutoRef_gfc__Value__gfc__AutoRef_gfc__Value__std__less_gfc__AutoRef_gfc__Value____std__allocator_std__pair_gfc__AutoRef_gfc__Value__const__gfc__AutoRef_gfc__Value______0___value_compare
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Value__gfc__AutoRef_gfc__Value__std__less_gfc__AutoRef_gfc__Value____std__allocator_std__pair_gfc__AutoRef_gfc__Value__const__gfc__AutoRef_gfc__Value______0______Node
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___value_compare
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tree_unchecked_iterator_std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Value__gfc__AutoRef_gfc__Value__gfc__ValueCompare_std__allocator_std__pair_gfc__AutoRef_gfc__Value__const__gfc__AutoRef_gfc__Value______0_____
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__istreambuf_iterator_unsigned_short_std__char_traits_unsigned_short___ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__num_get_unsigned_short_std__istreambuf_iterator_unsigned_short_std__char_traits_unsigned_short_____
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__reverse_iterator_std___Tree_iterator_std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_______
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__reverse_iterator_std___Tree_const_iterator_std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_______
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___value_compare
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__reverse_iterator_std___Tree_iterator_std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_______
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__reverse_iterator_std___Tree_const_iterator_std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_______
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__pair_std___Tree_const_iterator_std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______std___Tree_const_iterator_std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_______
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std__pair_gfc__HString_gfc__ResourceCache___ {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___value_compare
{
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct std___String_iterator_wchar_t_std__char_traits_wchar_t__std__allocator_wchar_t___ {
    _opaque: [u8; 0],
}
