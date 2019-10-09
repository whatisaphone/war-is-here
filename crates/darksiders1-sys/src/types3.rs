#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{types::*, types2::*};
use pdbindgen_runtime::AsPtr;

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
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub deferredCommandBuffer: keen__GraphicsCommandBuffer,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub emptyFragmentShader: keen__FragmentShader,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pDefaultSwapChain: *mut compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pCurrentSwapChain: *mut compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub currentFrameNumber: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pScreenCapture: *mut compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub previousFullscreenMode: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub fullscreenMode: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeWidth: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeHeight: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeRefreshRateNumerator: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeRefreshRateDenominator: u32,
}

impl AsPtr<keen__GraphicsSystemBase> for *const keen__GraphicsSystem {
    fn as_ptr(self) -> *const keen__GraphicsSystemBase {
        self as *const _
    }
}

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

impl AsPtr<keen__InternalListBase__IteratorBase> for *const keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator {
    fn as_ptr(self) -> *const keen__InternalListBase__IteratorBase {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__GraphicsStateObjectPool_keen__DepthStencilState_ {
    pub m_allocator: keen__PoolAllocator_keen__DepthStencilState_,
    pub m_cache: keen__GraphicsStateObjectCache,
    pub m_peakSize: u32,
    pub m_cacheHits: u32,
    pub m_cacheRequests: u32,
}

#[repr(C)]
pub struct keen__DownsampleDepthVariants {
    pub m_fragmentShaders: [*const keen__FragmentShader; 2],
    pub m_vertexShaders: [*const keen__VertexShader; 1],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_pipelines: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_pipelines: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_shaderPipelines: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
    pub m_pStockVertexShaders: [*const keen__VertexShader; 2],
    pub m_pStockFragmentShaders: [*const keen__FragmentShader; 3],
    pub m_pFormats: [*const keen__VertexFormat; 3],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_textures: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 32],
}

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
    pub __vfptr: *const gfc__Character____vftable,
    // gfc__IRefObject
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
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
    __pdbindgen_padding: [u8; 4],
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

impl AsPtr<gfc__KinematicActor> for *const gfc__Character {
    fn as_ptr(self) -> *const gfc__KinematicActor {
        self as *const _
    }
}

impl AsPtr<gfc__Actor> for *const gfc__Character {
    fn as_ptr(self) -> *const gfc__Actor {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__Character {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__Character {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Character {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__Character____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
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
    pub __vfptr: *const gfc__Actor____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__WorldObject> for *const gfc__Actor {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__Actor {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Actor {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__Actor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
pub struct gfc__AutoRef_gfc__Item_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TriggerRegion {
    pub __vfptr: *const gfc__TriggerRegion____vftable,
    // gfc__IRefObject
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
    pub mBodyType: u8,
    pub mRegion: *mut gfc__DetectorRegion,
    pub mEnabled: bool,
    // gfc__TriggerRegion
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

impl AsPtr<gfc__DetectorObject> for *const gfc__TriggerRegion {
    fn as_ptr(self) -> *const gfc__DetectorObject {
        self as *const _
    }
}

impl AsPtr<gfc__PhysicsShapeObject> for *const gfc__TriggerRegion {
    fn as_ptr(self) -> *const gfc__PhysicsShapeObject {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__TriggerRegion {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__TriggerRegion {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__TriggerRegion {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__TriggerRegion____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGizmoColor: unsafe extern "thiscall" fn(
        this: *const gfc__PhysicsShapeObject,
    )
        -> *const gfc__TVector4_float_gfc__FloatMath_,
    pub getGizmoIcon:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeObject) -> *const gfc__HString,
    pub getPhantomBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub onEnter: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__Actor),
    pub onExit: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__Actor),
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
    pub __vfptr: *const gfc__Player____vftable,
    // gfc__IRefObject
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
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
    __pdbindgen_padding: [u8; 4],
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
    pub __vfptr_2: *const gfc__ResourceListener____vftable,
    // gfc__ResourceListener
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
    __pdbindgen_padding_2: [u8; 4],
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
    __pdbindgen_padding_3: [u8; 12],
}

impl AsPtr<gfc__Character> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__Character {
        self as *const _
    }
}

impl AsPtr<gfc__KinematicActor> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__KinematicActor {
        self as *const _
    }
}

impl AsPtr<gfc__Actor> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__Actor {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

impl AsPtr<gfc__ResourceListener> for *const gfc__Player {
    fn as_ptr(self) -> *const gfc__ResourceListener {
        (self as usize + 0x4d0) as *const _
    }
}

#[repr(C)]
pub struct gfc__Player____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: u32) -> *mut (),
    pub packageUnloading: unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: i32),
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
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
pub struct gfc__CharacterDoInteractiveDesc {
    pub __vfptr: *const gfc__CharacterDoInteractiveDesc____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__CharMoveStateDesc> for *const gfc__CharacterDoInteractiveDesc {
    fn as_ptr(self) -> *const gfc__CharMoveStateDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__CharacterDoInteractiveDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__CharacterDoInteractiveDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__CharacterDoInteractiveDesc____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub createMoveState: unsafe extern "thiscall" fn(
        this: *mut gfc__CharMoveStateDesc,
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
    pub __vfptr: *const gfc__Mount____vftable,
    // gfc__IRefObject
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
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
    __pdbindgen_padding: [u8; 4],
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
    __pdbindgen_padding_2: [u8; 12],
}

impl AsPtr<gfc__Character> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__Character {
        self as *const _
    }
}

impl AsPtr<gfc__KinematicActor> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__KinematicActor {
        self as *const _
    }
}

impl AsPtr<gfc__Actor> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__Actor {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Mount {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__Mount____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
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
pub struct gfc__CharMoveVars {
    pub __vfptr: *const gfc__CharMoveVars____vftable,
}

#[repr(C)]
pub struct gfc__CharMoveVars____vftable {
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
    pub __vfptr: *const gfc__KinematicActor____vftable,
    // gfc__IRefObject
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
}

impl AsPtr<gfc__Actor> for *const gfc__KinematicActor {
    fn as_ptr(self) -> *const gfc__Actor {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__KinematicActor {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__KinematicActor {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__KinematicActor {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__KinematicActor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
    pub __vfptr: *const gfc__CharMoveStateDesc____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__Object> for *const gfc__CharMoveStateDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__CharMoveStateDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__CharMoveStateDesc____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
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
    pub __vfptr: *const gfc__HitInfo____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__Object> for *const gfc__HitInfo {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__HitInfo {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__HitInfo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
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
    pub __vfptr: *const gfc__DetectorRegion____vftable,
    // hkpWorldPostSimulationListener
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

impl AsPtr<gfc__PhysicsDetectRegion> for *const gfc__DetectorRegion {
    fn as_ptr(self) -> *const gfc__PhysicsDetectRegion {
        self as *const _
    }
}

impl AsPtr<hkpWorldPostSimulationListener> for *const gfc__DetectorRegion {
    fn as_ptr(self) -> *const hkpWorldPostSimulationListener {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__DetectorRegion____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpEntity),
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
pub struct gfc__AutoRef_gfc__ChaosLevel_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WindowHelper {
    pub __vfptr: *const gfc__WindowHelper____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__Helper> for *const gfc__WindowHelper {
    fn as_ptr(self) -> *const gfc__Helper {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__WindowHelper {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__WindowHelper {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__WindowHelper____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc___UIControl__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc___UIControl_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TeleportHelper {
    pub __vfptr: *const gfc__TeleportHelper____vftable,
    // gfc__IRefObject
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

impl AsPtr<gfc__Helper> for *const gfc__TeleportHelper {
    fn as_ptr(self) -> *const gfc__Helper {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__TeleportHelper {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__TeleportHelper {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__TeleportHelper____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__Helper),
}

#[repr(C)]
pub struct gfc__DraggableActor {
    pub __vfptr: *const gfc__DraggableActor____vftable,
    // gfc__IRefObject
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
    // gfc__Character
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
    __pdbindgen_padding: [u8; 4],
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
    __pdbindgen_padding_2: [u8; 12],
}

impl AsPtr<gfc__Character> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__Character {
        self as *const _
    }
}

impl AsPtr<gfc__KinematicActor> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__KinematicActor {
        self as *const _
    }
}

impl AsPtr<gfc__Actor> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__Actor {
        self as *const _
    }
}

impl AsPtr<gfc__WorldObject> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__WorldObject {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__DraggableActor {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__DraggableActor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject),
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
    pub updateAnimation: unsafe extern "thiscall" fn(this: *mut gfc__KinematicActor, _: f32),
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
pub struct gfc__AutoRef_gfc__LiquidRegionDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PhysicsShapeGizmo {
    pub __vfptr: *const gfc__PhysicsShapeGizmo____vftable,
    // gfc__SceneObject
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
    // gfc__IRenderCallback
    pub mLocked: bool,
    // gfc__PhysicsShapeGizmo
    pub mObject: *mut gfc__PhysicsShapeObject,
    pub mMaterial: gfc__AutoRef_gfc__Material_,
}

impl AsPtr<gfc__SceneObject> for *const gfc__PhysicsShapeGizmo {
    fn as_ptr(self) -> *const gfc__SceneObject {
        self as *const _
    }
}

impl AsPtr<gfc__IRenderCallback> for *const gfc__PhysicsShapeGizmo {
    fn as_ptr(self) -> *const gfc__IRenderCallback {
        (self as usize + 0x50) as *const _
    }
}

#[repr(C)]
pub struct gfc__PhysicsShapeGizmo____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__IRenderCallback, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__IRenderCallback,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__IRenderCallback,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IRenderCallback, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IRenderCallback, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__IRenderCallback, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *mut gfc__Camera3D, _: u32),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__SceneObject) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__SceneObject) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__SceneObject) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject) -> bool,
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
    pub __vfptr: *const hkpPhantomListener____vftable,
}

#[repr(C)]
pub struct hkpPhantomListener____vftable {
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
    pub __vfptr: *const gfc__LoadMapMenu____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub __vfptr_2: *const gfc__Hierarchical_gfc___UIControl_____vftable,
    // gfc__Hierarchical_gfc___UIControl_
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
    pub mListItems: gfc__Vector_gfc__String_0_gfc__CAllocator_,
    pub mMaps: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mSelectedMapType: i32,
    pub mSelectedMapIdx: i32,
    pub mLastMapName: gfc__String,
    pub mLastMapRegion: gfc__String,
    pub mHasLastMap: bool,
}

impl AsPtr<gfc___UIControl> for *const gfc__LoadMapMenu {
    fn as_ptr(self) -> *const gfc___UIControl {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__LoadMapMenu {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__LoadMapMenu {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

impl AsPtr<gfc__Hierarchical_gfc___UIControl_> for *const gfc__LoadMapMenu {
    fn as_ptr(self) -> *const gfc__Hierarchical_gfc___UIControl_ {
        (self as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct gfc__LoadMapMenu____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: u32,
    ) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub addBack: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub add: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc___UIControl_),
    pub remove_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc___UIControl_),
    pub added: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub removed: unsafe extern "thiscall" fn(
        this: *mut gfc__Hierarchical_gfc___UIControl_,
        _: *mut gfc___UIControl,
    ),
    pub invalidateHierarchy:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc___UIControl_),
    pub getEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setVisible: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getVisible: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getFocusTraversable: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getMouseEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getLayoutEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setClipChildren: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getClipChildren: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
    pub getRegisterControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> bool,
    pub setText: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *const gfc__WString),
    pub setSize: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub setSizeValid: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getSize: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getPreferredSize: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getAbsolutePosition:
        unsafe extern "thiscall" fn(
            this: *mut gfc___UIControl,
            result: *mut gfc__TVector2_float_gfc__FloatMath_,
        ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: f32),
    pub getRotation: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> f32,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: gfc__AutoRef_gfc__UILayoutManager_,
    ),
    pub getLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__AutoRef_gfc__UILayoutManager_,
    )
        -> *mut gfc__AutoRef_gfc__UILayoutManager_,
    pub setLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: f32),
    pub getLayoutHint: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> f32,
    pub addAction:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: gfc__AutoRef_gfc___UIAction_),
    pub removeAction:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: gfc__AutoRef_gfc___UIAction_),
    pub clearAllActions: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub updateActions: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: f32),
    pub invalidateLayout: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub doAnchorLayout: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub doLayout: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub setAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: gfc__TVector2_float_gfc__FloatMath_,
    ),
    pub getAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub draw: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *mut gfc__UIRenderer,
        _: *mut gfc__TRect_long_,
    ),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: f32),
    pub pick: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: gfc__TVector2_int_gfc__FloatMath_,
        _: bool,
    ) -> *mut gfc___UIControl,
    pub getControlByID:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: i32) -> *mut gfc___UIControl,
    pub getControlByName: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__HString,
    ) -> *mut gfc___UIControl,
    pub setControlText: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__HString,
        _: *const gfc__WString,
    ) -> bool,
    pub setControlTextA: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__HString,
        _: *const gfc__String,
    ) -> bool,
    pub setControlVisible: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub setControlEnabled: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *const gfc__HString,
        _: bool,
    ) -> bool,
    pub processMouseEvent:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc__MouseEvent),
    pub processKeyboardEvent:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc__KeyboardEvent),
    pub processFocusEvent:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc__FocusEvent),
    pub onInit: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub onReInit: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub onDeInit: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub onVisibilityLost: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub setDialogResults:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: gfc__AutoRef_gfc__Value_),
    pub getLastDialogResult: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *mut gfc__AutoRef_gfc__Value_,
    ) -> bool,
    pub unregisterToolTipEvent: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub getInputListener: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__AutoRef_gfc___UIControl_,
    ) -> *mut gfc__AutoRef_gfc___UIControl_,
    pub initControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub postInitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub deinitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub reinitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub doInit: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub drawInternal:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc__UIRenderer),
    pub getAnchorPosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: *mut gfc___UIControl,
        _: u8,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getGlobalScale: unsafe extern "thiscall" fn(
        this: *const gfc___UIControl,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
    )
        -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub getParentSize: unsafe extern "thiscall" fn(
        this: *const gfc___UIControl,
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
pub struct gfc__KGMeshCache {
    pub __vfptr: *const gfc__KGMeshCache____vftable,
    // gfc__ResourceCache
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__MeshCache
    pub mReloadInfo: gfc__Vector_gfc__MeshCache__ReloadInfo_0_gfc__CAllocator_,
    // gfc__KGMeshCache
    pub mOurExt: i32,
    pub mOurLegacyExt: i32,
}

impl AsPtr<gfc__MeshCache> for *const gfc__KGMeshCache {
    fn as_ptr(self) -> *const gfc__MeshCache {
        self as *const _
    }
}

impl AsPtr<gfc__ResourceCache> for *const gfc__KGMeshCache {
    fn as_ptr(self) -> *const gfc__ResourceCache {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__KGMeshCache____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourceCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__ResourceCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__ResourceCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__ResourceCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourceCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__ResourceCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourceCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__ResourceCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourceCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourceCache,
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
    #[cfg(pdb_issue = "unimplemented feature: primitive kind 0x8")]
    pub QueryInterface: *mut unsafe extern "stdcall" fn(
        _: *mut IUnknown,
        _: *const _GUID,
        _: *mut *mut (),
    ) -> compile_error!(
        "unimplemented feature: primitive kind 0x8"
    ),
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

impl AsPtr<hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___> for *const hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_ {
    fn as_ptr(self) -> *const hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___ {
        self as *const _
    }
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
    pub __vfptr: *const hkExternalJobProfiler____vftable,
}

#[repr(C)]
pub struct hkExternalJobProfiler____vftable {
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_customJobs: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 16],
    pub m_numCustomJobs: i32,
    pub m_cpuSemaphoreBegin: i32,
    pub m_directMapSemaphoreEnd: i32,
    pub m_masterThreadQueue: i32,
    pub m_hwSetup: hkJobQueueHwSetup,
    pub m_queryRulesAreUpdated: hkBool,
    pub m_queueSemaphores: [*mut hkSemaphore; 5],
    pub m_numQueueSemaphores: i32,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_nextQueueToGet: compile_error!("unimplemented feature: sizeof array 0x0"),
    __pdbindgen_padding_2: [u8; 110],
    pub m_cpuThreadIndexToSemaphoreIndex: [i8; 12],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_jobFuncs: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_jobQueue: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 412],
}

#[repr(C)]
pub struct hkJobQueue__CustomJobTypeSetup {
    pub m_jobType: hkJobType,
    pub m_jobSubType: u8,
    pub m_threadId: i32,
}

#[repr(C)]
pub struct hkSweptTransformf {
    pub m_centerOfMass0: hkVector4f,
    pub m_centerOfMass1: hkVector4f,
    pub m_rotation0: hkQuaternionf,
    pub m_rotation1: hkQuaternionf,
    pub m_centerOfMassLocal: hkVector4f,
}

#[repr(C)]
pub struct hkMotionState {
    pub m_transform: hkTransformf,
    pub m_sweptTransform: hkSweptTransformf,
    pub m_deltaAngle: hkVector4f,
    pub m_objectRadius: f32,
    pub m_linearDamping: hkHalf,
    pub m_angularDamping: hkHalf,
    pub m_timeFactor: hkHalf,
    pub m_maxLinearVelocity: hkUFloat8,
    pub m_maxAngularVelocity: hkUFloat8,
    pub m_deactivationClass: u8,
}

#[repr(C)]
pub struct hkQuaternionf {
    pub m_vec: hkVector4f,
}

#[repr(C)]
pub struct hkSimdFloat32 {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_real: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct hkVector4f {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_quad: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct hkCustomAttributes {
    pub m_attributes: *const hkCustomAttributes__Attribute,
    pub m_numAttributes: i32,
}

#[repr(C)]
pub struct hkCustomAttributes__Attribute {
    pub m_name: *const i8,
    pub m_value: hkVariant,
}

#[repr(C)]
pub struct hkHalf {
    pub m_value: i16,
}

#[repr(C)]
pub struct hkMemoryAllocator {
    pub __vfptr: *const hkMemoryAllocator____vftable,
}

#[repr(C)]
pub struct hkMemoryAllocator____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: u32) -> *mut (),
    pub blockAlloc: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: i32) -> *mut (),
    pub blockFree: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: *mut (), _: i32),
    pub bufAlloc: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: *mut i32) -> *mut (),
    pub bufFree: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: *mut (), _: i32),
    pub bufRealloc: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator,
        _: *mut (),
        _: i32,
        _: *mut i32,
    ) -> *mut (),
    pub blockAllocBatch:
        unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: *mut *mut (), _: i32, _: i32),
    pub blockFreeBatch:
        unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator, _: *mut *mut (), _: i32, _: i32),
    pub getMemoryStatistics: unsafe extern "thiscall" fn(
        this: *const hkMemoryAllocator,
        _: *mut hkMemoryAllocator__MemoryStatistics,
    ),
    pub getAllocatedSize:
        unsafe extern "thiscall" fn(this: *const hkMemoryAllocator, _: *const (), _: i32) -> i32,
    pub resetPeakMemoryStatistics: unsafe extern "thiscall" fn(this: *mut hkMemoryAllocator),
    pub getExtendedInterface:
        unsafe extern "thiscall" fn(
            this: *mut hkMemoryAllocator,
        ) -> *mut hkMemoryAllocator__ExtendedInterface,
}

#[repr(C)]
pub struct hkRefCountedProperties {
    pub m_entries: hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkRefCountedProperties__Entry {
    pub m_object: hkRefPtr_hkReferencedObject_,
    pub m_key: u16,
    pub m_flags: u16,
}

#[repr(C)]
pub struct hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_ {
    // hkArrayBase_hkRefCountedProperties__Entry_
    pub m_data: *mut hkRefCountedProperties__Entry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_
}

impl AsPtr<hkArrayBase_hkRefCountedProperties__Entry_>
    for *const hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_
{
    fn as_ptr(self) -> *const hkArrayBase_hkRefCountedProperties__Entry_ {
        self as *const _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkRefCountedProperties__Entry_ {
    pub m_data: *mut hkRefCountedProperties__Entry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkTransformf {
    pub m_rotation: hkRotationf,
    pub m_translation: hkVector4f,
}

#[repr(C)]
pub struct hkFourTransposedPointsf {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_vertices: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 48],
}

#[repr(C)]
pub struct hkcdVertex {
    // hkVector4f
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_quad: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
    // hkcdVertex
}

impl AsPtr<hkVector4f> for *const hkcdVertex {
    fn as_ptr(self) -> *const hkVector4f {
        self as *const _
    }
}

#[repr(C)]
pub struct hkRotationf {
    // hkMatrix3f
    pub m_col0: hkVector4f,
    pub m_col1: hkVector4f,
    pub m_col2: hkVector4f,
    // hkRotationf
}

impl AsPtr<hkMatrix3f> for *const hkRotationf {
    fn as_ptr(self) -> *const hkMatrix3f {
        self as *const _
    }
}

#[repr(C)]
pub struct hkMatrix3f {
    pub m_col0: hkVector4f,
    pub m_col1: hkVector4f,
    pub m_col2: hkVector4f,
}

#[repr(C)]
pub struct hkVector4fComparison {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_mask: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct hkStringPtr {
    pub m_stringAndFlag: *const i8,
}

#[repr(C)]
pub struct hkSphere {
    pub m_pos: hkVector4f,
}

#[repr(C)]
pub struct hkcdShape {
    pub __vfptr: *const hkcdShape____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkcdShape
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
}

impl AsPtr<hkReferencedObject> for *const hkcdShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkcdShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkcdShape____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkReferencedObject {
    pub __vfptr: *const hkReferencedObject____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
}

impl AsPtr<hkBaseObject> for *const hkReferencedObject {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkReferencedObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpConstraintAtom {
    pub m_type: hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_,
}

#[repr(C)]
pub struct hkpSolverInfo {
    pub m_padding: f32,
    pub m_tau: f32,
    pub m_damping: f32,
    pub m_frictionTau: f32,
    pub m_globalAccelerationPerSubStep: hkVector4f,
    pub m_globalAccelerationPerStep: hkVector4f,
    pub m_integrateVelocityFactor: hkVector4f,
    pub m_invIntegrateVelocityFactor: hkVector4f,
    pub m_dampDivTau: f32,
    pub m_tauDivDamp: f32,
    pub m_dampDivFrictionTau: f32,
    pub m_frictionTauDivDamp: f32,
    pub m_contactRestingVelocity: f32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_deactivationInfo: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 192],
    pub m_deltaTime: f32,
    pub m_invDeltaTime: f32,
    pub m_numSteps: i32,
    pub m_numMicroSteps: i32,
    pub m_invNumMicroSteps: f32,
    pub m_invNumSteps: f32,
    pub m_forceCoherentConstraintOrderingInSolver: hkBool,
    pub m_deactivationNumInactiveFramesSelectFlag: [u8; 2],
    pub m_deactivationIntegrateCounter: u8,
    pub m_maxConstraintViolationSqrd: f32,
    __pdbindgen_padding_2: [u8; 12],
}

#[repr(C)]
pub struct hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_ {
    pub m_storage: u16,
}

#[repr(C)]
pub struct hkpConstraintData {
    pub __vfptr: *const hkpConstraintData____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintData
    pub m_userData: u32,
}

impl AsPtr<hkReferencedObject> for *const hkpConstraintData {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpConstraintData {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpConstraintData____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub getType: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> i32,
    pub getConstraintInfo: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        _: *mut hkpConstraintData__ConstraintInfo,
    ),
    pub isValid: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        result: *mut hkBool,
    ) -> *mut hkBool,
    pub setMaximumLinearImpulse: unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: f32),
    pub setMaximumAngularImpulse: unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: f32),
    pub setBreachImpulse: unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: f32),
    pub getMaximumLinearImpulse: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> f32,
    pub getMaximumAngularImpulse:
        unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> f32,
    pub getBreachImpulse: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> f32,
    pub setBodyToNotify: unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: i32),
    pub getNotifiedBodyIndex: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> u8,
    pub setSolvingMethod: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        _: hkpConstraintAtom__SolvingMethod,
    ),
    pub setInertiaStabilizationFactor: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        result: *mut hkResult,
        _: f32,
    ) -> *mut hkResult,
    pub getInertiaStabilizationFactor: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        result: *mut hkResult,
        _: *mut f32,
    ) -> *mut hkResult,
    pub getAppliedLinearImpulse: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        _: *const hkTransformf,
        _: *const hkTransformf,
        _: *const hkpConstraintRuntime,
        _: *mut hkVector4f,
    ),
    pub getRuntimeInfo: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        _: hkBool,
        _: *mut hkpConstraintData__RuntimeInfo,
    ),
    pub getSolverResults: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        _: *mut hkpConstraintRuntime,
    ) -> *mut hkpSolverResults,
    pub addInstance: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        _: *mut hkpConstraintRuntime,
        _: i32,
    ),
    pub updateDirtyAtoms: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
    )
        -> hkpConstraintData__UpdateAtomsResult__Enum,
    pub buildJacobian: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        _: *const hkpConstraintQueryIn,
        _: *mut hkpConstraintQueryOut,
    ),
    pub isBuildJacobianCallbackRequired: unsafe extern "thiscall" fn(
        this: *const hkpConstraintData,
        result: *mut hkBool,
    ) -> *mut hkBool,
    pub buildJacobianCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        _: *const hkpConstraintQueryIn,
        _: *const hkpConstraintQueryOut,
    ),
}

#[repr(C)]
pub struct hkpVelocityAccumulator {
    pub m_type: hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorType_unsigned_char_,
    pub m_context: hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorContext_unsigned_char_,
    pub m_deactivationClass: u32,
    pub m_gravityFactor: f32,
    __pdbindgen_padding: [u8; 4],
    pub m_linearVel: hkVector4f,
    pub m_angularVel: hkVector4f,
    pub m_invMasses: hkVector4f,
    pub m_scratch0: hkVector4f,
    pub m_scratch1: hkVector4f,
    pub m_scratch2: hkVector4f,
    pub m_scratch3: hkVector4f,
}

#[repr(C)]
pub struct hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpJacobianSchema {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorContext_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpShapeRayCastInput {
    pub m_from: hkVector4f,
    pub m_to: hkVector4f,
    pub m_filterInfo: u32,
    pub m_rayShapeCollectionFilter: *const hkpRayShapeCollectionFilter,
    pub m_collidable: *const hkpCollidable,
    pub m_userData: u32,
}

#[repr(C)]
pub struct hkpShapeContainer {
    pub __vfptr: *const hkpShapeContainer____vftable,
}

#[repr(C)]
pub struct hkpShapeContainer____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpShapeContainer, _: u32) -> *mut (),
    pub getNumChildShapes: unsafe extern "thiscall" fn(this: *const hkpShapeContainer) -> i32,
    pub getFirstKey: unsafe extern "thiscall" fn(this: *const hkpShapeContainer) -> u32,
    pub getNextKey: unsafe extern "thiscall" fn(this: *const hkpShapeContainer, _: u32) -> u32,
    pub getCollisionFilterInfo:
        unsafe extern "thiscall" fn(this: *const hkpShapeContainer, _: u32) -> u32,
    pub getChildShape: unsafe extern "thiscall" fn(
        this: *const hkpShapeContainer,
        _: u32,
        _: *mut hkpShapeBufferStorage,
    ) -> *const hkpShape,
    pub isWeldingEnabled: unsafe extern "thiscall" fn(this: *const hkpShapeContainer) -> bool,
}

#[repr(C)]
pub struct hkpBvTreeShape {
    pub __vfptr: *const hkpBvTreeShape____vftable,
    // hkBaseObject
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
}

impl AsPtr<hkpShape> for *const hkpBvTreeShape {
    fn as_ptr(self) -> *const hkpShape {
        self as *const _
    }
}

impl AsPtr<hkpShapeBase> for *const hkpBvTreeShape {
    fn as_ptr(self) -> *const hkpShapeBase {
        self as *const _
    }
}

impl AsPtr<hkcdShape> for *const hkpBvTreeShape {
    fn as_ptr(self) -> *const hkcdShape {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpBvTreeShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBvTreeShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpBvTreeShape____vftable {
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
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
    pub queryAabb: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkAabb,
        _: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ),
    pub castAabbImpl: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkAabb,
        _: *const hkVector4f,
        _: *mut hkpAabbCastCollector,
    ),
    pub queryAabbImpl: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkAabb,
        _: *mut u32,
        _: i32,
    ) -> u32,
}

#[repr(C)]
pub struct hkEnum_enum_hkpBvTreeShape__BvTreeType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpBroadPhase {
    pub __vfptr: *const hkpBroadPhase____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBroadPhase
    pub m_type: u16,
    pub m_size: u16,
    pub m_caps: u32,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_criticalSection: *mut hkCriticalSection,
}

impl AsPtr<hkReferencedObject> for *const hkpBroadPhase {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBroadPhase {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpBroadPhase____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub getType:
        unsafe extern "thiscall" fn(this: *const hkpBroadPhase) -> hkpBroadPhase__BroadPhaseType,
    pub getCapabilityDelegate: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: hkpBroadPhase__Capabilities,
    ) -> *const hkpBroadPhase,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *mut hkpBroadPhaseHandle,
        _: *const hkAabb,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        _: bool,
    ),
    pub addObject_2: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *mut hkpBroadPhaseHandle,
        _: *const hkAabbUint32,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        _: bool,
    ),
    pub addObjectBatch: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *const hkArrayBase_hkpBroadPhaseHandle___,
        _: *const hkArrayBase_hkAabb_,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *mut hkpBroadPhaseHandle,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub removeObjectBatch: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *const hkArrayBase_hkpBroadPhaseHandle___,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub getNumObjects: unsafe extern "thiscall" fn(this: *const hkpBroadPhase) -> i32,
    pub updateAabbs: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *mut *mut hkpBroadPhaseHandle,
        _: *const hkAabb,
        _: i32,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub updateAabbsUint32: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *mut *mut hkpBroadPhaseHandle,
        _: *const hkAabbUint32,
        _: i32,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub defragment: unsafe extern "thiscall" fn(this: *mut hkpBroadPhase),
    pub checkDeterminism: unsafe extern "thiscall" fn(this: *mut hkpBroadPhase),
    pub getAllAabbs: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *mut hkArray_hkAabb_hkContainerHeapAllocator_,
    ),
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkpBroadPhaseHandle,
        _: *mut hkAabb,
    ),
    pub getExtents: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *mut hkVector4f,
        _: *mut hkVector4f,
    ),
    pub querySingleAabb: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkAabb,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub reQuerySingleObject: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkpBroadPhaseHandle,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub querySingleAabbWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkAabb,
        _: *mut hkpBroadPhaseCastCollector,
    ),
    pub areAabbsOverlapping: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkpBroadPhaseHandle,
        _: *const hkpBroadPhaseHandle,
    ) -> bool,
    pub shiftAllObjects: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *const hkVector4f,
        _: *mut hkVector4f,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub shiftBroadPhase: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *const hkVector4f,
        _: *mut hkVector4f,
        _: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ),
    pub getOffsetLowHigh32bit: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *mut hkVector4f,
        _: *mut hkVector4f,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkpBroadPhase__hkpCastRayInput,
        _: *mut hkpBroadPhaseCastCollector,
        _: i32,
    ),
    pub getAabbCacheSize: unsafe extern "thiscall" fn(this: *const hkpBroadPhase) -> i32,
    pub calcAabbCache: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkArrayBase_hkpCollidable___,
        _: *mut i8,
    ),
    pub calcAabbCache_2:
        unsafe extern "thiscall" fn(this: *const hkpBroadPhase, _: *const hkAabb, _: *mut i8),
    pub castAabb: unsafe extern "thiscall" fn(
        this: *const hkpBroadPhase,
        _: *const hkpBroadPhase__hkpCastAabbInput,
        _: *mut hkpBroadPhaseCastCollector,
    ),
    pub set32BitOffsetAndScale: unsafe extern "thiscall" fn(
        this: *mut hkpBroadPhase,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
}

#[repr(C)]
pub struct hkpBroadPhaseCastCollector {
    pub __vfptr: *const hkpBroadPhaseCastCollector____vftable,
}

#[repr(C)]
pub struct hkpBroadPhaseCastCollector____vftable {
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
    pub __vfptr: *const hkpCollisionFilter____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpCollidableCollidableFilter____vftable,
    // hkpCollidableCollidableFilter
    pub __vfptr_3: *const hkpShapeCollectionFilter____vftable,
    // hkpShapeCollectionFilter
    pub __vfptr_4: *const hkpRayShapeCollectionFilter____vftable,
    // hkpRayShapeCollectionFilter
    pub __vfptr_5: *const hkpRayCollidableFilter____vftable,
    // hkpRayCollidableFilter
    // hkpCollisionFilter
    pub m_prepad: [u32; 2],
    pub m_type: hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_,
    pub m_postpad: [u32; 3],
}

impl AsPtr<hkReferencedObject> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

impl AsPtr<hkpCollidableCollidableFilter> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkpCollidableCollidableFilter {
        (self as usize + 0x8) as *const _
    }
}

impl AsPtr<hkpShapeCollectionFilter> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkpShapeCollectionFilter {
        (self as usize + 0xc) as *const _
    }
}

impl AsPtr<hkpRayShapeCollectionFilter> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkpRayShapeCollectionFilter {
        (self as usize + 0x10) as *const _
    }
}

impl AsPtr<hkpRayCollidableFilter> for *const hkpCollisionFilter {
    fn as_ptr(self) -> *const hkpRayCollidableFilter {
        (self as usize + 0x14) as *const _
    }
}

#[repr(C)]
pub struct hkpCollisionFilter____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpRayCollidableFilter, _: u32) -> *mut (),
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpRayCollidableFilter,
        result: *mut hkBool,
        _: *const hkpWorldRayCastInput,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
    pub numShapeKeyHitsLimitBreached: unsafe extern "thiscall" fn(
        this: *const hkpShapeCollectionFilter,
        _: *const hkpCollisionInput,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpBvTreeShape,
        _: *mut hkAabb,
        _: *mut u32,
        _: i32,
    ) -> i32,
    pub __vecDelDtor_2:
        unsafe extern "thiscall" fn(this: *mut hkpShapeCollectionFilter, _: u32) -> *mut (),
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
    pub __vfptr: *const hkpSphereRepShape____vftable,
    // hkBaseObject
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

impl AsPtr<hkpShape> for *const hkpSphereRepShape {
    fn as_ptr(self) -> *const hkpShape {
        self as *const _
    }
}

impl AsPtr<hkpShapeBase> for *const hkpSphereRepShape {
    fn as_ptr(self) -> *const hkpShapeBase {
        self as *const _
    }
}

impl AsPtr<hkcdShape> for *const hkpSphereRepShape {
    fn as_ptr(self) -> *const hkcdShape {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpSphereRepShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpSphereRepShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkpSphereRepShape> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkpSphereRepShape {
        self as *const _
    }
}

impl AsPtr<hkpShape> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkpShape {
        self as *const _
    }
}

impl AsPtr<hkpShapeBase> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkpShapeBase {
        self as *const _
    }
}

impl AsPtr<hkcdShape> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkcdShape {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpConvexShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub m_from: hkFourTransposedPointsf,
    pub m_to: hkFourTransposedPointsf,
    pub m_filterInfo: u32,
    pub m_userData: u32,
    pub m_rayShapeCollectionFilter: *const hkpRayShapeCollectionFilter,
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct hkpAabbCastCollector {
    pub __vfptr: *const hkpAabbCastCollector____vftable,
    __pdbindgen_padding: [u8; 12],
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConvexListFilter
}

impl AsPtr<hkReferencedObject> for *const hkpConvexListFilter {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpConvexListFilter {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkpWorldObject> for *const hkpPhantom {
    fn as_ptr(self) -> *const hkpWorldObject {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpPhantom {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpPhantom {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub __vfptr: *const hkpWorld____vftable,
    // hkBaseObject
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_broadPhaseExtents: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_3: [u8; 32],
    pub m_broadPhaseNumMarkers: i32,
    pub m_sizeOfToiEventQueue: i32,
    pub m_broadPhaseQuerySize: i32,
    pub m_broadPhaseUpdateSize: i32,
    pub m_contactPointGeneration: hkEnum_enum_hkpWorldCinfo__ContactPointGeneration_signed_char_,
    pub m_useCompoundSpuElf: hkBool,
    __pdbindgen_padding_4: [u8; 14],
}

impl AsPtr<hkReferencedObject> for *const hkpWorld {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpWorld {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub m_stepInfo: hkStepInfo,
    pub m_solverInfo: hkpSolverInfo,
}

#[repr(C)]
pub struct hkEnum_enum_hkpWorldCinfo__SimulationType_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpRigidBody {
    pub __vfptr: *const hkpRigidBody____vftable,
    // hkBaseObject
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
    // hkpRigidBody
}

impl AsPtr<hkpEntity> for *const hkpRigidBody {
    fn as_ptr(self) -> *const hkpEntity {
        self as *const _
    }
}

impl AsPtr<hkpWorldObject> for *const hkpRigidBody {
    fn as_ptr(self) -> *const hkpWorldObject {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpRigidBody {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpRigidBody {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub __vfptr: *const hkpSimulationIsland____vftable,
    // hkBaseObject
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

impl AsPtr<hkpConstraintOwner> for *const hkpSimulationIsland {
    fn as_ptr(self) -> *const hkpConstraintOwner {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpSimulationIsland {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpSimulationIsland {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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

impl AsPtr<hkpCollisionEvent> for *const hkpContactPointEvent {
    fn as_ptr(self) -> *const hkpCollisionEvent {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpDynamicsContactMgr {
    pub __vfptr: *const hkpDynamicsContactMgr____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
    // hkpDynamicsContactMgr
    pub m_world: *mut hkpWorld,
}

impl AsPtr<hkpContactMgr> for *const hkpDynamicsContactMgr {
    fn as_ptr(self) -> *const hkpContactMgr {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpDynamicsContactMgr {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpDynamicsContactMgr {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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

impl AsPtr<hkpConstraintAtom> for *const hkpModifierConstraintAtom {
    fn as_ptr(self) -> *const hkpConstraintAtom {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr {
    pub __vfptr: *const hkpSimpleConstraintContactMgr____vftable,
    // hkBaseObject
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

impl AsPtr<hkpDynamicsContactMgr> for *const hkpSimpleConstraintContactMgr {
    fn as_ptr(self) -> *const hkpDynamicsContactMgr {
        self as *const _
    }
}

impl AsPtr<hkpContactMgr> for *const hkpSimpleConstraintContactMgr {
    fn as_ptr(self) -> *const hkpContactMgr {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpSimpleConstraintContactMgr {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpSimpleConstraintContactMgr {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
pub struct hkEnum_enum_hkpContactPointAddedEvent__Type_int_ {
    pub m_storage: i32,
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

impl AsPtr<hkpCollisionInput> for *const hkpLinearCastCollisionInput {
    fn as_ptr(self) -> *const hkpCollisionInput {
        self as *const _
    }
}

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

impl AsPtr<hkpShapeRayCastCollectorOutput> for *const hkpShapeRayCastOutput {
    fn as_ptr(self) -> *const hkpShapeRayCastCollectorOutput {
        self as *const _
    }
}

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
    pub __vfptr: *const hkThreadPool____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkThreadPool
}

impl AsPtr<hkReferencedObject> for *const hkThreadPool {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkThreadPool {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkpProcessCollisionData
    pub m_firstFreeContactPoint: hkPadSpu_hkpProcessCdPoint___,
    pub m_constraintOwner: hkPadSpu_hkpConstraintOwner___,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_contactPoints: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 12296],
    pub m_toi: hkpProcessCollisionData__ToiInfo,
    // hkpProcessCollisionOutput
    pub m_potentialContacts: hkPadSpu_hkpProcessCollisionOutput__PotentialInfo___,
    __pdbindgen_padding_2: [u8; 12],
}

impl AsPtr<hkpProcessCollisionData> for *const hkpProcessCollisionOutput {
    fn as_ptr(self) -> *const hkpProcessCollisionData {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpGskCache {
    pub m_vertices: [u16; 4],
    pub m_dimA: u8,
    pub m_dimB: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimA: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimB: compile_error!("unimplemented feature: type kind 0x1205"),
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

impl AsPtr<hkpCdBody> for *const hkpCollidable {
    fn as_ptr(self) -> *const hkpCdBody {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableMaterial
    pub m_strength: f32,
    pub m_typeAndFlags: i32,
    pub m_properties: *mut hkRefCountedProperties,
}

impl AsPtr<hkReferencedObject> for *const hkpBreakableMaterial {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBreakableMaterial {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableShape
    pub m_physicsShape: hkRefPtr_hkcdShape_const__,
    pub m_material: hkRefPtr_hkpBreakableMaterial_,
}

impl AsPtr<hkReferencedObject> for *const hkpBreakableShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBreakableShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableBody
    pub m_controller: hkRefPtr_hkpBreakableBody__Controller_,
    pub m_breakableShape: hkRefPtr_hkpBreakableShape_const__,
    pub m_bodyTypeAndFlags: u8,
    pub m_constraintStrength: hkHalf,
}

impl AsPtr<hkReferencedObject> for *const hkpBreakableBody {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBreakableBody {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableBody__Controller
    pub m_breakingImpulse: f32,
}

impl AsPtr<hkReferencedObject> for *const hkpBreakableBody__Controller {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBreakableBody__Controller {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkArrayBase_hkpBodyOperationEntry_
    pub m_data: *mut hkpBodyOperationEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_
}

impl AsPtr<hkArrayBase_hkpBodyOperationEntry_>
    for *const hkArray_hkpBodyOperationEntry_hkContainerHeapAllocator_
{
    fn as_ptr(self) -> *const hkArrayBase_hkpBodyOperationEntry_ {
        self as *const _
    }
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

impl AsPtr<hkArrayBase_hkWorldOperation__BiggestOperation_>
    for *const hkArray_hkWorldOperation__BiggestOperation_hkContainerHeapAllocator_
{
    fn as_ptr(self) -> *const hkArrayBase_hkWorldOperation__BiggestOperation_ {
        self as *const _
    }
}

#[repr(C)]
pub struct hkWorldOperation__BiggestOperation {
    // hkWorldOperation__BaseOperation
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
    // hkWorldOperation__BiggestOperation
    pub dummy: [u32; 7],
}

impl AsPtr<hkWorldOperation__BaseOperation> for *const hkWorldOperation__BiggestOperation {
    fn as_ptr(self) -> *const hkWorldOperation__BaseOperation {
        self as *const _
    }
}

#[repr(C)]
pub struct hkWorldOperation__BaseOperation {
    pub m_type: hkEnum_enum_hkWorldOperation__Type_unsigned_char_,
}

#[repr(C)]
pub struct hkpBodyOperation {
    pub __vfptr: *const hkpBodyOperation____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBodyOperation
}

impl AsPtr<hkReferencedObject> for *const hkpBodyOperation {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBodyOperation {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkReferencedObject> for *const hkpSimulation {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpSimulation {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintTrackerData
}

impl AsPtr<hkReferencedObject> for *const hkpConstraintTrackerData {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpConstraintTrackerData {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub __vfptr: *const hkpCollisionDispatcher____vftable,
    // hkBaseObject
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

impl AsPtr<hkReferencedObject> for *const hkpCollisionDispatcher {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpCollisionDispatcher {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpWorldDeletionListener____vftable,
    // hkpWorldDeletionListener
    pub __vfptr_3: *const hkpPhantomOverlapListener____vftable,
    // hkpPhantomOverlapListener
    pub __vfptr_4: *const hkpWorldPostSimulationListener____vftable,
    // hkpWorldPostSimulationListener
    // hkpBroadPhaseBorder
    pub m_world: *mut hkpWorld,
    pub m_phantoms: [*mut hkpPhantom; 6],
    pub m_type: hkpWorldCinfo__BroadPhaseBorderBehaviour,
    pub m_postponeAndSortCallbacks: hkBool,
    pub m_entitiesExitingBroadPhase: hkArray_hkpEntity___hkContainerHeapAllocator_,
}

impl AsPtr<hkReferencedObject> for *const hkpBroadPhaseBorder {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBroadPhaseBorder {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

impl AsPtr<hkpWorldDeletionListener> for *const hkpBroadPhaseBorder {
    fn as_ptr(self) -> *const hkpWorldDeletionListener {
        (self as usize + 0x8) as *const _
    }
}

impl AsPtr<hkpPhantomOverlapListener> for *const hkpBroadPhaseBorder {
    fn as_ptr(self) -> *const hkpPhantomOverlapListener {
        (self as usize + 0xc) as *const _
    }
}

impl AsPtr<hkpWorldPostSimulationListener> for *const hkpBroadPhaseBorder {
    fn as_ptr(self) -> *const hkpWorldPostSimulationListener {
        (self as usize + 0x10) as *const _
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorder____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: u32) -> *mut (),
    pub postSimulationCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpWorld),
    pub inactiveEntityMovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldPostSimulationListener, _: *mut hkpEntity),
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpContinuousSimulation____vftable,
    // hkBaseObject
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

impl AsPtr<hkpSimulation> for *const hkpContinuousSimulation {
    fn as_ptr(self) -> *const hkpSimulation {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpContinuousSimulation {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpContinuousSimulation {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkArrayBase_hkpToiEvent_
    pub m_data: *mut hkpToiEvent,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpToiEvent_hkContainerHeapAllocator_
}

impl AsPtr<hkArrayBase_hkpToiEvent_> for *const hkArray_hkpToiEvent_hkContainerHeapAllocator_ {
    fn as_ptr(self) -> *const hkArrayBase_hkpToiEvent_ {
        self as *const _
    }
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

impl AsPtr<hkArrayBase_short_> for *const hkArray_short_hkContainerHeapAllocator_ {
    fn as_ptr(self) -> *const hkArrayBase_short_ {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpBroadPhaseListener____vftable,
    /* hkpBroadPhaseListener
     * hkpPhantomBroadPhaseListener */
}

impl AsPtr<hkReferencedObject> for *const hkpPhantomBroadPhaseListener {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpPhantomBroadPhaseListener {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

impl AsPtr<hkpBroadPhaseListener> for *const hkpPhantomBroadPhaseListener {
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        (self as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct hkpPhantomBroadPhaseListener____vftable {
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
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpContactMgr {
    pub __vfptr: *const hkpContactMgr____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
}

impl AsPtr<hkReferencedObject> for *const hkpContactMgr {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpContactMgr {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub __vfptr: *const hkpWorldExtension____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldExtension
    pub m_world: *mut hkpWorld,
    pub m_id: i32,
    pub m_attachmentCount: u16,
}

impl AsPtr<hkReferencedObject> for *const hkpWorldExtension {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpWorldExtension {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkpContinuousSimulation> for *const hkpMultiThreadedSimulation {
    fn as_ptr(self) -> *const hkpContinuousSimulation {
        self as *const _
    }
}

impl AsPtr<hkpSimulation> for *const hkpMultiThreadedSimulation {
    fn as_ptr(self) -> *const hkpSimulation {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpMultiThreadedSimulation {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpMultiThreadedSimulation {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkpBroadPhaseListener
    // hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener
    pub m_simulation: *mut hkpMultiThreadedSimulation,
}

impl AsPtr<hkpBroadPhaseListener>
    for *const hkpMultiThreadedSimulation__MtEntityEntityBroadPhaseListener
{
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        self as *const _
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
    // hkpBroadPhaseListener
    // hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener
    pub m_criticalSection: *mut hkCriticalSection,
}

impl AsPtr<hkpBroadPhaseListener>
    for *const hkpMultiThreadedSimulation__MtPhantomBroadPhaseListener
{
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        self as *const _
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
    // hkpBroadPhaseListener
    // hkpMultiThreadedSimulation__MtBroadPhaseBorderListener
    pub m_criticalSection: *mut hkCriticalSection,
}

impl AsPtr<hkpBroadPhaseListener>
    for *const hkpMultiThreadedSimulation__MtBroadPhaseBorderListener
{
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        self as *const _
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
    // hkArrayBase_hkpTypedBroadPhaseHandlePair_
    pub m_data: *mut hkpTypedBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_
}

impl AsPtr<hkArrayBase_hkpTypedBroadPhaseHandlePair_>
    for *const hkArray_hkpTypedBroadPhaseHandlePair_hkContainerHeapAllocator_
{
    fn as_ptr(self) -> *const hkArrayBase_hkpTypedBroadPhaseHandlePair_ {
        self as *const _
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
    pub __vfptr: *const hkpWorldMaintenanceMgr____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpWorldMaintenanceMgr
}

impl AsPtr<hkReferencedObject> for *const hkpWorldMaintenanceMgr {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpWorldMaintenanceMgr {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpBroadPhaseListener____vftable,
    // hkpBroadPhaseListener
    // hkpEntityEntityBroadPhaseListener
    pub m_world: *mut hkpWorld,
}

impl AsPtr<hkReferencedObject> for *const hkpEntityEntityBroadPhaseListener {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpEntityEntityBroadPhaseListener {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

impl AsPtr<hkpBroadPhaseListener> for *const hkpEntityEntityBroadPhaseListener {
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        (self as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct hkpEntityEntityBroadPhaseListener____vftable {
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
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener {
    pub __vfptr: *const hkpBroadPhaseBorderListener____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpBroadPhaseListener____vftable,
    /* hkpBroadPhaseListener
     * hkpBroadPhaseBorderListener */
}

impl AsPtr<hkReferencedObject> for *const hkpBroadPhaseBorderListener {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpBroadPhaseBorderListener {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

impl AsPtr<hkpBroadPhaseListener> for *const hkpBroadPhaseBorderListener {
    fn as_ptr(self) -> *const hkpBroadPhaseListener {
        (self as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct hkpBroadPhaseBorderListener____vftable {
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
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpToiResourceMgr {
    pub __vfptr: *const hkpToiResourceMgr____vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpToiResourceMgr
}

impl AsPtr<hkReferencedObject> for *const hkpToiResourceMgr {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpToiResourceMgr {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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

impl AsPtr<hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_>
    for *const hkArray_hkpToiResourceMgr__ConstraintViolationInfo_hkContainerHeapAllocator_
{
    fn as_ptr(self) -> *const hkArrayBase_hkpToiResourceMgr__ConstraintViolationInfo_ {
        self as *const _
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
    pub __vfptr: *const hkpEntity____vftable,
    // hkBaseObject
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

impl AsPtr<hkpWorldObject> for *const hkpEntity {
    fn as_ptr(self) -> *const hkpWorldObject {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpEntity {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpEntity {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkReferencedObject> for *const hkpConstraintInstance {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpConstraintInstance {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkcdShape> for *const hkpShapeBase {
    fn as_ptr(self) -> *const hkcdShape {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpShapeBase {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpShapeBase {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    pub __vfptr: *const hkpMotion____vftable,
    // hkBaseObject
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

impl AsPtr<hkReferencedObject> for *const hkpMotion {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpMotion {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
    }
}

#[repr(C)]
pub struct hkpMotion____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkpWorldObject____vftable,
    // hkBaseObject
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

impl AsPtr<hkReferencedObject> for *const hkpWorldObject {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpWorldObject {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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
    // hkBaseObject
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

impl AsPtr<hkpShapeBase> for *const hkpShape {
    fn as_ptr(self) -> *const hkpShapeBase {
        self as *const _
    }
}

impl AsPtr<hkcdShape> for *const hkpShape {
    fn as_ptr(self) -> *const hkcdShape {
        self as *const _
    }
}

impl AsPtr<hkReferencedObject> for *const hkpShape {
    fn as_ptr(self) -> *const hkReferencedObject {
        self as *const _
    }
}

impl AsPtr<hkBaseObject> for *const hkpShape {
    fn as_ptr(self) -> *const hkBaseObject {
        self as *const _
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

pub type kaiko__LocalGameSessionInteractionId = i32;

pub type keen__PixelFormat = i32;

pub type keen__IoError = i32;

pub type DXGI_FORMAT = i32;

pub type D3D_NAME = i32;

pub type D3D_REGISTER_COMPONENT_TYPE = i32;

pub type keen__PrimitiveType = i32;

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
pub struct hkPlacementNewArg {
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
