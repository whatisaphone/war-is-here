#![allow(non_snake_case, unused_imports)]

use super::{
    types10::*,
    types11::*,
    types12::*,
    types2::*,
    types3::*,
    types4::*,
    types5::*,
    types6::*,
    types7::*,
    types8::*,
    types9::*,
};

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
    pub __: *const (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_steamid")]
#[repr(C)]
pub struct CSteamID {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub m_steamid: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct CCallbackBase {
    pub __vfptr: *const CCallbackBase____vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

#[repr(C)]
pub struct CCallbackBase____vftable {
    pub __: *const (),
    pub ___2: *const (),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct keen__UserAccountSystemBase {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field runningOperations")]
#[repr(C)]
pub struct keen__UserAccountSystemBase {
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub runningOperations: compile_error!("unimplemented feature: class layout 0x0"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field runningOperations")]
#[repr(C)]
pub struct keen__UserAccountSystem {
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub runningOperations: compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub accounts: compile_error!("unimplemented feature: class layout 0x0"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_steamIDUser")]
#[repr(C)]
pub struct LeaderboardEntry_t {
    #[cfg(pdb_issue = "error in CSteamID")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field FindOrCreateLeaderboard")]
#[repr(C)]
pub struct ISteamUserStats____vftable {
    pub RequestCurrentStats: unsafe extern "thiscall" fn(this: *mut ISteamUserStats) -> bool,
    pub __: *const (),
    pub ___2: *const (),
    pub ___3: *const (),
    pub ___4: *const (),
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
    pub ___5: *const (),
    pub ___6: *const (),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub FindOrCreateLeaderboard: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: *const i8,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
        _: compile_error!("unimplemented feature: type kind 0x1507"),
    ) -> u64,
    pub FindLeaderboard:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: *const i8) -> u64,
    pub GetLeaderboardName:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> *const i8,
    pub GetLeaderboardEntryCount:
        unsafe extern "thiscall" fn(this: *mut ISteamUserStats, _: u64) -> i32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub GetLeaderboardSortMethod: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1507"
    ),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub GetLeaderboardDisplayType: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1507"
    ),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub DownloadLeaderboardEntries: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub UploadLeaderboardScore: unsafe extern "thiscall" fn(
        this: *mut ISteamUserStats,
        _: u64,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
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
    pub ___7: *const (),
    pub ___8: *const (),
    pub ___9: *const (),
    pub ___10: *const (),
}

#[repr(C)]
pub struct keen__Array_unsigned_char_ {
    pub m_pData: *mut u8,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__SaveDataSystem {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field saveDataSystemWin32Type")]
#[repr(C)]
pub struct keen__SaveDataSystem {
    pub pAllocator: *mut keen__MemoryAllocator,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub saveDataSystemWin32Type: compile_error!("unimplemented feature: type kind 0x1507"),
    pub pProvider: *mut keen__SaveData__SaveDataProviderWin32Interface,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub currentOperation: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub currentOperationStatus: compile_error!("unimplemented feature: type kind 0x1507"),
    pub operationProfileIndex: u32,
    pub pOperationProfile: *const keen__SaveData__Profile,
    pub pOperationSaveDataInput: *const keen__SaveData__SaveData,
    pub pOperationSaveDataOutput: *mut keen__SaveData__SaveData,
    pub pOperationStorageData: *mut keen__SaveData__StorageOperation,
    pub suppressSystemDialogs: bool,
}

#[repr(C)]
pub struct keen__SaveDataHandler {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_state")]
#[repr(C)]
pub struct keen__SaveDataHandler {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub m_state: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field state")]
#[repr(C)]
pub struct keen__SaveDataHandler__SaveGameClientData {
    pub currentSaveDatas: keen__SizedArray_keen__SaveData__SaveData_,
    pub saveData: keen__SaveDataHandler__SaveGameData,
    pub userAccountId: keen__UserAccountId,
    pub lastSavedDataId: u32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub state: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field id")]
#[repr(C)]
pub struct keen__SaveDataInteractionData {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub id: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub responseOptions: compile_error!("unimplemented feature: type kind 0x1507"),
    pub responseOptionCount: u32,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface {
    pub __vfptr: *const keen__SaveData__SaveDataProviderWin32Interface____vftable,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface____vftable {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getOperationResult")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub getOperationResult: unsafe extern "thiscall" fn(
        this: *mut keen__SaveData__SaveDataProviderWin32Interface,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1507"
    ),
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
pub struct keen__SaveData__SaveDataSystemCreationProviderParameters {}

#[repr(C)]
pub struct keen__SaveData__SaveDataSystemCreationParameters {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field saveDataSystemType")]
#[repr(C)]
pub struct keen__SaveData__SaveDataSystemCreationParameters {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub saveDataSystemType: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field controllerInfos")]
#[repr(C)]
pub struct keen__InputSystem {
    pub eventQueue: keen__Queue_keen__InputEvent_,
    pub controllerState: keen__PlatformControllerState,
    pub storedEvents: keen__SizedArray_keen__InputEvent_,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub controllerInfos: compile_error!("unimplemented feature: class layout 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub autoCatchType: compile_error!("unimplemented feature: type kind 0x1507"),
    pub autoCatchPlayerId: *const keen__LocalPlayerIdStructureType,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub mappedAxisButtonStates: compile_error!("unimplemented feature: class layout 0x0"),
    pub gestureHelper: keen__GestureHelper,
}

#[repr(C)]
pub struct keen__ControllerInfo {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field controllerClass")]
#[repr(C)]
pub struct keen__ControllerInfo {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub controllerClass: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub r#type: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field data")]
#[repr(C)]
pub struct keen__InputEvent {
    pub controllerId: u8,
    pub controllerClass: u8,
    pub localPlayerId: u8,
    pub r#type: u8,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub data: compile_error!("unimplemented feature: type kind 0x1506"),
}

#[repr(C)]
pub struct keen__PlatformControllerState {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field controllers")]
#[repr(C)]
pub struct keen__PlatformControllerState {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub controllers: compile_error!("unimplemented feature: class layout 0x0"),
    pub mouseVisible: bool,
    pub mousePositionRelative: bool,
    pub lastMousePosition: keen__float2,
    pub pDirect8: *mut IDirectInput8A,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub directInputControllers: compile_error!("unimplemented feature: class layout 0x0"),
    pub directInputControllerCount: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub lastState: compile_error!("unimplemented feature: class layout 0x0"),
    pub mouseWheelButtonFlags: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub steamController: compile_error!("unimplemented feature: class layout 0x0"),
    pub steamControllerCount: u32,
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
pub struct std__allocator_char_ {}

impl std__allocator_char_ {
    pub fn as_std___Allocator_base_char__ptr(&self) -> *const std___Allocator_base_char_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_char__mut_ptr(&mut self) -> *mut std___Allocator_base_char_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Container_base0 {}

#[repr(C)]
pub struct std___String_val_char_std__allocator_char___ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field _Bx")]
#[repr(C)]
pub struct std___String_val_char_std__allocator_char___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
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
pub struct std__binary_function_gfc__HString_gfc__HString_bool_ {}

#[repr(C)]
pub struct std__less_gfc__HString_ {}

impl std__less_gfc__HString_ {
    pub fn as_std__binary_function_gfc__HString_gfc__HString_bool__ptr(
        &self,
    ) -> *const std__binary_function_gfc__HString_gfc__HString_bool_ {
        self as *const _ as _
    }

    pub fn as_std__binary_function_gfc__HString_gfc__HString_bool__mut_ptr(
        &mut self,
    ) -> *mut std__binary_function_gfc__HString_gfc__HString_bool_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Allocator_base_char_ {}

#[repr(C)]
pub struct std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field _Bx")]
#[repr(C)]
pub struct std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
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
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub array: [f32; 3],
}

#[repr(C)]
pub struct gfc__Vector_gfc__String_0_gfc__CAllocator_ {
    pub mData: *mut gfc__String,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__TVector4_float_gfc__FloatMath_ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub array: [f32; 4],
}

#[repr(C)]
pub struct gfc__TBox_float_gfc__FloatMath_ {
    pub min: gfc__TVector3_float_gfc__FloatMath_,
    pub max: gfc__TVector3_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__ValueStack {
    pub mSize: i32,
    pub mStack: *mut gfc__ValueStack___Stack,
}

#[repr(C)]
pub struct gfc__ValueStack___Stack {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field Stack")]
#[repr(C)]
pub struct gfc__ValueStack___Stack {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub Stack: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct gfc__Event {
    pub mEvent: keen__Event,
}

#[repr(C)]
pub struct gfc__ThreadSafeBool {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mValue")]
#[repr(C)]
pub struct gfc__ThreadSafeBool {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mValue: compile_error!("unimplemented feature: type kind 0x1001"),
}

#[repr(C)]
pub struct gfc__WString {
    pub mStringData: *mut gfc__WString__StringData,
}

#[repr(C)]
pub struct gfc__WString__StringData {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mRefCount")]
#[repr(C)]
pub struct gfc__WString__StringData {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mRefCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mCapacity: u16,
    pub mCurrentSize: u16,
    pub mData: [u16; 1],
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
pub struct gfc__AutoRef_gfc__Value_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Matrix4 {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m")]
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
    pub x: gfc__Matrix4Row,
    pub y: gfc__Matrix4Row,
    pub z: gfc__Matrix4Row,
    pub w: gfc__Matrix4Row,
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m: compile_error!("unimplemented feature: sizeof array 0x0"),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__InputStream_ {
    pub p: *mut gfc__IRefObject,
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
    pub u: f32,
    pub v: f32,
    pub array: [f32; 2],
}

#[repr(C)]
pub struct gfc__String {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mString")]
#[repr(C)]
pub struct gfc__String {
    #[cfg(
        pdb_issue = "error in std__basic_string_char_std__char_traits_char__std__allocator_char___"
    )]
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
pub struct gfc__AutoRef_gfc__Object_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__IRefObject {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__IRefObject {
    pub __vfptr: *const gfc__IRefObject____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__AutoRef_gfc__IVoidDelegate_ {
    pub p: *mut gfc__IRefObject,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Object {
    pub __vfptr: *const gfc__Object____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Method__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
}

#[repr(C)]
pub struct gfc__Mutex {
    pub mMutex: keen__Mutex,
}

#[repr(C)]
pub struct gfc__Property {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Property {
    pub __vfptr: *const gfc__Property____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
        _: *mut gfc__Object,
    ) -> gfc__AutoRef_gfc__Value_,
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
        _: *mut gfc__Object,
        _: gfc__AutoRef_gfc__Value_,
    ) -> gfc__AutoRef_gfc__Value_,
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
        _: *mut gfc__Object,
    ) -> gfc__AutoRef_gfc__Value_,
    pub getValues: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
    ) -> gfc__AutoRef_gfc__Value_,
    pub clone: unsafe extern "thiscall" fn(
        this: *mut gfc__Property,
        _: *mut gfc__Object,
        _: *mut gfc__Object,
    ),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Class {
    pub __vfptr: *const gfc__Class____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub __: *const (),
    pub isAbstract: unsafe extern "thiscall" fn(this: *const gfc__Class) -> bool,
    pub newInstance:
        unsafe extern "thiscall" fn(this: *mut gfc__Class) -> gfc__AutoRef_gfc__Object_,
    pub ___2: *const (),
    pub ___3: *const (),
    pub getMethodByName: unsafe extern "thiscall" fn(
        this: *mut gfc__Class,
        _: *const gfc__HString,
    ) -> *mut gfc__Method,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Class, _: *const u64) -> *mut gfc__Method,
}

#[repr(C)]
pub struct gfc__Environment {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Environment {
    pub __vfptr: *const gfc__Environment____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Matrix4Row {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Constructor_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Method {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Method {
    pub __vfptr: *const gfc__Method____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{}

#[repr(C)]
pub struct std__binary_function_gfc__Class___gfc__Class___bool_ {}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
{}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
    pub comp: std__less_gfc__HString_,
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
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node
{
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field _Myval")]
#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    #[cfg(pdb_issue = "error in std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___")]
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {}

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
pub struct std__less_gfc__AutoRef_gfc__Object___ {}

impl std__less_gfc__AutoRef_gfc__Object___ {
    pub fn as_std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool__ptr(
        &self,
    ) -> *const std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_ {
        self as *const _ as _
    }

    pub fn as_std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool__mut_ptr(
        &mut self,
    ) -> *mut std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
{
    pub comp: std__less_gfc__HString_,
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
{}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{}

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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub comp: std__less_gfc__HString_,
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
    pub comp: std__less_gfc__Class___,
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
pub struct std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_ {}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node
{
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field _Myval")]
#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    #[cfg(pdb_issue = "error in std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___")]
    pub _Myval: std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
{
    pub comp: std__less_gfc__Class___,
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
pub struct std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
pub struct std__less_gfc__Class___ {}

impl std__less_gfc__Class___ {
    pub fn as_std__binary_function_gfc__Class___gfc__Class___bool__ptr(
        &self,
    ) -> *const std__binary_function_gfc__Class___gfc__Class___bool_ {
        self as *const _ as _
    }

    pub fn as_std__binary_function_gfc__Class___gfc__Class___bool__mut_ptr(
        &mut self,
    ) -> *mut std__binary_function_gfc__Class___gfc__Class___bool_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
    pub second: gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
{}

impl std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node__ptr(&self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
        self as *const _ as _
    }

    pub fn as_std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node__mut_ptr(&mut self) -> *mut std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {}

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
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {}

#[repr(C)]
pub struct std__less_gfc__String_ {}

impl std__less_gfc__String_ {
    pub fn as_std__binary_function_gfc__String_gfc__String_bool__ptr(
        &self,
    ) -> *const std__binary_function_gfc__String_gfc__String_bool_ {
        self as *const _ as _
    }

    pub fn as_std__binary_function_gfc__String_gfc__String_bool__mut_ptr(
        &mut self,
    ) -> *mut std__binary_function_gfc__String_gfc__String_bool_ {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    pub comp: std__less_gfc__Class___,
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
    pub comp: std__less_gfc__Class___,
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
pub struct std__binary_function_gfc__String_gfc__String_bool_ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    pub comp: std__less_gfc__HString_,
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
    pub comp: std__less_gfc__Class___,
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
{}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    pub comp: std__less_gfc__HString_,
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
{}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{}

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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field first")]
#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
    pub second: gfc__AutoRef_gfc__Parameter_,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
{}

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
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
pub struct gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__AmbientDesc_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__VertexBuffer {
    pub __vfptr: *const gfc__VertexBuffer____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__IndexBuffer {
    pub __vfptr: *const gfc__IndexBuffer____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub u: i32,
    pub v: i32,
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
pub struct gfc__AutoRef_gfc__HDRDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__DynamicRenderer {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mCurrentViewInv")]
#[repr(C)]
pub struct gfc__DynamicRenderer {
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mCurrentViewInv: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mCurrentView: gfc__Matrix4,
    pub mCanDraw: bool,
    pub mCurrent: *mut gfc__DynamicRenderNode,
}

#[repr(C)]
pub struct gfc__FogDesc {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__FogDesc {
    pub __vfptr: *const gfc__FogDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mApplied: bool,
    pub mBlendDuration: f32,
    pub mType: i32,
    pub mColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mDensity: f32,
    pub mStart: f32,
    pub mEnd: f32,
    pub mParameters: gfc__TVector4_float_gfc__FloatMath_,
    pub mCurrentColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__WorldObject_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Graphics {
    pub __vfptr: *const gfc__Graphics____vftable,
}

#[repr(C)]
pub struct gfc__Graphics____vftable {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field setVertexBufferMasked")]
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
    pub __: *const (),
    pub ___2: *const (),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub setVertexBufferMasked: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: *mut gfc__VertexBuffer,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
    ),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub setRenderTarget: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
    ),
    pub setDefaultRenderTarget: unsafe extern "thiscall" fn(this: *const gfc__Graphics),
    pub setDefaultDepthStencilTarget: unsafe extern "thiscall" fn(this: *const gfc__Graphics),
    pub createVertexDeclaration:
        unsafe extern "thiscall" fn(
            this: *mut gfc__Graphics,
            _: *const gfc__VertexFormat,
        ) -> gfc__AutoRef_gfc__VertexDeclaration_,
    pub ___3: *const (),
    pub ___4: *const (),
    pub createIndexBuffer: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u32,
        _: bool,
    ) -> gfc__AutoRef_gfc__IndexBuffer_,
    pub createMeshBuilder:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> gfc__AutoRef_gfc__MeshBuilder_,
    pub ___5: *const (),
    pub ___6: *const (),
    pub ___7: *const (),
    pub ___8: *const (),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub createRenderTexture: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u16,
        _: u16,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
    ) -> gfc__AutoRef_gfc__RenderTexture_,
    pub ___9: *const (),
    pub ___10: *const (),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub createRenderDepth: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u16,
        _: u16,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
    ) -> gfc__AutoRef_gfc__Texture_,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub createRenderCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: u16,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
    ) -> gfc__AutoRef_gfc__Texture_,
    pub ___11: *const (),
    pub ___12: *const (),
    pub ___13: *const (),
    pub ___14: *const (),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub createCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: i32,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
        _: bool,
    ) -> gfc__AutoRef_gfc__Texture_,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub createLightmap: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: i32,
        _: i32,
        _: compile_error!("unimplemented feature: type kind 0x1507"),
    ) -> gfc__AutoRef_gfc__Texture_,
    pub createShader:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> gfc__AutoRef_gfc__Shader_,
    pub createShaderCompiler:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics) -> gfc__AutoRef_gfc__ShaderCompiler_,
    pub createQuery:
        unsafe extern "thiscall" fn(this: *mut gfc__Graphics, _: i32) -> gfc__AutoRef_gfc__Query_,
    pub createCamera: unsafe extern "thiscall" fn(
        this: *mut gfc__Graphics,
        _: i32,
    ) -> gfc__AutoRef_gfc__Camera3D_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Camera3D {
    pub __vfptr: *const gfc__Camera3D____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mFrameID: i32,
    pub mInsideOutside: i32,
    pub mUnderwater: bool,
    pub mHasSun: bool,
    pub mSun: *mut gfc__LightNode,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mCameraMatrix: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mNumUsedRenderNodes: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mLightNodePool: *mut gfc__LightNode,
    pub mNumLightNodes: u32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mNumUsedLightNodes: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mDynMeshNodePool: *mut gfc__DynMeshNode,
    pub mNumDynMeshNodes: u32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mNumUsedDynMeshNodes: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mDynMeshMutex: gfc__Mutex,
    pub mDynMeshNodes: gfc__ThreadSafeVector_gfc__DynMeshNode___,
    pub mDynMeshBuffers: gfc__ThreadSafeVector_gfc__DynMeshBuffer___,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mCurrentDynMeshBuffer: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mMatrixPool: gfc__MatrixArrayPool,
    pub mTerrainChunkSize: i32,
    pub mTerrainHeightMapSize: i32,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__DepthOfFieldDesc {
    pub __vfptr: *const gfc__DepthOfFieldDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mApplied: bool,
    pub mFocusDistance: f32,
    pub mBlurStart: f32,
    pub mBlurRange: f32,
    pub mMaxBlur: f32,
    pub mBlendDuration: f32,
    pub mEnableTint: bool,
    pub mNearTintColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__CameraBlurDesc {
    pub __vfptr: *const gfc__CameraBlurDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Shader {
    pub __vfptr: *const gfc__Shader____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field hasPass")]
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
    pub getDesc:
        unsafe extern "thiscall" fn(this: *mut gfc__Shader) -> gfc__AutoRef_gfc__ShaderDesc_,
    pub isUsed: unsafe extern "thiscall" fn(this: *const gfc__Shader) -> bool,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub hasPass: unsafe extern "thiscall" fn(
        this: *const gfc__Shader,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
    ) -> bool,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub beginPass: unsafe extern "thiscall" fn(
        this: *mut gfc__Shader,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
        _: compile_error!("unimplemented feature: type kind 0x1001"),
    ),
    pub endPass: unsafe extern "thiscall" fn(this: *mut gfc__Shader),
    pub setWorldMatrix: unsafe extern "thiscall" fn(this: *mut gfc__Shader, _: *const gfc__Matrix4),
    pub setDefaultValues: unsafe extern "thiscall" fn(this: *const gfc__Shader),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub setMaterialParams: unsafe extern "thiscall" fn(
        this: *mut gfc__Shader,
        _: *mut gfc__Material,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
    ),
    pub __: *const (),
    pub ___2: *const (),
    pub ___3: *const (),
    pub ___4: *const (),
    pub ___5: *const (),
    pub ___6: *const (),
    pub ___7: *const (),
    pub ___8: *const (),
    pub ___9: *const (),
    pub ___10: *const (),
    pub ___11: *const (),
    pub ___12: *const (),
    pub ___13: *const (),
    pub ___14: *const (),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ShaderCompiler_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldObject {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__WorldObject {
    pub __vfptr: *const gfc__WorldObject____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getAnimation")]
#[repr(C)]
pub struct gfc__WorldObject____vftable {
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Plane {
    pub normalAndDistance: gfc__TVector4_float_gfc__FloatMath_,
}

#[repr(C)]
pub struct gfc__TSphere_float_gfc__FloatMath_ {
    pub center: gfc__TVector3_float_gfc__FloatMath_,
    pub radius: f32,
}

#[repr(C)]
pub struct gfc__HDRBase {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mDesc")]
#[repr(C)]
pub struct gfc__HDRBase {
    pub __vfptr: *const gfc__HDRBase____vftable,
    #[cfg(pdb_issue = "error in gfc__HDRDesc")]
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
    pub comp: std__less_gfc__Class___,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field Elements")]
#[repr(C)]
pub struct gfc__AffineTransform {
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub Elements: compile_error!("unimplemented feature: sizeof array 0x0"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__World {
    pub __vfptr: *const gfc__World____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    #[cfg(pdb_issue = "error in gfc__String")]
    pub mPath: gfc__String,
    pub mMMOService: *mut gfc__Object,
    pub mActiveCinematic: *mut gfc__Cinematic,
    pub mUseLookAtCamera: bool,
    pub mLookAtCameraPos: gfc__TVector3_float_gfc__FloatMath_,
    #[cfg(pdb_issue = "error in gfc__String")]
    pub mTip: gfc__String,
    pub mTipCounter: f32,
    #[cfg(pdb_issue = "error in gfc__String")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__AmbientDesc {
    pub __vfptr: *const gfc__AmbientDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mApplied: bool,
    pub mBlendDuration: f32,
    pub mLowerAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mUpperAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mRimColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__DepthOfFieldDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Texture {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Texture {
    pub __vfptr: *const gfc__Texture____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getType")]
#[repr(C)]
pub struct gfc__Texture____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub getType: unsafe extern "thiscall" fn(
        this: *const gfc__Texture,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1507"
    ),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub getFormat: unsafe extern "thiscall" fn(
        this: *const gfc__Texture,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1507"
    ),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mCurrentView")]
#[repr(C)]
pub struct gfc__DynMeshNode {
    pub mVertices: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mNumVertices: u32,
    pub mPrimType: u8,
    pub mCurrentVtx: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mCurrentVtxCount: u32,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mCurrentView: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub mCurrentViewInv: gfc__Matrix4,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ShaderDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__UIRenderer {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mRenderer")]
#[repr(C)]
pub struct gfc__UIRenderer {
    pub mGraphics: *mut gfc__Graphics,
    #[cfg(pdb_issue = "error in gfc__DynamicRenderer")]
    pub mRenderer: gfc__DynamicRenderer,
    pub mTransformStack: gfc__Vector_gfc__AffineTransform_0_gfc__CAllocator_,
    pub mParamStack: gfc__Vector_gfc__UIRenderer__Params_0_gfc__CAllocator_,
    pub mClipStack: gfc__Vector_gfc__UIRenderer__Clip_0_gfc__CAllocator_,
    pub mCurrentShader: gfc__AutoRef_gfc__Shader_,
    pub mCurrentMaterial: gfc__AutoRef_gfc__Material_,
    pub mStartTime: u32,
    pub mSolidMaterial: gfc__AutoRef_gfc__Material_,
    pub mClipID: i32,
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
pub struct gfc__MatrixArray {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mCount")]
#[repr(C)]
pub struct gfc__MatrixArray {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mPad: [u32; 3],
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub mFirstMatrix: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Animation_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Frustum {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field planes")]
#[repr(C)]
pub struct gfc__Frustum {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub planes: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CameraBlurDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Material_ {
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
pub struct gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FogDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__MeshInstance {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__MeshInstance {
    pub __vfptr: *const gfc__MeshInstance____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mLocked: bool,
}

impl gfc__MeshInstance {
    pub fn as_gfc__IRefObject_ptr(&self) -> *const gfc__IRefObject {
        self as *const _ as _
    }

    pub fn as_gfc__IRefObject_mut_ptr(&mut self) -> *mut gfc__IRefObject {
        self as *mut _ as _
    }
}

#[repr(C)]
pub struct gfc__MeshInstance____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub lockMesh:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
    pub compute: unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
    pub unlockMesh:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SkyDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__DynMeshNode___ {
    pub m_vector: gfc__Vector_gfc__DynMeshNode___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__Material {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Material {
    pub __vfptr: *const gfc__Material____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__LightNode {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field Transform")]
#[repr(C)]
pub struct gfc__LightNode {
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub Transform: gfc__Matrix4,
    pub Position: gfc__TVector4_float_gfc__FloatMath_,
    pub Color: gfc__TVector4_float_gfc__FloatMath_,
    pub Attenuation: gfc__TVector4_float_gfc__FloatMath_,
    pub InvLightRange: gfc__TVector4_float_gfc__FloatMath_,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
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
    #[cfg(pdb_issue = "error in gfc__Frustum")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field Transform")]
#[repr(C)]
pub struct gfc__RenderNode {
    pub Flags: gfc__TFlags_unsigned_long_,
    pub Camera: *mut gfc__Camera3D,
    pub Shader: *mut gfc__Shader,
    pub Material: *mut gfc__Material,
    pub RenderCallback: *mut gfc__IRenderCallback,
    pub MeshInstance: *mut gfc__MeshInstance,
    pub Context: i32,
    pub SceneContext: *mut (),
    pub ObjectContext: *mut (),
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub Transform: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub World: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub WorldView: gfc__Matrix4,
    #[cfg(pdb_issue = "error in gfc__Matrix4")]
    pub WorldViewProj: gfc__Matrix4,
    pub MatrixArray: *mut gfc__MatrixArray,
    pub BVolume: gfc__BoundingVolume,
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
    pub DynMeshNode: *mut gfc__DynMeshNode,
    pub ObjectColor: gfc__TVector4_float_gfc__FloatMath_,
    pub FadeAmount: gfc__TVector4_float_gfc__FloatMath_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__VertexDeclaration {
    pub __vfptr: *const gfc__VertexDeclaration____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mNumVerticesUsed")]
#[repr(C)]
pub struct gfc__DynMeshBuffer {
    pub mVertices: *mut gfc__MeshFormat__DynamicVertex__Stream0,
    pub mVertexCount: u32,
    pub mSlot: u32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mNumVerticesUsed: compile_error!("unimplemented feature: type kind 0x1001"),
}

#[repr(C)]
pub struct gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
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
pub struct gfc__Vector_gfc__AutoRef_gfc__WorldRegionData__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__WorldRegionData_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    pub comp: std__less_gfc__HString_,
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
pub struct gfc__HDRDesc {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__HDRDesc {
    pub __vfptr: *const gfc__HDRDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__EnvironmentDesc {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__EnvironmentDesc {
    pub __vfptr: *const gfc__EnvironmentDesc____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__MeshBuilder_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc___UIControl_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Darksiders {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mRenderThreadRunning")]
#[repr(C)]
pub struct gfc__Darksiders {
    pub __vfptr: *const gfc__Darksiders____vftable,
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
    #[cfg(pdb_issue = "error in gfc__ThreadSafeBool")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mApplicationSuspended: compile_error!("unimplemented feature: type kind 0x1001"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mGraphicsSuspended: compile_error!("unimplemented feature: type kind 0x1001"),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mLoadState: compile_error!("unimplemented feature: type kind 0x1507"),
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
    pub getCameraMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> gfc__Matrix4,
    pub getPlayerPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: bool,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__WorldManager {
    pub __vfptr: *const gfc__WorldManager____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
    pub mWorld: gfc__AutoRef_gfc__World_,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mWorldState: compile_error!("unimplemented feature: type kind 0x1507"),
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
    pub __: *const (),
    pub ___2: *const (),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Helper {
    pub __vfptr: *const gfc__Helper____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Helper__QueuedListenerInfo {
    pub obj: gfc__AutoRef_gfc__Object_,
    pub shouldAdd: bool,
}

#[repr(C)]
pub struct gfc__OblivionGame {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mRenderThreadRunning")]
#[repr(C)]
pub struct gfc__OblivionGame {
    pub __vfptr: *const gfc__OblivionGame____vftable,
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
    #[cfg(pdb_issue = "error in gfc__ThreadSafeBool")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mApplicationSuspended: compile_error!("unimplemented feature: type kind 0x1001"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub mGraphicsSuspended: compile_error!("unimplemented feature: type kind 0x1001"),
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
    pub getCameraMatrix: unsafe extern "thiscall" fn(this: *mut gfc__OblivionGame) -> gfc__Matrix4,
    pub getPlayerPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__OblivionGame,
        _: bool,
    ) -> gfc__TVector3_float_gfc__FloatMath_,
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc___UIControl {
    pub __vfptr: *const gfc___UIControl____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field setAnchorOffset")]
#[repr(C)]
pub struct gfc___UIControl____vftable {
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub setAnchorOffset: unsafe extern "thiscall" fn(
        this: *mut gfc___UIControl,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_lastUsedControllerInfo")]
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
    #[cfg(pdb_issue = "error in keen__ControllerInfo")]
    pub m_lastUsedControllerInfo: keen__ControllerInfo,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub m_lastInteractionStartedBy: compile_error!("unimplemented feature: type kind 0x1507"),
}

#[repr(C)]
pub struct unit4__LocalGameSession__LocalUserData {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field state")]
#[repr(C)]
pub struct unit4__LocalGameSession__LocalUserData {
    pub skippedSignIn: bool,
    pub hasExpectedUserId: bool,
    pub expectedUser: keen__UserAccount,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub state: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub stateBeforeProfileChange: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub playerType: compile_error!("unimplemented feature: type kind 0x1507"),
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
pub struct std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    pub comp: std__less_gfc__String_,
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
    pub comp: std__less_gfc__String_,
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
pub struct std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___ {}

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
{}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    pub comp: std__less_gfc__String_,
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
{}

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
    pub comp: std__less_gfc__String_,
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
    pub comp: std__less_gfc__String_,
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
pub struct gfc__Vector_gfc__Occluder___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__Occluder,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Clipper {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field planes")]
#[repr(C)]
pub struct gfc__Clipper {
    pub pOccluders: *const gfc__Vector_gfc__Occluder___0_gfc__CAllocator_,
    pub numPlanes: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub planes: compile_error!("unimplemented feature: class layout 0x0"),
}

#[repr(C)]
pub struct gfc__VisScriptVariable {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__VisScriptVariable {
    pub __vfptr: *const gfc__VisScriptVariable____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getColor")]
#[repr(C)]
pub struct gfc__VisScriptVariable____vftable {
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
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *const i8,
    pub compile:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity),
    pub clearDeadLinks:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *mut gfc__ModuleSystem),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub getColor: unsafe extern "thiscall" fn(
        this: *const gfc__VisScriptVariable,
    ) -> compile_error!(
        "unimplemented feature: type kind 0x1001"
    ),
    pub getVariableType: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> i32,
    pub isArray: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> bool,
    pub getVariableValue: unsafe extern "thiscall" fn(
        this: *const gfc__VisScriptVariable,
    ) -> gfc__AutoRef_gfc__Value_,
    pub setVariableValue:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: gfc__AutoRef_gfc__Value_),
}

#[repr(C)]
pub struct gfc__StateMapValue {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mValueMode")]
#[repr(C)]
pub struct gfc__StateMapValue {
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mValueMode: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "error in gfc__String")]
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__ModuleSystem {
    pub __vfptr: *const gfc__ModuleSystem____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__VisScriptModule {
    pub __vfptr: *const gfc__VisScriptModule____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__VisScriptEntity {
    pub __vfptr: *const gfc__VisScriptEntity____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Vector_gfc__SceneOccluder___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__SceneOccluder,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ModuleInputLink,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__SceneObject {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mType")]
#[repr(C)]
pub struct gfc__SceneObject {
    pub __vfptr: *const gfc__SceneObject____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mType: compile_error!("unimplemented feature: type kind 0x1507"),
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
pub struct gfc__SceneManager {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__SceneManager {
    pub __vfptr: *const gfc__SceneManager____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Cinematic {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Cinematic {
    pub __vfptr: *const gfc__Cinematic____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getAnimation")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__StaticLightingVisualOpt {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__StaticLightingVisualOpt {
    pub __vfptr: *const gfc__StaticLightingVisualOpt____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__StaticLightingObjectOpt {
    pub __vfptr: *const gfc__StaticLightingObjectOpt____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__AutoRef_gfc__CShape_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AnimationController_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__IconGizmo {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mType")]
#[repr(C)]
pub struct gfc__IconGizmo {
    pub __vfptr: *const gfc__IconGizmo____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mType: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Object3D {
    pub __vfptr: *const gfc__Object3D____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__Body {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__Body {
    pub __vfptr: *const gfc__Body____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mType")]
#[repr(C)]
pub struct gfc__WorldObjectGizmo {
    pub __vfptr: *const gfc__WorldObjectGizmo____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mType: compile_error!("unimplemented feature: type kind 0x1507"),
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
pub struct gfc__AutoRef_gfc__Skeleton3D_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Gizmo {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mType")]
#[repr(C)]
pub struct gfc__Gizmo {
    pub __vfptr: *const gfc__Gizmo____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mType: compile_error!("unimplemented feature: type kind 0x1507"),
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
pub struct gfc__AutoRef_gfc__RagdollMapper_ {
    pub p: *mut gfc__IRefObject,
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
pub struct gfc__Vector_gfc__AutoRef_gfc__TraversalLink__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__TraversalLink_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__WorldLoadRequest {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field State")]
#[repr(C)]
pub struct gfc__WorldLoadRequest {
    pub Regions: gfc__Vector_gfc__RegionLoadRequest_0_gfc__CAllocator_,
    pub Marker: gfc__AutoRef_gfc__PackageMarker_,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub State: compile_error!("unimplemented feature: type kind 0x1001"),
    pub Required: bool,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Constraint_ {
    pub p: *mut gfc__IRefObject,
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
pub struct gfc__AutoRef_gfc__Actor_ {
    pub p: *mut gfc__IRefObject,
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

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__TraversalWaypoint {
    pub __vfptr: *const gfc__TraversalWaypoint____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field getAnimation")]
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldObject,
        _: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field mType")]
#[repr(C)]
pub struct gfc__TraversalWaypoint__TraversalWaypointGizmo {
    pub __vfptr: *const gfc__TraversalWaypoint__TraversalWaypointGizmo____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub mType: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__CollisionManager {
    pub __vfptr: *const gfc__CollisionManager____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub first: compile_error!("unimplemented feature: type kind 0x1001"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__ModSysContainerModule {
    pub __vfptr: *const gfc__ModSysContainerModule____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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

#[cfg(pdb_issue = "error in field m_eResult")]
#[repr(C)]
pub struct UserStatsReceived_t {
    pub m_nGameID: u64,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub m_eResult: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field r#type")]
#[repr(C)]
pub struct unit4__RankingReceiveData {
    pub cachedBoards: keen__Array_unit4__RankingBoardCacheEntry_,
    pub servingFromCache: bool,
    pub downloadInProgress: bool,
    pub tableId: u16,
    pub user: keen__UserAccount,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub r#type: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field rankingReceiveData")]
#[repr(C)]
pub struct unit4__SystemServicesBase {
    pub currentTimeInMs: u32,
    pub rankingSendData: unit4__RankingSendData,
    #[cfg(pdb_issue = "error in unit4__RankingReceiveData")]
    pub rankingReceiveData: unit4__RankingReceiveData,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub currentRankingError: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field rankingReceiveData")]
#[repr(C)]
pub struct unit4__SystemServices {
    pub currentTimeInMs: u32,
    pub rankingSendData: unit4__RankingSendData,
    #[cfg(pdb_issue = "error in unit4__RankingReceiveData")]
    pub rankingReceiveData: unit4__RankingReceiveData,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub currentRankingError: compile_error!("unimplemented feature: type kind 0x1507"),
    pub rankingSendInteraction: unit4__SystemServicesInteractionData,
    pub onlineInteraction: unit4__SystemServicesInteractionData,
    pub steamAchievements: keen__SteamAchievements,
    pub steamStats: keen__SteamStats,
    pub pPresenceStrings: [*const i8; 64],
    pub presenceStringCount: u32,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub sendRankingStep: compile_error!("unimplemented feature: type kind 0x1507"),
    pub sendRankingCall: u64,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub receiveRankingStep: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field r#type")]
#[repr(C)]
pub struct unit4__RankingBoardCacheEntry {
    pub tableId: u16,
    pub user: keen__UserAccount,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub r#type: compile_error!("unimplemented feature: type kind 0x1507"),
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
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field m_eResult")]
#[repr(C)]
pub struct UserStatsStored_t {
    pub m_nGameID: u64,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub m_eResult: compile_error!("unimplemented feature: type kind 0x1507"),
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
pub struct gfc__ActorTarget {
    pub Actor: gfc__AutoRef_gfc__Actor_,
    pub Body: gfc__AutoRef_gfc__Body_,
    pub WorldObject: gfc__AutoRef_gfc__WorldObject_,
    pub Position: gfc__TVector3_float_gfc__FloatMath_,
    pub Normal: gfc__TVector3_float_gfc__FloatMath_,
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
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub MoveDirectionQuad: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub MoveFBHemisphere: compile_error!("unimplemented feature: type kind 0x1507"),
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1507")]
    pub MoveLRHemisphere: compile_error!("unimplemented feature: type kind 0x1507"),
}

#[repr(C)]
pub struct gfc__WindowHelper {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__WindowHelper {
    pub __vfptr: *const gfc__WindowHelper____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
pub struct gfc__AutoRef_gfc__LiquidRegionDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LoadMapMenu {
    _opaque: [u8; 0],
}

#[cfg(pdb_issue = "error in field ReferenceCount")]
#[repr(C)]
pub struct gfc__LoadMapMenu {
    pub __vfptr: *const gfc__LoadMapMenu____vftable,
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1001")]
    pub ReferenceCount: compile_error!("unimplemented feature: type kind 0x1001"),
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
