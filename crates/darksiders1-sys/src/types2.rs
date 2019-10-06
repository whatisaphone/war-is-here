#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{types::*, types3::*};

#[repr(C)]
pub struct gfc__AutoRef_gfc__MeshBuilder_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__MBSubMesh__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__MBSubMesh_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__MeshCache__ReloadInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__MeshCache__ReloadInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__StaticMesh_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__MBBone_0_gfc__CAllocator_ {
    pub mData: *mut gfc__MBBone,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__MeshResource {
    pub __vfptr: *const gfc__MeshResource____vftable,
    pub ReferenceCount: i32,
    pub mState: i32,
    pub mPackageID: i32,
    pub mResource: gfc__AutoRef_gfc__Mesh_,
    pub mName: gfc__HString,
}

impl gfc__MeshResource {
    pub fn as_gfc__ResourceType_gfc__Mesh_2__ptr(&self) -> *const gfc__ResourceType_gfc__Mesh_2_ {
        self as *const _ as _
    }

    pub fn as_gfc__ResourceType_gfc__Mesh_2__mut_ptr(
        &mut self,
    ) -> *mut gfc__ResourceType_gfc__Mesh_2_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MeshResource____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Resource) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub isUnoptimized: unsafe extern "thiscall" fn(this: *const gfc__MeshResource) -> bool,
}

#[repr(C)]
pub struct gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {
    pub fn as_std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String______ptr(&self) -> *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____{
        self as *const _ as _
    }

pub fn as_std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String______mut_ptr(&mut self) -> *mut std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____{
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc___UIControl_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Darksiders {
    pub __vfptr: *const gfc__Darksiders____vftable,
    __pdbindgen_padding: [u8; 4],
    pub mpLocalGameSession: *mut unit4__LocalGameSession,
    pub mpInputSystem: *mut keen__InputSystem,
    pub mDisplayReady: bool,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mVideoEnabled: bool,
    pub mMediaMgr: gfc__AutoRef_gfc__MediaManager_,
    pub mStartTime: u32,
    pub mCurrentTime: u32,
    pub mManualDeltaInSeconds: f32,
    pub mDeltaTime: f32,
    pub mDeltaTimeUnTouched: f32,
    pub mTimeInterval: f32,
    pub mTimeIntervalAlt: f32,
    pub mUpdateWorld: bool,
    pub mGraphics: *mut gfc__Graphics,
    pub mRenderer: *mut gfc__Renderer,
    pub mUIRenderer: *mut gfc__UIRenderer,
    pub mSceneViewport: gfc__Viewport,
    pub mScreenViewport: gfc__Viewport,
    pub mNewResolutionWidth: u16,
    pub mNewResolutionHeight: u16,
    pub mNewResolutionRefresh: u16,
    pub mNewFullscreen: bool,
    pub mMouse: gfc__AutoRef_gfc__InputDevice_,
    pub mKeyboard: gfc__AutoRef_gfc__InputDevice_,
    pub mJoystick: gfc__AutoRef_gfc__InputDevice_,
    pub mJoystick2: gfc__AutoRef_gfc__InputDevice_,
    pub mPrimaryInputIndex: i32,
    pub mWorldMgr: gfc__AutoRef_gfc__WorldManager_,
    pub mOverridenWorldMgr: gfc__AutoRef_gfc__WorldManager_,
    pub mWorldMgrState: u8,
    pub mLoadWorld: gfc__HString,
    pub mLoadRegion: gfc__HString,
    pub mLastActiveRegionIdx: i32,
    pub mRenderThreadRunning: gfc__ThreadSafeBool,
    pub mRenderThreadWaitFrame: u32,
    pub mRenderThreadDone: gfc__Event,
    pub mUpdateThreadDone: gfc__Event,
    pub mActiveCamera: gfc__AutoRef_gfc__Camera3D_,
    pub mDebugDraw: *mut gfc__OblivionGameDebug,
    pub mPaused: i32,
    pub mPauseDuration: f32,
    pub mMinNearDistance: f32,
    pub mDefaultPackageMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mVideoIsQueued: bool,
    pub mLockedQueue: bool,
    pub mApplicationSuspended: bool,
    pub mGraphicsSuspended: bool,
    pub mGameSessionHasPlayer: bool,
    pub mFailedScriptModules: gfc__Vector_gfc__VisScriptModule___0_gfc__CAllocator_,
    pub mWaitingForFailedScripts: bool,
    pub mRenderThread: gfc__Thread,
    pub mOverSampling: bool,
    pub mShouldRestartMainMenu: bool,
    pub mFirstAutoCatchStarted: bool,
    pub mState: u8,
    pub mPreview: bool,
    pub mEnableCharacterControl: bool,
    pub mGamepadUnplugged: bool,
    pub mGameInBackground: bool,
    pub mMusicPaused: bool,
    pub mUnpluggedPause: bool,
    pub mInBackgroundPause: bool,
    pub mRumblePaused: bool,
    pub mTitleScreenEnabled: bool,
    pub mPlayerDiedToMonster: bool,
    pub mGameCamera: gfc__AutoRef_gfc__GameCamera_,
    pub mCachedCamera: gfc__AutoRef_gfc__GameCamera_,
    pub mPlayerActor: gfc__AutoRef_gfc__Player_,
    pub mInitPlayerActor: gfc__AutoRef_gfc__Player_,
    pub mTimePlayed: u32,
    pub mResetTime: u32,
    pub mRichPresence: gfc__AutoRef_gfc__RichPresenceManager_,
    pub mAchievements: gfc__AutoRef_gfc__Achievements_,
    pub mBoss: gfc__AutoRef_gfc__Monster_,
    pub mPreviousInput: *mut gfc__MoveInput,
    pub mMoveStickDown: bool,
    pub mStartDown: bool,
    pub mBackDown: bool,
    pub mBlockDownTime: f32,
    pub mExit: bool,
    pub mInEditor: bool,
    pub mStoppingPreviewWorld: bool,
    pub mExitingMediaSequence: bool,
    pub mLastDPadState: i32,
    pub mCurrentActivePad: i32,
    pub mArmorLoadMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mLoadState: gfc__Darksiders__LoadState,
    pub mLogoMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mUIMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mGameMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mLiquidRegionDescs: gfc__Vector_gfc__AutoRef_gfc__LiquidRegionDesc__0_gfc__CAllocator_,
    pub mCurrentLanguage: u32,
    pub mInsideLanguageSwitch: bool,
    pub mOnRestartMainMenu: *mut unsafe extern "C" fn(_: *mut ()),
    pub mpOnRestartMainMenuArg: *mut (),
    pub mMinimizeGameWindow: *mut unsafe extern "C" fn(_: *mut ()),
    pub mpMinimizeGameWindowArg: *mut (),
    pub mAmazingSecretUnlocked: bool,
    pub mSecretInputs: u64,
    pub mSecretWindow: gfc__AutoRef_gfc___UIControl_,
    pub mSecretTimer: f32,
    pub mForceCutscenes: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
}

impl gfc__Darksiders {
    pub fn as_gfc__OblivionGame_ptr(&self) -> *const gfc__OblivionGame {
        self as *const _ as _
    }

    pub fn as_gfc__OblivionGame_mut_ptr(&mut self) -> *mut gfc__OblivionGame {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Darksiders____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u32) -> *mut (),
    pub onDraw: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onDrawUI: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPostDraw: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPostVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onStartup: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onShutdown: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onLoss: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onRecovery: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreWorldDestroy: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onFinalizeDeoverrideWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onFinalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onPreUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onUpdateWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32, _: f32),
    pub onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onPostUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub getCharacterClass:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> *mut gfc__Class,
    pub startParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub waitParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub getCorePackages: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *mut gfc__Vector_int_0_gfc__CAllocator_,
    ),
    pub setPaused: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub setMinimized: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub getFOV: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> f32,
    pub getNearDistance: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> f32,
    pub getFarDistance: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> f32,
    pub getCameraMatrix: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4,
    pub getPlayerPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        _: bool,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u16, _: u16, _: u16, _: u16),
    pub hasOverSamplingChanged: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> bool,
    pub setFullscreen: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub unloadWorldManagers: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub loadRegions: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: bool,
    ),
    pub playVideo: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *const gfc__HString,
        _: *mut gfc__Object,
    ) -> bool,
    pub stopVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub abortVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onCinematicStart:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: *mut gfc__Cinematic),
    pub onCinematicStop:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: *mut gfc__Cinematic),
    pub updateUI: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub updateProfile: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub updateInputDevices: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub changeResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u16, _: u16, _: bool),
    pub createWorldManager:
        unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> *mut gfc__WorldManager,
    pub initWorldManager: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub finalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub loadConfigSettings: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub setupEnvSettings: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onFinalizePreviewWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TUIEventDelegate_gfc___UIEvent___ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Camera3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldManager {
    pub __vfptr: *const gfc__WorldManager____vftable,
    pub ReferenceCount: i32,
    pub mWorld: gfc__AutoRef_gfc__World_,
    pub mWorldState: gfc__WorldManager__WorldState,
    pub mActiveRegionIdx: i32,
    pub mActiveRegionIdxOverride: i32,
    pub mNumRequired: i32,
    pub mQueueLock: gfc__Mutex,
    pub mQueue: gfc__Vector_gfc__WorldQueueItem___0_gfc__CAllocator_,
    pub mCurrent: *mut gfc__WorldLoadRequest,
    pub mGlobalRegionID: i32,
    pub mNeedWorldAdd: bool,
    pub mCheckUnloads: bool,
    pub mShuttingDown: bool,
    pub mInitialLoadDone: bool,
    pub mInit: gfc__WorldManager__InitParams,
}

impl gfc__WorldManager {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldManager____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager),
    pub loadRegions: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldManager,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: bool,
    ),
    pub onRestore: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager),
    pub preAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager),
    pub postAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager),
    pub preRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager),
    pub setupInitialLoad: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldManager,
        _: *mut gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: i32,
    ),
}

#[repr(C)]
pub struct gfc__WorldManager__InitParams {
    pub WorldName: gfc__HString,
    pub StartRegion: gfc__HString,
    pub Flags: u32,
    pub HavokWorldExtents: gfc__TBox_float_gfc__FloatMath_,
    pub VisualDebuggerPort: i32,
    pub UseExtents: bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TUIEventDelegate_gfc__KeyboardEvent___ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc___UIVisual_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__MouseEvent____0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TUIEventDelegate_gfc__MouseEvent___,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TUIEventBroadcaster_gfc__KeyboardEvent_ {
    pub mDelegates:
        gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__KeyboardEvent____0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__RegionLoadInfo {
    pub RegionID: i32,
    pub LayerID: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Variable_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PackageMarker_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc___UIAction_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__InputDevice_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__KeyboardEvent {
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
    pub mKeyCode: i32,
    pub mChar: i8,
    pub mShiftPressed: bool,
    pub mCtrlPressed: bool,
    pub mAltPressed: bool,
    pub mStateChange: bool,
}

impl gfc__KeyboardEvent {
    pub fn as_gfc___UIEvent_ptr(&self) -> *const gfc___UIEvent {
        self as *const _ as _
    }

    pub fn as_gfc___UIEvent_mut_ptr(&mut self) -> *mut gfc___UIEvent {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc___UIControl_ {
    pub __vfptr: *const gfc__Hierarchical_gfc___UIControl_____vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc___UIControl_____vftable {
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
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Player_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TUIEventBroadcaster_gfc__MouseEvent_ {
    pub mDelegates:
        gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__MouseEvent____0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Achievements_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TUIEventDelegate_gfc__MouseEvent___ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__KeyboardEvent____0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TUIEventDelegate_gfc__KeyboardEvent___,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__World_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__FocusEvent {
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
}

impl gfc__FocusEvent {
    pub fn as_gfc___UIEvent_ptr(&self) -> *const gfc___UIEvent {
        self as *const _ as _
    }

    pub fn as_gfc___UIEvent_mut_ptr(&mut self) -> *mut gfc___UIEvent {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__MediaManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TUIEventDelegate_gfc__FocusEvent___ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__MouseEvent {
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
    pub mLocation: gfc__TVector2_int_gfc__FloatMath_,
    pub mButton: u8,
    pub mClickCount: u8,
    pub mScrollDelta: i32,
    pub mShiftPressed: bool,
    pub mCtrlPressed: bool,
    pub mAltPressed: bool,
}

impl gfc__MouseEvent {
    pub fn as_gfc___UIEvent_ptr(&self) -> *const gfc___UIEvent {
        self as *const _ as _
    }

    pub fn as_gfc___UIEvent_mut_ptr(&mut self) -> *mut gfc___UIEvent {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Vector_gfc__VisScriptModule___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__VisScriptModule,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Helper {
    pub __vfptr: *const gfc__Helper____vftable,
    pub ReferenceCount: i32,
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
}

impl gfc__Helper {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Helper____vftable {
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
pub struct gfc__Helper__QueuedListenerInfo {
    pub obj: gfc__AutoRef_gfc__Object_,
    pub shouldAdd: bool,
}

#[repr(C)]
pub struct gfc__OblivionGame {
    pub __vfptr: *const gfc__OblivionGame____vftable,
    __pdbindgen_padding: [u8; 4],
    pub mpLocalGameSession: *mut unit4__LocalGameSession,
    pub mpInputSystem: *mut keen__InputSystem,
    pub mDisplayReady: bool,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mVideoEnabled: bool,
    pub mMediaMgr: gfc__AutoRef_gfc__MediaManager_,
    pub mStartTime: u32,
    pub mCurrentTime: u32,
    pub mManualDeltaInSeconds: f32,
    pub mDeltaTime: f32,
    pub mDeltaTimeUnTouched: f32,
    pub mTimeInterval: f32,
    pub mTimeIntervalAlt: f32,
    pub mUpdateWorld: bool,
    pub mGraphics: *mut gfc__Graphics,
    pub mRenderer: *mut gfc__Renderer,
    pub mUIRenderer: *mut gfc__UIRenderer,
    pub mSceneViewport: gfc__Viewport,
    pub mScreenViewport: gfc__Viewport,
    pub mNewResolutionWidth: u16,
    pub mNewResolutionHeight: u16,
    pub mNewResolutionRefresh: u16,
    pub mNewFullscreen: bool,
    pub mMouse: gfc__AutoRef_gfc__InputDevice_,
    pub mKeyboard: gfc__AutoRef_gfc__InputDevice_,
    pub mJoystick: gfc__AutoRef_gfc__InputDevice_,
    pub mJoystick2: gfc__AutoRef_gfc__InputDevice_,
    pub mPrimaryInputIndex: i32,
    pub mWorldMgr: gfc__AutoRef_gfc__WorldManager_,
    pub mOverridenWorldMgr: gfc__AutoRef_gfc__WorldManager_,
    pub mWorldMgrState: u8,
    pub mLoadWorld: gfc__HString,
    pub mLoadRegion: gfc__HString,
    pub mLastActiveRegionIdx: i32,
    pub mRenderThreadRunning: gfc__ThreadSafeBool,
    pub mRenderThreadWaitFrame: u32,
    pub mRenderThreadDone: gfc__Event,
    pub mUpdateThreadDone: gfc__Event,
    pub mActiveCamera: gfc__AutoRef_gfc__Camera3D_,
    pub mDebugDraw: *mut gfc__OblivionGameDebug,
    pub mPaused: i32,
    pub mPauseDuration: f32,
    pub mMinNearDistance: f32,
    pub mDefaultPackageMarker: gfc__AutoRef_gfc__PackageMarker_,
    pub mVideoIsQueued: bool,
    pub mLockedQueue: bool,
    pub mApplicationSuspended: bool,
    pub mGraphicsSuspended: bool,
    pub mGameSessionHasPlayer: bool,
    pub mFailedScriptModules: gfc__Vector_gfc__VisScriptModule___0_gfc__CAllocator_,
    pub mWaitingForFailedScripts: bool,
    pub mRenderThread: gfc__Thread,
    pub mOverSampling: bool,
}

#[repr(C)]
pub struct gfc__OblivionGame____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u32) -> *mut (),
    pub onDraw: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onDrawUI: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPostDraw: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPostVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onStartup: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onShutdown: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onLoss: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onRecovery: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreWorldDestroy: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onFinalizeDeoverrideWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onFinalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onPreUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onPreUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onUpdateWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32, _: f32),
    pub onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub onPostUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub getCharacterClass:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> *mut gfc__Class,
    pub startParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub waitParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub getCorePackages: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *mut gfc__Vector_int_0_gfc__CAllocator_,
    ),
    pub setPaused: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub setMinimized: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub getFOV: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> f32,
    pub getNearDistance: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> f32,
    pub getFarDistance: unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> f32,
    pub getCameraMatrix: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4,
    pub getPlayerPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        _: bool,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u16, _: u16, _: u16, _: u16),
    pub hasOverSamplingChanged: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> bool,
    pub setFullscreen: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub unloadWorldManagers: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub loadRegions: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: bool,
    ),
    pub playVideo: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: *const gfc__HString,
        _: *mut gfc__Object,
    ) -> bool,
    pub stopVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub abortVideo: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub onCinematicStart:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: *mut gfc__Cinematic),
    pub onCinematicStop:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: *mut gfc__Cinematic),
    pub updateUI: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub updateProfile: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub updateInputDevices: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: f32),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: bool),
    pub changeResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame, _: u16, _: u16, _: bool),
    pub createWorldManager:
        unsafe extern "thiscall" fn(this: *const gfc__OblivionGame) -> *mut gfc__WorldManager,
    pub initWorldManager: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub finalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub loadConfigSettings: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
    pub setupEnvSettings: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc___UIEvent____0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TUIEventDelegate_gfc___UIEvent___,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__FocusEvent____0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TUIEventDelegate_gfc__FocusEvent___,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc___UIAction__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc___UIAction_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RichPresenceManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc___UIControl {
    pub __vfptr: *const gfc___UIControl____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc__Hierarchical_gfc___UIControl_____vftable,
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
}

impl gfc___UIControl {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }

    pub fn as_gfc__Hierarchical_gfc___UIControl__ptr(
        &self,
    ) -> *const gfc__Hierarchical_gfc___UIControl_ {
        self as *const _ as _
    }

    pub fn as_gfc__Hierarchical_gfc___UIControl__mut_ptr(
        &mut self,
    ) -> *mut gfc__Hierarchical_gfc___UIControl_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc___UIControl____vftable {
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
pub struct gfc__AutoRef_gfc__UILayoutManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Monster_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TUIEventBroadcaster_gfc__FocusEvent_ {
    pub mDelegates:
        gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc__FocusEvent____0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc___UIEvent {
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
}

#[repr(C)]
pub struct gfc__TUIEventBroadcaster_gfc___UIEvent_ {
    pub mDelegates:
        gfc__Vector_gfc__AutoRef_gfc__TUIEventDelegate_gfc___UIEvent____0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__LiquidRegionDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__LiquidRegionDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__Helper__QueuedListenerInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__WorldQueueItem___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__WorldQueueItem,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__GameCamera_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct unit4__LocalGameSession {
    pub m_pSaveDataInterface: *mut unit4__SaveDataDescriptionInterface,
    pub m_pSaveDataHandler: *mut keen__SaveDataHandler,
    pub m_pUserAccountSystem: *mut keen__UserAccountSystem,
    pub m_pSystemServices: *mut unit4__SystemServices,
    pub m_pInputSystem: *mut keen__InputSystem,
    pub m_showingSignInUi: bool,
    pub m_userData: keen__Array_unit4__LocalGameSession__LocalUserData_,
    pub m_userAccountOperationHandle: u32,
    pub m_lastUsedControllerInfo: keen__ControllerInfo,
    pub m_lastInteractionStartedBy: unit4__LocalGameSession__InteractionStarter,
}

#[repr(C)]
pub struct unit4__LocalGameSession__LocalUserData {
    pub skippedSignIn: bool,
    pub hasExpectedUserId: bool,
    pub expectedUser: keen__UserAccount,
    pub state: unit4__LocalGameSession__LocalUserState,
    pub stateBeforeProfileChange: unit4__LocalGameSession__LocalUserState,
    pub playerType: unit4__LocalPlayerType,
    pub saveProfileIndex: u32,
    pub controllerDisconnected: bool,
}

#[repr(C)]
pub struct unit4__SaveDataDescriptionInterface {
    pub __vfptr: *const unit4__SaveDataDescriptionInterface____vftable,
}

#[repr(C)]
pub struct unit4__SaveDataDescriptionInterface____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut unit4__SaveDataDescriptionInterface,
        _: u32,
    ) -> *mut (),
    pub getSaveDataSize:
        unsafe extern "thiscall" fn(this: *const unit4__SaveDataDescriptionInterface) -> u32,
    pub isSaveDataValid: unsafe extern "thiscall" fn(
        this: *mut unit4__SaveDataDescriptionInterface,
        _: *const (),
    ) -> bool,
    pub initializeSaveDataToDefault:
        unsafe extern "thiscall" fn(this: *mut unit4__SaveDataDescriptionInterface, _: *mut ()),
}

#[repr(C)]
pub struct keen__Array_unit4__LocalGameSession__LocalUserData_ {
    pub m_pData: *mut unit4__LocalGameSession__LocalUserData,
    pub m_size: u32,
}

#[repr(C)]
pub struct hkArrayBase_hkpActionListener___ {
    pub m_data: *mut *mut hkpActionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkJobQueue__CustomJobTypeSetup_ {
    pub m_data: *mut hkJobQueue__CustomJobTypeSetup,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpWorldExtension___ {
    pub m_data: *mut *mut hkpWorldExtension,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpIslandPostIntegrateListener___ {
    pub m_data: *mut *mut hkpIslandPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpContactImpulseLimitBreachedListener___ {
    pub m_data: *mut *mut hkpContactImpulseLimitBreachedListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpConstraintListener___ {
    pub m_data: *mut *mut hkpConstraintListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpIslandPostCollideListener___ {
    pub m_data: *mut *mut hkpIslandPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpWorldPostCollideListener___ {
    pub m_data: *mut *mut hkpWorldPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkMemoryAllocator__MemoryStatistics {
    pub m_allocated: i32,
    pub m_inUse: i32,
    pub m_peakInUse: i32,
    pub m_available: i32,
    pub m_totalAvailable: i32,
    pub m_largestBlock: i32,
}

#[repr(C)]
pub struct hkPadSpu_float_ {
    pub m_storage: f32,
}

#[repr(C)]
pub struct hkArrayBase_unsigned_char_ {
    pub m_data: *mut u8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpPhantomOverlapListener {
    pub __vfptr: *const hkpPhantomOverlapListener____vftable,
}

#[repr(C)]
pub struct hkpPhantomOverlapListener____vftable {
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
pub struct hkArrayBase_hkpWorldPostIntegrateListener___ {
    pub m_data: *mut *mut hkpWorldPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpWorldPostSimulationListener___ {
    pub m_data: *mut *mut hkpWorldPostSimulationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkSmallArray_hkpEntityListener___ {
    pub m_data: *mut *mut hkpEntityListener,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkSimplePropertyValue {
    pub m_data: u64,
}

#[repr(C)]
pub struct hkpShapeCollectionFilter {
    pub __vfptr: *const hkpShapeCollectionFilter____vftable,
}

#[repr(C)]
pub struct hkpShapeCollectionFilter____vftable {
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpShapeCollectionFilter,
        result: *mut hkBool,
        _: *const hkpCollisionInput,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpShapeContainer,
        _: *const hkpShapeContainer,
        _: u32,
        _: u32,
    ) -> *mut hkBool,
    pub isCollisionEnabled_2: unsafe extern "thiscall" fn(
        this: *const hkpShapeCollectionFilter,
        result: *mut hkBool,
        _: *const hkpCollisionInput,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpShapeContainer,
        _: u32,
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
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpShapeCollectionFilter, _: u32) -> *mut (),
}

#[repr(C)]
pub struct hkpEntity__ExtendedListeners {
    pub m_activationListeners: hkSmallArray_hkpEntityActivationListener___,
    pub m_entityListeners: hkSmallArray_hkpEntityListener___,
}

#[repr(C)]
pub struct hkpEntity__SpuCollisionCallback {
    pub m_util: *mut hkSpuCollisionCallbackUtil,
    pub m_capacity: u16,
    pub m_eventFilter: u8,
    pub m_userFilter: u8,
}

#[repr(C)]
pub struct hkArray_hkpWorldExtension___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpWorldExtension,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpWorldExtension___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpWorldExtension____ptr(
        &self,
    ) -> *const hkArrayBase_hkpWorldExtension___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpWorldExtension____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpWorldExtension___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
}

impl std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    pub fn as_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0__ptr(&self) -> *const std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_ {
        self as *const _ as _
    }

    pub fn as_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0__mut_ptr(&mut self) -> *mut std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___ {
    pub fn as_std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue____ptr(
        &self,
    ) -> *const std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue____mut_ptr(
        &mut self,
    ) -> *mut std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
}

impl std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____ptr(&self) -> *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____mut_ptr(&mut self) -> *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
}

impl std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    pub fn as_std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____ptr(&self) -> *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____mut_ptr(&mut self) -> *mut std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Myval: std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
}

impl std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {
    pub fn as_std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____ptr(&self) -> *const std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0____mut_ptr(&mut self) -> *mut std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
    pub first: *mut gfc__Class,
    pub second: gfc__AutoRef_gfc__WorldComponent_,
}

#[repr(C)]
pub struct std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
    pub first: *mut gfc__Class,
    pub second: gfc__AutoRef_gfc__WorldComponent_,
}

impl std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
    pub fn as_std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent____ptr(
        &self,
    ) -> *const std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
        self as *const _ as _
    }

    pub fn as_std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent____mut_ptr(
        &mut self,
    ) -> *mut std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkSmallArray_hkConstraintInternal_ {
    pub m_data: *mut hkConstraintInternal,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpWorldPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpWorldPostIntegrateListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpWorldPostIntegrateListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpWorldPostIntegrateListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpWorldPostIntegrateListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpWorldPostSimulationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpWorldPostSimulationListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpWorldPostSimulationListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpWorldPostSimulationListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpWorldPostSimulationListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpSimpleContactConstraintDataInfo {
    pub m_flags: u16,
    pub m_biNormalAxis: u16,
    pub m_rollingFrictionMultiplier: hkHalf,
    pub m_internalData1: hkHalf,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_rhsRolling: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 4],
    pub m_contactRadius: f32,
    pub m_data: [f32; 4],
}

#[repr(C)]
pub struct hkpSimpleContactConstraintAtom {
    pub m_type: hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_,
    pub m_sizeOfAllAtoms: u16,
    pub m_numContactPoints: u16,
    pub m_numReservedContactPoints: u16,
    pub m_numUserDatasForBodyA: u8,
    pub m_numUserDatasForBodyB: u8,
    pub m_contactPointPropertiesStriding: u8,
    pub m_maxNumContactPoints: u16,
    pub m_info: hkpSimpleContactConstraintDataInfo,
}

impl hkpSimpleContactConstraintAtom {
    pub fn as_hkpConstraintAtom_ptr(&self) -> *const hkpConstraintAtom {
        self as *const _ as _
    }

    pub fn as_hkpConstraintAtom_mut_ptr(&mut self) -> *mut hkpConstraintAtom {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpRayShapeCollectionFilter {
    pub __vfptr: *const hkpRayShapeCollectionFilter____vftable,
}

#[repr(C)]
pub struct hkpRayShapeCollectionFilter____vftable {
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpRayShapeCollectionFilter,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *const hkpShapeContainer,
        _: u32,
    ) -> *mut hkBool,
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpRayShapeCollectionFilter, _: u32) -> *mut (),
}

#[repr(C)]
pub struct List_gfc__AutoRef_gfc__WorldObject___ {
    pub mList: *mut List_gfc__AutoRef_gfc__WorldObject_____ListNode,
    pub mTail: *mut List_gfc__AutoRef_gfc__WorldObject_____ListNode,
    pub mSize: i32,
}

#[repr(C)]
pub struct hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpPhantomOverlapListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpPhantomOverlapListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpPhantomOverlapListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpPhantomOverlapListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpPhantomOverlapListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollidableRemovedEvent {
    pub m_phantom: *const hkpPhantom,
    pub m_collidable: *const hkpCollidable,
    pub m_collidableWasAdded: hkBool,
}

#[repr(C)]
pub struct hkSmallArray_hkpAction___ {
    pub m_data: *mut *mut hkpAction,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkpLinkedCollidable {
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
    pub m_collisionEntries: hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_,
}

impl hkpLinkedCollidable {
    pub fn as_hkpCollidable_ptr(&self) -> *const hkpCollidable {
        self as *const _ as _
    }

    pub fn as_hkpCollidable_mut_ptr(&mut self) -> *mut hkpCollidable {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpLinkedCollidable__CollisionEntry {
    pub m_agentEntry: *mut hkpAgentNnEntry,
    pub m_partner: *mut hkpLinkedCollidable,
}

#[repr(C)]
pub struct hkpProcessCollisionOutput__PotentialInfo {
    pub m_firstFreePotentialContact: *mut hkpProcessCollisionOutput__ContactRef,
    pub m_firstFreeRepresentativeContact: *mut *mut hkpProcessCdPoint,
    pub m_representativeContacts: [*mut hkpProcessCdPoint; 256],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_potentialContacts: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 3072],
}

#[repr(C)]
pub struct hkpProcessCollisionOutput__ContactRef {
    pub m_contactPoint: *mut hkpProcessCdPoint,
    pub m_agentEntry: *mut hkpAgentEntry,
    pub m_agentData: *mut (),
}

#[repr(C)]
pub struct hkArray_hkpConstraintListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpConstraintListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpConstraintListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpConstraintListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpConstraintListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpConstraintListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpConstraintListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkArray_int_hkContainerHeapAllocator___ {
    pub m_data: *mut hkArray_int_hkContainerHeapAllocator_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_unsigned_short_ {
    pub m_data: *mut u16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkpAction___ {
    pub m_data: *mut *mut hkpAction,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpContactImpulseLimitBreachedListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpContactImpulseLimitBreachedListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpContactImpulseLimitBreachedListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpContactImpulseLimitBreachedListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpContactImpulseLimitBreachedListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpContactImpulseLimitBreachedListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpContactImpulseLimitBreachedListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMaxSizeMotion {
    pub __vfptr: *const hkpMaxSizeMotion____vftable,
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
    __pdbindgen_padding: [u8; 32],
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
}

impl hkpMaxSizeMotion {
    pub fn as_hkpKeyframedRigidMotion_ptr(&self) -> *const hkpKeyframedRigidMotion {
        self as *const _ as _
    }

    pub fn as_hkpKeyframedRigidMotion_mut_ptr(&mut self) -> *mut hkpKeyframedRigidMotion {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpMaxSizeMotion____vftable {
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
    pub setStepPosition:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: f32, _: f32),
    pub setStoredMotion:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *mut hkpMaxSizeMotion),
}

#[repr(C)]
pub struct hkpTypedBroadPhaseHandle {
    pub m_id: u32,
    pub m_type: i8,
    pub m_ownerOffset: i8,
    pub m_objectQualityType: i8,
    pub m_collisionFilterInfo: u32,
}

impl hkpTypedBroadPhaseHandle {
    pub fn as_hkpBroadPhaseHandle_ptr(&self) -> *const hkpBroadPhaseHandle {
        self as *const _ as _
    }

    pub fn as_hkpBroadPhaseHandle_mut_ptr(&mut self) -> *mut hkpBroadPhaseHandle {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_ {
    pub m_data: *mut u8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    pub m_storage: [u8; 8],
}

impl hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_ {
    pub fn as_hkArray_unsigned_char_hkContainerHeapAllocator__ptr(
        &self,
    ) -> *const hkArray_unsigned_char_hkContainerHeapAllocator_ {
        self as *const _ as _
    }

    pub fn as_hkArray_unsigned_char_hkContainerHeapAllocator__mut_ptr(
        &mut self,
    ) -> *mut hkArray_unsigned_char_hkContainerHeapAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCollidable__BoundingVolumeData {
    pub m_min: [u32; 3],
    pub m_expansionMin: [u8; 3],
    pub m_expansionShift: u8,
    pub m_max: [u32; 3],
    pub m_expansionMax: [u8; 3],
    pub m_padding: u8,
    pub m_numChildShapeAabbs: u16,
    pub m_capacityChildShapeAabbs: u16,
    pub m_childShapeAabbs: *mut hkAabbUint32,
    pub m_childShapeKeys: *mut u32,
}

#[repr(C)]
pub struct hkpContactListener {
    pub __vfptr: *const hkpContactListener____vftable,
}

#[repr(C)]
pub struct hkpContactListener____vftable {
    pub contactPointCallback:
        unsafe extern "thiscall" fn(this: *mut hkpContactListener, _: *const hkpContactPointEvent),
    pub collisionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpContactListener, _: *const hkpCollisionEvent),
    pub collisionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpContactListener, _: *const hkpCollisionEvent),
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpContactListener, _: u32) -> *mut (),
    pub contactPointAddedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpContactListener,
        _: *mut hkpContactPointAddedEvent,
    ),
    pub contactPointRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpContactListener,
        _: *mut hkpContactPointRemovedEvent,
    ),
    pub contactProcessCallback:
        unsafe extern "thiscall" fn(this: *mut hkpContactListener, _: *mut hkpContactProcessEvent),
}

#[repr(C)]
pub struct hkJobQueue__JobQueueEntryInput {
    pub m_jobPriority: hkPadSpu_unsigned_int_,
    __pdbindgen_padding: [u8; 12],
    pub m_job: hkJobQueue__JobQueueEntry,
}

#[repr(C)]
pub struct hkJobQueue__JobQueueEntry {
    pub m_jobSubType: u8,
    pub m_jobType: hkEnum_enum_hkJobType_unsigned_char_,
    pub m_jobSpuType: hkEnum_enum_hkJobSpuType_unsigned_char_,
    pub m_size: u16,
    pub m_threadAffinity: i16,
    __pdbindgen_padding: [u8; 8],
    pub m_data: [u8; 112],
}

impl hkJobQueue__JobQueueEntry {
    pub fn as_hkJob_ptr(&self) -> *const hkJob {
        self as *const _ as _
    }

    pub fn as_hkJob_mut_ptr(&mut self) -> *mut hkJob {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkSmallArray_hkpEntityActivationListener___ {
    pub m_data: *mut *mut hkpEntityActivationListener,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkpAgentNnSector {
    pub m_data: [u8; 512],
}

#[repr(C)]
pub struct hkArray_hkpContactListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpContactListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpContactListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpContactListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpContactListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpContactListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkpContactListener___ {
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkGskCache16 {
    pub m_vertices: [u16; 4],
    pub m_dimA: u8,
    pub m_dimB: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimA: compile_error!("unimplemented feature: type kind 0x1205"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1205")]
    pub m_maxDimB: compile_error!("unimplemented feature: type kind 0x1205"),
    __pdbindgen_padding: [u8; 1],
    pub m_gskFlags: u8,
    __pdbindgen_padding_2: [u8; 4],
}

impl hkGskCache16 {
    pub fn as_hkpGskCache_ptr(&self) -> *const hkpGskCache {
        self as *const _ as _
    }

    pub fn as_hkpGskCache_mut_ptr(&mut self) -> *mut hkpGskCache {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_ {
    pub m_data: *mut hkJobQueue__CustomJobTypeSetup,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkJobQueue__CustomJobTypeSetup__ptr(
        &self,
    ) -> *const hkArrayBase_hkJobQueue__CustomJobTypeSetup_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkJobQueue__CustomJobTypeSetup__mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkJobQueue__CustomJobTypeSetup_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkpIslandActivationListener___ {
    pub m_data: *mut *mut hkpIslandActivationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkViewPtr_hkpConstraintInstance___ {
    pub m_data: *mut hkViewPtr_hkpConstraintInstance_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkAabbUint32 {
    pub m_min: [u32; 3],
    pub m_expansionMin: [u8; 3],
    pub m_expansionShift: u8,
    pub m_max: [u32; 3],
    pub m_expansionMax: [u8; 3],
    pub m_shapeKeyByte: u8,
}

#[repr(C)]
pub struct hkSemaphore {
    pub m_semaphore: *mut (),
}

#[repr(C)]
pub struct hkRefPtr_hkWorldMemoryAvailableWatchDog_ {
    pub m_pntr: *mut hkWorldMemoryAvailableWatchDog,
}

#[repr(C)]
pub struct hkpConstraintOwner {
    pub __vfptr: *const hkpConstraintOwner____vftable,
    pub m_memSizeAndRefCount: u32,
    pub m_constraintInfo: hkpConstraintInfo,
}

impl hkpConstraintOwner {
    pub fn as_hkReferencedObject_ptr(&self) -> *const hkReferencedObject {
        self as *const _ as _
    }

    pub fn as_hkReferencedObject_mut_ptr(&mut self) -> *mut hkReferencedObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpConstraintOwner____vftable {
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
pub struct hkBool {
    pub m_bool: i8,
}

#[repr(C)]
pub struct hkArrayBase_hkpLinkedCollidable__CollisionEntry_ {
    pub m_data: *mut hkpLinkedCollidable__CollisionEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkStackTracer__CallTree {
    pub m_nodes: hkArrayBase_hkStackTracer__CallTree__Node_,
    pub m_allocator: *mut hkMemoryAllocator,
    pub m_rootNode: i32,
    pub m_firstFreeNode: i32,
}

#[repr(C)]
pub struct hkStackTracer__CallTree__Node {
    pub m_value: u32,
    pub m_parent: i32,
    pub m_firstChild: i32,
    pub m_next: i32,
    pub m_usageCount: i32,
}

#[repr(C)]
pub struct hkArray_char_hkContainerTempAllocator_ {
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_char_hkContainerTempAllocator_ {
    pub fn as_hkArrayBase_char__ptr(&self) -> *const hkArrayBase_char_ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_char__mut_ptr(&mut self) -> *mut hkArrayBase_char_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkpCdBody {
    pub m_shape: *const hkpShape,
    pub m_shapeKey: u32,
    pub m_motion: *const (),
    pub m_parent: *const hkpCdBody,
}

#[repr(C)]
pub struct hkArray_hkpActionListener___hkContainerHeapAllocator_ {
    pub m_data: *mut *mut hkpActionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkpActionListener___hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkpActionListener____ptr(
        &self,
    ) -> *const hkArrayBase_hkpActionListener___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkpActionListener____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkpActionListener___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct hkArrayBase_hkpPhantomListener___ {
    pub m_data: *mut *mut hkpPhantomListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkPadSpu_hkpProcessCdPoint___ {
    pub m_storage: *mut hkpProcessCdPoint,
}

#[repr(C)]
pub struct hkArrayBase_hkpWorldDeletionListener___ {
    pub m_data: *mut *mut hkpWorldDeletionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkpRayCollidableFilter {
    pub __vfptr: *const hkpRayCollidableFilter____vftable,
}

#[repr(C)]
pub struct hkpRayCollidableFilter____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpRayCollidableFilter, _: u32) -> *mut (),
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpRayCollidableFilter,
        result: *mut hkBool,
        _: *const hkpWorldRayCastInput,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
}

#[repr(C)]
pub struct hkpConstraintInfo {
    pub m_maxSizeOfSchema: i32,
    pub m_sizeOfSchemas: i32,
    pub m_numSolverResults: i32,
    pub m_numSolverElemTemps: i32,
}

#[repr(C)]
pub struct hkpBroadPhaseListener {
    pub __vfptr: *const hkpBroadPhaseListener____vftable,
}

#[repr(C)]
pub struct hkpBroadPhaseListener____vftable {
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
pub struct hkpContactPointRemovedEvent {
    pub m_contactPointId: u16,
    pub m_contactPointProperties: *mut hkpContactPointProperties,
    pub m_entityA: *mut hkpEntity,
    pub m_entityB: *mut hkpEntity,
    pub m_callbackFiredFrom: *mut hkpEntity,
    pub m_internalContactMgr: *mut hkpDynamicsContactMgr,
    pub m_constraintOwner: *mut hkpConstraintOwner,
}

#[repr(C)]
pub struct hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_ {
    pub m_data: *mut hkViewPtr_hkpConstraintInstance_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

impl hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_ {
    pub fn as_hkArrayBase_hkViewPtr_hkpConstraintInstance____ptr(
        &self,
    ) -> *const hkArrayBase_hkViewPtr_hkpConstraintInstance___ {
        self as *const _ as _
    }

    pub fn as_hkArrayBase_hkViewPtr_hkpConstraintInstance____mut_ptr(
        &mut self,
    ) -> *mut hkArrayBase_hkViewPtr_hkpConstraintInstance___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Vector_gfc__Occluder___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__Occluder,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Clipper {
    pub pOccluders: *const gfc__Vector_gfc__Occluder___0_gfc__CAllocator_,
    pub numPlanes: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub planes: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 1024],
}

#[repr(C)]
pub struct gfc__VisScriptVariable {
    pub __vfptr: *const gfc__VisScriptVariable____vftable,
    pub ReferenceCount: i32,
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    pub mName: gfc__HString,
    pub mRead: bool,
    pub mWrite: bool,
    pub mExternal: bool,
    pub mModContainerModule: *mut gfc__ModSysContainerModule,
    pub mConnectionID: u32,
}

impl gfc__VisScriptVariable {
    pub fn as_gfc__VisScriptEntity_ptr(&self) -> *const gfc__VisScriptEntity {
        self as *const _ as _
    }

    pub fn as_gfc__VisScriptEntity_mut_ptr(&mut self) -> *mut gfc__VisScriptEntity {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__VisScriptVariable____vftable {
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
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *const i8,
    pub compile:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity),
    pub clearDeadLinks:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub getColor: unsafe extern "thiscall" fn(
        this: *const gfc__VisScriptVariable,
        result: *mut gfc__TVector4_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector4_float_gfc__FloatMath_,
    pub getVariableType: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> i32,
    pub isArray: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> bool,
    pub getVariableValue: unsafe extern "thiscall" fn(
        this: *const gfc__VisScriptVariable,
        result: *mut gfc__AutoRef_gfc__Value_,
    ) -> *mut gfc__AutoRef_gfc__Value_,
    pub setVariableValue:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: gfc__AutoRef_gfc__Value_),
}

#[repr(C)]
pub struct gfc__StateMapValue {
    pub mValueMode: gfc__StateMapValue__ValueMode,
    pub mStringValue: gfc__String,
    pub mNumberValue: f64,
}

#[repr(C)]
pub struct gfc__ModuleInputLink {
    pub ModuleID: u32,
    pub SrcProperty: gfc__HString,
    pub DestProperty: gfc__HString,
    pub _Module: *mut gfc__VisScriptModule,
    pub _SrcProperty: *mut gfc__Property,
    pub _DestProperty: *mut gfc__Property,
}

#[repr(C)]
pub struct gfc__Occluder {
    pub Center: gfc__TVector3_float_gfc__FloatMath_,
    pub Planes: *mut gfc__Plane,
    pub PlaneCount: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__FullScreenFXGroup__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FullScreenFXGroup_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ModuleSystem {
    pub __vfptr: *const gfc__ModuleSystem____vftable,
    pub ReferenceCount: i32,
    pub mViewXOffset: f32,
    pub mViewYOffset: f32,
    pub mViewZoom: f32,
    pub mEntityIDGen: i32,
    pub mEntities: gfc__Vector_gfc__AutoRef_gfc__VisScriptEntity__0_gfc__CAllocator_,
    pub mContext: *mut gfc__Object,
    pub mOwner: *mut gfc__WorldObject,
    pub mStateMap: std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____,
    pub mModuleIDGen: i32,
    pub mModules: gfc__Vector_gfc__AutoRef_gfc__VisScriptModule__0_gfc__CAllocator_,
}

impl gfc__ModuleSystem {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ModuleSystem____vftable {
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
    pub start: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem),
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__CameraCinematicGroup__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CameraCinematicGroup_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VisScriptModule {
    pub __vfptr: *const gfc__VisScriptModule____vftable,
    pub ReferenceCount: i32,
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    pub mEventLinks: gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_,
    pub mInputLinks: gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_,
    pub mVariableLinks: gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_,
}

impl gfc__VisScriptModule {
    pub fn as_gfc__VisScriptEntity_ptr(&self) -> *const gfc__VisScriptEntity {
        self as *const _ as _
    }

    pub fn as_gfc__VisScriptEntity_mut_ptr(&mut self) -> *mut gfc__VisScriptEntity {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__VisScriptModule____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object, result: *mut gfc__HString) -> *mut gfc__HString,
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
    pub getVariableConnectionInfo: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_, _: i32) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub doEvent: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub getVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__AutoRef_gfc__Value_, _: u32) -> *mut gfc__AutoRef_gfc__Value_,
    pub setVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32, _: gfc__AutoRef_gfc__Value_),
    pub tryAgain: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule),
    pub getVariablesIn: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub getVariablesOut: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub executeInternal: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub hasVariableIn: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> bool,
    pub hasVariableOut: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> bool,
}

#[repr(C)]
pub struct gfc__ModuleVariableLink {
    pub isInput: bool,
    pub VariableConnectionID: u32,
    pub VariableID: u32,
    pub _Variable: *mut gfc__VisScriptVariable,
}

#[repr(C)]
pub struct gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneObject,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneCell,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VisScriptEntity {
    pub __vfptr: *const gfc__VisScriptEntity____vftable,
    pub ReferenceCount: i32,
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
}

impl gfc__VisScriptEntity {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__VisScriptEntity____vftable {
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
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *const i8,
    pub compile:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity),
    pub clearDeadLinks:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VisScriptEntity_ {
    pub p: *mut gfc__IRefObject,
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
pub struct gfc__Visual {
    pub __vfptr: *const gfc__Visual____vftable,
    pub ReferenceCount: i32,
    pub mFadeStartDistance: f32,
    pub mFadeEndDistance: f32,
    pub mInvertFade: bool,
    pub mLightGroup: u32,
    pub mCharacterShadow: bool,
    pub mForceFade: f32,
    pub mObjectColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mObject: *mut gfc__Object3D,
}

impl gfc__Visual {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Visual____vftable {
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
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> bool,
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: i32),
    pub setMaterialOverride: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        _: *const gfc__HString,
        _: gfc__AutoRef_gfc__Material_,
    ),
    pub setMaterial: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        _: *const gfc__HString,
        _: gfc__AutoRef_gfc__Material_,
    ),
    pub clearMaterialOverride: unsafe extern "thiscall" fn(this: *mut gfc__Visual),
    pub getMaterialCount: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> i32,
    pub getMaterialAt: unsafe extern "thiscall" fn(
        this: *const gfc__Visual,
        result: *mut gfc__AutoRef_gfc__Material_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Material_,
    pub setMaterialAt:
        unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: i32, _: gfc__AutoRef_gfc__Material_),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ) -> bool,
    pub getBoundingVolume:
        unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: *mut gfc__BoundingVolume) -> bool,
    pub getForceFade: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> f32,
    pub setForceFade: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: f32),
    pub getObjectColor: unsafe extern "thiscall" fn(
        this: *const gfc__Visual,
    )
        -> *const gfc__TVector3_float_gfc__FloatMath_,
    pub setObjectColor: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRenderLightGroup: unsafe extern "thiscall" fn(this: *mut gfc__Visual) -> u32,
    pub setCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub getCastShadows: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> bool,
    pub setAlphaShadows: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub getAlphaShadows: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> bool,
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__Visual),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Visual),
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
pub struct gfc__StaticMeshVisual {
    pub __vfptr: *const gfc__StaticMeshVisual____vftable,
    pub ReferenceCount: i32,
    pub mFadeStartDistance: f32,
    pub mFadeEndDistance: f32,
    pub mInvertFade: bool,
    pub mLightGroup: u32,
    pub mCharacterShadow: bool,
    pub mForceFade: f32,
    pub mObjectColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mObject: *mut gfc__Object3D,
    pub __vfptr_2: *const gfc__SceneObject____vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub __vfptr_3: *const gfc__IRenderCallback____vftable,
    pub mLocked: bool,
    pub mChecksum: u32,
    pub mRefNode: gfc__HString,
    pub mMeshName: gfc__HString,
    pub mMeshID: i32,
    pub mFade: f32,
    pub mWorldBBoxVersion: i32,
    pub mDrawOrder: u32,
    pub mDecalOrder: u32,
    pub mDecalBias: f32,
    pub mMesh: gfc__AutoRef_gfc__StaticMesh_,
    pub mRenderNodes: *mut gfc__RenderNode,
    pub mMaterials: *mut gfc__StaticMeshVisual__MeshMaterial,
    pub mMaterialCount: i32,
    pub mNode: gfc__AutoRef_gfc__Node3D_,
    pub mRenderNodeFlags: gfc__TFlags_unsigned_long_,
    pub mStaticLightingInfo: *mut gfc__StaticLightingVisualOpt,
}

impl gfc__StaticMeshVisual {
    pub fn as_gfc__Visual_ptr(&self) -> *const gfc__Visual {
        self as *const _ as _
    }

    pub fn as_gfc__Visual_mut_ptr(&mut self) -> *mut gfc__Visual {
        self as *mut _ as _
    }

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
pub struct gfc__StaticMeshVisual____vftable {
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
    pub getForceFade: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> f32,
    pub setForceFade: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: f32),
    pub getObjectColor: unsafe extern "thiscall" fn(
        this: *const gfc__Visual,
    )
        -> *const gfc__TVector3_float_gfc__FloatMath_,
    pub setObjectColor: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRenderLightGroup: unsafe extern "thiscall" fn(this: *mut gfc__Visual) -> u32,
    pub setCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub getCastShadows: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> bool,
    pub setAlphaShadows: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub getAlphaShadows: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> bool,
    pub attach: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__Visual),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: bool),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Visual),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub setDoNotSubmit: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub getDoNotSubmit: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
}

#[repr(C)]
pub struct gfc__StaticMeshVisual__MeshMaterial {
    pub mMaterial: gfc__AutoRef_gfc__Material_,
    pub mOverride: gfc__AutoRef_gfc__Material_,
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
pub struct gfc__DetectorObject {
    pub __vfptr: *const gfc__DetectorObject____vftable,
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
    pub mShape: i32,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
    pub mBounds: gfc__TBox_float_gfc__FloatMath_,
    pub mGizmo: *mut gfc__PhysicsShapeGizmo,
    pub mBodyType: u8,
    pub mRegion: *mut gfc__DetectorRegion,
    pub mEnabled: bool,
}

impl gfc__DetectorObject {
    pub fn as_gfc__PhysicsShapeObject_ptr(&self) -> *const gfc__PhysicsShapeObject {
        self as *const _ as _
    }

    pub fn as_gfc__PhysicsShapeObject_mut_ptr(&mut self) -> *mut gfc__PhysicsShapeObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__DetectorObject____vftable {
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
    pub cacheBoundingVolumes: unsafe extern "thiscall" fn(this: *mut gfc__SceneManager),
    pub cacheBoundingVolumes_2: unsafe extern "thiscall" fn(
        this: *mut gfc__SceneManager,
        _: *mut gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
        _: i32,
    ),
}

#[repr(C)]
pub struct gfc__PhysicsShapeObject {
    pub __vfptr: *const gfc__PhysicsShapeObject____vftable,
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
    pub mShape: i32,
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: gfc__TVector3_float_gfc__FloatMath_,
    pub mBounds: gfc__TBox_float_gfc__FloatMath_,
    pub mGizmo: *mut gfc__PhysicsShapeGizmo,
}

impl gfc__PhysicsShapeObject {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__PhysicsShapeObject____vftable {
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
}

#[repr(C)]
pub struct gfc__WorldGroup {
    pub __vfptr: *const gfc__WorldGroup____vftable,
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
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mObjects: List_gfc__AutoRef_gfc__WorldObject___,
}

impl gfc__WorldGroup {
    pub fn as_gfc__WorldObject_ptr(&self) -> *const gfc__WorldObject {
        self as *const _ as _
    }

    pub fn as_gfc__WorldObject_mut_ptr(&mut self) -> *mut gfc__WorldObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldGroup____vftable {
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
    pub setPropertyValue: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *const gfc__String,
        _: gfc__AutoRef_gfc__Value_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VisScriptModule_ {
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
        result: *mut hkBool,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
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
    __pdbindgen_padding: [u8; 4],
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
    __pdbindgen_padding: [u8; 32],
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_data: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24576],
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
    __pdbindgen_padding: [u8; 4],
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
pub struct hkArrayBase_hkLocalFrame_const___ {
    pub m_data: *mut *const hkLocalFrame,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
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
pub struct List_gfc__AutoRef_gfc__WorldObject_____ListNode {
    pub next: *mut List_gfc__AutoRef_gfc__WorldObject_____ListNode,
    pub value: gfc__AutoRef_gfc__WorldObject_,
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
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
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
pub struct gfc__CollisionObject {
    pub __vfptr: *const gfc__CollisionObject____vftable,
    pub mCollisionManager: *mut gfc__CollisionManager,
}

#[repr(C)]
pub struct gfc__CollisionObject____vftable {
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
    pub __vfptr: *const gfc__RegionLayer____vftable,
    pub ReferenceCount: i32,
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

impl gfc__RegionLayer {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__RegionLayer____vftable {
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
pub struct gfc__Object3DCache {
    pub __vfptr: *const gfc__Object3DCache____vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
}

impl gfc__Object3DCache {
    pub fn as_gfc__ResourceCache_ptr(&self) -> *const gfc__ResourceCache {
        self as *const _ as _
    }

    pub fn as_gfc__ResourceCache_mut_ptr(&mut self) -> *mut gfc__ResourceCache {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Object3DCache____vftable {
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
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
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
    pub getStatic: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
}

#[repr(C)]
pub struct gfc__Body {
    pub __vfptr: *const gfc__Body____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc__CollisionObject____vftable,
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
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
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
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
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
    pub m_dispatcher: hkPadSpu_hkpCollisionDispatcher___,
    pub m_weldClosestPoints: hkPadSpu_unsigned_int_,
    pub m_forceAcceptContactPoints: hkPadSpu_unsigned_int_,
    pub m_tolerance: hkPadSpu_float_,
    pub m_filter: hkPadSpu_hkpCollisionFilter_const___,
    pub m_convexListFilter: hkPadSpu_hkpConvexListFilter_const___,
    pub m_createPredictiveAgents: hkPadSpu_unsigned_int_,
    __pdbindgen_padding: [u8; 4],
    pub m_aabb32Info: hkpCollisionInput__Aabb32Info,
    pub m_stepInfo: hkStepInfo,
    pub m_collisionQualityInfo: hkPadSpu_hkpCollisionQualityInfo___,
    pub m_spareAgentSector: *mut hkpAgent1nSector,
    pub m_dynamicsInfo: *mut (),
    pub m_enableDeprecatedWelding: hkBool,
    pub m_allowToSkipConfirmedCallbacks: hkBool,
    pub m_config: *mut hkpCollisionAgentConfig,
    __pdbindgen_padding_2: [u8; 12],
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
    pub __vfptr_2: *const hkpBroadPhaseListener____vftable,
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
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_broadPhaseListeners: compile_error!("unimplemented feature: sizeof array 0x0"),
    __pdbindgen_padding: [u8; 256],
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
    pub __vfptr_2: *const gfc__IRenderCallback____vftable,
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
    pub first: gfc__String,
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
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Object) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Object) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__Object, result: *mut gfc__HString) -> *mut gfc__HString,
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
    pub getVariableConnectionInfo: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule, result: *mut gfc__AutoRef_gfc__VariableConnectionInfo_, _: i32) -> *mut gfc__AutoRef_gfc__VariableConnectionInfo_,
    pub doEvent: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub execute: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32),
    pub getVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__AutoRef_gfc__Value_, _: u32) -> *mut gfc__AutoRef_gfc__Value_,
    pub setVariableValue: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32, _: gfc__AutoRef_gfc__Value_),
    pub tryAgain: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule),
    pub getVariablesIn: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
    pub getVariablesOut: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_, _: u32) -> *mut gfc__Vector_gfc__AutoRef_gfc__VisScriptVariable__0_gfc__CAllocator_,
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
pub struct gfc__OOObjectWriter {
    pub __vfptr: *const gfc__OOObjectWriter____vftable,
    pub ReferenceCount: i32,
    pub mObjectDatabase: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mWriteDefaults: bool,
}

impl gfc__OOObjectWriter {
    pub fn as_gfc__ObjectWriter_ptr(&self) -> *const gfc__ObjectWriter {
        self as *const _ as _
    }

    pub fn as_gfc__ObjectWriter_mut_ptr(&mut self) -> *mut gfc__ObjectWriter {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__OOObjectWriter____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct UserStatsReceived_t {
    pub m_nGameID: u64,
    pub m_eResult: EResult,
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
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
    pub __vfptr_2: *const keen__ISteamStatsCallback____vftable,
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
        unsafe extern "thiscall" fn(this: *mut keen__ISteamStatsCallback, _: u32) -> *mut (),
    pub onUserStatsReceived: unsafe extern "thiscall" fn(
        this: *mut keen__ISteamStatsCallback,
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
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
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct ID3D11DeviceContext {
    pub lpVtbl: *mut IUnknownVtbl,
}

impl ID3D11DeviceContext {
    pub fn as_ID3D11DeviceChild_ptr(&self) -> *const ID3D11DeviceChild {
        self as *const _ as _
    }

    pub fn as_ID3D11DeviceChild_mut_ptr(&mut self) -> *mut ID3D11DeviceChild {
        self as *mut _ as _
    }
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub vertexData: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
}
