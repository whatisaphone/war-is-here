#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{types2::*, types3::*};

#[repr(C)]
pub struct unit4__StatUpdateData {
    pub statId: u32,
    pub statName: [i8; 64],
}

#[repr(C)]
pub struct keen__float2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct keen__MemoryAllocator {
    pub __vfptr: *const keen__MemoryAllocator____vftable,
}

#[repr(C)]
pub struct keen__MemoryAllocator____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__MemoryAllocator, _: u32) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryAllocator,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(this: *mut keen__MemoryAllocator, _: *mut ()),
    pub getName: unsafe extern "thiscall" fn(this: *const keen__MemoryAllocator) -> *const i8,
}

#[repr(C)]
pub struct keen__Mutex {
    pub m_name: [i8; 32],
    pub m_criticalSectionData: [u32; 6],
}

#[repr(C)]
pub struct keen__Event {
    pub m_eventHandle: *mut (),
}

#[repr(C)]
pub struct keen__Thread {
    pub m_threadHandle: *mut (),
    pub m_threadId: u32,
    pub m_pArgument: *mut (),
    pub m_identifier: [i8; 64],
    pub m_quitRequested: bool,
    pub m_pFunction: *mut unsafe extern "C" fn(_: *const keen__Thread) -> i32,
}

#[repr(C)]
pub struct _GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

#[repr(C)]
pub struct CSteamID {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_steamid: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct CCallbackBase {
    pub __vfptr: *const CCallbackBase____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

#[repr(C)]
pub struct CCallbackBase____vftable {
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct keen__UserAccountSystemBase {
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub runningOperations: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
}

#[repr(C)]
pub struct keen__UserAccountSystemCreationParameters {
    pub pSteamworksSystem: *mut keen__SteamworksSystem,
    pub userNameWin32: [i8; 256],
}

#[repr(C)]
pub struct keen__UserAccount {
    pub guestIndex: u8,
    pub isLoggedIn: bool,
    pub id: keen__UserAccountId,
    pub name: keen__UserAccountName,
}

#[repr(C)]
pub struct keen__UserAccountName {
    pub providerData: [i8; 257],
}

#[repr(C)]
pub struct keen__UserAccountId {
    pub providerData: [u8; 8],
    pub isValid: bool,
}

#[repr(C)]
pub struct keen__UserAccountSystem {
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub runningOperations: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub accounts: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 4256],
    pub accountCount: u32,
}

impl keen__UserAccountSystem {
    pub fn as_keen__UserAccountSystemBase_ptr(&self) -> *const keen__UserAccountSystemBase {
        self as *const _ as _
    }

    pub fn as_keen__UserAccountSystemBase_mut_ptr(&mut self) -> *mut keen__UserAccountSystemBase {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct LeaderboardEntry_t {
    pub m_steamIDUser: CSteamID,
    pub m_nGlobalRank: i32,
    pub m_nScore: i32,
    pub m_cDetails: i32,
    pub m_hUGC: u64,
}

#[repr(C)]
pub struct ISteamUserStats {
    pub __vfptr: *const ISteamUserStats____vftable,
}

#[repr(C)]
pub struct ISteamUserStats____vftable {
    pub RequestCurrentStats: unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> bool,
    pub GetStat:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut f32) -> bool,
    pub GetStat_2:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut i32) -> bool,
    pub SetStat:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: f32) -> bool,
    pub SetStat_2:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: i32) -> bool,
    pub UpdateAvgRateStat: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: f32,
        _: f64,
    ) -> bool,
    pub GetAchievement:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut bool) -> bool,
    pub SetAchievement:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8) -> bool,
    pub ClearAchievement:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8) -> bool,
    pub GetAchievementAndUnlockTime: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: *mut bool,
        _: *mut u32,
    ) -> bool,
    pub StoreStats: unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> bool,
    pub GetAchievementIcon:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8) -> i32,
    pub GetAchievementDisplayAttribute: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: *const i8,
    ) -> *const i8,
    pub IndicateAchievementProgress: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: u32,
        _: u32,
    ) -> bool,
    pub GetNumAchievements: unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> u32,
    pub GetAchievementName:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u32) -> *const i8,
    pub RequestUserStats:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: CSteamID) -> u64,
    pub GetUserStat: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: CSteamID,
        _: *const i8,
        _: *mut f32,
    ) -> bool,
    pub GetUserStat_2: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: CSteamID,
        _: *const i8,
        _: *mut i32,
    ) -> bool,
    pub GetUserAchievement: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: CSteamID,
        _: *const i8,
        _: *mut bool,
    ) -> bool,
    pub GetUserAchievementAndUnlockTime: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: CSteamID,
        _: *const i8,
        _: *mut bool,
        _: *mut u32,
    ) -> bool,
    pub ResetAllStats: unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: bool) -> bool,
    pub FindOrCreateLeaderboard: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: ELeaderboardSortMethod,
        _: ELeaderboardDisplayType,
    ) -> u64,
    pub FindLeaderboard:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8) -> u64,
    pub GetLeaderboardName:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> *const i8,
    pub GetLeaderboardEntryCount:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> i32,
    pub GetLeaderboardSortMethod:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> ELeaderboardSortMethod,
    pub GetLeaderboardDisplayType:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> ELeaderboardDisplayType,
    pub DownloadLeaderboardEntries: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: ELeaderboardDataRequest,
        _: i32,
        _: i32,
    ) -> u64,
    pub DownloadLeaderboardEntriesForUsers: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: *mut CSteamID,
        _: i32,
    ) -> u64,
    pub GetDownloadedLeaderboardEntry: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: i32,
        _: *mut LeaderboardEntry_t,
        _: *mut i32,
        _: i32,
    ) -> bool,
    pub UploadLeaderboardScore: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: ELeaderboardUploadScoreMethod,
        _: i32,
        _: *const i32,
        _: i32,
    ) -> u64,
    pub AttachLeaderboardUGC:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64, _: u64) -> u64,
    pub GetNumberOfCurrentPlayers: unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> u64,
    pub RequestGlobalAchievementPercentages:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> u64,
    pub GetMostAchievedAchievementInfo: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *mut i8,
        _: u32,
        _: *mut f32,
        _: *mut bool,
    ) -> i32,
    pub GetNextMostAchievedAchievementInfo: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: i32,
        _: *mut i8,
        _: u32,
        _: *mut f32,
        _: *mut bool,
    ) -> i32,
    pub GetAchievementAchievedPercent:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut f32) -> bool,
    pub RequestGlobalStats: unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: i32) -> u64,
    pub GetGlobalStat:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut f64) -> bool,
    pub GetGlobalStat_2:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8, _: *mut i64) -> bool,
    pub GetGlobalStatHistory: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: *mut f64,
        _: u32,
    ) -> i32,
    pub GetGlobalStatHistory_2: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: *mut i64,
        _: u32,
    ) -> i32,
}

#[repr(C)]
pub struct keen__Array_unsigned_char_ {
    pub m_pData: *mut u8,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__SaveDataSystem {
    pub pAllocator: *mut keen__MemoryAllocator,
    pub saveDataSystemWin32Type: keen__SaveData__SaveDataSystemType,
    pub pProvider: *mut keen__SaveData__SaveDataProviderWin32Interface,
    pub currentOperation: keen__SaveData__CurrentOperation,
    pub currentOperationStatus: keen__SaveData__OperationStatus,
    pub operationProfileIndex: u32,
    pub pOperationProfile: *const keen__SaveData__Profile,
    pub pOperationSaveDataInput: *const keen__SaveData__SaveData,
    pub pOperationSaveDataOutput: *mut keen__SaveData__SaveData,
    pub pOperationStorageData: *mut keen__SaveData__StorageOperation,
    pub suppressSystemDialogs: bool,
}

#[repr(C)]
pub struct keen__SaveDataHandler {
    pub m_state: keen__SaveFlowState,
    pub m_activeClientIndex: u32,
    pub m_currentUserInteraction: keen__SaveDataInteractionData,
    pub m_pSaveDataSystem: *mut keen__SaveDataSystem,
    pub m_saveDataForIO: keen__SaveDataHandler__SaveGameData,
    pub m_clientData: keen__Array_keen__SaveDataHandler__SaveGameClientData_,
    pub m_defaultSaveGameData: keen__Array_unsigned_char_,
    pub m_pCheckSaveGameDataCallback: *mut unsafe extern "C" fn(_: *const (), _: u32) -> bool,
    pub m_saveDataOp: keen__SaveData__StorageOperation,
    pub m_isDefaultSaveGameDataSet: bool,
    pub m_profileList: keen__SizedArray_keen__SaveData__Profile_,
}

#[repr(C)]
pub struct keen__SaveDataHandler__SaveGameClientData {
    pub currentSaveDatas: keen__SizedArray_keen__SaveData__SaveData_,
    pub saveData: keen__SaveDataHandler__SaveGameData,
    pub userAccountId: keen__UserAccountId,
    pub lastSavedDataId: u32,
    pub state: keen__SaveDataHandler__SaveGameClientState,
    pub isSaveEnabled: bool,
    pub isShutdownRequested: bool,
    pub isNewlyCreatedSaveGame: bool,
}

#[repr(C)]
pub struct keen__SaveDataHandler__SaveGameData {
    pub data: keen__Array_unsigned_char_,
    pub id: u32,
}

#[repr(C)]
pub struct keen__SizedArray_keen__SaveData__SaveData_ {
    pub m_pData: *mut keen__SaveData__SaveData,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__SaveDataInteractionData {
    pub id: kaiko__LocalGameSessionInteractionId,
    #[cfg(pdb_issue = "unimplemented feature: enum layout 0x0")]
    pub responseOptions: compile_error!("unimplemented feature: enum layout 0x0"),
    __pdbindgen_padding: [u8; 8],
    pub responseOptionCount: u32,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface {
    pub __vfptr: *const keen__SaveData__SaveDataProviderWin32Interface____vftable,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ) -> *mut (),
    pub create: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__MemoryAllocator,
        _: *const keen__SaveData__SaveDataSystemCreationParameters,
    ) -> bool,
    pub destroy: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__MemoryAllocator,
    ),
    pub enumerateProfiles: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__SizedArray_keen__SaveData__Profile_,
    ) -> bool,
    pub enumerateSaveData: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__SizedArray_keen__SaveData__SaveData_,
        _: u32,
        _: u32,
    ) -> bool,
    pub connectProfileWithUser: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: keen__UserAccountId,
    ) -> bool,
    pub disconnectProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ) -> bool,
    pub isProfileConnected: unsafe extern "thiscall" fn(
        this: *const keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ) -> bool,
    pub updateCreateProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__Profile,
    ),
    pub updateUpdateProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__Profile,
    ),
    pub updateEraseProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ),
    pub updateConnectProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__Profile,
    ),
    pub updateDisconnectProfile: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__Profile,
    ),
    pub updateCheckSaveDataSet: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ),
    pub updateCreateSaveDataSet: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ),
    pub updateEraseSaveDataSet: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ),
    pub updateCreateSaveData: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__SaveData__SaveData,
        _: u32,
        _: *const keen__SaveData__StorageOperation,
    ),
    pub updateWriteSaveData: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__SaveData,
        _: *const keen__SaveData__StorageOperation,
    ),
    pub updateReadSaveData: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: *mut keen__SaveData__StorageOperation,
        _: u32,
        _: *const keen__SaveData__SaveData,
    ),
    pub updateEraseSaveData: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
        _: *const keen__SaveData__SaveData,
    ),
    pub hasFinishedOperation: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
    ) -> bool,
    pub getOperationResult: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
    ) -> keen__SaveData__OperationResult,
    pub getSegmentCount: unsafe extern "thiscall" fn(
        this: *const keen__SaveData__SaveDataProviderWin32Interface,
        _: u32,
    ) -> u32,
}

#[repr(C)]
pub struct keen__SaveData__Profile {
    pub isUsed: bool,
    pub name: [i8; 64],
}

#[repr(C)]
pub struct keen__SaveData__SaveDataSystemCreationProviderParameters {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct keen__SaveData__SaveDataSystemCreationParameters {
    pub saveDataSystemType: keen__SaveData__SaveDataSystemType,
    pub maxProfileCount: u32,
    pub pSaveDataFormats: *const keen__SaveData__SaveDataFormat,
    pub saveDataFormatCount: u32,
    pub suppressSystemDialogs: bool,
    pub pGlobalSaveDataFormats: *const keen__SaveData__SaveDataFormat,
    pub globalsaveDataFormatCount: u32,
    pub pProviderParameters: *mut keen__SaveData__SaveDataSystemCreationProviderParameters,
}

#[repr(C)]
pub struct keen__SaveData__StorageOperation {
    pub segmentIndex: u32,
    pub pBuffer: *mut (),
    pub pNext: *mut keen__SaveData__StorageOperation,
}

#[repr(C)]
pub struct keen__SaveData__SaveData {
    pub formatIndex: u32,
    pub id: u32,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataFormat {
    pub minimumInstallments: u32,
    pub maximumInstallments: u32,
    pub numSegments: u32,
    pub pSegmentSizes: *const u32,
}

#[repr(C)]
pub struct keen__SizedArray_keen__SaveData__Profile_ {
    pub m_pData: *mut keen__SaveData__Profile,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__Array_keen__SaveDataHandler__SaveGameClientData_ {
    pub m_pData: *mut keen__SaveDataHandler__SaveGameClientData,
    pub m_size: u32,
}

#[repr(C)]
pub struct _LIST_ENTRY {
    pub Flink: *mut _LIST_ENTRY,
    pub Blink: *mut _LIST_ENTRY,
}

#[repr(C)]
pub struct _RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut _RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: *mut (),
    pub LockSemaphore: *mut (),
    pub SpinCount: u32,
}

#[repr(C)]
pub struct _RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut _RTL_CRITICAL_SECTION,
    pub ProcessLocksList: _LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}

#[repr(C)]
pub struct IDirectInput8A {
    pub lpVtbl: *mut IUnknownVtbl,
}

impl IDirectInput8A {
    pub fn as_IUnknown_ptr(&self) -> *const IUnknown {
        self as *const _ as _
    }

    pub fn as_IUnknown_mut_ptr(&mut self) -> *mut IUnknown {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct keen__InputSystem {
    pub eventQueue: keen__Queue_keen__InputEvent_,
    __pdbindgen_padding: [u8; 4],
    pub controllerState: keen__PlatformControllerState,
    pub storedEvents: keen__SizedArray_keen__InputEvent_,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub controllerInfos: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 384],
    pub autoCatchType: keen__InputSystemControllerAutoCatchType,
    pub autoCatchPlayerId: *const keen__LocalPlayerIdStructureType,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub mappedAxisButtonStates: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_3: [u8; 112],
    pub gestureHelper: keen__GestureHelper,
}

#[repr(C)]
pub struct keen__ControllerInfo {
    pub controllerClass: keen__ControllerClass,
    pub r#type: keen__ControllerType,
    pub features: u32,
    pub buttonCount: u32,
    pub axisCount: u32,
    pub localPlayerId: u8,
}

#[repr(C)]
pub struct keen__SizedArray_keen__InputEvent_ {
    pub m_pData: *mut keen__InputEvent,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__GestureHelper {
    pub m_moveHistory: keen__RingBuffer_keen__GestureHelper__MoveData_,
    pub m_isTouchInProgress: bool,
    pub m_startDragPosition: keen__float2,
    pub m_lastPosition: keen__float2,
    pub m_accumulatedFrameDeltaPosition: keen__float2,
    pub m_currentDeltaTime: f32,
    pub m_controllerId: u32,
    pub m_touchId: u32,
}

#[repr(C)]
pub struct keen__GestureHelper__MoveData {
    pub distance: keen__float2,
    pub time: f32,
}

#[repr(C)]
pub struct keen__Array_keen__InputEvent_ {
    pub m_pData: *mut keen__InputEvent,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__InputEvent {
    pub controllerId: u8,
    pub controllerClass: u8,
    pub localPlayerId: u8,
    pub r#type: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub data: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct keen__PlatformControllerState {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub controllers: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 48],
    pub mouseVisible: bool,
    pub mousePositionRelative: bool,
    pub lastMousePosition: keen__float2,
    pub pDirect8: *mut IDirectInput8A,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub directInputControllers: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 64],
    pub directInputControllerCount: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub lastState: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_3: [u8; 1796],
    pub mouseWheelButtonFlags: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub steamController: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_4: [u8; 68],
    pub steamControllerCount: u32,
    __pdbindgen_padding_5: [u8; 4],
}

#[repr(C)]
pub struct keen__StaticArray_keen__GestureHelper__MoveData_ {
    pub m_pData: *mut keen__GestureHelper__MoveData,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__RingBuffer_keen__GestureHelper__MoveData_ {
    pub m_data: keen__StaticArray_keen__GestureHelper__MoveData_,
    pub m_start: u32,
    pub m_end: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__Queue_keen__InputEvent_ {
    pub m_size: u32,
    pub m_bottom: u32,
    pub m_top: u32,
    pub m_data: keen__Array_keen__InputEvent_,
}

#[repr(C)]
pub struct std__allocator_char_ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_char_ {
    pub fn as_std___Allocator_base_char__ptr(&self) -> *const std___Allocator_base_char_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_char__mut_ptr(&mut self) -> *mut std___Allocator_base_char_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Container_base0 {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___String_val_char_std__allocator_char___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding_2: [u8; 15],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
}

impl std___String_val_char_std__allocator_char___ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_char_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding_2: [u8; 15],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
}

impl std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    pub fn as_std___String_val_char_std__allocator_char____ptr(
        &self,
    ) -> *const std___String_val_char_std__allocator_char___ {
        self as *const _ as _
    }

    pub fn as_std___String_val_char_std__allocator_char____mut_ptr(
        &mut self,
    ) -> *mut std___String_val_char_std__allocator_char___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TVector3_float_gfc__FloatMath_ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub r: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub g: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub b: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub u: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub v: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub w: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub array: [f32; 3],
}

#[repr(C)]
pub struct gfc__Vector_gfc__String_0_gfc__CAllocator_ {
    pub mData: *mut gfc__String,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ResourceCache {
    pub __vfptr: *const gfc__ResourceCache____vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__ResourceCache____vftable {
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
pub struct gfc__ResourceCache__PackageInfo {
    pub mPackageName: gfc__HString,
    pub mResources: gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_,
    pub mEnabled: bool,
}

#[repr(C)]
pub struct gfc__TVector4_float_gfc__FloatMath_ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub r: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub g: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub b: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub a: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub array: [f32; 4],
}

#[repr(C)]
pub struct gfc__TBox_float_gfc__FloatMath_ {
    pub min: gfc__TVector3_float_gfc__FloatMath_,
    pub max: gfc__TVector3_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__ResourceLoadInfo {
    pub PackageID: i32,
    pub Name: gfc__HString,
    pub ExtIdx: i32,
    pub UserData: u32,
    pub NumBuffers: i32,
    pub Buffers: *mut gfc__ResourceBuffer,
    pub SystemMemory: i32,
    pub VideoMemory: i32,
    pub Unfinished: *mut gfc__LockFreeQueue_gfc__Resource___,
    pub Alloc: *mut gfc__LockFreeQueue_gfc__Resource___,
}

#[repr(C)]
pub struct gfc__ValueStack {
    pub mSize: i32,
    pub mStack: *mut gfc__ValueStack___Stack,
}

#[repr(C)]
pub struct gfc__ValueStack___Stack {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub Stack: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 8192],
}

#[repr(C)]
pub struct gfc__LinkMyNew {
    pub unused: u32,
}

#[repr(C)]
pub struct gfc__Event {
    pub mEvent: keen__Event,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ThreadSafeBool {
    pub mValue: bool,
}

#[repr(C)]
pub struct gfc__LockFreeQueue_gfc__Resource___ {
    pub mHead: *mut gfc__LockFreeNode_gfc__Resource___,
    pub mPops: u32,
    pub mTail: *mut gfc__LockFreeNode_gfc__Resource___,
    pub mPushes: u32,
    pub mDummyNode: gfc__LockFreeNode_gfc__Resource___,
}

#[repr(C)]
pub struct gfc__WString {
    pub mStringData: *mut gfc__WString__StringData,
}

#[repr(C)]
pub struct gfc__WString__StringData {
    pub mRefCount: i32,
    pub mCapacity: u16,
    pub mCurrentSize: u16,
    pub mData: [u16; 1],
}

#[repr(C)]
pub struct gfc__ClassRegistry {
    pub mClassRegistry: gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_,
    pub mClassLoaders: gfc__Vector_gfc__AutoRef_gfc__ClassLoader__0_gfc__CAllocator_,
    pub mLastGarbagePos: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_gfc__HString_gfc__AutoRef_gfc__Class__gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__HString {
    pub mHash: u64,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Property_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Method__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Method_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_int_0_gfc__CAllocator_ {
    pub mData: *mut i32,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Half {
    pub value: u16,
}

#[repr(C)]
pub struct gfc__SoundDesc {
    pub __vfptr: *const gfc__SoundDesc____vftable,
    pub ReferenceCount: i32,
    pub mSounds: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mName: gfc__HString,
    pub mGroup: gfc__HString,
    pub mID: i32,
    pub mPackageID: i32,
    pub mFadeInTime: gfc__Half,
    pub mFadeOutTime: gfc__Half,
    pub mEventFadeTime: gfc__Half,
    pub mDuration: gfc__Half,
    pub mDelay: gfc__Half,
    pub mVolume: gfc__Half,
    pub mRandomVolume: gfc__Half,
    pub mPitch: gfc__Half,
    pub mRandomPitch: gfc__Half,
    pub mMin: gfc__Half,
    pub mMax: gfc__Half,
    pub mIndex: i8,
    pub mChance: u8,
    pub mPriority: u8,
    pub mRandomBits: u32,
    pub mFlags: gfc__TFlags_unsigned_short_,
}

impl gfc__SoundDesc {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__SoundDesc____vftable {
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
pub struct gfc__TFlags_unsigned_short_ {
    pub flags: u16,
}

#[repr(C)]
pub struct gfc__Resource {
    pub __vfptr: *const gfc__Resource____vftable,
    pub ReferenceCount: i32,
    pub mState: i32,
    pub mPackageID: i32,
}

impl gfc__Resource {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Resource____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Resource) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Value_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Matrix4 {
    pub xx: f32,
    pub xy: f32,
    pub xz: f32,
    pub xw: f32,
    pub yx: f32,
    pub yy: f32,
    pub yz: f32,
    pub yw: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
    pub zw: f32,
    pub wx: f32,
    pub wy: f32,
    pub wz: f32,
    pub ww: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub x: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub y: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub z: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub w: compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m: compile_error!("unimplemented feature: sizeof array 0x0"),
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ResourceCache__PackageInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__InputStream_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_void___gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair
{
    pub mNext: u32,
    pub mKey: gfc__HString,
    pub mValue: *mut (),
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TVector2_float_gfc__FloatMath_ {
    pub x: f32,
    pub y: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub u: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub v: f32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub array: [f32; 2],
}

#[repr(C)]
pub struct gfc__String {
    pub mString: std__basic_string_char_std__char_traits_char__std__allocator_char___,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Method_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__OutputStream_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ResourceBufferInfo {
    pub PackageID: i32,
    pub Name: gfc__HString,
    pub ExtIdx: i32,
    pub UserData: u32,
    pub Analysis: gfc__AutoRef_gfc__InputStream_,
    pub Buffers: *mut gfc__Vector_gfc__ResourceBuffer_0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Object_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__IRefObject {
    pub __vfptr: *const gfc__IRefObject____vftable,
    pub ReferenceCount: i32,
}

#[repr(C)]
pub struct gfc__IRefObject____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__Thread {
    pub mThread: keen__Thread,
    pub mThreadDelegate: gfc__AutoRef_gfc__IVoidDelegate_,
    pub mThreadName: [i8; 64],
    pub mThreadCreated: bool,
}

#[repr(C)]
pub struct gfc__TFlags_unsigned_char_ {
    pub flags: u8,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__IVoidDelegate_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LockFreeNode_gfc__Resource___ {
    pub Value: *mut gfc__Resource,
    pub Next: *mut gfc__LockFreeNode_gfc__Resource___,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Environment_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Property__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Property_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Object {
    pub __vfptr: *const gfc__Object____vftable,
    pub ReferenceCount: i32,
}

impl gfc__Object {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Object____vftable {
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
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__File_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__OutputStream {
    pub __vfptr: *const gfc__OutputStream____vftable,
    pub ReferenceCount: i32,
    pub mEndianess: i32,
}

impl gfc__OutputStream {
    pub fn as_gfc__Stream_ptr(&self) -> *const gfc__Stream {
        self as *const _ as _
    }

    pub fn as_gfc__Stream_mut_ptr(&mut self) -> *mut gfc__Stream {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__OutputStream____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub write: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: *const (), _: i32),
    pub flush: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub isSeekable: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> i64,
}

#[repr(C)]
pub struct gfc__Mutex {
    pub mMutex: keen__Mutex,
}

#[repr(C)]
pub struct gfc__Property {
    pub __vfptr: *const gfc__Property____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
    pub mAnnotation: gfc__HString,
    pub mContextFlags: u8,
}

impl gfc__Property {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Property____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Property) -> i32,
    pub getTypeClass: unsafe extern "thiscall" fn(this: *const gfc__Property) -> *mut gfc__Class,
    pub getElementType: unsafe extern "thiscall" fn(this: *const gfc__Property) -> i32,
    pub isReadOnly: unsafe extern "thiscall" fn(this: *const gfc__Property) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__Property) -> bool,
    pub isTransient: unsafe extern "thiscall" fn(this: *const gfc__Property) -> bool,
    pub isEditable: unsafe extern "thiscall" fn(this: *const gfc__Property) -> bool,
    pub setValue: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub getValue: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        result: *mut gfc__AutoRef_gfc__Value_,
        _: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_,
    pub getElementCount:
        unsafe extern "thiscall" fn(this: *mut gfc__Property, _: *mut gfc__Object) -> i32,
    pub reserveElements:
        unsafe extern "thiscall" fn(this: *mut gfc__Property, _: *mut gfc__Object, _: i32),
    pub addElement: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub removeElement: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub containsElement: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ) -> bool,
    pub setElementAt: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub getElementAt: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        result: *mut gfc__AutoRef_gfc__Value_,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ) -> *mut gfc__AutoRef_gfc__Value_,
    pub addElementAt: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub removeElementAt: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ),
    pub containsElementAt: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ) -> bool,
    pub removeAllElements:
        unsafe extern "thiscall" fn(this: *mut gfc__Property, _: *mut gfc__Object),
    pub getKeys: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        result: *mut gfc__AutoRef_gfc__Value_,
        _: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_,
    pub getValues: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        result: *mut gfc__AutoRef_gfc__Value_,
        _: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_,
    pub clone: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: *mut gfc__Object,
    ),
}

#[repr(C)]
pub struct gfc__ResourceBuffer {
    pub Length: u32,
    pub Buffer: *mut u8,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__ClassLoader__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__ClassLoader_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Stream {
    pub __vfptr: *const gfc__Stream____vftable,
    pub ReferenceCount: i32,
}

impl gfc__Stream {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Stream____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
}

#[repr(C)]
pub struct gfc__Vector_unsigned_long_0_gfc__CAllocator_ {
    pub mData: *mut u32,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SoundList_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ResourceAnalyzeInfo {
    pub Name: gfc__HString,
    pub ExtIdx: i32,
    pub Input: gfc__AutoRef_gfc__InputStream_,
    pub Analysis: gfc__AutoRef_gfc__OutputStream_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HString_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HString,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Class_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Class {
    pub __vfptr: *const gfc__Class____vftable,
    pub ReferenceCount: i32,
    pub mParent: gfc__AutoRef_gfc__Class_,
    pub mName: gfc__HString,
    pub mConstructor: gfc__AutoRef_gfc__Constructor_,
    pub mProperties: gfc__Vector_gfc__AutoRef_gfc__Property__0_gfc__CAllocator_,
    pub mMethods: gfc__Vector_gfc__AutoRef_gfc__Method__0_gfc__CAllocator_,
    pub mPropertyMap: gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Property__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_,
    pub mMethodMap: gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_,
    pub mContextFlags: u8,
    pub mStubCalled: bool,
}

impl gfc__Class {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Class____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getTypeID: unsafe extern "thiscall" fn(this: *mut gfc__Class) -> i32,
    pub getName: unsafe extern "thiscall" fn(this: *const gfc__Class) -> *const gfc__HString,
    pub getParent: unsafe extern "thiscall" fn(this: *const gfc__Class) -> *mut gfc__Class,
    pub instanceof:
        unsafe extern "thiscall" fn(this: *const gfc__Class, _: *const gfc__Class) -> bool,
    pub isAbstract: unsafe extern "thiscall" fn(this: *const gfc__Class) -> bool,
    pub newInstance: unsafe extern "thiscall" fn(
        this: *mut gfc__Class,
        result: *mut gfc__AutoRef_gfc__Object_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
    pub getPropertyByName: unsafe extern "thiscall" fn(
        this: *const gfc__Class,
        _: *const gfc__HString,
        _: *mut *const gfc__Class,
    ) -> *mut gfc__Property,
    pub getPropertyByID: unsafe extern "thiscall" fn(
        this: *const gfc__Class,
        _: *const u64,
        _: *mut *const gfc__Class,
    ) -> *mut gfc__Property,
    pub getMethodByName: unsafe extern "thiscall" fn(
        this: *mut gfc__Class,
        _: *const gfc__HString,
    ) -> *mut gfc__Method,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const u64) -> *mut gfc__Method,
}

#[repr(C)]
pub struct gfc__Environment {
    pub __vfptr: *const gfc__Environment____vftable,
    pub ReferenceCount: i32,
    pub mParent: gfc__AutoRef_gfc__Environment_,
    pub mSymbols: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_,
    pub mContextFlags: u8,
    pub mHasRun: bool,
    pub mTempEnv: bool,
}

impl gfc__Environment {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Environment____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Constructor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Method {
    pub __vfptr: *const gfc__Method____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
    pub mContextFlags: u8,
}

impl gfc__Method {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Method____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getParamCount: unsafe extern "thiscall" fn(this: *const gfc__Method) -> i32,
    pub getParamTypeAt: unsafe extern "thiscall" fn(this: *const gfc__Method, _: i32) -> i32,
    pub getReturnType: unsafe extern "thiscall" fn(this: *const gfc__Method) -> i32,
    pub getParamClassAt:
        unsafe extern "thiscall" fn(this: *const gfc__Method, _: i32) -> *mut gfc__Class,
    pub getReturnClass: unsafe extern "thiscall" fn(this: *const gfc__Method) -> *mut gfc__Class,
    pub isScriptMethod: unsafe extern "thiscall" fn(this: *const gfc__Method) -> bool,
    pub execute: unsafe extern "thiscall" fn(
        this: *mut gfc__Method,
        _: *mut gfc__Object,
        _: *mut gfc__ValueStack,
        _: *mut gfc__Environment,
    ) -> i32,
}

#[repr(C)]
pub struct gfc__TFlags_unsigned_long_ {
    pub flags: u32,
}

#[repr(C)]
pub struct gfc__Vector_float_0_gfc__CAllocator_ {
    pub mData: *mut f32,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourceBuffer_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ResourceBuffer,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
}

impl
    std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
{
    pub fn as_std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface______ptr(&self) -> *const std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____{
        self as *const _ as _
    }

pub fn as_std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface______mut_ptr(&mut self) -> *mut std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____{
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    pub fn as_std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____ptr(&self) -> *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____mut_ptr(&mut self) -> *mut std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    pub fn as_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0__ptr(&self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _ as _
    }

    pub fn as_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0__mut_ptr(&mut self) -> *mut std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {
    pub fn as_std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______ptr(
        &self,
    ) -> *const std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
    {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______mut_ptr(
        &mut self,
    ) -> *mut std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
    {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    pub first: gfc__AutoRef_gfc__Object_,
    pub second: gfc__AutoRef_gfc__Object_,
}

impl std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    pub fn as_std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object____ptr(
        &self,
    ) -> *const std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
        self as *const _ as _
    }

    pub fn as_std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object____mut_ptr(
        &mut self,
    ) -> *mut std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    pub fn as_std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____ptr(&self) -> *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____mut_ptr(&mut self) -> *mut std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    pub fn as_std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____ptr(&self) -> *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____mut_ptr(&mut self) -> *mut std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
}

impl std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    pub fn as_std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter____ptr(
        &self,
    ) -> *const std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
        self as *const _ as _
    }

    pub fn as_std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter____mut_ptr(
        &mut self,
    ) -> *mut std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    pub fn as_std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____ptr(&self) -> *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____mut_ptr(&mut self) -> *mut std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    pub fn as_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0__ptr(&self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _ as _
    }

    pub fn as_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0__mut_ptr(&mut self) -> *mut std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Myval: std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__allocator_gfc__AutoRef_gfc__ImageSurface___ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_gfc__AutoRef_gfc__ImageSurface___ {
    pub fn as_std___Allocator_base_gfc__AutoRef_gfc__ImageSurface____ptr(
        &self,
    ) -> *const std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_gfc__AutoRef_gfc__ImageSurface____mut_ptr(
        &mut self,
    ) -> *mut std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____ptr(&self) -> *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0____mut_ptr(&mut self) -> *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    pub first: gfc__AutoRef_gfc__Object_,
    pub second: gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {
    pub fn as_std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______ptr(&self) -> *const std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____{
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______mut_ptr(
        &mut self,
    ) -> *mut std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
    {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
}

impl std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    pub fn as_std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____ptr(&self) -> *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____mut_ptr(&mut self) -> *mut std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    pub fn as_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0__ptr(&self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _ as _
    }

    pub fn as_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0__mut_ptr(&mut self) -> *mut std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____ptr(&self) -> *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____mut_ptr(&mut self) -> *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____ptr(&self) -> *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0____mut_ptr(&mut self) -> *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    pub fn as_std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____ptr(&self) -> *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0____mut_ptr(&mut self) -> *mut std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
    pub fn as_std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______ptr(&self) -> *const std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____{
        self as *const _ as _
    }

pub fn as_std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______mut_ptr(&mut self) -> *mut std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____{
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__EventHandler_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TRect_float_ {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Object_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FullScreenEffectDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RegionLayerData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__AmbientDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_ {
    pub mData: *mut gfc__TVector3_float_gfc__FloatMath_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__HDRDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__DepthOfFieldDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__IRefObject_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CameraBlurDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RenderTexture_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__VertexBuffer {
    pub __vfptr: *const gfc__VertexBuffer____vftable,
    pub ReferenceCount: i32,
}

impl gfc__VertexBuffer {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__VertexBuffer____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub release: unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer),
    pub getSize: unsafe extern "thiscall" fn(this: *const gfc__VertexBuffer) -> u32,
    pub getVertexCount: unsafe extern "thiscall" fn(this: *const gfc__VertexBuffer) -> u32,
    pub getVertexStride: unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer) -> u32,
    pub getVertexFormat:
        unsafe extern "thiscall" fn(this: *const gfc__VertexBuffer) -> *const gfc__VertexFormat,
    pub getElementData: unsafe extern "thiscall" fn(
        this: *mut gfc__VertexBuffer,
        _: u32,
        _: u32,
        _: u32,
        _: i32,
    ) -> *mut u8,
    pub lockall: unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer, _: i32),
    pub finish: unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer),
    pub write: unsafe extern "thiscall" fn(
        this: *mut gfc__VertexBuffer,
        _: gfc__AutoRef_gfc__OutputStream_,
    ) -> bool,
    pub read: unsafe extern "thiscall" fn(
        this: *mut gfc__VertexBuffer,
        _: gfc__AutoRef_gfc__InputStream_,
    ) -> bool,
    pub updateAddress:
        unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer, _: *mut (), _: *mut ()),
}

#[repr(C)]
pub struct gfc__Vector_gfc__UIRenderer__Clip_0_gfc__CAllocator_ {
    pub mData: *mut gfc__UIRenderer__Clip,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__IndexBuffer {
    pub __vfptr: *const gfc__IndexBuffer____vftable,
    pub ReferenceCount: i32,
}

impl gfc__IndexBuffer {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__IndexBuffer____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub lock: unsafe extern "thiscall" fn(this: *mut gfc__IndexBuffer, _: i32),
    pub unlock: unsafe extern "thiscall" fn(this: *mut gfc__IndexBuffer),
    pub locked: unsafe extern "thiscall" fn(this: *const gfc__IndexBuffer) -> bool,
    pub getIndexCount: unsafe extern "thiscall" fn(this: *const gfc__IndexBuffer) -> i32,
    pub getData: unsafe extern "thiscall" fn(this: *const gfc__IndexBuffer) -> *mut u8,
    pub getStride: unsafe extern "thiscall" fn(this: *const gfc__IndexBuffer) -> i32,
    pub updateAddress:
        unsafe extern "thiscall" fn(this: *mut gfc__IndexBuffer, _: *mut (), _: *mut ()),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DepthOfFieldDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__UIRenderer__Params_0_gfc__CAllocator_ {
    pub mData: *mut gfc__UIRenderer__Params,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Parameter_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__TVector2_int_gfc__FloatMath_ {
    pub x: i32,
    pub y: i32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub u: i32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub v: i32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub array: [i32; 2],
}

#[repr(C)]
pub struct gfc__MeshFormat__DynamicVertex__Stream0 {
    pub mPosition: [f32; 3],
    pub mColor: [f32; 4],
    pub mTex0: [f32; 3],
    pub mTex1: gfc__TVector2_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc__Node3D_ {
    pub __vfptr: *const gfc__Hierarchical_gfc__Node3D_____vftable,
    pub mParent: *mut gfc__Node3D,
    pub mHead: gfc__AutoRef_gfc__Node3D_,
    pub mTail: gfc__AutoRef_gfc__Node3D_,
    pub mNext: gfc__AutoRef_gfc__Node3D_,
    pub mPrev: gfc__AutoRef_gfc__Node3D_,
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc__Node3D_____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: u32) -> *mut (),
    pub addFront:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub addBack:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub add:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_),
    pub remove_2:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_),
    pub added:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub removed:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub invalidateHierarchy: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__HDRDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DynamicRenderer {
    pub mCurrentViewInv: gfc__Matrix4,
    pub mCurrentView: gfc__Matrix4,
    pub mCanDraw: bool,
    pub mCurrent: *mut gfc__DynamicRenderNode,
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RegionCell_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__FogDesc {
    pub __vfptr: *const gfc__FogDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
    pub mBlendDuration: f32,
    pub mType: i32,
    pub mColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mDensity: f32,
    pub mStart: f32,
    pub mEnd: f32,
    __pdbindgen_padding: [u8; 4],
    pub mParameters: gfc__TVector4_float_gfc__FloatMath_,
    pub mCurrentColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
    __pdbindgen_padding_2: [u8; 12],
}

impl gfc__FogDesc {
    pub fn as_gfc__EnvironmentDesc_ptr(&self) -> *const gfc__EnvironmentDesc {
        self as *const _ as _
    }

    pub fn as_gfc__EnvironmentDesc_mut_ptr(&mut self) -> *mut gfc__EnvironmentDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__FogDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldObject_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__RegionPortal__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__RegionPortal_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Graphics {
    pub __vfptr: *const gfc__Graphics____vftable,
}

#[repr(C)]
pub struct gfc__Graphics____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32) -> *mut (),
    pub init: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u16,
        _: u16,
        _: u8,
        _: bool,
    ) -> bool,
    pub isInitialized: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> bool,
    pub release: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub getSceneBackBufferSize:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics, _: *mut u16, _: *mut u16),
    pub getScreenBackBufferSize:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics, _: *mut u16, _: *mut u16),
    pub getAspectRatio: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> f32,
    pub acquireThreadOwnership: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub releaseThreadOwnership: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> bool,
    pub begin3D: unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> bool,
    pub end3D: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub getWaitForVSync: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> bool,
    pub setWaitForVSync: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: bool),
    pub unsetSamplers: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub getWorldMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics, _: *mut gfc__Matrix4),
    pub setWorldMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *const gfc__Matrix4),
    pub commitWorldMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *const gfc__Matrix4),
    pub getViewMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> *const gfc__Matrix4,
    pub setViewMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *const gfc__Matrix4),
    pub getProjectionMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> *const gfc__Matrix4,
    pub setProjectionMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *const gfc__Matrix4),
    pub getViewProjMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> *const gfc__Matrix4,
    pub setViewport: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *const gfc__Viewport),
    pub getViewport: unsafe extern "thiscall" fn(
        this: *const gfc__Graphics,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
    ),
    pub clear: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: bool,
        _: bool,
        _: bool,
        _: u32,
        _: f32,
        _: u32,
    ),
    pub clear_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub pushClip: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub popClip: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub enableClip: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub disableClip: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub setDepthTestMode: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub setCullMode: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub setBlendMode: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32),
    pub getBlendMode: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> i32,
    pub setBlendFactor: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
    ),
    pub setGamma: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: f32),
    pub setPostProcessEffect: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32),
    pub setAntiAliasingType: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32),
    pub setTextureFilteringMode: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32),
    pub setShader: unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *mut gfc__Shader),
    pub getShader: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> *mut gfc__Shader,
    pub setVertexDeclaration:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *mut gfc__VertexDeclaration),
    pub setIndexBuffer:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *mut gfc__IndexBuffer),
    pub setVertexBuffer:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *mut gfc__VertexBuffer),
    pub setVertexBufferMasked:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: *mut gfc__VertexBuffer, _: u32),
    pub drawFullScreenQuad: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
    pub drawRect:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32, _: i32, _: i32, _: i32),
    pub drawPrimitiveUP:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32, _: u32, _: *const (), _: u32),
    pub drawPrimitiveRetained:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32, _: u32, _: *const (), _: u32),
    pub drawIndexedPrimitiveUP: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u32,
        _: u32,
        _: u32,
        _: *const (),
        _: *const (),
        _: u32,
    ),
    pub drawPrimitives:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: u32, _: u32, _: u32),
    pub drawPrimitivesIndexed: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u32,
        _: u32,
        _: u32,
        _: u32,
        _: u32,
    ),
    pub setRenderTarget: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: i32,
    ),
    pub setDefaultRenderTarget: unsafe extern "thiscall" fn(this: *const gfc__Graphics),
    pub setDefaultDepthStencilTarget: unsafe extern "thiscall" fn(this: *const gfc__Graphics),
    pub createVertexDeclaration:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Graphics,
            result: *mut gfc__AutoRef_gfc__VertexDeclaration_,
            _: *const gfc__VertexFormat,
        ) -> *mut gfc__AutoRef_gfc__VertexDeclaration_,
    pub createVertexBuffer: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
        _: i32,
        _: u32,
    )
        -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub createVertexBuffer_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    )
        -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub createIndexBuffer: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
        _: u32,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub createMeshBuilder: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__MeshBuilder_,
    ) -> *mut gfc__AutoRef_gfc__MeshBuilder_,
    pub createStaticMesh: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
        _: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_,
    pub createStaticMesh_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_,
    pub createSkinMesh: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
        _: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_,
    pub createSkinMesh_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_,
    pub createRenderTexture: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__RenderTexture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    )
        -> *mut gfc__AutoRef_gfc__RenderTexture_,
    pub createRenderTarget: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderTarget_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderDepth: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *mut (),
        _: u32,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *const gfc__String,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_3: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *mut gfc__Image,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_4: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: i32,
        _: gfc__ImageFormat,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: gfc__ImageFormat,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createLightmap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: i32,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createShader: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Shader_,
    ) -> *mut gfc__AutoRef_gfc__Shader_,
    pub createShaderCompiler: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__ShaderCompiler_,
    )
        -> *mut gfc__AutoRef_gfc__ShaderCompiler_,
    pub createQuery: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Query_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Query_,
    pub createCamera: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        result: *mut gfc__AutoRef_gfc__Camera3D_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Camera3D_,
    pub createRenderer: unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> *mut gfc__Renderer,
    pub getPlatform: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> i32,
    pub getDepthBias: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> f32,
    pub getDecalDepthBias: unsafe extern "thiscall" fn(this: *const gfc__Graphics) -> f32,
    pub recover: unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> bool,
    pub isMeshFormatSupported:
        unsafe extern "thiscall" fn(this: *const gfc__Graphics, _: i32) -> bool,
    pub blockUntilDefragged: unsafe extern "thiscall" fn(this: *mut gfc__Graphics),
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__AmbientDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__LightNode___ {
    pub m_vector: gfc__Vector_gfc__LightNode___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AmbientDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Camera3D {
    pub __vfptr: *const gfc__Camera3D____vftable,
    pub ReferenceCount: i32,
    pub mFrameID: i32,
    pub mInsideOutside: i32,
    pub mUnderwater: bool,
    pub mHasSun: bool,
    pub mSun: *mut gfc__LightNode,
    __pdbindgen_padding: [u8; 8],
    pub mCameraMatrix: gfc__Matrix4,
    pub mViewMatrix: gfc__Matrix4,
    pub mCameraPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCameraFOV: f32,
    pub mCameraAspect: f32,
    pub mCameraNear: f32,
    pub mCameraFar: f32,
    pub mFogEnd: f32,
    pub mRenderNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mTerrainRenderNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mNotVisRenderNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mBlendNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mUnlitNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mMotionBlurNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mSkyNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mGizmoNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mLightNodes: gfc__ThreadSafeVector_gfc__LightNode___,
    pub mInGameUINodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mClothPrepNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mGeoPrepNodes: gfc__ThreadSafeVector_gfc__RenderNode___,
    pub mVisibleNodePool: *mut gfc__LightNode__VisNode,
    pub mNumVisibleNodes: u32,
    pub mNumUsedVisibleNodes: u32,
    pub mRenderNodePool: *mut gfc__RenderNode,
    pub mNumRenderNodes: u32,
    pub mNumUsedRenderNodes: u32,
    pub mLightNodePool: *mut gfc__LightNode,
    pub mNumLightNodes: u32,
    pub mNumUsedLightNodes: u32,
    pub mDynMeshNodePool: *mut gfc__DynMeshNode,
    pub mNumDynMeshNodes: u32,
    pub mNumUsedDynMeshNodes: u32,
    pub mDynMeshMutex: gfc__Mutex,
    pub mDynMeshNodes: gfc__ThreadSafeVector_gfc__DynMeshNode___,
    pub mDynMeshBuffers: gfc__ThreadSafeVector_gfc__DynMeshBuffer___,
    pub mCurrentDynMeshBuffer: i32,
    __pdbindgen_padding_2: [u8; 12],
    pub mMatrixPool: gfc__MatrixArrayPool,
    pub mTerrainChunkSize: i32,
    pub mTerrainHeightMapSize: i32,
    __pdbindgen_padding_3: [u8; 8],
}

impl gfc__Camera3D {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Camera3D____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub sortRenderNodes: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub setLightInfluences: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub computeMatrices:
        unsafe extern "thiscall" fn(this: *mut gfc__Camera3D, _: *const gfc__Matrix4, _: bool),
    pub computeLightVis: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub startGeoPrep: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub computeGeoPrep: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
    pub finishGeoPrep: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D),
}

#[repr(C)]
pub struct gfc__DepthOfFieldDesc {
    pub __vfptr: *const gfc__DepthOfFieldDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
    pub mFocusDistance: f32,
    pub mBlurStart: f32,
    pub mBlurRange: f32,
    pub mMaxBlur: f32,
    pub mBlendDuration: f32,
    pub mEnableTint: bool,
    __pdbindgen_padding: [u8; 15],
    pub mNearTintColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
    __pdbindgen_padding_2: [u8; 12],
}

impl gfc__DepthOfFieldDesc {
    pub fn as_gfc__EnvironmentDesc_ptr(&self) -> *const gfc__EnvironmentDesc {
        self as *const _ as _
    }

    pub fn as_gfc__EnvironmentDesc_mut_ptr(&mut self) -> *mut gfc__EnvironmentDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__DepthOfFieldDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__EnvironmentManager {
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mFogDesc: gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_,
    pub mAmbientDesc: gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_,
    pub mCameraBlurDesc: gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_,
    pub mDepthOfFieldDesc:
        gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_,
    pub mHDRDesc: gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_,
    pub mFullScreenEffectDesc:
        gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_,
    pub mLiquidFogDesc: gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_,
    pub mLiquidAmbientDesc: gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_,
    pub mLiquidDepthOfFieldDesc:
        gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_,
    pub mLiquidHDRDesc: gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_,
    pub mDefaultFog: gfc__AutoRef_gfc__FogDesc_,
    pub mDefaultAmbient: gfc__AutoRef_gfc__AmbientDesc_,
    pub mDefaultCameraBlur: gfc__AutoRef_gfc__CameraBlurDesc_,
    pub mDefaultDepthOfField: gfc__AutoRef_gfc__DepthOfFieldDesc_,
    pub mDefaultHDR: gfc__AutoRef_gfc__HDRDesc_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldGroup_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__WorldObject_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Renderer {
    pub __vfptr: *const gfc__Renderer____vftable,
}

#[repr(C)]
pub struct gfc__Renderer____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: u32) -> *mut (),
    pub init: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: u16, _: u16),
    pub reinit: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub loadDefaultResources: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub releaseDefaultResources: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub setBackgroundColor: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
    ),
    pub setFrameBufferAlpha: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: f32),
    pub setFogDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__FogDesc, _: bool),
    pub getFogDesc: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__FogDesc,
    pub setAmbientDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__AmbientDesc, _: bool),
    pub getAmbientDesc:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__AmbientDesc,
    pub setCameraBlurDesc: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: *const gfc__CameraBlurDesc,
        _: bool,
    ),
    pub getCameraBlurDesc:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__CameraBlurDesc,
    pub setDepthOfFieldDesc: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: *const gfc__DepthOfFieldDesc,
        _: bool,
    ),
    pub getDepthOfFieldDesc:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__DepthOfFieldDesc,
    pub setHDRDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__HDRDesc, _: bool),
    pub getHDRDesc: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__HDRDesc,
    pub getHDRRenderer: unsafe extern "thiscall" fn(this: *mut gfc__Renderer) -> *mut gfc__HDRBase,
    pub getViewport:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__Viewport,
    pub setViewport: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__Viewport),
    pub setNearDistance: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: f32),
    pub setFarDistance: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: f32),
    pub getViewMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__Matrix4,
    pub setViewMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__Matrix4),
    pub getProjectionMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__Matrix4,
    pub setProjectionMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__Matrix4),
    pub setDoLighting: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getDoLighting: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setDoShadows: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getDoShadows: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setDoFogging: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getDoFogging: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setDoPostRender: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getDoPostRender: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setDoDepthOfField: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getDoDepthOfField: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setBlendEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getBlendEnvironment: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub setBlendFocusDistance: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub getBlendFocusDistance: unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> bool,
    pub forceDynamicLights: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub forceRenderAllLights: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub setDoZPass: unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: bool),
    pub setSkyAmbientDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__AmbientDesc),
    pub getSkyAmbientDesc:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__AmbientDesc,
    pub setSkyFogDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *const gfc__FogDesc),
    pub getSkyFogDesc:
        unsafe extern "thiscall" fn(this: *const gfc__Renderer) -> *const gfc__FogDesc,
    pub addFullScreenEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ),
    pub removeFullScreenEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ),
    pub removeAllFullScreenEffects: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub hasFullScreenEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__Renderer,
        _: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ) -> bool,
    pub setupFrame:
        unsafe extern "thiscall" fn(this: *mut gfc__Renderer, _: *mut gfc__Camera3D, _: f32),
    pub render: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub setupView: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub renderShadows: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
    pub renderFrame: unsafe extern "thiscall" fn(this: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__CameraBlurDesc {
    pub __vfptr: *const gfc__CameraBlurDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
    pub mScale: f32,
    pub mSampleCount: u32,
    pub mManager: *mut gfc__EnvironmentManager,
}

impl gfc__CameraBlurDesc {
    pub fn as_gfc__EnvironmentDesc_ptr(&self) -> *const gfc__EnvironmentDesc {
        self as *const _ as _
    }

    pub fn as_gfc__EnvironmentDesc_mut_ptr(&mut self) -> *mut gfc__EnvironmentDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__CameraBlurDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__IndexBuffer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__LightNode___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__LightNode,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PhysicsManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__BakedLightData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Viewport {
    pub Left: i32,
    pub Top: i32,
    pub Right: i32,
    pub Bottom: i32,
    pub MinZ: f32,
    pub MaxZ: f32,
}

#[repr(C)]
pub struct gfc__TRect_long_ {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct gfc__Shader {
    pub __vfptr: *const gfc__Shader____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
    pub mLocked: bool,
}

impl gfc__Shader {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Shader____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub createFromDesc: unsafe extern "thiscall" fn(
        this: *mut gfc__Shader,
        _: gfc__AutoRef_gfc__ShaderDesc_,
    ) -> bool,
    pub getVersion: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> u32,
    pub getLightingModel: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> u8,
    pub getBlendMode: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> u8,
    pub getCullMode: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> u8,
    pub getSize: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> u32,
    pub getDesc: unsafe extern "thiscall" fn(
        this: *mut gfc__Shader,
        result: *mut gfc__AutoRef_gfc__ShaderDesc_,
    ) -> *mut gfc__AutoRef_gfc__ShaderDesc_,
    pub isUsed: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> bool,
    pub hasPass: unsafe extern "thiscall" fn(this: *const gfc__Shader, _: i32) -> bool,
    pub beginPass: unsafe extern "thiscall" fn(this: *mut gfc__Shader, _: i32, _: i32),
    pub endPass: unsafe extern "thiscall" fn(this: *mut gfc__Shader),
    pub setWorldMatrix: unsafe extern "thiscall" fn(this: *mut gfc__Shader, _: *const gfc__Matrix4),
    pub setDefaultValues: unsafe extern "thiscall" fn(this: *const gfc__Shader),
    pub setMaterialParams:
        unsafe extern "thiscall" fn(this: *mut gfc__Shader, _: *mut gfc__Material, _: f32),
    pub getParameterHandle:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: i32) -> u32,
    pub getParameterHandle_2:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8) -> u32,
    pub setConstant1f: unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: f32),
    pub setConstant1f_2: unsafe extern "thiscall" fn(this: *const gfc__Shader, _: u32, _: f32),
    pub setConstant4f: unsafe extern "thiscall" fn(
        this: *const gfc__Shader,
        _: *const i8,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
    ),
    pub setConstant4f_2: unsafe extern "thiscall" fn(
        this: *const gfc__Shader,
        _: u32,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
    ),
    pub setConstantArray1f:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: *const f32, _: u32),
    pub setConstantArray1f_2:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: u32, _: *const f32, _: u32),
    pub setMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: *const gfc__Matrix4),
    pub setMatrix_2:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: u32, _: *const gfc__Matrix4),
    pub setTexture:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: *const gfc__Texture),
    pub setTexture_2:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: u32, _: *const gfc__Texture),
    pub setSamplerState:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: *const i8, _: u32, _: u32),
    pub setSamplerState_2:
        unsafe extern "thiscall" fn(this: *const gfc__Shader, _: u32, _: u32, _: u32),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ShaderCompiler_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldObject {
    pub __vfptr: *const gfc__WorldObject____vftable,
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
}

impl gfc__WorldObject {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldObject____vftable {
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
pub struct gfc__WorldObject__EventHandlerNode {
    pub EventHandler: gfc__AutoRef_gfc__EventHandler_,
    pub Next: *mut gfc__WorldObject__EventHandlerNode,
    pub QueueState: u8,
}

#[repr(C)]
pub struct gfc__Vector_unsigned_short_0_gfc__CAllocator_ {
    pub mData: *mut u16,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__DynMeshBuffer___ {
    pub m_vector: gfc__Vector_gfc__DynMeshBuffer___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TerrainManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldRegionData_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__EnvironmentRegion_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__IRenderCallback {
    pub __vfptr: *const gfc__IRenderCallback____vftable,
    pub mLocked: bool,
}

#[repr(C)]
pub struct gfc__IRenderCallback____vftable {
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
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ImageSurface_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Plane {
    pub normalAndDistance: gfc__TVector4_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__TSphere_float_gfc__FloatMath_ {
    pub center: gfc__TVector3_float_gfc__FloatMath_,
    pub radius: f32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RegionPortal_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__HDRBase {
    pub __vfptr: *const gfc__HDRBase____vftable,
    pub mDesc: gfc__HDRDesc,
}

#[repr(C)]
pub struct gfc__HDRBase____vftable {
    pub setDesc: unsafe extern "thiscall" fn(this: *mut gfc__HDRBase, _: *const gfc__HDRDesc),
    pub getDesc: unsafe extern "thiscall" fn(this: *const gfc__HDRBase) -> *const gfc__HDRDesc,
    pub initializeLuminance: unsafe extern "thiscall" fn(this: *mut gfc__HDRBase),
    pub clearBloom: unsafe extern "thiscall" fn(this: *mut gfc__HDRBase),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TerrainDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    pub fn as_std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent________ptr(&self) -> *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______{
        self as *const _ as _
    }

pub fn as_std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent________mut_ptr(&mut self) -> *mut std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______{
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__FogDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__FullScreenEffectDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AffineTransform {
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub Elements: compile_error!("unimplemented feature: sizeof array 0x0"),
    __pdbindgen_padding: [u8; 36],
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__RegionLayerData__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__RegionLayerData_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Image {
    pub __vfptr: *const gfc__Image____vftable,
    pub ReferenceCount: i32,
    pub mType: gfc__ImageType,
    pub mFormat: gfc__ImageFormat,
    pub mSurfaces: std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____,
}

impl gfc__Image {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Image____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__MatrixArrayPool {
    pub mpFirst: *mut u8,
    pub mpLast: *mut u8,
    pub mpEnd: *mut u8,
    pub mpOrig: *mut u8,
}

#[repr(C)]
pub struct gfc__World {
    pub __vfptr: *const gfc__World____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
    pub mDefaultRegionIdx: i32,
    pub mNextRegionID: i32,
    pub mRegionData: gfc__Vector_gfc__AutoRef_gfc__WorldRegionData__0_gfc__CAllocator_,
    pub mTerrainDesc: gfc__AutoRef_gfc__TerrainDesc_,
    pub mSkyDesc: gfc__AutoRef_gfc__SkyDesc_,
    pub mExternallyModified: bool,
    pub mRoot: gfc__AutoRef_gfc__WorldGroup_,
    pub mDoPhysics: bool,
    pub mSceneManager: gfc__AutoRef_gfc__SceneManager_,
    pub mPhysicsManager: gfc__AutoRef_gfc__PhysicsManager_,
    pub mParticleManager: gfc__AutoRef_gfc__ParticleManager_,
    pub mAnimationManager: gfc__AutoRef_gfc__AnimationManager_,
    pub mTerrainManager: gfc__AutoRef_gfc__TerrainManager_,
    pub mTimer: gfc__AutoRef_gfc__WorldTimer_,
    pub mAlternateTimer: gfc__AutoRef_gfc__WorldTimer_,
    pub mPath: gfc__String,
    pub mMMOService: *mut gfc__Object,
    pub mActiveCinematic: *mut gfc__Cinematic,
    pub mUseLookAtCamera: bool,
    pub mLookAtCameraPos: gfc__TVector3_float_gfc__FloatMath_,
    pub mTip: gfc__String,
    pub mTipCounter: f32,
    pub mInfoMessage: gfc__String,
    pub mInfoMessageXoffset: f32,
    pub mInfoMessageYoffset: f32,
    pub mInfoMessageCounter: f32,
    pub mInfoMessageLines: i32,
    pub mScriptEnvironment: gfc__AutoRef_gfc__Environment_,
    pub mRemoveQueue: gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_,
    pub mRemoveTemp: gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_,
    pub mComponents:
        gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____,
    pub mEditorActiveRegionIdx: i32,
    pub mPathMan: gfc__AutoRef_gfc__IRefObject_,
    pub mVisualDebuggerPort: i32,
    pub mUseSharedVisualDebugger: bool,
    pub mHavokWorldExtents: gfc__TBox_float_gfc__FloatMath_,
    pub mUse32BitBroadphase: bool,
    pub mUseKdTree: bool,
    pub mAllowCinematic: bool,
    pub mInEditor: bool,
    pub mInExporter: bool,
    pub mIsSettling: bool,
}

impl gfc__World {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__World____vftable {
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
pub struct gfc__Vector_gfc__AffineTransform_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AffineTransform,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldTimer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ObjectCloner {
    pub mObjectDatabase: gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____,
}

#[repr(C)]
pub struct gfc__AmbientDesc {
    pub __vfptr: *const gfc__AmbientDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
    pub mBlendDuration: f32,
    pub mLowerAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mUpperAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mRimColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
    __pdbindgen_padding: [u8; 12],
}

impl gfc__AmbientDesc {
    pub fn as_gfc__EnvironmentDesc_ptr(&self) -> *const gfc__EnvironmentDesc {
        self as *const _ as _
    }

    pub fn as_gfc__EnvironmentDesc_mut_ptr(&mut self) -> *mut gfc__EnvironmentDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AmbientDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__Vector_unsigned_char_0_gfc__CAllocator_ {
    pub mData: *mut u8,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__DepthOfFieldDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Texture {
    pub __vfptr: *const gfc__Texture____vftable,
    pub ReferenceCount: i32,
    pub mName: gfc__HString,
}

impl gfc__Texture {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Texture____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> gfc__Texture__Type,
    pub getFormat: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> gfc__ImageFormat,
    pub getWidth: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> u16,
    pub getHeight: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> u16,
    pub getDepth: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> u16,
    pub getSize: unsafe extern "thiscall" fn(this: *const gfc__Texture) -> u32,
    pub updateAddress: unsafe extern "thiscall" fn(this: *mut gfc__Texture, _: *mut (), _: *mut ()),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AnimationManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DynMeshNode {
    pub mVertices: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mNumVertices: u32,
    pub mPrimType: u8,
    pub mCurrentVtx: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mCurrentVtxCount: u32,
    __pdbindgen_padding: [u8; 12],
    pub mCurrentView: gfc__Matrix4,
    pub mCurrentViewInv: gfc__Matrix4,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ShaderDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__UIRenderer {
    pub mGraphics: *mut gfc__Graphics,
    __pdbindgen_padding: [u8; 12],
    pub mRenderer: gfc__DynamicRenderer,
    pub mTransformStack: gfc__Vector_gfc__AffineTransform_0_gfc__CAllocator_,
    pub mParamStack: gfc__Vector_gfc__UIRenderer__Params_0_gfc__CAllocator_,
    pub mClipStack: gfc__Vector_gfc__UIRenderer__Clip_0_gfc__CAllocator_,
    pub mCurrentShader: gfc__AutoRef_gfc__Shader_,
    pub mCurrentMaterial: gfc__AutoRef_gfc__Material_,
    pub mStartTime: u32,
    pub mSolidMaterial: gfc__AutoRef_gfc__Material_,
    pub mClipID: i32,
    __pdbindgen_padding_2: [u8; 8],
}

#[repr(C)]
pub struct gfc__UIRenderer__Clip {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
    pub UV0: gfc__TVector4_float_gfc__FloatMath_,
    pub UV1: gfc__TVector4_float_gfc__FloatMath_,
    pub Mat: gfc__AutoRef_gfc__Material_,
}

#[repr(C)]
pub struct gfc__UIRenderer__Params {
    pub AddColor: gfc__TVector4_float_gfc__FloatMath_,
    pub MulColor: gfc__TVector4_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Animation_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VertexBuffer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Frustum {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub planes: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 96],
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CameraBlurDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ByteOutputStream {
    pub __vfptr: *const gfc__ByteOutputStream____vftable,
    pub ReferenceCount: i32,
    pub mEndianess: i32,
    pub mPosition: i32,
    pub mOutput: gfc__Vector_unsigned_char_0_gfc__CAllocator_,
}

impl gfc__ByteOutputStream {
    pub fn as_gfc__OutputStream_ptr(&self) -> *const gfc__OutputStream {
        self as *const _ as _
    }

    pub fn as_gfc__OutputStream_mut_ptr(&mut self) -> *mut gfc__OutputStream {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ByteOutputStream____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub write: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: *const (), _: i32),
    pub flush: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub isSeekable: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> i64,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Material_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__LayerLoadCondition__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__LayerLoadCondition_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__LayerLoadCondition_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SceneManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FullScreenEffectDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldRegion_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__DynMeshBuffer___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__DynMeshBuffer,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__RegionLayer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ShaderContext_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldRegion {
    pub __vfptr: *const gfc__WorldRegion____vftable,
    pub ReferenceCount: i32,
    pub mID: i32,
    pub mName: gfc__HString,
    pub mNextLayerID: i32,
    pub mLayerData: gfc__Vector_gfc__AutoRef_gfc__RegionLayerData__0_gfc__CAllocator_,
    pub mCells: gfc__Vector_gfc__AutoRef_gfc__RegionCell__0_gfc__CAllocator_,
    pub mPortals: gfc__Vector_gfc__AutoRef_gfc__RegionPortal__0_gfc__CAllocator_,
    pub mLoadRegions: gfc__Vector_gfc__AutoRef_gfc__WorldObject__0_gfc__CAllocator_,
    pub mNearDistance: f32,
    pub mFarDistance: f32,
    pub mBackgroundColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mIsSky: bool,
    pub mReverbType: i32,
    pub mSoundID: i32,
    pub mDSPEffectID: i32,
    pub mMusicListID: i32,
    pub mPathfindingData: gfc__AutoRef_gfc__Object_,
    pub mCreatures: gfc__AutoRef_gfc__WorldGroup_,
    pub mLoadConditions: gfc__Vector_gfc__AutoRef_gfc__LayerLoadCondition__0_gfc__CAllocator_,
    pub mExternallyModified: bool,
    pub mWorld: *mut gfc__World,
    pub mEnvironment: gfc__AutoRef_gfc__EnvironmentRegion_,
    pub mActiveLayerIndex: i32,
    pub mSoundDescFile: gfc__String,
    pub mSounds: gfc__AutoRef_gfc__SoundList_,
    pub mBakedLightData: gfc__AutoRef_gfc__BakedLightData_,
    pub mStaticLighting: *mut gfc__StaticLightingRegionOpt,
    pub mBounds: gfc__TBox_float_gfc__FloatMath_,
    pub mHide: bool,
}

impl gfc__WorldRegion {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__WorldRegion____vftable {
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
pub struct gfc__Vector_gfc__RenderNode___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__RenderNode,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__HDRDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Shader_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Texture_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ObjectReader {
    pub __vfptr: *const gfc__ObjectReader____vftable,
    pub ReferenceCount: i32,
}

impl gfc__ObjectReader {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ObjectReader____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub readObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ObjectReader,
        result: *mut gfc__AutoRef_gfc__Object_,
        _: gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FogDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__RegionCell__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__RegionCell_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SkyDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__Node3D___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__Node3D,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__DynMeshNode___ {
    pub m_vector: gfc__Vector_gfc__DynMeshNode___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__Quaternion {
    pub q: gfc__TVector4_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__Material {
    pub __vfptr: *const gfc__Material____vftable,
    pub ReferenceCount: i32,
    pub mLocked: bool,
    pub mName: gfc__HString,
    pub mLightingModel: u8,
    pub mBlendMode: u8,
    pub mCullMode: u8,
    pub mDepthTestMode: u8,
    pub mDrawOrder: u8,
    pub mSortBias: i32,
    pub mCreateInstance: bool,
    pub mHidden: bool,
    pub mParameters: gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___,
    pub mShaderName: gfc__HString,
    pub mShader: gfc__AutoRef_gfc__Shader_,
    pub mShaderContext: gfc__AutoRef_gfc__ShaderContext_,
}

impl gfc__Material {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Material____vftable {
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
pub struct gfc__LightNode {
    pub Transform: gfc__Matrix4,
    pub Position: gfc__TVector4_float_gfc__FloatMath_,
    pub Color: gfc__TVector4_float_gfc__FloatMath_,
    pub Attenuation: gfc__TVector4_float_gfc__FloatMath_,
    pub InvLightRange: gfc__TVector4_float_gfc__FloatMath_,
    pub LightViewProj: gfc__Matrix4,
    pub LightType: i32,
    pub LightGroup: u32,
    pub AttenStart: f32,
    pub AttenEnd: f32,
    pub FOV: f32,
    pub ShadowBias: f32,
    pub OrthoWidth: f32,
    pub OrthoHeight: f32,
    pub CastShadows: bool,
    pub DynShadowsOnly: bool,
    pub Static: bool,
    pub Flag: bool,
    pub Frust: gfc__Frustum,
    pub BVolume: gfc__BoundingVolume,
    pub ProjTexture: *mut gfc__Material,
    pub ProjTextureFaceFlags: u32,
    pub SceneContext: *mut (),
    pub ShadowContext: *mut (),
    pub NumVisibleObjects: [u32; 6],
    pub NumDynamicObjects: [u32; 6],
    pub VisibleObjects: gfc__Vector_gfc__LightNode__VisNode_0_gfc__CAllocator_,
    pub SplitNum: i32,
    pub NumSplits: i32,
    pub Splits: [*mut gfc__LightNode; 5],
    pub SplitStart: f32,
    pub SplitEnd: f32,
}

#[repr(C)]
pub struct gfc__LightNode__VisNode {
    pub VisFlags: u32,
    pub Node: *mut gfc__RenderNode,
}

#[repr(C)]
pub struct gfc__Vector_gfc__LightNode__VisNode_0_gfc__CAllocator_ {
    pub mData: *mut gfc__LightNode__VisNode,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Node3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__RenderNode___ {
    pub m_vector: gfc__Vector_gfc__RenderNode___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__RenderNode {
    __pdbindgen_padding: [u8; 340],
    pub Flags: gfc__TFlags_unsigned_long_,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub Camera: *mut gfc__Camera3D,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub Shader: *mut gfc__Shader,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub Material: *mut gfc__Material,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub RenderCallback: *mut gfc__IRenderCallback,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub MeshInstance: *mut compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub Context: i32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub SceneContext: *mut (),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub ObjectContext: *mut (),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub Transform: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub World: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub WorldView: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub WorldViewProj: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub MatrixArray: *mut compile_error!("malformed PDB: oops"),
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub BVolume: gfc__BoundingVolume,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub LightGroup: u32,
    pub BlendMode: i32,
    pub MeshFormat: u32,
    pub ObjectID: u32,
    pub Priority: u32,
    pub MeshIndex: u32,
    pub SortBias: i32,
    pub DecalOrder: i32,
    pub ZPos: i32,
    pub LODLevel: u32,
    pub LightCount: i32,
    pub OmniLightCount: i32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub Lights: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 64],
    pub DynMeshNode: *mut gfc__DynMeshNode,
    pub ObjectColor: gfc__TVector4_float_gfc__FloatMath_,
    pub FadeAmount: gfc__TVector4_float_gfc__FloatMath_,
    __pdbindgen_padding_3: [u8; 8],
}

#[repr(C)]
pub struct gfc__DynamicRenderNode {
    pub mNext: *mut gfc__DynamicRenderNode,
    pub mPrev: *mut gfc__DynamicRenderNode,
    pub mData: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mPrimType: u8,
    pub mCurrentVertex: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mNum: u32,
    pub mMaxVertices: u32,
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CameraBlurDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Vector_gfc__DynMeshNode___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__DynMeshNode,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VertexDeclaration {
    pub __vfptr: *const gfc__VertexDeclaration____vftable,
    pub ReferenceCount: i32,
}

impl gfc__VertexDeclaration {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__VertexDeclaration____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FogDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    pub mFixedData: [u8; 40],
}

impl gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_ {
    pub fn as_gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator__ptr(
        &self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
        self as *const _ as _
    }

    pub fn as_gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator__mut_ptr(
        &mut self,
    ) -> *mut gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__DynMeshBuffer {
    pub mVertices: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mVertexCount: u32,
    pub mSlot: u32,
    pub mNumVerticesUsed: u32,
}

#[repr(C)]
pub struct gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    pub fn as_std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object________ptr(&self) -> *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
        self as *const _ as _
    }

    pub fn as_std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object________mut_ptr(&mut self) -> *mut std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_ {
    pub mData: *mut gfc__TVector4_float_gfc__FloatMath_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__WorldRegionData__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__WorldRegionData_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    pub fn as_std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter________ptr(&self) -> *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______{
        self as *const _ as _
    }

pub fn as_std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter________mut_ptr(&mut self) -> *mut std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______{
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ParticleManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__VertexFormat {
    pub mFormat: gfc__Vector_unsigned_short_0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__BoundingVolume {
    pub b: gfc__TBox_float_gfc__FloatMath_,
    pub s: gfc__TSphere_float_gfc__FloatMath_,
    pub r#type: i32,
}

#[repr(C)]
pub struct gfc__Node3D {
    pub __vfptr: *const gfc__Node3D____vftable,
    pub ReferenceCount: i32,
    pub __vfptr_2: *const gfc__Hierarchical_gfc__Node3D_____vftable,
    pub mParent: *mut gfc__Node3D,
    pub mHead: gfc__AutoRef_gfc__Node3D_,
    pub mTail: gfc__AutoRef_gfc__Node3D_,
    pub mNext: gfc__AutoRef_gfc__Node3D_,
    pub mPrev: gfc__AutoRef_gfc__Node3D_,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mVersion: i32,
    __pdbindgen_padding: [u8; 8],
    pub o: gfc__Node3D___o,
    pub mVisibility: f32,
    pub mHVisibility: f32,
    __pdbindgen_padding_2: [u8; 8],
    pub mWorldMatrix: gfc__Matrix4,
    pub mBlendWeight: f32,
    pub mHashNext: *mut gfc__Node3D,
    pub mName: gfc__HString,
}

impl gfc__Node3D {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }

    pub fn as_gfc__Hierarchical_gfc__Node3D__ptr(&self) -> *const gfc__Hierarchical_gfc__Node3D_ {
        self as *const _ as _
    }

    pub fn as_gfc__Hierarchical_gfc__Node3D__mut_ptr(
        &mut self,
    ) -> *mut gfc__Hierarchical_gfc__Node3D_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__Node3D____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: u32) -> *mut (),
    pub addFront:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub addBack:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub add:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_),
    pub remove_2:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_),
    pub added:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
    pub removed:
        unsafe extern "thiscall" fn(this: *mut gfc__Hierarchical_gfc__Node3D_, _: *mut gfc__Node3D),
}

#[repr(C)]
pub struct gfc__Node3D___o {
    pub p: gfc__TVector3_float_gfc__FloatMath_,
    __pdbindgen_padding: [u8; 4],
    pub r: gfc__TVector3_float_gfc__FloatMath_,
    __pdbindgen_padding_2: [u8; 4],
    pub s: gfc__TVector3_float_gfc__FloatMath_,
    __pdbindgen_padding_3: [u8; 4],
    pub q: gfc__Quaternion,
}

#[repr(C)]
pub struct gfc__HDRDesc {
    pub __vfptr: *const gfc__HDRDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
    pub mToneMap: bool,
    pub mMidTone: f32,
    pub mMinToneMapMult: f32,
    pub mMaxToneMapMult: f32,
    pub mAdaptSpeed: f32,
    pub mBrightPassValue: f32,
    pub mBloomIntensity: f32,
    pub mBloomPassCount: u32,
    pub mBloomPersistence: f32,
    pub mBlendDuration: f32,
    pub mManager: *mut gfc__EnvironmentManager,
}

impl gfc__HDRDesc {
    pub fn as_gfc__EnvironmentDesc_ptr(&self) -> *const gfc__EnvironmentDesc {
        self as *const _ as _
    }

    pub fn as_gfc__EnvironmentDesc_mut_ptr(&mut self) -> *mut gfc__EnvironmentDesc {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__HDRDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__EnvironmentDesc {
    pub __vfptr: *const gfc__EnvironmentDesc____vftable,
    pub ReferenceCount: i32,
    pub mApplied: bool,
}

impl gfc__EnvironmentDesc {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__EnvironmentDesc____vftable {
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__String_const__gfc__String___ {
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std__pair_gfc__String_const__gfc__String___ {
    pub fn as_std___Allocator_base_std__pair_gfc__String_const__gfc__String____ptr(
        &self,
    ) -> *const std___Allocator_base_std__pair_gfc__String_const__gfc__String___ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std__pair_gfc__String_const__gfc__String____mut_ptr(
        &mut self,
    ) -> *mut std___Allocator_base_std__pair_gfc__String_const__gfc__String___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    pub fn as_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0__ptr(&self) -> *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *const _ as _
    }

    pub fn as_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0__mut_ptr(&mut self) -> *mut std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Myval: std__pair_gfc__String_const__gfc__String_,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
{
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
    pub fn as_std___Container_base0_ptr(&self) -> *const std___Container_base0 {
        self as *const _ as _
    }

    pub fn as_std___Container_base0_mut_ptr(&mut self) -> *mut std___Container_base0 {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__String_ {
    pub first: gfc__String,
    pub second: gfc__String,
}

impl std__pair_gfc__String_const__gfc__String_ {
    pub fn as_std___Pair_base_gfc__String_const__gfc__String__ptr(
        &self,
    ) -> *const std___Pair_base_gfc__String_const__gfc__String_ {
        self as *const _ as _
    }

    pub fn as_std___Pair_base_gfc__String_const__gfc__String__mut_ptr(
        &mut self,
    ) -> *mut std___Pair_base_gfc__String_const__gfc__String_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__String_const__gfc__String_ {
    pub first: gfc__String,
    pub second: gfc__String,
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__String_const__gfc__String___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    pub fn as_std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____ptr(&self) -> *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____mut_ptr(&mut self) -> *mut std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____ptr(&self) -> *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____mut_ptr(&mut self) -> *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    __pdbindgen_padding: [u8; 1],
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    pub fn as_std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____ptr(&self) -> *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _ as _
    }

    pub fn as_std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0____mut_ptr(&mut self) -> *mut std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MBWeights {
    pub Weight0: gfc__MBWeight,
    pub Weight1: gfc__MBWeight,
    pub Weight2: gfc__MBWeight,
    pub Weight3: gfc__MBWeight,
}

#[repr(C)]
pub struct gfc__Vector_gfc__MBWeights_0_gfc__CAllocator_ {
    pub mData: *mut gfc__MBWeights,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__MBSubMesh_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__MeshBuilder {
    pub __vfptr: *const gfc__MeshBuilder____vftable,
    pub ReferenceCount: i32,
    pub mBounds: gfc__BoundingVolume,
    pub mVertexFormat: gfc__VertexFormat,
    pub mSubMeshes: gfc__Vector_gfc__AutoRef_gfc__MBSubMesh__0_gfc__CAllocator_,
    pub mBones: gfc__Vector_gfc__MBBone_0_gfc__CAllocator_,
    pub mUserData: gfc__Map_gfc__String_gfc__String_std__less_gfc__String___,
    pub mFlags: gfc__TFlags_unsigned_long_,
}

impl gfc__MeshBuilder {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MeshBuilder____vftable {
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
    pub computeTangentVectors:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder, _: *mut gfc__MBSubMesh),
    pub optimize:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder, _: *mut gfc__MBSubMesh) -> bool,
}

#[repr(C)]
pub struct gfc__MBWeight {
    pub Index: i32,
    pub Value: f32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SkinMesh_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__MeshReader {
    pub __vfptr: *const gfc__MeshReader____vftable,
    pub ReferenceCount: i32,
}

impl gfc__MeshReader {
    pub fn as_gfc__ObjectReader_ptr(&self) -> *const gfc__ObjectReader {
        self as *const _ as _
    }

    pub fn as_gfc__ObjectReader_mut_ptr(&mut self) -> *mut gfc__ObjectReader {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MeshReader____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub readObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ObjectReader,
        result: *mut gfc__AutoRef_gfc__Object_,
        _: gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct gfc__MBBone {
    pub Offset: gfc__Matrix4,
    pub ID: i32,
    pub Name: gfc__String,
}

#[repr(C)]
pub struct gfc__MBSubMesh {
    pub __vfptr: *const gfc__MBSubMesh____vftable,
    pub ReferenceCount: i32,
    pub PrimType: i32,
    pub HasWeights: bool,
    pub MaterialID: i32,
    pub MaterialName: gfc__String,
    pub VertexCount: i32,
    pub Position: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Normal: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Tangent: gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Binormal: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Tex0: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Tex1: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Tex2: gfc__Vector_gfc__TVector3_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Color0: gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Color1: gfc__Vector_gfc__TVector4_float_gfc__FloatMath__0_gfc__CAllocator_,
    pub Weights: gfc__Vector_gfc__MBWeights_0_gfc__CAllocator_,
    pub Indices: gfc__Vector_unsigned_long_0_gfc__CAllocator_,
}

impl gfc__MBSubMesh {
    pub fn as_gfc__Object_ptr(&self) -> *const gfc__Object {
        self as *const _ as _
    }

    pub fn as_gfc__Object_mut_ptr(&mut self) -> *mut gfc__Object {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MBSubMesh____vftable {
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
pub struct gfc__ObjectWriter {
    pub __vfptr: *const gfc__ObjectWriter____vftable,
    pub ReferenceCount: i32,
}

impl gfc__ObjectWriter {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__ObjectWriter____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

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
