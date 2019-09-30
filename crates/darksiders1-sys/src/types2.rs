#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{
    types::*,
    types10::*,
    types11::*,
    types12::*,
    types13::*,
    types3::*,
    types4::*,
    types5::*,
    types6::*,
    types7::*,
    types8::*,
    types9::*,
};

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__TrailParticle_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ModuleEventLink,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VariableConnectionInfo_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__SpriteParticle_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__Particle {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field FlipX")]
#[repr(C)]
pub struct gfc__Particle {
    pub Position: [f32; 3],
    pub Velocity: [f32; 3],
    pub SpinRate: f32,
    pub SpinAngle: f32,
    pub Age: f32,
    pub Life: f32,
    pub Size: f32,
    pub Mass: f32,
    pub EmitterInst: *mut gfc__EmitterInstance,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub FlipX: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub FlipY: compile_error!("unimplemented feature: type kind 0x1205"),
}

#[repr(C)]
pub struct gfc__Emitter {
    pub __vfptr: *const gfc__Emitter____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
    pub mRefNode: gfc__HString,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mRaycastDir: gfc__TVector3_float_gfc__FloatMath_,
    pub mMaxRaycastDist: f32,
    pub mStartDelay: f32,
    pub mDuration: f32,
    pub mLoopCount: i32,
    pub mParticleSystem: *mut gfc__PSystem,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mPackageID: i32,
    pub mWasTouched: bool,
}

impl gfc__Emitter {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Emitter____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: i32, _: *mut gfc__PSystem),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Emitter),
    pub createSceneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
        _: gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_,
    ) -> *mut gfc__EmitterSceneObject,
    pub setupEmitter:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *mut gfc__EmitterInstance),
    pub updateEmitterST:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *mut gfc__EmitterInstance, _: f32),
    pub updateEmitter:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *mut gfc__EmitterInstance, _: f32),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
        _: *mut gfc__EmitterInstance,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
        _: *mut gfc__EmitterInstance,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub prepGeometry: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
        _: *mut gfc__EmitterInstance,
        _: *mut gfc__RenderNode,
    ),
    pub onDestroy:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *mut gfc__EmitterInstance, _: bool),
    pub convertEmitter: unsafe extern "thiscall" fn(this: *mut gfc__Emitter) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__CinematicGroup__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CinematicGroup_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__SceneLight___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneLight,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldComponent_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DirectorCinematicGroup_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ResourceListener {
    pub __vfptr: *const gfc__ResourceListener____vftable,
}

#[repr(C)]
pub struct gfc__ResourceListener____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: u32) -> *mut (),
    pub packageUnloading: unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: i32),
}

#[repr(C)]
pub struct gfc__Vector_gfc__SceneOccluder___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneOccluder,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Object3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ModuleInputLink,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__SceneObject {
    pub __vfptr: *const gfc__SceneObject____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
}

#[repr(C)]
pub struct gfc__SceneObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ModuleVariableLink,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__VisScriptModule__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__VisScriptModule_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__VisScriptEntity__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__VisScriptEntity_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TrailParticle {
    pub Position: [f32; 3],
    pub Normal: [f32; 3],
    pub P1: [f32; 3],
    pub T1: [f32; 3],
    pub P2: [f32; 3],
    pub T2: [f32; 3],
    pub Age: f32,
    pub Life: f32,
    pub FracLength: f32,
    pub EmitterInst: *mut gfc__EmitterInstance,
    pub Prev: gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_,
    pub Next: gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_,
}

#[repr(C)]
pub struct gfc__SceneManager {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mClipper")]
#[repr(C)]
pub struct gfc__SceneManager {
    pub __vfptr: *const gfc__SceneManager____vftable,
    pub ReferenceCount: i32,
    pub mDrawCounter: u32,
    pub mPortalDrawCounter: u32,
    pub mInsideOutside: i32,
    pub mRootCell: *mut gfc__SceneCell,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mPortals: gfc__Vector_gfc__ScenePortal___0_gfc__CAllocator_,
    pub mOccluders: gfc__Vector_gfc__SceneOccluder___0_gfc__CAllocator_,
    pub mObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mLights: gfc__Vector_gfc__SceneLight___0_gfc__CAllocator_,
    pub mDynamicObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mHPDynamicObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mRelevantObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mJobStack: keen__FixedSizedArray_gfc__SceneManager__DistributedCulling_1024_,
    #[cfg(pdb_issue = "error in gfc__Clipper")]
    pub mClipper: gfc__Clipper,
    pub mLightIDGen: u32,
    pub mDynamicVisIndex: u32,
    pub mLightVisCalculated: bool,
    pub mDefaultObserver: *mut gfc__SceneObserver,
}

impl gfc__SceneManager {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__SceneManager____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub cull: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneManager,
        _: *const gfc__Matrix4,
        _: *const gfc__Frustum,
        _: *mut gfc__Camera3D,
        _: bool,
        _: *mut gfc__SceneObserver,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub startCullingJobs: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneManager,
        _: *mut gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
        _: *const gfc__Clipper,
        _: i32,
    ),
    pub waitForCullingJobs: unsafe extern "thiscall" fn(this: *mut gfc__SceneManager),
    pub __: *const (),
    pub ___2: *const (),
}

#[repr(C)]
pub struct gfc__EmitterSceneObject {
    pub __vfptr: *const gfc__EmitterSceneObject____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__EmitterSceneObject____vftable,
    pub mLocked: bool,
    pub mBVolume: gfc__BoundingVolume,
    pub mEmitter: *mut gfc__Emitter,
    pub mEmitterHandle: gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_,
    pub mEmitterInstance: *mut gfc__EmitterInstance,
    pub mHide: bool,
}

impl gfc__EmitterSceneObject {
    pub fn as_gfc__SceneObject_ptr(&self) -> *const gfc__SceneObject {
        self as *const _ as _
    }

    pub fn as_gfc__SceneObject_mut_ptr(&mut self) -> *mut gfc__SceneObject {
        self as *mut _ as _
    }

    pub fn as_gfc__IRenderCallback_ptr(&self) -> *const gfc__IRenderCallback {
        self as *const _ as _
    }

    pub fn as_gfc__IRenderCallback_mut_ptr(&mut self) -> *mut gfc__IRenderCallback {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__EmitterSceneObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__Cinematic {
    pub __vfptr: *const gfc__Cinematic____vftable,
    pub ReferenceCount: i32,
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
    pub mHideUI: bool,
    pub mDisableChronomancer: bool,
    pub mJump: bool,
    pub mJumpTime: f32,
    pub mCancelled: bool,
    pub mExecuteZeroTime: bool,
    pub mGizmo: *mut gfc__IconGizmo,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mDirector: gfc__AutoRef_gfc__DirectorCinematicGroup_,
    pub mGroups: gfc__Vector_gfc__AutoRef_gfc__CinematicGroup__0_gfc__CAllocator_,
    pub mCameraGroups: gfc__Vector_gfc__AutoRef_gfc__CameraCinematicGroup__0_gfc__CAllocator_,
    pub mFullScreenFXGroups: gfc__Vector_gfc__AutoRef_gfc__FullScreenFXGroup__0_gfc__CAllocator_,
    pub mCurrentTime: f32,
    pub mStopTime: f32,
    pub mRenderer: *mut gfc__Renderer,
    pub mVisualScriptSystem: *mut gfc__ModuleSystem,
    pub mNotifyCompleteObjects: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
}

impl gfc__Cinematic {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Cinematic____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VisScriptModule_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RigidBody_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ModuleEventLink {
    pub EventID: u32,
    pub ModuleID: u32,
    pub ActionID: u32,
    pub _Module: *mut gfc__VisScriptModule,
}

#[repr(C)]
pub struct gfc__LockFreePoolMarker_gfc__MeshParticle_ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field State")]
#[repr(C)]
pub struct gfc__LockFreePoolMarker_gfc__MeshParticle_ {
    pub Object: *mut gfc__MeshParticle,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub State: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct gfc__Vector_gfc__ScenePortal___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ScenePortal,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpAgentNnSector,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    pub m_storage: [*mut hkpAgentNnSector; 1],
}

impl hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_ {
    pub fn as_hkArray_hkpAgentNnSector___hkContainerHeapAllocator__ptr(
        &self,
    ) -> *const hkArray_hkpAgentNnSector___hkContainerHeapAllocator_ {
        self as *const _ as _
    }

    pub fn as_hkArray_hkpAgentNnSector___hkContainerHeapAllocator__mut_ptr(
        &mut self,
    ) -> *mut hkArray_hkpAgentNnSector___hkContainerHeapAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkInplaceArray_char_128_hkContainerTempAllocator_ {
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    pub m_storage: [i8; 128],
}

impl hkInplaceArray_char_128_hkContainerTempAllocator_ {
    pub fn as_hkArray_char_hkContainerTempAllocator__ptr(
        &self,
    ) -> *const hkArray_char_hkContainerTempAllocator_ {
        self as *const _ as _
    }

    pub fn as_hkArray_char_hkContainerTempAllocator__mut_ptr(
        &mut self,
    ) -> *mut hkArray_char_hkContainerTempAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkContactPointPropertiesWithExtendedUserData16 {
    pub m_impulseApplied: f32,
    pub m_internalSolverData: f32,
    pub m_userData: u32,
    pub m_friction: hkUFloat8,
    pub m_restitution: u8,
    pub m_maxImpulse: hkUFloat8,
    pub m_flags: u8,
    pub m_internalDataA: f32,
    pub m_extendedUserDatas: [u32; 7],
}

impl hkContactPointPropertiesWithExtendedUserData16 {
    pub fn as_hkpContactPointProperties_ptr(&self) -> *const hkpContactPointProperties {
        self as *const _ as _
    }

    pub fn as_hkpContactPointProperties_mut_ptr(&mut self) -> *mut hkpContactPointProperties {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkPadSpu_hkTransformf_const___ {
    pub m_storage: *const hkTransformf,
}

#[repr(C)]
pub struct hkpSolverResults {
    pub m_impulseApplied: f32,
    pub m_internalSolverData: f32,
}

#[repr(C)]
pub struct hkArray_hkpIslandActivationListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpIslandActivationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpIslandActivationListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpIslandActivationListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpIslandActivationListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpIslandActivationListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpIslandActivationListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpAction___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpAction,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpAction___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpAction____ptr(&self) -> *const hkArrayBase_hkpAction___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpAction____mut_ptr(&mut self) -> *mut hkArrayBase_hkpAction___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_int_hkContainerHeapAllocator_ {
    pub m_data: *mut i32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_int_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_int__ptr(&self) -> *const hkArrayBase_int_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_int__mut_ptr(&mut self) -> *mut hkArrayBase_int_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_ {
    pub m_data: *mut hkArray_int_hkContainerHeapAllocator_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkArray_int_hkContainerHeapAllocator____ptr(
        &self,
    ) -> *const hkArrayBase_hkArray_int_hkContainerHeapAllocator___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkArray_int_hkContainerHeapAllocator____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkArray_int_hkContainerHeapAllocator___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkpPhantomOverlapListener___ {
    pub m_data: *mut *mut hkpPhantomOverlapListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpBroadPhaseHandle {
    pub m_id: u32,
}

#[repr(C)]
pub struct hkpDynamicsCpIdMgr {
    pub m_values: hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_,
}

#[repr(C)]
pub struct hkPadSpu_hkpConstraintOwner___ {
    pub m_storage: *mut hkpConstraintOwner,
}

#[repr(C)]
pub struct hkArray_hkpIslandPostCollideListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpIslandPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpIslandPostCollideListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpIslandPostCollideListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpIslandPostCollideListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpIslandPostCollideListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpIslandPostCollideListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrameCollector {
    pub __vfptr: *const hkLocalFrameCollector____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkLocalFrameCollector {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrameCollector____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
    pub addFrame: unsafe extern "thiscall" fn(
        this: *mut hkLocalFrameCollector,
        _: *const hkLocalFrame,
        _: f32,
    ),
}

#[repr(C)]
pub struct hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpEntity,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    pub m_storage: [*mut hkpEntity; 1],
}

impl hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_ {
    pub fn as_hkArray_hkpEntity___hkContainerHeapAllocator__ptr(
        &self,
    ) -> *const hkArray_hkpEntity___hkContainerHeapAllocator_ {
        self as *const _ as _
    }

    pub fn as_hkArray_hkpEntity___hkContainerHeapAllocator__mut_ptr(
        &mut self,
    ) -> *mut hkArray_hkpEntity___hkContainerHeapAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_char_ {
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_unsigned_char_hkContainerHeapAllocator_ {
    pub m_data: *mut u8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_unsigned_char_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_unsigned_char__ptr(&self) -> *const hkArrayBase_unsigned_char_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_unsigned_char__mut_ptr(&mut self) -> *mut hkArrayBase_unsigned_char_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpIslandPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpIslandPostIntegrateListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpIslandPostIntegrateListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpIslandPostIntegrateListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpIslandPostIntegrateListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkStepInfo {
    pub m_startTime: hkPadSpu_float_,
    pub m_endTime: hkPadSpu_float_,
    pub m_deltaTime: hkPadSpu_float_,
    pub m_invDeltaTime: hkPadSpu_float_,
}

#[repr(C)]
pub struct hkSimpleProperty {
    pub m_key: u32,
    pub m_alignmentPadding: u32,
    pub m_value: hkSimplePropertyValue,
}

#[repr(C)]
pub struct hkpViolatedConstraintArray {
    pub m_nextFreeElement: u32,
    pub m_constraints: [*mut hkpConstraintInstance; 128],
}

#[repr(C)]
pub struct hkArray_hkpWorldPostCollideListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpWorldPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpWorldPostCollideListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpWorldPostCollideListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpWorldPostCollideListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpWorldPostCollideListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpWorldPostCollideListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollidableCollidableFilter {
    pub __vfptr: *const hkpCollidableCollidableFilter____vftable,
}

#[repr(C)]
pub struct hkpCollidableCollidableFilter____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCollidableCollidableFilter, _: u32) -> *mut (),
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpCollidableCollidableFilter,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
    ) -> hkBool,
}

#[repr(C)]
pub struct hkSmallArray_hkpConstraintListener___ {
    pub m_data: *mut *mut hkpConstraintListener,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkpConstraintQueryStepInfo {
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
}

#[repr(C)]
pub struct hkpAgentEntry {
    pub m_streamCommand: u8,
    pub m_agentType: u8,
    pub m_numContactPoints: u8,
    pub m_size: u8,
}

#[repr(C)]
pub struct hkPadSpu_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpMultithreadConfig {
    pub m_maxNumConstraintsSolvedSingleThreaded: u32,
}

#[repr(C)]
pub struct hkArray_hkpEntity___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpEntity,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpEntity___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpEntity____ptr(&self) -> *const hkArrayBase_hkpEntity___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpEntity____mut_ptr(&mut self) -> *mut hkArrayBase_hkpEntity___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkPadSpu_hkpConstraintInstance___ {
    pub m_storage: *mut hkpConstraintInstance,
}

#[repr(C)]
pub struct hkpShapeModifier {
    pub __vfptr: *const hkpShapeModifier____vftable,
}

#[repr(C)]
pub struct hkpShapeModifier____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpShapeModifier, _: u32) -> *mut (),
    pub modifyShape: unsafe extern "thiscall" fn(this: *mut hkpShapeModifier, _: *mut hkpShape),
}

#[repr(C)]
pub struct hkArrayBase_hkpEntity___ {
    pub m_data: *mut *mut hkpEntity,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpKeyframedRigidMotion {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_deactivationRefPosition")]
#[repr(C)]
pub struct hkpKeyframedRigidMotion {
    pub __vfptr: *const hkpKeyframedRigidMotion____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkpMotion__MotionType_unsigned_char_,
    pub m_deactivationIntegrateCounter: u8,
    pub m_deactivationNumInactiveFrames: [u16; 2],
    pub m_motionState: hkMotionState,
    pub m_inertiaAndMassInv: hkVector4f,
    pub m_linearVelocity: hkVector4f,
    pub m_angularVelocity: hkVector4f,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_deactivationRefPosition: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
}

impl hkpKeyframedRigidMotion {
    pub fn as_hkpMotion_ptr(&self) -> *const hkpMotion {
        self as *const _ as _
    }

    pub fn as_hkpMotion_mut_ptr(&mut self) -> *mut hkpMotion {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpKeyframedRigidMotion____vftable {
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
    pub setStepPosition:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: f32, _: f32),
    pub setStoredMotion:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *mut hkpMaxSizeMotion),
}

#[repr(C)]
pub struct hkArray_hkpPhantomListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpPhantomListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpPhantomListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpPhantomListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpPhantomListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpPhantomListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpPhantomListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct keen__FixedSizedArray_gfc__SceneManager__DistributedCulling_1024_ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_data")]
#[repr(C)]
pub struct keen__FixedSizedArray_gfc__SceneManager__DistributedCulling_1024_ {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_data: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_size: u32,
}

#[repr(C)]
pub struct hkCriticalSection {
    pub m_section: _RTL_CRITICAL_SECTION,
}

#[repr(C)]
pub struct hkpAgentNnEntry {
    pub m_streamCommand: u8,
    pub m_agentType: u8,
    pub m_numContactPoints: u8,
    pub m_size: u8,
    pub m_agentIndexOnCollidable: [u16; 2],
    pub m_contactMgr: *mut hkpContactMgr,
    pub m_collisionQualityIndex: u8,
    pub m_forceCollideOntoPpu: u8,
    pub m_nnTrackType: hkEnum_enum_hkpAgentNnTrackType_unsigned_char_,
    pub m_padding: u8,
    pub m_collidable: [*mut hkpLinkedCollidable; 2],
}

impl hkpAgentNnEntry {
    pub fn as_hkpAgentEntry_ptr(&self) -> *const hkpAgentEntry {
        self as *const _ as _
    }

    pub fn as_hkpAgentEntry_mut_ptr(&mut self) -> *mut hkpAgentEntry {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpWorldDeletionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpWorldDeletionListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpWorldDeletionListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpWorldDeletionListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpWorldDeletionListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkSimpleProperty_hkContainerHeapAllocator_ {
    pub m_data: *mut hkSimpleProperty,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkSimpleProperty_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkSimpleProperty__ptr(&self) -> *const hkArrayBase_hkSimpleProperty_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkSimpleProperty__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkSimpleProperty_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkPadSpu_hkpViolatedConstraintArray___ {
    pub m_storage: *mut hkpViolatedConstraintArray,
}

#[repr(C)]
pub struct hkPadSpu_hkpVelocityAccumulator_const___ {
    pub m_storage: *const hkpVelocityAccumulator,
}

#[repr(C)]
pub struct hkPadSpu_hkpProcessCollisionOutput__PotentialInfo___ {
    pub m_storage: *mut hkpProcessCollisionOutput__PotentialInfo,
}

#[repr(C)]
pub struct hkpSimpleContactConstraintData {
    pub __vfptr: *const hkpSimpleContactConstraintData____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_userData: u32,
    pub m_idMgrA: hkpDynamicsCpIdMgr,
    pub m_clientData: *mut (),
    pub m_constraint: *mut hkpConstraintInstance,
    pub m_atom: *mut hkpSimpleContactConstraintAtom,
    pub m_atomSize: i32,
}

impl hkpSimpleContactConstraintData {
    pub fn as_hkpConstraintData_ptr(&self) -> *const hkpConstraintData {
        self as *const _ as _
    }

    pub fn as_hkpConstraintData_mut_ptr(&mut self) -> *mut hkpConstraintData {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimpleContactConstraintData____vftable {
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
    pub isValid: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> hkBool,
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
    pub setInertiaStabilizationFactor:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: f32) -> hkResult,
    pub getInertiaStabilizationFactor:
        unsafe extern "thiscall" fn(this: *const hkpConstraintData, _: *mut f32) -> hkResult,
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
    pub isBuildJacobianCallbackRequired:
        unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> hkBool,
    pub buildJacobianCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        _: *const hkpConstraintQueryIn,
        _: *const hkpConstraintQueryOut,
    ),
    pub collisionResponseBeginCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
        _: *const hkContactPoint,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub collisionResponseEndCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
        _: *const hkContactPoint,
        _: f32,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
}

#[repr(C)]
pub struct hkConstraintInternal {
    pub m_constraint: *mut hkpConstraintInstance,
    pub m_entities: [*mut hkpEntity; 2],
    pub m_atoms: *mut hkpConstraintAtom,
    pub m_atomsSize: u16,
    pub m_callbackRequest: u8,
    pub m_priority: hkEnum_enum_hkpConstraintInstance__ConstraintPriority_unsigned_char_,
    pub m_numSolverResults: u16,
    pub m_sizeOfSchemas: u32,
    pub m_numSolverElemTemps: u32,
    pub m_whoIsMaster: u8,
    pub m_constraintType: hkEnum_enum_hkpConstraintInstance__InstanceType_unsigned_char_,
    pub m_runtime: *mut hkpConstraintRuntime,
    pub m_runtimeSize: u16,
    pub m_slaveIndex: u16,
}

#[repr(C)]
pub struct hkArrayBase_hkpAgentNnSector___ {
    pub m_data: *mut *mut hkpAgentNnSector,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpEntityListener___ {
    pub m_data: *mut *mut hkpEntityListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkRefPtr_hkLocalFrame_ {
    pub m_pntr: *mut hkLocalFrame,
}

#[repr(C)]
pub struct hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_ {
    pub m_data: *mut hkpLinkedCollidable__CollisionEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpLinkedCollidable__CollisionEntry__ptr(
        &self,
    ) -> *const hkArrayBase_hkpLinkedCollidable__CollisionEntry_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpLinkedCollidable__CollisionEntry__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpLinkedCollidable__CollisionEntry_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpPhantom___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpPhantom,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpPhantom___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpPhantom____ptr(&self) -> *const hkArrayBase_hkpPhantom___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpPhantom____mut_ptr(&mut self) -> *mut hkArrayBase_hkpPhantom___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkSmallArray_hkpContactListener___ {
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkArray_unsigned_short_hkContainerHeapAllocator_ {
    pub m_data: *mut u16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_unsigned_short_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_unsigned_short__ptr(&self) -> *const hkArrayBase_unsigned_short_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_unsigned_short__mut_ptr(&mut self) -> *mut hkArrayBase_unsigned_short_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkRefPtr_hkReferencedObject_ {
    pub m_pntr: *mut hkReferencedObject,
}

#[repr(C)]
pub struct hkArrayBase_hkpPhantom___ {
    pub m_data: *mut *mut hkpPhantom,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkUFloat8 {
    pub m_value: u8,
}

#[repr(C)]
pub struct hkArrayBase_hkpSimulationIsland___ {
    pub m_data: *mut *mut hkpSimulationIsland,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

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
    pub m_impulseApplied: f32,
    pub m_internalSolverData: f32,
    pub m_userData: u32,
    pub m_friction: hkUFloat8,
    pub m_restitution: u8,
    pub m_maxImpulse: hkUFloat8,
    pub m_flags: u8,
    pub m_internalDataA: f32,
}

impl hkpContactPointProperties {
    pub fn as_hkpSolverResults_ptr(&self) -> *const hkpSolverResults {
        self as *const _ as _
    }

    pub fn as_hkpSolverResults_mut_ptr(&mut self) -> *mut hkpSolverResults {
        self as *mut _ as _
    }

    pub fn as_hkContactPointMaterial_ptr(&self) -> *const hkContactPointMaterial {
        self as *const _ as _
    }

    pub fn as_hkContactPointMaterial_mut_ptr(&mut self) -> *mut hkContactPointMaterial {
        self as *mut _ as _
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
    pub m_maxSizeOfSchema: i32,
    pub m_sizeOfSchemas: i32,
    pub m_numSolverResults: i32,
    pub m_numSolverElemTemps: i32,
    pub m_atoms: *mut hkpConstraintAtom,
    pub m_sizeOfAllAtoms: u32,
    pub m_extraSchemaSize: u32,
}

impl hkpConstraintData__ConstraintInfo {
    pub fn as_hkpConstraintInfo_ptr(&self) -> *const hkpConstraintInfo {
        self as *const _ as _
    }

    pub fn as_hkpConstraintInfo_mut_ptr(&mut self) -> *mut hkpConstraintInfo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkStackTracer__CallTree__Node_ {
    pub m_data: *mut hkStackTracer__CallTree__Node,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkWorldMemoryAvailableWatchDog {
    pub __vfptr: *const hkWorldMemoryAvailableWatchDog____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkWorldMemoryAvailableWatchDog {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkWorldMemoryAvailableWatchDog____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __vfptr: *const hkMemoryAllocator__ExtendedInterface____vftable,
}

#[repr(C)]
pub struct hkMemoryAllocator__ExtendedInterface____vftable {
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
        _: u32,
    ) -> hkResult,
    pub getMemorySoftLimit:
        unsafe extern "thiscall" fn(this: *const hkMemoryAllocator__ExtendedInterface) -> u32,
    pub canAllocTotal: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: i32,
    ) -> bool,
    pub walkMemory: unsafe extern "thiscall" fn(
        this: *mut hkMemoryAllocator__ExtendedInterface,
        _: *mut unsafe extern "C" fn(_: *mut (), _: u32, _: bool, _: i32, _: *mut ()),
        _: *mut (),
    ) -> hkResult,
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
    pub m_data: *mut *mut hkpAgentNnSector,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpAgentNnSector___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpAgentNnSector____ptr(&self) -> *const hkArrayBase_hkpAgentNnSector___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpAgentNnSector____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpAgentNnSector___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpEntityListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpEntityListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpEntityListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpEntityListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpEntityListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpEntityListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpEntityListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrameGroup {
    pub __vfptr: *const hkLocalFrameGroup____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_name: hkStringPtr,
}

impl hkLocalFrameGroup {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrameGroup____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkArray_hkpSimulationIsland___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpSimulationIsland,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpSimulationIsland___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpSimulationIsland____ptr(
        &self,
    ) -> *const hkArrayBase_hkpSimulationIsland___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpSimulationIsland____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpSimulationIsland___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrame {
    pub __vfptr: *const hkLocalFrame____vftable,
    pub m_memSizeAndRefCount: u32,
}

impl hkLocalFrame {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkLocalFrame____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub __: *const (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_maxConstraintViolationSqrd_2")]
#[repr(C)]
pub struct hkpConstraintQueryIn {
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
    pub m_bodyA: hkPadSpu_hkpVelocityAccumulator_const___,
    pub m_bodyB: hkPadSpu_hkpVelocityAccumulator_const___,
    pub m_transformA: hkPadSpu_hkTransformf_const___,
    pub m_transformB: hkPadSpu_hkTransformf_const___,
    pub m_tau: hkPadSpu_float_,
    pub m_damping: hkPadSpu_float_,
    #[cfg(pdb_issue = "error in hkSimdFloat32")]
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
}

impl hkpConstraintQueryIn {
    pub fn as_hkpConstraintQueryStepInfo_ptr(&self) -> *const hkpConstraintQueryStepInfo {
        self as *const _ as _
    }

    pub fn as_hkpConstraintQueryStepInfo_mut_ptr(&mut self) -> *mut hkpConstraintQueryStepInfo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpShape__CalcSizeForSpuInput {
    pub m_midphaseAgent3Registered: bool,
    pub m_isFixedOrKeyframed: bool,
    pub m_hasDynamicMotionSaved: bool,
}

#[repr(C)]
pub struct hkBaseObject {
    pub __vfptr: *const hkBaseObject____vftable,
}

#[repr(C)]
pub struct hkBaseObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
}

#[repr(C)]
pub struct gfc__OmniLightGizmo {
    pub __vfptr: *const gfc__OmniLightGizmo____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__Gizmo____vftable,
    pub mLocked: bool,
    pub mObject: *mut gfc__WorldObject,
}

impl gfc__OmniLightGizmo {
    pub fn as_gfc__Gizmo_ptr(&self) -> *const gfc__Gizmo {
        self as *const _ as _
    }

    pub fn as_gfc__Gizmo_mut_ptr(&mut self) -> *mut gfc__Gizmo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__OmniLightGizmo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__StaticLightingVisualOpt {
    pub __vfptr: *const gfc__StaticLightingVisualOpt____vftable,
    pub ReferenceCount: i32,
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

impl gfc__StaticLightingVisualOpt {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__StaticLightingVisualOpt____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
pub struct gfc__CollisionObject {
    pub __vfptr: *const gfc__CollisionObject____vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
}

#[repr(C)]
pub struct gfc__CollisionObject____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CollisionObject, _: u32) -> *mut (),
    pub getShape:
        unsafe extern "thiscall" fn(this: *const gfc__CollisionObject) -> gfc__AutoRef_gfc__CShape_,
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
    pub __vfptr: *const gfc__StaticLightingObjectOpt____vftable,
    pub ReferenceCount: i32,
    pub mLayerID: u32,
    pub mObjectID: u32,
    pub mDataElementCount: u32,
    pub mData: *mut *mut gfc__StaticLightingVisualOpt,
}

impl gfc__StaticLightingObjectOpt {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__StaticLightingObjectOpt____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
pub struct gfc__ParticleController {
    pub __vfptr: *const gfc__ParticleController____vftable,
    pub ReferenceCount: i32,
    pub mNextChannelId: i32,
    pub mObject: *mut gfc__Object3D,
    pub mEmitters: gfc__Vector_gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance__0_gfc__CAllocator_,
    pub mUseAlternateTime: bool,
}

impl gfc__ParticleController {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ParticleController____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
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
    pub __vfptr: *const gfc__IconGizmo____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__Gizmo____vftable,
    pub mLocked: bool,
    pub mObject: *mut gfc__WorldObject,
    pub mMaterial: gfc__AutoRef_gfc__Material_,
}

impl gfc__IconGizmo {
    pub fn as_gfc__Gizmo_ptr(&self) -> *const gfc__Gizmo {
        self as *const _ as _
    }

    pub fn as_gfc__Gizmo_mut_ptr(&mut self) -> *mut gfc__Gizmo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__IconGizmo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__Object3D {
    pub __vfptr: *const gfc__Object3D____vftable,
    pub ReferenceCount: i32,
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

impl gfc__Object3D {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Object3D____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
pub struct gfc__ScenePortal {
    pub __vfptr: *const gfc__ScenePortal____vftable,
    pub mCellAID: u32,
    pub mCellBID: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCellA: *mut gfc__SceneCell,
    pub mCellB: *mut gfc__SceneCell,
    pub mDrawCounter: u32,
}

#[repr(C)]
pub struct gfc__ScenePortal____vftable {
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
    pub __vfptr: *const gfc__StaticObject____vftable,
    pub ReferenceCount: i32,
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
    pub mPackageName: gfc__HString,
    pub mObjectName: gfc__HString,
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mScale: gfc__TVector3_float_gfc__FloatMath_,
    pub mAORayLength: i32,
}

impl gfc__StaticObject {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__StaticObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
}

#[repr(C)]
pub struct gfc__RigidBody {
    pub __vfptr: *const gfc__RigidBody____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc__Body____vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
    pub mObject: *mut gfc__Object3D,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mNodeName: gfc__HString,
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mElementData: gfc__AutoRef_gfc__Object_,
    pub mElementType: u16,
    pub mAutoActivate: bool,
    pub mBodyType: u8,
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

impl gfc__RigidBody {
    pub fn as_gfc__Body_ptr(&self) -> *const gfc__Body {
        self as *const _ as _
    }

    pub fn as_gfc__Body_mut_ptr(&mut self) -> *mut gfc__Body {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__RigidBody____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
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
    pub buildHavokRigidBody: unsafe extern "thiscall" fn(this: *mut gfc__RigidBody),
}

#[repr(C)]
pub struct gfc__StaticLightingRegionOpt {
    pub __vfptr: *const gfc__StaticLightingRegionOpt____vftable,
    pub ReferenceCount: i32,
    pub LightTable: gfc__AutoRef_gfc__VertexBuffer_,
    pub UVTable: gfc__AutoRef_gfc__VertexBuffer_,
    pub mDirty: bool,
    pub mInPlace: bool,
    pub mMapsElementCount: u32,
    pub mObjectsElementCount: u32,
    pub mMaps: *mut gfc__AutoRef_gfc__Texture_,
    pub mObjects: *mut *mut gfc__StaticLightingObjectOpt,
}

impl gfc__StaticLightingRegionOpt {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__StaticLightingRegionOpt____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
pub struct gfc__EventHandler {
    pub __vfptr: *const gfc__EventHandler____vftable,
    pub ReferenceCount: i32,
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
    pub mGizmo: *mut gfc__IconGizmo,
    pub mWatchObjectNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mQueriedNames: bool,
}

impl gfc__EventHandler {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__EventHandler____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub queryAttach:
        unsafe extern "thiscall" fn(this: *mut gfc__EventHandler, _: *mut gfc__WorldObject) -> bool,
    pub getEventHandler:
        unsafe extern "thiscall" fn(this: *mut gfc__EventHandler) -> *mut gfc__Object,
    pub Execute: unsafe extern "thiscall" fn(
        this: *mut gfc__EventHandler,
        _: *mut gfc__Object,
        _: *const gfc__HString,
    ) -> bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__LightVisual_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__SceneLight {
    pub __vfptr: *const gfc__SceneLight____vftable,
    pub mDrawCounter: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mDynamicVis: bool,
    pub mVisible: bool,
}

#[repr(C)]
pub struct gfc__SceneLight____vftable {
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
pub struct gfc__OmniLight {
    pub __vfptr: *const gfc__OmniLight____vftable,
    pub ReferenceCount: i32,
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
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mEnabled: bool,
    pub mJitterAmount: f32,
    pub mStaticOnly: bool,
    pub mLight: gfc__AutoRef_gfc__LightVisual_,
    pub mGizmo: *mut gfc__OmniLightGizmo,
}

impl gfc__OmniLight {
    pub fn as_gfc__WorldLight_ptr(&self) -> *const gfc__WorldLight {
        self as *const _ as _
    }

    pub fn as_gfc__WorldLight_mut_ptr(&mut self) -> *mut gfc__WorldLight {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__OmniLight____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub getStatic: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
}

#[repr(C)]
pub struct gfc__Body {
    pub __vfptr: *const gfc__Body____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc__Body____vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
    pub mObject: *mut gfc__Object3D,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mNodeName: gfc__HString,
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mElementData: gfc__AutoRef_gfc__Object_,
    pub mElementType: u16,
    pub mAutoActivate: bool,
    pub mBodyType: u8,
}

impl gfc__Body {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }

    pub fn as_gfc__CollisionObject_ptr(&self) -> *const gfc__CollisionObject {
        self as *const _ as _
    }

    pub fn as_gfc__CollisionObject_mut_ptr(&mut self) -> *mut gfc__CollisionObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Body____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
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
    pub __vfptr: *const gfc__WorldObjectGizmo____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__Gizmo____vftable,
    pub mLocked: bool,
    pub mObject: *mut gfc__WorldObject,
    pub mGizmoColor: gfc__TVector4_float_gfc__FloatMath_,
}

impl gfc__WorldObjectGizmo {
    pub fn as_gfc__Gizmo_ptr(&self) -> *const gfc__Gizmo {
        self as *const _ as _
    }

    pub fn as_gfc__Gizmo_mut_ptr(&mut self) -> *mut gfc__Gizmo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldObjectGizmo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__PSystem {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mSoundDescFile")]
#[repr(C)]
pub struct gfc__PSystem {
    pub __vfptr: *const gfc__PSystem____vftable,
    pub ReferenceCount: i32,
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
    #[cfg(pdb_issue = "error in gfc__String")]
    pub mSoundDescFile: gfc__String,
}

impl gfc__PSystem {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__PSystem____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
pub struct gfc__AutoRef_gfc__Skeleton3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Gizmo {
    pub __vfptr: *const gfc__Gizmo____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__Gizmo____vftable,
    pub mLocked: bool,
    pub mObject: *mut gfc__WorldObject,
}

impl gfc__Gizmo {
    pub fn as_gfc__SceneObject_ptr(&self) -> *const gfc__SceneObject {
        self as *const _ as _
    }

    pub fn as_gfc__SceneObject_mut_ptr(&mut self) -> *mut gfc__SceneObject {
        self as *mut _ as _
    }

    pub fn as_gfc__IRenderCallback_ptr(&self) -> *const gfc__IRenderCallback {
        self as *const _ as _
    }

    pub fn as_gfc__IRenderCallback_mut_ptr(&mut self) -> *mut gfc__IRenderCallback {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Gizmo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
    pub __vfptr: *const gfc__SceneCell____vftable,
    pub mObjects: gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
    pub mLights: gfc__Vector_gfc__SceneLight___0_gfc__CAllocator_,
    pub mPortals: gfc__Vector_gfc__ScenePortal___0_gfc__CAllocator_,
    pub mObservers: gfc__Vector_gfc__SceneObserver___0_gfc__CAllocator_,
    pub m_ID: u32,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mDrawCounter: u32,
    pub mInCell: bool,
}

#[repr(C)]
pub struct gfc__SceneCell____vftable {
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
pub struct gfc__WorldLight {
    pub __vfptr: *const gfc__WorldLight____vftable,
    pub ReferenceCount: i32,
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
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mEnabled: bool,
    pub mJitterAmount: f32,
}

impl gfc__WorldLight {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldLight____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub getStatic: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
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
    pub __vfptr: *const gfc__SceneOccluder____vftable,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mDrawCounter: u32,
}

#[repr(C)]
pub struct gfc__SceneOccluder____vftable {
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
    pub __vfptr: *const hkpRayHitCollector____vftable,
    pub m_earlyOutHitFraction: f32,
}

#[repr(C)]
pub struct hkpRayHitCollector____vftable {
    pub addRayHit: unsafe extern "thiscall" fn(
        this: *mut hkpRayHitCollector,
        _: *const hkpCdBody,
        _: *const hkpShapeRayCastCollectorOutput,
    ),
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpRayHitCollector, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__ConstraintSetup {
    pub Bodies: gfc__Vector_gfc__ConstraintSetup__BodyInfo_0_gfc__CAllocator_,
    pub NextSubsystem: i32,
}

#[repr(C)]
pub struct gfc__ConstraintSetup__BodyInfo {
    pub RBody: *mut gfc__RigidBody,
    pub NumConstrained: i32,
    pub Subsystem: i32,
    pub IgnoreSubsys: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ConstraintSetup__BodyInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ConstraintSetup__BodyInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Constraint_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RagdollBoneMapping_ {
    pub p: *mut gfc__IRefObject,
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
    pub m_data: *mut hkpCollisionDispatcher__ShapeInheritance,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpCollisionDispatcher__ShapeInheritance_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpCollisionDispatcher__ShapeInheritance__ptr(
        &self,
    ) -> *const hkArrayBase_hkpCollisionDispatcher__ShapeInheritance_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpCollisionDispatcher__ShapeInheritance__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpCollisionDispatcher__ShapeInheritance_ {
        self as *mut _ as _
    }
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
    pub __vfptr: *const hkpCollisionAgent____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_contactMgr: *mut hkpContactMgr,
}

impl hkpCollisionAgent {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollisionAgent____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub m_data: *mut u32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_unsigned_int_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_unsigned_int__ptr(&self) -> *const hkArrayBase_unsigned_int_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_unsigned_int__mut_ptr(&mut self) -> *mut hkArrayBase_unsigned_int_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkPadSpu_hkpCollisionQualityInfo___ {
    pub m_storage: *mut hkpCollisionQualityInfo,
}

#[repr(C)]
pub struct hkpProcessCollisionInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_aabb32Info")]
#[repr(C)]
pub struct hkpProcessCollisionInput {
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    #[cfg(pdb_issue = "error in hkpCollisionInput__Aabb32Info")]
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
    pub m_stepInfo: hkStepInfo,
    pub m_collisionQualityInfo: hkPadSpu_hkpCollisionQualityInfo___,
    pub m_spareAgentSector: *mut hkpAgent1nSector,
    pub m_dynamicsInfo: *mut (),
    pub m_enableDeprecatedWelding: hkBool,
    pub m_allowToSkipConfirmedCallbacks: hkBool,
    pub m_config: *mut hkpCollisionAgentConfig,
}

impl hkpProcessCollisionInput {
    pub fn as_hkpCollisionInput_ptr(&self) -> *const hkpCollisionInput {
        self as *const _ as _
    }

    pub fn as_hkpCollisionInput_mut_ptr(&mut self) -> *mut hkpCollisionInput {
        self as *mut _ as _
    }
}

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
    pub __vfptr: *const hkpCdPointCollector____vftable,
    pub m_earlyOutDistance: f32,
}

#[repr(C)]
pub struct hkpCdPointCollector____vftable {
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
    pub __vfptr: *const hkpCdBodyPairCollector____vftable,
    pub m_earlyOut: hkBool,
}

#[repr(C)]
pub struct hkpCdBodyPairCollector____vftable {
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
pub struct hkArrayBase_unsigned_int_ {
    pub m_data: *mut u32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__SceneObserver {
    pub __vfptr: *const gfc__SceneObserver____vftable,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mInvalid: bool,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mWarpDistanceThreshold: f32,
}

#[repr(C)]
pub struct gfc__SceneObserver____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObserver, _: u32) -> *mut (),
    pub getContext: unsafe extern "thiscall" fn(this: *mut gfc__SceneObserver) -> *mut gfc__Object,
}

#[repr(C)]
pub struct hkPadSpu_hkpConstraintRuntime___ {
    pub m_storage: *mut hkpConstraintRuntime,
}

#[repr(C)]
pub struct hkpNullBroadPhaseListener {
    pub __vfptr: *const hkpNullBroadPhaseListener____vftable,
    pub m_memSizeAndRefCount: u32,
    pub __vfptr_2: *const hkpNullBroadPhaseListener____vftable,
}

impl hkpNullBroadPhaseListener {
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
pub struct hkpNullBroadPhaseListener____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
}

#[repr(C)]
pub struct hkpAction {
    pub __vfptr: *const hkpAction____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_world: *mut hkpWorld,
    pub m_island: *mut hkpSimulationIsland,
    pub m_userData: u32,
    pub m_name: hkStringPtr,
}

impl hkpAction {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpAction____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkBaseObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkBaseObject),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkReferencedObject) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkReferencedObject),
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
    pub m_data: *mut hkAabb,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkAabb_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkAabb__ptr(&self) -> *const hkArrayBase_hkAabb_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkAabb__mut_ptr(&mut self) -> *mut hkArrayBase_hkAabb_ {
        self as *mut _ as _
    }
}

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
    pub __vfptr: *const hkpWorldDeletionListener____vftable,
}

#[repr(C)]
pub struct hkpWorldDeletionListener____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: u32) -> *mut (),
    pub worldDeletedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: *mut hkpWorld),
    pub worldRemoveAllCallback:
        unsafe extern "thiscall" fn(this: *mut hkpWorldDeletionListener, _: *mut hkpWorld),
}

#[repr(C)]
pub struct hkpEntityListener {
    pub __vfptr: *const hkpEntityListener____vftable,
}

#[repr(C)]
pub struct hkpEntityListener____vftable {
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
pub struct hkFlags_enum_hkClass__FlagValues_unsigned_int_ {
    pub m_storage: u32,
}

#[repr(C)]
pub struct hkpBroadPhaseHandlePair {
    pub m_a: *mut hkpBroadPhaseHandle,
    pub m_b: *mut hkpBroadPhaseHandle,
}

#[repr(C)]
pub struct gfc__CachedKeyframe {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field Position")]
#[repr(C)]
pub struct gfc__CachedKeyframe {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub Position: hkVector4f,
    pub Rotation: hkQuaternionf,
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
    pub m_data: *mut hkpBroadPhaseHandlePair,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpBroadPhaseHandlePair__ptr(
        &self,
    ) -> *const hkArrayBase_hkpBroadPhaseHandlePair_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpBroadPhaseHandlePair__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpBroadPhaseHandlePair_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpTypedBroadPhaseDispatcher {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_broadPhaseListeners")]
#[repr(C)]
pub struct hkpTypedBroadPhaseDispatcher {
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_broadPhaseListeners: compile_error!("unimplemented feature: sizeof array 0x0"),
    pub m_nullBroadPhaseListener: hkpNullBroadPhaseListener,
}

#[repr(C)]
pub struct hkArrayBase_hkpBroadPhaseHandle___ {
    pub m_data: *mut *mut hkpBroadPhaseHandle,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpTypedBroadPhaseHandlePair {
    pub m_a: *mut hkpBroadPhaseHandle,
    pub m_b: *mut hkpBroadPhaseHandle,
}

impl hkpTypedBroadPhaseHandlePair {
    pub fn as_hkpBroadPhaseHandlePair_ptr(&self) -> *const hkpBroadPhaseHandlePair {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseHandlePair_mut_ptr(&mut self) -> *mut hkpBroadPhaseHandlePair {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpWorldPostSimulationListener {
    pub __vfptr: *const hkpWorldPostSimulationListener____vftable,
}

#[repr(C)]
pub struct hkpWorldPostSimulationListener____vftable {
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
pub struct gfc__AutoRef_gfc__BezierCurve_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion {
    pub __vfptr: *const gfc__PhysicsDetectRegion____vftable,
    pub mPhantomOverlapListenerProxy: gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy,
    pub mWorld: *mut gfc__World,
    pub mPhantom: *mut hkpPhantom,
    pub mBodies: gfc__Vector_gfc__PhysicsDetectRegion__BodyInfo_0_gfc__CAllocator_,
    pub mNumContainedBodies: i32,
    pub mIsShapeDetector: bool,
    pub mAABBListChanged: bool,
}

impl gfc__PhysicsDetectRegion {
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
pub struct gfc__PhysicsDetectRegion____vftable {
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
    pub __: *const (),
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
    pub __vfptr: *const gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy____vftable,
}

impl gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy {
    pub fn as_hkpPhantomOverlapListener_ptr(&self) -> *const hkpPhantomOverlapListener {
        self as *const _ as _
    }

    pub fn as_hkpPhantomOverlapListener_mut_ptr(&mut self) -> *mut hkpPhantomOverlapListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy____vftable {
    pub collidableAddedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpPhantomOverlapListener,
        _: *const hkpCollidableAddedEvent,
    ),
    pub collidableRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpPhantomOverlapListener,
        _: *const hkpCollidableRemovedEvent,
    ),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpPhantomOverlapListener, _: u32) -> *mut (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mInvTransform")]
#[repr(C)]
pub struct gfc__TraversalWaypoint {
    pub __vfptr: *const gfc__TraversalWaypoint____vftable,
    pub ReferenceCount: i32,
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
    pub mSeed: i32,
    pub mWaypointType: i32,
    pub mOpenSpaceID: i32,
    pub mLinks: gfc__Vector_gfc__AutoRef_gfc__TraversalLink__0_gfc__CAllocator_,
    pub mUnreachableWaypoints:
        gfc__Vector_gfc__TraversalWaypoint__UnreachableWaypoint_0_gfc__CAllocator_,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mUpVector: gfc__TVector3_float_gfc__FloatMath_,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mInvTransform: gfc__Matrix4,
    pub mBoundingBox: gfc__TBox_float_gfc__FloatMath_,
    pub mScale: f32,
    pub mGizmo: *mut gfc__TraversalWaypoint__TraversalWaypointGizmo,
}

impl gfc__TraversalWaypoint {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__TraversalWaypoint____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__UnreachableWaypoint {
    pub waypoint: gfc__AutoRef_gfc__TraversalWaypoint_,
    pub pathFlags: u32,
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__TraversalWaypointGizmo {
    pub __vfptr: *const gfc__TraversalWaypoint__TraversalWaypointGizmo____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_2: *const gfc__Gizmo____vftable,
    pub mLocked: bool,
    pub mObject: *mut gfc__WorldObject,
    pub mGizmoColor: gfc__TVector4_float_gfc__FloatMath_,
}

impl gfc__TraversalWaypoint__TraversalWaypointGizmo {
    pub fn as_gfc__WorldObjectGizmo_ptr(&self) -> *const gfc__WorldObjectGizmo {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObjectGizmo_mut_ptr(&mut self) -> *mut gfc__WorldObjectGizmo {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__TraversalWaypoint__TraversalWaypointGizmo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: u32) -> *mut (),
    pub removeFromScene: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub cacheBoundingVolume: unsafe extern "thiscall" fn(this: *mut gfc__SceneObject),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneObject,
        _: *mut gfc__BoundingVolume,
    ) -> bool,
    pub cull:
        unsafe extern "thiscall" fn(this: *mut gfc__SceneObject, _: *const gfc__Clipper) -> bool,
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
pub struct gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__CollisionObject,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__CollisionManager {
    pub __vfptr: *const gfc__CollisionManager____vftable,
    pub ReferenceCount: i32,
    pub mObjects: gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_,
    pub mQueryResult: gfc__Vector_gfc__CollisionObject___0_gfc__CAllocator_,
}

impl gfc__CollisionManager {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__CollisionManager____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std___Pair_base_gfc__String_const__gfc__StateMapValue_ {
    #[cfg(pdb_issue = "error in gfc__String")]
    pub first: gfc__String,
    #[cfg(pdb_issue = "error in gfc__StateMapValue")]
    pub second: gfc__StateMapValue,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node
{
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field _Myval")]
#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    #[cfg(pdb_issue = "error in std__pair_gfc__String_const__gfc__StateMapValue_")]
    pub _Myval: std__pair_gfc__String_const__gfc__StateMapValue_,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__StateMapValue_ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__StateMapValue_ {
    #[cfg(pdb_issue = "error in gfc__String")]
    pub first: gfc__String,
    #[cfg(pdb_issue = "error in gfc__StateMapValue")]
    pub second: gfc__StateMapValue,
}

impl std__pair_gfc__String_const__gfc__StateMapValue_ {
    pub fn as_std___Pair_base_gfc__String_const__gfc__StateMapValue__ptr(
        &self,
    ) -> *const std___Pair_base_gfc__String_const__gfc__StateMapValue_ {
        self as *const _ as _
    }

    pub fn as_std___Pair_base_gfc__String_const__gfc__StateMapValue__mut_ptr(
        &mut self,
    ) -> *mut std___Pair_base_gfc__String_const__gfc__StateMapValue_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ModSysContainerModule {
    pub __vfptr: *const gfc__ModSysContainerModule____vftable,
    pub ReferenceCount: i32,
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    pub mEventLinks: gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_,
    pub mInputLinks: gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_,
    pub mVariableLinks: gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_,
    pub mEnable: bool,
    pub mIncludeName: gfc__HString,
    pub mInternalModuleSystem: gfc__AutoRef_gfc__ModuleSystem_,
    pub mInputModule: gfc__AutoRef_gfc__InputModule_,
    pub mActionNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mEventNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mVariableConnections:
        gfc__Vector_gfc__AutoRef_gfc__VariableConnectionInfo__0_gfc__CAllocator_,
}

impl gfc__ModSysContainerModule {
    pub fn as_gfc__VisScriptModule_ptr(&self) -> *const gfc__VisScriptModule {
        self as *const _ as _
    }

    pub fn as_gfc__VisScriptModule_mut_ptr(&mut self) -> *mut gfc__VisScriptModule {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ModSysContainerModule____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *mut gfc__ObjectCloner, _: gfc__AutoRef_gfc__Object_),
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *const i8,
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity),
    pub clearDeadLinks: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub getCategory: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> i32,
    pub getNumActions: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> i32,
    pub getActionID: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> u32,
    pub getActionName: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> *const i8,
    pub getNumEvents: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> i32,
    pub getEventID: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> u32,
    pub getEventName: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> *const i8,
    pub getNumVariableConnections: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> i32,
    pub getVariableConnectionID: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> u32,
    pub getVariableConnectionInfo: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, _: i32) -> gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub doEvent: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub getVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> gfc__AutoRef_gfc__Value_,
    pub setVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32, _: gfc__AutoRef_gfc__Value_),
    pub tryAgain: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule),
    pub getVariablesIn: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub getVariablesOut: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub executeInternal: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub hasVariableIn: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> bool,
    pub hasVariableOut: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> bool,
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
pub struct gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: gfc__HString,
    pub mValue: gfc__AutoRef_gfc__Class_,
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_ {
    pub __vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    pub m_pObj: *mut keen__ISteamStatsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsReceived_t,
    ),
}

impl CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_ {
    pub fn as_CCallbackImpl_24__ptr(&self) -> *const CCallbackImpl_24_ {
        self as *const _ as _
    }

    pub fn as_CCallbackImpl_24__mut_ptr(&mut self) -> *mut CCallbackImpl_24_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct UserStatsReceived_t {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_steamIDUser")]
#[repr(C)]
pub struct UserStatsReceived_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
    #[cfg(pdb_issue = "error in CSteamID")]
    pub m_steamIDUser: CSteamID,
}

#[repr(C)]
pub struct CCallbackImpl_16_ {
    pub __vfptr: *const CCallbackImpl_16_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

impl CCallbackImpl_16_ {
    pub fn as_CCallbackBase_ptr(&self) -> *const CCallbackBase {
        self as *const _ as _
    }

    pub fn as_CCallbackBase_mut_ptr(&mut self) -> *mut CCallbackBase {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallbackImpl_16_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
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
    pub currentTimeInMs: u32,
    pub rankingSendData: unit4__RankingSendData,
    pub rankingReceiveData: unit4__RankingReceiveData,
    pub currentRankingError: unit4__RankingError,
    pub rankingSendInteraction: unit4__SystemServicesInteractionData,
    pub onlineInteraction: unit4__SystemServicesInteractionData,
    pub steamAchievements: keen__SteamAchievements,
    pub steamStats: keen__SteamStats,
    pub pPresenceStrings: [*const i8; 64],
    pub presenceStringCount: u32,
    pub sendRankingStep: unit4__SystemServices__SendRankingStep,
    pub sendRankingCall: u64,
    pub receiveRankingStep: unit4__SystemServices__ReceiveRankingStep,
    pub receiveRankingCall: u64,
}

impl unit4__SystemServices {
    pub fn as_unit4__SystemServicesBase_ptr(&self) -> *const unit4__SystemServicesBase {
        self as *const _ as _
    }

    pub fn as_unit4__SystemServicesBase_mut_ptr(&mut self) -> *mut unit4__SystemServicesBase {
        self as *mut _ as _
    }
}

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
    pub __vfptr: *const keen__ISteamAchievementsCallback____vftable,
    pub m_onUserAchievementStored:
        CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
}

#[repr(C)]
pub struct keen__ISteamAchievementsCallback____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__ISteamAchievementsCallback, _: u32) -> *mut (),
    pub onUserAchievementStored: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamAchievementsCallback,
        _: *mut UserAchievementStored_t,
    ),
}

#[repr(C)]
pub struct keen__SteamStats {
    pub __vfptr: *const keen__SteamStats____vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
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

impl keen__SteamStats {
    pub fn as_keen__ISteamStatsCallback_ptr(&self) -> *const keen__ISteamStatsCallback {
        self as *const _ as _
    }

    pub fn as_keen__ISteamStatsCallback_mut_ptr(&mut self) -> *mut keen__ISteamStatsCallback {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct keen__SteamStats____vftable {
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
pub struct keen__Array_unit4__RankingBoardCacheEntry_ {
    pub m_pData: *mut unit4__RankingBoardCacheEntry,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__SteamAchievements {
    pub __vfptr: *const keen__SteamAchievements____vftable,
    pub m_onUserAchievementStored:
        CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
    pub __vfptr_2: *const keen__SteamAchievements____vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
    pub m_pSteamUserStats: *mut ISteamUserStats,
    pub m_gameSteamID64: u64,
    pub m_pAchievementIdMap: *const u32,
    pub m_pAchievementUnlockCountMap: *const u32,
    pub m_achievementCount: u32,
    pub m_platinumAchievementId: u32,
    pub m_platinumAchievementUnlocked: bool,
}

impl keen__SteamAchievements {
    pub fn as_keen__ISteamAchievementsCallback_ptr(
        &self,
    ) -> *const keen__ISteamAchievementsCallback {
        self as *const _ as _
    }

    pub fn as_keen__ISteamAchievementsCallback_mut_ptr(
        &mut self,
    ) -> *mut keen__ISteamAchievementsCallback {
        self as *mut _ as _
    }

    pub fn as_keen__ISteamStatsCallback_ptr(&self) -> *const keen__ISteamStatsCallback {
        self as *const _ as _
    }

    pub fn as_keen__ISteamStatsCallback_mut_ptr(&mut self) -> *mut keen__ISteamStatsCallback {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct keen__SteamAchievements____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__ISteamAchievementsCallback, _: u32) -> *mut (),
    pub onUserAchievementStored: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamAchievementsCallback,
        _: *mut UserAchievementStored_t,
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
    pub __vfptr: *const keen__ISteamStatsCallback____vftable,
    pub m_onUserStatsReceived: CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0_,
    pub m_onUserStatsStored: CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_,
}

#[repr(C)]
pub struct keen__ISteamStatsCallback____vftable {
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
    pub __vfptr: *const CCallbackImpl_152_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

impl CCallbackImpl_152_ {
    pub fn as_CCallbackBase_ptr(&self) -> *const CCallbackBase {
        self as *const _ as _
    }

    pub fn as_CCallbackBase_mut_ptr(&mut self) -> *mut CCallbackBase {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallbackImpl_152_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_ {
    pub __vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    pub m_pObj: *mut keen__ISteamStatsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
        _: *mut UserStatsStored_t,
    ),
}

impl CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_ {
    pub fn as_CCallbackImpl_16__ptr(&self) -> *const CCallbackImpl_16_ {
        self as *const _ as _
    }

    pub fn as_CCallbackImpl_16__mut_ptr(&mut self) -> *mut CCallbackImpl_16_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_ {
    pub __vfptr:
        *const CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
    pub m_pObj: *mut keen__ISteamAchievementsCallback,
    pub m_Func: *mut unsafe extern "thiscall" fn(
        this: *mut keen__ISteamAchievementsCallback,
        _: *mut UserAchievementStored_t,
    ),
}

impl CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_ {
    pub fn as_CCallbackImpl_152__ptr(&self) -> *const CCallbackImpl_152_ {
        self as *const _ as _
    }

    pub fn as_CCallbackImpl_152__mut_ptr(&mut self) -> *mut CCallbackImpl_152_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct CCallbackImpl_24_ {
    pub __vfptr: *const CCallbackImpl_24_____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

impl CCallbackImpl_24_ {
    pub fn as_CCallbackBase_ptr(&self) -> *const CCallbackBase {
        self as *const _ as _
    }

    pub fn as_CCallbackBase_mut_ptr(&mut self) -> *mut CCallbackBase {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct CCallbackImpl_24_____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mOrientation")]
#[repr(C)]
pub struct gfc__Character {
    pub __vfptr: *const gfc__Character____vftable,
    pub ReferenceCount: i32,
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
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
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientation: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientationInv: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mLastBodyMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mBodyRelativeMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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

impl gfc__Character {
    pub fn as_gfc__KinematicActor_ptr(&self) -> *const gfc__KinematicActor {
        self as *const _ as _
    }

    pub fn as_gfc__KinematicActor_mut_ptr(&mut self) -> *mut gfc__KinematicActor {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Character____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
        _: *mut gfc__Character,
    ) -> gfc__HString,
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
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub ___8: *const (),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ReferenceCount: i32,
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

impl gfc__Actor {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Actor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
    pub ReferenceCount: i32,
    pub mObjectID: u32,
    pub mRegionID: u16,
    pub mLayerID: u16,
    pub mName: gfc__HString,
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mOrientation")]
#[repr(C)]
pub struct gfc__Player {
    pub __vfptr: *const gfc__Player____vftable,
    pub ReferenceCount: i32,
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
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
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientation: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientationInv: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mLastBodyMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mBodyRelativeMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
    pub __vfptr_2: *const gfc__Player____vftable,
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
}

impl gfc__Player {
    pub fn as_gfc__Character_ptr(&self) -> *const gfc__Character {
        self as *const _ as _
    }

    pub fn as_gfc__Character_mut_ptr(&mut self) -> *mut gfc__Character {
        self as *mut _ as _
    }

    pub fn as_gfc__ResourceListener_ptr(&self) -> *const gfc__ResourceListener {
        self as *const _ as _
    }

    pub fn as_gfc__ResourceListener_mut_ptr(&mut self) -> *mut gfc__ResourceListener {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Player____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
        _: *mut gfc__Character,
    ) -> gfc__HString,
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
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub ___8: *const (),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
}

#[repr(C)]
pub struct gfc__CharacterDoInteractiveDesc {
    pub __vfptr: *const gfc__CharacterDoInteractiveDesc____vftable,
    pub ReferenceCount: i32,
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

impl gfc__CharacterDoInteractiveDesc {
    pub fn as_gfc__CharMoveStateDesc_ptr(&self) -> *const gfc__CharMoveStateDesc {
        self as *const _ as _
    }

    pub fn as_gfc__CharMoveStateDesc_mut_ptr(&mut self) -> *mut gfc__CharMoveStateDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__CharacterDoInteractiveDesc____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
        _: *mut gfc__Character,
    ) -> gfc__AutoRef_gfc__CharacterMoveState_,
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
            _: *mut gfc__Player,
        ) -> gfc__AutoRef_gfc__PlayerDoInteractive_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__HavokWeaponProxy__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__HavokWeaponProxy_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Mount {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mOrientation")]
#[repr(C)]
pub struct gfc__Mount {
    pub __vfptr: *const gfc__Mount____vftable,
    pub ReferenceCount: i32,
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
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
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientation: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientationInv: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mLastBodyMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mBodyRelativeMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
    pub mPlayer: gfc__AutoRef_gfc__Player_,
}

impl gfc__Mount {
    pub fn as_gfc__Character_ptr(&self) -> *const gfc__Character {
        self as *const _ as _
    }

    pub fn as_gfc__Character_mut_ptr(&mut self) -> *mut gfc__Character {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Mount____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
        _: *mut gfc__Character,
    ) -> gfc__HString,
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
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub ___8: *const (),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ReferenceCount: i32,
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
}

impl gfc__KinematicActor {
    pub fn as_gfc__Actor_ptr(&self) -> *const gfc__Actor {
        self as *const _ as _
    }

    pub fn as_gfc__Actor_mut_ptr(&mut self) -> *mut gfc__Actor {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__KinematicActor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
    pub ReferenceCount: i32,
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

impl gfc__CharMoveStateDesc {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__CharMoveStateDesc____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
        _: *mut gfc__Character,
    ) -> gfc__AutoRef_gfc__CharacterMoveState_,
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
    pub ReferenceCount: i32,
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

impl gfc__HitInfo {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__HitInfo____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field CameraMatrix")]
#[repr(C)]
pub struct gfc__MoveInput {
    pub MoveLR: f32,
    pub MoveUD: f32,
    pub MoveLRRaw: f32,
    pub MoveUDRaw: f32,
    pub MoveDirection: f32,
    pub MoveSpeed: f32,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
    pub ReferenceCount: i32,
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mWindowStack: gfc__Vector_gfc__AutoRef_gfc___UIControl__0_gfc__CAllocator_,
    pub mOverlayWindow: gfc__AutoRef_gfc___UIControl_,
    pub mEvenOverlayederWindow: gfc__AutoRef_gfc___UIControl_,
    pub mWindow: gfc__AutoRef_gfc___UIControl_,
    pub mRootWindow: gfc__AutoRef_gfc___UIControl_,
}

impl gfc__WindowHelper {
    pub fn as_gfc__Helper_ptr(&self) -> *const gfc__Helper {
        self as *const _ as _
    }

    pub fn as_gfc__Helper_mut_ptr(&mut self) -> *mut gfc__Helper {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WindowHelper____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
    pub ReferenceCount: i32,
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mTeleportTransition: bool,
}

impl gfc__TeleportHelper {
    pub fn as_gfc__Helper_ptr(&self) -> *const gfc__Helper {
        self as *const _ as _
    }

    pub fn as_gfc__Helper_mut_ptr(&mut self) -> *mut gfc__Helper {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__TeleportHelper____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mOrientation")]
#[repr(C)]
pub struct gfc__DraggableActor {
    pub __vfptr: *const gfc__DraggableActor____vftable,
    pub ReferenceCount: i32,
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
    pub mAnimController: gfc__AutoRef_gfc__AnimController_,
    pub mAnimControllers: List_gfc__KinematicActor__KAnimation_,
    pub mAnimIDGenerator: i32,
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
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientation: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mOrientationInv: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mLastBodyMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mBodyRelativeMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
}

impl gfc__DraggableActor {
    pub fn as_gfc__Character_ptr(&self) -> *const gfc__Character {
        self as *const _ as _
    }

    pub fn as_gfc__Character_mut_ptr(&mut self) -> *mut gfc__Character {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__DraggableActor____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub ___3: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___4: *const (),
    pub ___5: *const (),
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
        _: i32,
    ) -> gfc__AutoRef_gfc__Animation_,
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
    pub ___6: *const (),
    pub getCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
    pub getTopCenterPosition:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> gfc__TVector3_float_gfc__FloatMath_,
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
    pub ___7: *const (),
    pub playSoundEx:
        unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: *mut gfc__SoundDesc) -> i32,
    pub isSoundPlaying: unsafe extern "thiscall" fn(this: *mut gfc__Actor, _: i32) -> bool,
    pub visualChanged: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getMoveWeight: unsafe extern "thiscall" fn(this: *mut gfc__Actor) -> f32,
    pub onEnterLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub onExitLiquidRegion: unsafe extern "thiscall" fn(this: *mut gfc__Actor),
    pub getCurrentSpline: unsafe extern "thiscall" fn(
        this: *mut gfc__Actor,
        _: *mut f32,
        _: *mut f32,
    ) -> gfc__AutoRef_gfc__BezierCurve_,
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
        _: *mut gfc__Character,
    ) -> gfc__HString,
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
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub ___8: *const (),
    pub setupCharacterProxy: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub getActualVelocity: unsafe extern "thiscall" fn(
        this: *mut gfc__Character,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
    pub updateLastGroundMaterial: unsafe extern "thiscall" fn(this: *mut gfc__Character),
    pub doHitPause: unsafe extern "thiscall" fn(this: *mut gfc__Character, _: f32),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__LiquidRegionDesc_ {
    pub p: *mut gfc__IRefObject,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mLastMapName")]
#[repr(C)]
pub struct gfc__LoadMapMenu {
    pub __vfptr: *const gfc__LoadMapMenu____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc___UIControl____vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
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
    pub mListItems: gfc__Vector_gfc__String_0_gfc__CAllocator_,
    pub mMaps: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mSelectedMapType: i32,
    pub mSelectedMapIdx: i32,
    #[cfg(pdb_issue = "error in gfc__String")]
    pub mLastMapName: gfc__String,
    #[cfg(pdb_issue = "error in gfc__String")]
    pub mLastMapRegion: gfc__String,
    pub mHasLastMap: bool,
}

impl gfc__LoadMapMenu {
    pub fn as_gfc___UIControl_ptr(&self) -> *const gfc___UIControl {
        self as *const _ as _
    }

    pub fn as_gfc___UIControl_mut_ptr(&mut self) -> *mut gfc___UIControl {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__LoadMapMenu____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub __: *const (),
    pub ___2: *const (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Object,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setEnabled: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: bool),
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
    pub ___3: *const (),
    pub ___4: *const (),
    pub getSize: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
    pub getPreferredSize: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
    pub ___5: *const (),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
    pub getAbsolutePosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    )
        -> gfc__TVector2_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: f32),
    pub getRotation: unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> f32,
    pub ___6: *const (),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
    pub setLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: gfc__AutoRef_gfc__UILayoutManager_,
    ),
    pub getLayoutManager: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
    ) -> gfc__AutoRef_gfc__UILayoutManager_,
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
    ) -> gfc__TVector2_float_gfc__FloatMath_,
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
    pub getInputListener:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl) -> gfc__AutoRef_gfc___UIControl_,
    pub initControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub postInitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub deinitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub reinitControl: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub doInit: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub drawInternal:
        unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc__UIRenderer),
    pub getAnchorPosition: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: *mut gfc___UIControl,
        _: u8,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
    pub getGlobalScale: unsafe extern "thiscall" fn(
        this: *const gfc___UIControl,
    ) -> gfc__TVector2_float_gfc__FloatMath_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_position")]
#[repr(C)]
pub struct hkContactPoint {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_position: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
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
    pub m_elem: *mut hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long_____Pair,
    pub m_numElems: i32,
    pub m_hashMod: i32,
}

impl hkMap_unsigned_long_unsigned_long_hkMapOperations_unsigned_long__hkContainerHeapAllocator_ {
    pub fn as_hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long____ptr(
        &self,
    ) -> *const hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___ {
        self as *const _ as _
    }

    pub fn as_hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long____mut_ptr(
        &mut self,
    ) -> *mut hkMapBase_unsigned_long_unsigned_long_hkMapOperations_unsigned_long___ {
        self as *mut _ as _
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_min")]
#[repr(C)]
pub struct hkAabb {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_min: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_customJobs")]
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
    pub m_cpuThreadIndexToSemaphoreIndex: [i8; 12],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_jobFuncs: compile_error!("unimplemented feature: class layout 0x0"),
    pub m_threadPool: *mut hkSpuThreadPool,
    pub m_customJobSetup: hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_,
    pub m_externalJobProfiler: *mut hkExternalJobProfiler,
}

#[repr(C)]
pub struct hkJobQueue__DynamicData {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_jobQueue")]
#[repr(C)]
pub struct hkJobQueue__DynamicData {
    pub m_numActiveJobs: [i16; 15],
    pub m_masterThreadFinishingFlags: i32,
    pub m_waitPolicy: hkJobQueue__WaitPolicy,
    pub m_outOfMemory: hkBool,
    pub m_numThreadsWaiting: [u16; 5],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_jobQueue: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct hkJobQueue__CustomJobTypeSetup {
    pub m_jobType: hkJobType,
    pub m_jobSubType: u8,
    pub m_threadId: i32,
}

#[repr(C)]
pub struct hkSweptTransformf {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_centerOfMass0")]
#[repr(C)]
pub struct hkSweptTransformf {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_centerOfMass0: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_centerOfMass1: hkVector4f,
    pub m_rotation0: hkQuaternionf,
    pub m_rotation1: hkQuaternionf,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_centerOfMassLocal: hkVector4f,
}

#[repr(C)]
pub struct hkMotionState {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_transform")]
#[repr(C)]
pub struct hkMotionState {
    #[cfg(pdb_issue = "error in hkTransformf")]
    pub m_transform: hkTransformf,
    #[cfg(pdb_issue = "error in hkSweptTransformf")]
    pub m_sweptTransform: hkSweptTransformf,
    #[cfg(pdb_issue = "error in hkVector4f")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_vec")]
#[repr(C)]
pub struct hkQuaternionf {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_vec: hkVector4f,
}

#[repr(C)]
pub struct hkSimdFloat32 {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_real")]
#[repr(C)]
pub struct hkSimdFloat32 {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_real: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct hkVector4f {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_quad")]
#[repr(C)]
pub struct hkVector4f {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_quad: compile_error!("unimplemented feature: type kind 0x1506"),
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
    pub m_data: *mut hkRefCountedProperties__Entry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkRefCountedProperties__Entry__ptr(
        &self,
    ) -> *const hkArrayBase_hkRefCountedProperties__Entry_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkRefCountedProperties__Entry__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkRefCountedProperties__Entry_ {
        self as *mut _ as _
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_translation")]
#[repr(C)]
pub struct hkTransformf {
    pub m_rotation: hkRotationf,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_translation: hkVector4f,
}

#[repr(C)]
pub struct hkFourTransposedPointsf {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_vertices")]
#[repr(C)]
pub struct hkFourTransposedPointsf {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_vertices: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct hkcdVertex {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_quad")]
#[repr(C)]
pub struct hkcdVertex {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_quad: compile_error!("unimplemented feature: type kind 0x1506"),
}

impl hkcdVertex {
    pub fn as_hkVector4f_ptr(&self) -> *const hkVector4f {
        self as *const _ as _
    }

    pub fn as_hkVector4f_mut_ptr(&mut self) -> *mut hkVector4f {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkRotationf {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_col0")]
#[repr(C)]
pub struct hkRotationf {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col0: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col1: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col2: hkVector4f,
}

impl hkRotationf {
    pub fn as_hkMatrix3f_ptr(&self) -> *const hkMatrix3f {
        self as *const _ as _
    }

    pub fn as_hkMatrix3f_mut_ptr(&mut self) -> *mut hkMatrix3f {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkMatrix3f {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_col0")]
#[repr(C)]
pub struct hkMatrix3f {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col0: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col1: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_col2: hkVector4f,
}

#[repr(C)]
pub struct hkVector4fComparison {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_mask")]
#[repr(C)]
pub struct hkVector4fComparison {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_mask: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct hkStringPtr {
    pub m_stringAndFlag: *const i8,
}

#[repr(C)]
pub struct hkSphere {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_pos")]
#[repr(C)]
pub struct hkSphere {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_pos: hkVector4f,
}

#[repr(C)]
pub struct hkcdShape {
    pub __vfptr: *const hkcdShape____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
}

impl hkcdShape {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
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
    pub m_memSizeAndRefCount: u32,
}

impl hkReferencedObject {
    pub fn as_hkBaseObject_ptr(&self) -> *const hkBaseObject {
        self as *const _ as _
    }

    pub fn as_hkBaseObject_mut_ptr(&mut self) -> *mut hkBaseObject {
        self as *mut _ as _
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_globalAccelerationPerSubStep")]
#[repr(C)]
pub struct hkpSolverInfo {
    pub m_padding: f32,
    pub m_tau: f32,
    pub m_damping: f32,
    pub m_frictionTau: f32,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_globalAccelerationPerSubStep: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_globalAccelerationPerStep: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_integrateVelocityFactor: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_invIntegrateVelocityFactor: hkVector4f,
    pub m_dampDivTau: f32,
    pub m_tauDivDamp: f32,
    pub m_dampDivFrictionTau: f32,
    pub m_frictionTauDivDamp: f32,
    pub m_contactRestingVelocity: f32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_deactivationInfo: compile_error!("unimplemented feature: class layout 0x0"),
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
}

#[repr(C)]
pub struct hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_ {
    pub m_storage: u16,
}

#[repr(C)]
pub struct hkpConstraintData {
    pub __vfptr: *const hkpConstraintData____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_userData: u32,
}

impl hkpConstraintData {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
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
    pub isValid: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> hkBool,
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
    pub setInertiaStabilizationFactor:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: f32) -> hkResult,
    pub getInertiaStabilizationFactor:
        unsafe extern "thiscall" fn(this: *const hkpConstraintData, _: *mut f32) -> hkResult,
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
    pub isBuildJacobianCallbackRequired:
        unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> hkBool,
    pub buildJacobianCallback: unsafe extern "thiscall" fn(
        this: *mut hkpConstraintData,
        _: *const hkpConstraintQueryIn,
        _: *const hkpConstraintQueryOut,
    ),
}

#[repr(C)]
pub struct hkpVelocityAccumulator {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_linearVel")]
#[repr(C)]
pub struct hkpVelocityAccumulator {
    pub m_type: hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorType_unsigned_char_,
    pub m_context: hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorContext_unsigned_char_,
    pub m_deactivationClass: u32,
    pub m_gravityFactor: f32,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_linearVel: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_angularVel: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_invMasses: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_scratch0: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_scratch1: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_scratch2: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_scratch3: hkVector4f,
}

#[repr(C)]
pub struct hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorType_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpJacobianSchema {}

#[repr(C)]
pub struct hkEnum_enum_hkpVelocityAccumulator__hkpAccumulatorContext_unsigned_char_ {
    pub m_storage: u8,
}

#[repr(C)]
pub struct hkpShapeRayCastInput {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_from")]
#[repr(C)]
pub struct hkpShapeRayCastInput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_from: hkVector4f,
    #[cfg(pdb_issue = "error in hkVector4f")]
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
    pub m_memSizeAndRefCount: u32,
    pub m_type: hkEnum_enum_hkcdShapeType__ShapeTypeEnum_unsigned_char_,
    pub m_dispatchType: hkEnum_enum_hkcdShapeDispatchType__ShapeDispatchTypeEnum_unsigned_char_,
    pub m_bitsPerKey: u8,
    pub m_shapeInfoCodecType:
        hkEnum_enum_hkcdShapeInfoCodecType__ShapeInfoCodecTypeEnum_unsigned_char_,
    pub m_userData: u32,
    pub m_bvTreeType: hkEnum_enum_hkpBvTreeShape__BvTreeType_unsigned_char_,
}

impl hkpBvTreeShape {
    pub fn as_hkpShape_ptr(&self) -> *const hkpShape {
        self as *const _ as _
    }

    pub fn as_hkpShape_mut_ptr(&mut self) -> *mut hkpShape {
        self as *mut _ as _
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
    pub __: *const (),
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
    pub m_memSizeAndRefCount: u32,
    pub m_type: u16,
    pub m_size: u16,
    pub m_caps: u32,
    pub m_multiThreadCheck: hkMultiThreadCheck,
    pub m_criticalSection: *mut hkCriticalSection,
}

impl hkpBroadPhase {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
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
    pub __: *const (),
    pub ___2: *const (),
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
    pub ___3: *const (),
    pub ___4: *const (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_from")]
#[repr(C)]
pub struct hkpBroadPhase__hkpCastRayInput {
    #[cfg(pdb_issue = "error in hkVector4f")]
    pub m_from: hkVector4f,
    pub m_numCasts: i32,
    pub m_toBase: *const hkVector4f,
    pub m_toStriding: i32,
    pub m_aabbCacheInfo: *const i8,
}

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
