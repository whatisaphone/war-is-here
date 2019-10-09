#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use super::{types2::*, types3::*};
use pdbindgen_runtime::AsPtr;

#[repr(C)]
pub struct unit4__StatUpdateData {
    pub statId: u32,
    pub statName: [i8; 64],
}

#[repr(C)]
pub struct keen__float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct keen__float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct keen__float2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct keen__MemoryBlock {
    pub pStart: *mut u8,
    pub size: u32,
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
pub struct keen__BasePoolAllocator {
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_capacity: u32,
    pub m_size: u32,
    pub m_elementSize: u32,
    pub m_firstFreeIndex: u32,
}

#[repr(C)]
pub struct keen__InternalListBase {
    pub m_pFirst: *mut keen__InternalListBase__Listable,
    pub m_pLast: *mut keen__InternalListBase__Listable,
    pub m_numObjects: u32,
    pub m_begin: keen__InternalListBase__IteratorBase,
    pub m_end: keen__InternalListBase__IteratorBase,
    pub m_local: keen__InternalListBase__IteratorBase,
}

#[repr(C)]
pub struct keen__InternalListBase__IteratorBase {
    pub m_pCurrent: *mut keen__InternalListBase__Listable,
}

#[repr(C)]
pub struct keen__InternalListBase__Listable {
    pub m_pNext: *mut keen__InternalListBase__Listable,
    pub m_pPrev: *mut keen__InternalListBase__Listable,
}

#[repr(C)]
pub struct keen__GraphicsMatrix44 {
    pub row0: [f32; 4],
    pub row1: [f32; 4],
    pub row2: [f32; 4],
    pub row3: [f32; 4],
}

#[repr(C)]
pub struct keen__Event {
    pub m_eventHandle: *mut (),
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__TlsfAllocator_ {
    pub __vfptr: *const keen__BaseMemoryAllocator_keen__TlsfAllocator_____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__TlsfAllocator_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__TlsfAllocator,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
}

impl AsPtr<keen__MemoryAllocator> for *const keen__BaseMemoryAllocator_keen__TlsfAllocator_ {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__TlsfAllocator_____vftable {
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
pub struct keen__IoResult_unsigned_int_ {
    pub result: u32,
    pub error: keen__IoError,
}

#[repr(C)]
pub struct keen__TlsfAllocator {
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_pPool: *mut (),
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
pub struct keen__TlsfMemoryAllocator {
    pub __vfptr: *const keen__TlsfMemoryAllocator____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__TlsfAllocator_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__TlsfAllocator,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
    // keen__TlsfMemoryAllocator
}

impl AsPtr<keen__BaseMemoryAllocator_keen__TlsfAllocator_> for *const keen__TlsfMemoryAllocator {
    fn as_ptr(self) -> *const keen__BaseMemoryAllocator_keen__TlsfAllocator_ {
        self as *const _
    }
}

impl AsPtr<keen__MemoryAllocator> for *const keen__TlsfMemoryAllocator {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__TlsfMemoryAllocator____vftable {
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
    // keen__UserAccountSystemBase
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub runningOperations: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
    // keen__UserAccountSystem
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub accounts: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 4256],
    pub accountCount: u32,
}

impl AsPtr<keen__UserAccountSystemBase> for *const keen__UserAccountSystem {
    fn as_ptr(self) -> *const keen__UserAccountSystemBase {
        self as *const _
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
pub struct keen__Array_keen__Task___ {
    pub m_pData: *mut *mut keen__Task,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__Queue_keen__Task___ {
    pub m_size: u32,
    pub m_bottom: u32,
    pub m_top: u32,
    pub m_data: keen__Array_keen__Task___,
}

#[repr(C)]
pub struct keen__TaskQueueContext {
    pub pQueue: *mut keen__TaskQueue,
    pub priority: u32,
}

#[repr(C)]
pub struct keen__Task {
    pub header: keen__TaskHeader,
}

#[repr(C)]
pub struct keen__TaskHeader {
    pub pTaskFunction: *mut unsafe extern "C" fn(_: *const keen__TaskContext),
}

#[repr(C)]
pub struct keen__Array_keen__WorkerThreadContext_ {
    pub m_pData: *mut keen__WorkerThreadContext,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__TaskContext {
    pub pTask: *const keen__Task,
    pub threadIndex: u32,
}

#[repr(C)]
pub struct keen__SizedArray_keen__TaskQueueContext_ {
    pub m_pData: *mut keen__TaskQueueContext,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__TaskQueue {
    pub pTaskSystem: *mut keen__TaskSystem,
    pub clientThreadId: u32,
    pub pCopyBuffer: *mut u8,
    pub copyBufferSlotIndex: u32,
    pub queue: keen__Queue_keen__Task___,
    pub queueMutex: keen__Mutex,
    pub pendingTaskCount: u32,
    pub inProgressTaskCount: u32,
    pub taskCompleteEvent: keen__Event,
    pub maxTaskSize: u32,
}

#[repr(C)]
pub struct keen__Semaphore {
    pub m_semaphoreHandle: *mut (),
    pub m_value: i32,
}

#[repr(C)]
pub struct keen__TaskSystem {
    pub clientThreadId: u32,
    pub workerThreadContexts: keen__Array_keen__WorkerThreadContext_,
    pub taskQueueContexts: keen__SizedArray_keen__TaskQueueContext_,
    pub queueArrayMutex: keen__Mutex,
    pub wakeUpSemaphore: keen__Semaphore,
}

#[repr(C)]
pub struct keen__WorkerThreadContext {
    pub thread: keen__Thread,
    pub threadIndex: u32,
    pub pTaskSystem: *mut keen__TaskSystem,
}

#[repr(C)]
pub struct ID3D11ShaderResourceView {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View
     * ID3D11ShaderResourceView */
}

impl AsPtr<ID3D11View> for *const ID3D11ShaderResourceView {
    fn as_ptr(self) -> *const ID3D11View {
        self as *const _
    }
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11ShaderResourceView {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11ShaderResourceView {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11View {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11View {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11View {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11InputLayout {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11InputLayout */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11InputLayout {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11InputLayout {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11DeviceChild {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // ID3D11DeviceChild
}

impl AsPtr<IUnknown> for *const ID3D11DeviceChild {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11DepthStencilState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11DepthStencilState */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11DepthStencilState {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11DepthStencilState {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11SamplerState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11SamplerState */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11SamplerState {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11SamplerState {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct _D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: *const i8,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: D3D_NAME,
    pub ComponentType: D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
}

#[repr(C)]
pub struct ID3D11RenderTargetView {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View
     * ID3D11RenderTargetView */
}

impl AsPtr<ID3D11View> for *const ID3D11RenderTargetView {
    fn as_ptr(self) -> *const ID3D11View {
        self as *const _
    }
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11RenderTargetView {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11RenderTargetView {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11Device {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // ID3D11Device
}

impl AsPtr<IUnknown> for *const ID3D11Device {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11DepthStencilView {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View
     * ID3D11DepthStencilView */
}

impl AsPtr<ID3D11View> for *const ID3D11DepthStencilView {
    fn as_ptr(self) -> *const ID3D11View {
        self as *const _
    }
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11DepthStencilView {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11DepthStencilView {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11VertexShader {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11VertexShader */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11VertexShader {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11VertexShader {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11BlendState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11BlendState */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11BlendState {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11BlendState {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11PixelShader {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11PixelShader */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11PixelShader {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11PixelShader {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__DepthStencilState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__DepthStencilState
    pub pState: *mut ID3D11DepthStencilState,
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__DepthStencilState {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__GraphicsStateObject {
    pub hash: u32,
    pub refCount: u32,
}

#[repr(C)]
pub struct keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_____ {
    pub m_pEntryAllocator: *mut keen__PoolAllocator_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_,
    pub m_ownsAllocator: bool,
    pub m_buckets: keen__Array_keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator_,
    pub m_entries: keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_,
    pub m_bucketCount: u32,
    pub m_bucketMask: u32,
}

#[repr(C)]
pub struct keen__Array__D3D11_SIGNATURE_PARAMETER_DESC_ {
    pub m_pData: *mut _D3D11_SIGNATURE_PARAMETER_DESC,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__RenderTargetBuffer {
    pub format: keen__PixelFormat,
    pub pDataBuffer: *mut keen__TextureData,
}

#[repr(C)]
pub struct keen__Array_keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator_ {
    pub m_pData: *mut keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry___Iterator,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__TextureDescription {
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub flags: u16,
    pub r#type: u8,
    pub format: u8,
    pub lutFormat: u8,
    pub multiSampleType: u8,
    pub addressModeU: u8,
    pub addressModeV: u8,
    pub addressModeW: u8,
    pub levelCount: u8,
    pub cpuAccessMode: u8,
    pub gpuAccessMode: u8,
}

#[repr(C)]
pub struct keen__SkinningD3D11 {
    pub m_pSkinningBuffer: *mut ID3D11Buffer,
    pub m_skinningBatch: keen__SkinningBatch,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_instances: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 24],
    pub m_currentFrameIndex: u32,
    pub m_currentBufferPosition: u32,
    pub m_size: u32,
    pub m_bufferSize: u32,
    pub m_pSkinningJointMatrices: *mut keen__SoftwareSkinningJointMatrixData,
    pub m_currentJointMatricesPosition: u32,
    pub m_pTaskQueue: *mut keen__TaskQueue,
    pub m_morphBuffer: keen__DataBuffer,
    pub m_morphBufferSize: u32,
    pub m_pMorphBufferData: *mut (),
}

#[repr(C)]
pub struct keen__GraphicsCommandBuffer {
    pub pContext: *mut ID3D11DeviceContext,
    pub pMappedConstantBuffer: *mut ID3D11Resource,
    pub pCurrentRenderTarget: *const keen__RenderTarget,
    pub pSkinningBuffer: *mut keen__SkinningD3D11,
    pub pImmediateVertexData: *mut ID3D11Buffer,
    pub immediateVertexBufferOffset: u32,
    pub immediateVertexBufferSize: u32,
    pub immediateVertexDataStride: u32,
    pub immediateVertexCount: u32,
    pub immediatePrimitiveType: keen__PrimitiveType,
    pub pDownsampleDepthContext: *mut keen__DownsampleDepthContext,
    pub pCurrentlyBoundVertexFormat: *const keen__VertexFormat,
    pub pRenderCommandBufferStorage: *mut (),
    pub quadlistImmediateCommand: bool,
    pub pCurrentImmediateBuffer: *mut u8,
    pub quadBuffer: [u8; 65536],
}

#[repr(C)]
pub struct keen__SkinningBatch {
    pub pSkinningBuffer: *mut keen__SkinningD3D11,
    pub pMappedBufferData: *mut u8,
}

#[repr(C)]
pub struct keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_
{
    // keen__InternalListBase
    pub m_pFirst: *mut keen__InternalListBase__Listable,
    pub m_pLast: *mut keen__InternalListBase__Listable,
    pub m_numObjects: u32,
    pub m_begin: keen__InternalListBase__IteratorBase,
    pub m_end: keen__InternalListBase__IteratorBase,
    pub m_local: keen__InternalListBase__IteratorBase,
    /* keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_ */
}

impl AsPtr<keen__InternalListBase> for *const keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_ {
    fn as_ptr(self) -> *const keen__InternalListBase {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__RasterizerState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__RasterizerState
    pub pState: *mut ID3D11RasterizerState,
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__RasterizerState {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_
{
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__FragmentShader {
    pub pPixelShader: *mut ID3D11PixelShader,
}

#[repr(C)]
pub struct keen__StaticConstantBuffer {
    pub pBuffer: *mut ID3D11Buffer,
    pub sizeInBytes: u32,
}

#[repr(C)]
pub struct keen__VertexShader {
    pub shaderCode: keen__Array_unsigned_char_,
    pub pVertexShader: *mut ID3D11VertexShader,
    pub inputSignature: keen__Array__D3D11_SIGNATURE_PARAMETER_DESC_,
    pub inputSignatureHash: u32,
}

#[repr(C)]
pub struct keen__GraphicsCommandWriter {
    pub m_pBuffer: *mut keen__GraphicsCommandBuffer,
    pub m_pGraphicsSystem: *mut keen__GraphicsSystem,
    pub m_pRenderTarget: *const keen__RenderTarget,
    pub m_pVertexInputBinding: *const keen__VertexInputBinding,
    pub m_pBlendState: *const keen__BlendState,
    pub m_pRasterizerState: *const keen__RasterizerState,
    pub m_pDepthStencilState: *const keen__DepthStencilState,
    pub m_fragmentShaderSamplerStates: [*const keen__SamplerState; 16],
    pub m_fragmentShaderTextures: [*const keen__TextureData; 16],
    pub m_pShaderPipeline: *const keen__ShaderPipeline,
    pub m_pVertexShader: *const keen__VertexShader,
    pub m_pFragmentShader: *const keen__FragmentShader,
    pub m_vertexShaderTextures: [*const keen__TextureData; 16],
    pub m_vertexShaderSamplerStates: [*const keen__SamplerState; 16],
    pub m_screenQuadVertexFormats: [*const keen__VertexFormat; 3],
    pub m_renderPassStack: [*const keen__RenderTarget; 4],
    pub m_currentStaticVertexConstantBuffers: [*const keen__StaticConstantBuffer; 4],
    pub m_currentStaticFragmentConstantBuffers: [*const keen__StaticConstantBuffer; 8],
    pub m_renderPassStackIndex: u32,
    pub m_stencilRefValue: u32,
}

#[repr(C)]
pub struct keen__Array_unsigned_char_ {
    pub m_pData: *mut u8,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__GraphicsStateObjectCache {
    pub m_stateObjects: keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_____,
}

#[repr(C)]
pub struct keen__SamplerState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__SamplerState
    pub pState: *mut ID3D11SamplerState,
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__SamplerState {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__BlendState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__BlendState
    pub pState: *mut ID3D11BlendState,
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__BlendState {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__ShaderPipeline {
    pub pVertexShader: *const keen__VertexShader,
    pub pFragmentShader: *const keen__FragmentShader,
}

#[repr(C)]
pub struct keen__VertexInputBinding {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__VertexInputBinding
    pub pVertexFormat: *const keen__VertexFormat,
    pub pLayout: *mut ID3D11InputLayout,
    pub geometryModeMask: u32,
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__VertexInputBinding {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__DataBuffer {
    pub m_pCurrentPosition: *mut u8,
    pub m_pEnd: *mut u8,
    pub m_pBufferStart: *mut u8,
}

#[repr(C)]
pub struct keen__DynamicConstantBuffer {
    pub pBuffer: *mut ID3D11Buffer,
    pub sizeInBytes: u32,
}

#[repr(C)]
pub struct keen__RenderTarget {
    pub renderTargetViews: [*mut ID3D11RenderTargetView; 8],
    pub pDepthBufferView: *mut ID3D11DepthStencilView,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub colorBuffers: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 64],
    pub depthBuffer: keen__RenderTargetBuffer,
    pub colorBufferCount: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct keen__SoftwareSkinningJointMatrixData {
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub jointMatrices: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 16384],
}

#[repr(C)]
pub struct keen__VertexFormat {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__VertexFormat
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub attributes: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 68],
    pub attributeOffsets: [u32; 17],
    pub attributeCount: u32,
    pub attributeIndices: [u32; 17],
    pub streamStride: [u32; 3],
}

impl AsPtr<keen__GraphicsStateObject> for *const keen__VertexFormat {
    fn as_ptr(self) -> *const keen__GraphicsStateObject {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11Buffer {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11Resource
     * ID3D11Buffer */
}

impl AsPtr<ID3D11Resource> for *const ID3D11Buffer {
    fn as_ptr(self) -> *const ID3D11Resource {
        self as *const _
    }
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11Buffer {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11Buffer {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11Resource {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11Resource */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11Resource {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11Resource {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
}

#[repr(C)]
pub struct ID3D11RasterizerState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11RasterizerState */
}

impl AsPtr<ID3D11DeviceChild> for *const ID3D11RasterizerState {
    fn as_ptr(self) -> *const ID3D11DeviceChild {
        self as *const _
    }
}

impl AsPtr<IUnknown> for *const ID3D11RasterizerState {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
    }
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
pub struct HWND__ {
    pub unused: i32,
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
pub struct keen__FileDeviceInterface {
    pub __vfptr: *const keen__FileDeviceInterface____vftable,
}

#[repr(C)]
pub struct keen__FileDeviceInterface____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__IoResult_unsigned___int64_ {
    pub result: u64,
    pub error: keen__IoError,
}

#[repr(C)]
pub struct keen__AliasPathFileDevice {
    pub __vfptr: *const keen__AliasPathFileDevice____vftable,
    // keen__FileDeviceInterface
    // keen__AliasPathFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__AliasPathFileDevice__StreamEntry_,
}

impl AsPtr<keen__FileDeviceInterface> for *const keen__AliasPathFileDevice {
    fn as_ptr(self) -> *const keen__FileDeviceInterface {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__AliasPathFileDevice____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__NativeFileDevice {
    pub __vfptr: *const keen__NativeFileDevice____vftable,
    // keen__FileDeviceInterface
    // keen__NativeFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__NativeFileDevice__FileStreamData_,
    pub m_readThreadContexts: keen__Array_keen__NativeFileDevice__ReadThreadContext_,
    pub m_mutex: keen__Mutex,
    pub m_useReadThread: bool,
}

impl AsPtr<keen__FileDeviceInterface> for *const keen__NativeFileDevice {
    fn as_ptr(self) -> *const keen__FileDeviceInterface {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__NativeFileDevice____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__NativeFileDevice__ReadThreadContext {
    pub thread: keen__Thread,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub buffers: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 68],
    pub bufferEmpty: keen__Event,
    pub bufferFilled: keen__Event,
    pub bufferReadIndex: u32,
    pub bufferWriteIndex: u32,
    pub hasBuffer: bool,
    pub fileHandle: *mut (),
    pub filePosition: u64,
}

#[repr(C)]
pub struct keen__PakFileDevice {
    pub __vfptr: *const keen__PakFileDevice____vftable,
    // keen__FileDeviceInterface
    // keen__PakFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__PakFileDevice__PakFileStream_,
    pub m_chunkAllocator: keen__TlsfMemoryAllocator,
}

impl AsPtr<keen__FileDeviceInterface> for *const keen__PakFileDevice {
    fn as_ptr(self) -> *const keen__FileDeviceInterface {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__PakFileDevice____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__FileDeviceMountData {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct keen__FileSystemDeviceEntry {
    // keen__ListEntry_keen__FileSystemDeviceEntry_
    pub pNext: *mut keen__FileSystemDeviceEntry,
    pub pPrev: *mut keen__FileSystemDeviceEntry,
    // keen__FileSystemDeviceEntry
    pub pMountData: *mut keen__FileDeviceMountData,
    pub pDevice: *mut keen__FileDeviceInterface,
    pub isInternalDevice: bool,
}

impl AsPtr<keen__ListEntry_keen__FileSystemDeviceEntry_> for *const keen__FileSystemDeviceEntry {
    fn as_ptr(self) -> *const keen__ListEntry_keen__FileSystemDeviceEntry_ {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__IoResult_unsigned_short_ {
    pub result: u16,
    pub error: keen__IoError,
}

#[repr(C)]
pub struct keen__MemoryFileDevice {
    pub __vfptr: *const keen__MemoryFileDevice____vftable,
    // keen__FileDeviceInterface
    // keen__MemoryFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__MemoryFileDevice__StreamEntry_,
    pub m_files: keen__Array_keen__MemoryFileDevice__FileEntry_,
    pub m_fileIndices: keen__IndexArray,
}

impl AsPtr<keen__FileDeviceInterface> for *const keen__MemoryFileDevice {
    fn as_ptr(self) -> *const keen__FileDeviceInterface {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__MemoryFileDevice____vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__FileDeviceInterface, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__FileDeviceInterface,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__FileDeviceInterface,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__MemoryFileDevice__FileEntry {
    pub pMountData: *mut keen__MemoryFileDeviceMountData,
    pub memoryBlock: keen__MemoryBlock,
    pub fileSize: u32,
    pub openCount: u32,
    pub fileNameCrc: u32,
    pub isWritten: bool,
    pub isWriteable: bool,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__NativeFileDevice__FileStreamData_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__LowOverheadAllocator {
    pub m_memory: keen__MemoryBlock,
    pub m_pFirstFree: *mut keen__LowOverheadAllocator__FreeBlock,
}

#[repr(C)]
pub struct keen__LowOverheadAllocator__FreeBlock {
    pub size: u32,
    pub pNext: *mut keen__LowOverheadAllocator__FreeBlock,
}

#[repr(C)]
pub struct keen__FileStreamAllocator_keen__MemoryFileDevice__StreamEntry_ {
    pub m_streamAllocator: keen__PoolAllocator_keen__MemoryFileDevice__StreamEntry_,
    pub m_streamMutex: keen__Mutex,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__PakFileDevice__PakFileStream_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__FileSystem {
    pub devices: keen__PoolAllocator_keen__FileSystemDeviceEntry_,
    pub mountPoints: keen__PoolAllocator_keen__FileSystemMountPoint_,
    pub nativeFileDevice: keen__NativeFileDevice,
    pub memoryFileDevice: keen__MemoryFileDevice,
    pub pakFileDevice: keen__PakFileDevice,
    pub aliasPathFileDevice: keen__AliasPathFileDevice,
    pub pRootPoint: *mut keen__FileSystemMountPoint,
    pub nativeMountHandle: u32,
    pub mutex: keen__Mutex,
    pub openFileStack: u32,
    pub pAllocator: *mut keen__MemoryAllocator,
    pub useIo: u32,
    pub ignoreFirstChild: bool,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__FileSystemMountPoint_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__Array_keen__MemoryFileDevice__FileEntry_ {
    pub m_pData: *mut keen__MemoryFileDevice__FileEntry,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__FileStreamAllocator_keen__AliasPathFileDevice__StreamEntry_ {
    pub m_streamAllocator: keen__PoolAllocator_keen__AliasPathFileDevice__StreamEntry_,
    pub m_streamMutex: keen__Mutex,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__AliasPathFileDevice__StreamEntry_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__TreeNode_keen__FileSystemMountPoint_ {
    pub pNextSibling: *mut keen__FileSystemMountPoint,
    pub pPrevSibling: *mut keen__FileSystemMountPoint,
    pub pParent: *mut keen__FileSystemMountPoint,
    pub pFirstChild: *mut keen__FileSystemMountPoint,
}

#[repr(C)]
pub struct keen__SizedArray_unsigned_int_ {
    pub m_pData: *mut u32,
    pub m_size: u32,
    pub m_capacity: u32,
}

#[repr(C)]
pub struct keen__FileStatus {
    pub lastModificationTime: i32,
    pub size: u64,
    pub nativeFileName: [i8; 260],
    pub isWriteable: bool,
    pub isDirectory: bool,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__FileSystemDeviceEntry_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__FileStreamAllocator_keen__NativeFileDevice__FileStreamData_ {
    pub m_streamAllocator: keen__PoolAllocator_keen__NativeFileDevice__FileStreamData_,
    pub m_streamMutex: keen__Mutex,
}

#[repr(C)]
pub struct keen__MemoryFileDeviceMountData {
    // keen__FileDeviceMountData
    __pdbindgen_padding: [u8; 1],
    // keen__MemoryFileDeviceMountData
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pAllocator: *mut keen__MemoryAllocator,
    pub fileHandle: u32,
}

impl AsPtr<keen__FileDeviceMountData> for *const keen__MemoryFileDeviceMountData {
    fn as_ptr(self) -> *const keen__FileDeviceMountData {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__Array_keen__NativeFileDevice__ReadThreadContext_ {
    pub m_pData: *mut keen__NativeFileDevice__ReadThreadContext,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__PoolAllocator_keen__MemoryFileDevice__StreamEntry_ {
    pub m_pool: keen__BasePoolAllocator,
}

#[repr(C)]
pub struct keen__FileSystemMountPoint {
    // keen__TreeNode_keen__FileSystemMountPoint_
    pub pNextSibling: *mut keen__FileSystemMountPoint,
    pub pPrevSibling: *mut keen__FileSystemMountPoint,
    pub pParent: *mut keen__FileSystemMountPoint,
    pub pFirstChild: *mut keen__FileSystemMountPoint,
    // keen__FileSystemMountPoint
    pub path: [i8; 128],
    pub pFirstDevice: *mut keen__FileSystemDeviceEntry,
}

impl AsPtr<keen__TreeNode_keen__FileSystemMountPoint_> for *const keen__FileSystemMountPoint {
    fn as_ptr(self) -> *const keen__TreeNode_keen__FileSystemMountPoint_ {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__IndexArray {
    pub m_indices: keen__SizedArray_unsigned_int_,
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__LowOverheadAllocator_ {
    pub __vfptr: *const keen__BaseMemoryAllocator_keen__LowOverheadAllocator_____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__LowOverheadAllocator_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__LowOverheadAllocator,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
}

impl AsPtr<keen__MemoryAllocator> for *const keen__BaseMemoryAllocator_keen__LowOverheadAllocator_ {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__LowOverheadAllocator_____vftable {
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
pub struct keen__ListEntry_keen__FileSystemDeviceEntry_ {
    pub pNext: *mut keen__FileSystemDeviceEntry,
    pub pPrev: *mut keen__FileSystemDeviceEntry,
}

#[repr(C)]
pub struct keen__FileStreamAllocator_keen__PakFileDevice__PakFileStream_ {
    pub m_streamAllocator: keen__PoolAllocator_keen__PakFileDevice__PakFileStream_,
    pub m_streamMutex: keen__Mutex,
}

#[repr(C)]
pub struct IDirectInput8A {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // IDirectInput8A
}

impl AsPtr<IUnknown> for *const IDirectInput8A {
    fn as_ptr(self) -> *const IUnknown {
        self as *const _
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
    // std___Allocator_base_char_
    __pdbindgen_padding: [u8; 1],
    // std__allocator_char_
}

impl AsPtr<std___Allocator_base_char_> for *const std__allocator_char_ {
    fn as_ptr(self) -> *const std___Allocator_base_char_ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Container_base0 {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___String_val_char_std__allocator_char___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___String_val_char_std__allocator_char___
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding_2: [u8; 15],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
}

impl AsPtr<std___Container_base0> for *const std___String_val_char_std__allocator_char___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Allocator_base_char_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___String_val_char_std__allocator_char___
    #[cfg(pdb_issue = "unimplemented feature: type kind 0x1506")]
    pub _Bx: compile_error!("unimplemented feature: type kind 0x1506"),
    __pdbindgen_padding_2: [u8; 15],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
    // std__basic_string_char_std__char_traits_char__std__allocator_char___
}

impl AsPtr<std___String_val_char_std__allocator_char___>
    for *const std__basic_string_char_std__char_traits_char__std__allocator_char___
{
    fn as_ptr(self) -> *const std___String_val_char_std__allocator_char___ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0>
    for *const std__basic_string_char_std__char_traits_char__std__allocator_char___
{
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
pub struct gfc__InputStream {
    pub __vfptr: *const gfc__InputStream____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__InputStream
    pub mEndianess: i32,
    pub mBufferAvail: u32,
    pub mBufferPtr: *mut u8,
}

impl AsPtr<gfc__Stream> for *const gfc__InputStream {
    fn as_ptr(self) -> *const gfc__Stream {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__InputStream {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__InputStream____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
    pub available: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> i64,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub markSupported: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> bool,
    pub mark: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub seekSupported: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> i64,
    pub skipBytes: unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: i32),
    pub clone: unsafe extern "thiscall" fn(
        this: *mut gfc__InputStream,
        result: *mut gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__InputStream_,
    pub getBuffer: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> *const u8,
    pub read_raw:
        unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: *mut (), _: i32) -> i32,
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__SoundDesc
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

impl AsPtr<gfc__Object> for *const gfc__SoundDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__SoundDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Resource {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
}

impl AsPtr<gfc__IRefObject> for *const gfc__Object {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__OutputStream
    pub mEndianess: i32,
}

impl AsPtr<gfc__Stream> for *const gfc__OutputStream {
    fn as_ptr(self) -> *const gfc__Stream {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__OutputStream {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Property
    pub mName: gfc__HString,
    pub mAnnotation: gfc__HString,
    pub mContextFlags: u8,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Property {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Stream
}

impl AsPtr<gfc__IRefObject> for *const gfc__Stream {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Class
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

impl AsPtr<gfc__IRefObject> for *const gfc__Class {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Environment
    pub mParent: gfc__AutoRef_gfc__Environment_,
    pub mSymbols: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_,
    pub mContextFlags: u8,
    pub mHasRun: bool,
    pub mTempEnv: bool,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Environment {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Method
    pub mName: gfc__HString,
    pub mContextFlags: u8,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Method {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
    /* std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ */
}

impl AsPtr<std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____> for *const std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {
    fn as_ptr(self) -> *const std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

impl AsPtr<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
    __pdbindgen_padding: [u8; 1],
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
}

impl AsPtr<std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____>
    for *const std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
{
    fn as_ptr(
        self,
    ) -> *const std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
    {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
{
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl AsPtr<std___Container_base0> for *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
    __pdbindgen_padding: [u8; 1],
    /* std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ */
}

impl AsPtr<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_> for *const std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {
    fn as_ptr(self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {
        self as *const _
    }
}

#[repr(C)]
pub struct std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    // std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
    pub first: gfc__AutoRef_gfc__Object_,
    pub second: gfc__AutoRef_gfc__Object_,
    // std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
}

impl AsPtr<std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___>
    for *const std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
{
    fn as_ptr(
        self,
    ) -> *const std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
}

impl AsPtr<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___>
    for *const std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
{
    fn as_ptr(self) -> *const std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
        self as *const _
    }
}

#[repr(C)]
pub struct std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

impl AsPtr<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl AsPtr<std___Container_base0> for *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std__allocator_gfc__AutoRef_gfc__ImageSurface___ {
    // std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___
    __pdbindgen_padding: [u8; 1],
    // std__allocator_gfc__AutoRef_gfc__ImageSurface___
}

impl AsPtr<std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___>
    for *const std__allocator_gfc__AutoRef_gfc__ImageSurface___
{
    fn as_ptr(self) -> *const std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
    __pdbindgen_padding: [u8; 1],
    /* std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ */
}

impl AsPtr<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_> for *const std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
    fn as_ptr(self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {
        self as *const _
    }
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {
    // std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
    __pdbindgen_padding: [u8; 1],
    // std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
}

impl AsPtr<std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____>
    for *const std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
{
    fn as_ptr(self) -> *const std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____{
        self as *const _
    }
}

#[repr(C)]
pub struct std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
{
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
}

impl AsPtr<std___Container_base0> for *const std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

impl AsPtr<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
    __pdbindgen_padding: [u8; 1],
    /* std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ */
}

impl AsPtr<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_> for *const std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {
    fn as_ptr(self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
    // std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
    __pdbindgen_padding: [u8; 1],
    // std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
}

impl AsPtr<std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____> for *const std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
{
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl AsPtr<std___Container_base0> for *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__LightData {
    pub pos: keen__float3,
    pub flag: f32,
    pub color: keen__float3,
    pub invRange: f32,
    pub viewProj: keen__GraphicsMatrix44,
    pub dir: keen__float3,
    pub castShadows: i32,
    pub atten: keen__float4,
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
    // gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__HDRDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(self) -> *const gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_ {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_ {
    // gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__DepthOfFieldDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(self) -> *const gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_ {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__StaticMesh {
    pub __vfptr: *const gfc__StaticMesh____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    /* gfc__Mesh
     * gfc__StaticMesh */
}

impl AsPtr<gfc__Mesh> for *const gfc__StaticMesh {
    fn as_ptr(self) -> *const gfc__Mesh {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__StaticMesh {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__StaticMesh____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> gfc__Mesh__Type,
    pub isCompressed: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> bool,
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: i32) -> *mut gfc__Mesh__Group,
    pub getGroupCount: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> i32,
    pub getGroupIDAt: unsafe extern "thiscall" fn(this: *const gfc__Mesh, _: i32) -> i32,
    pub getGroupNameAt:
        unsafe extern "thiscall" fn(this: *const gfc__Mesh, _: i32) -> *const gfc__String,
    pub beginCreate: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut gfc__MeshBuilder),
    pub create: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut gfc__MeshBuilder),
    pub endCreate: unsafe extern "thiscall" fn(this: *mut gfc__Mesh),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
    ) -> *const gfc__TBox_float_gfc__FloatMath_,
    pub getVertexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub getIndexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub updateAddress: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut (), _: *mut ()),
    pub isLightmapped: unsafe extern "thiscall" fn(this: *const gfc__StaticMesh) -> bool,
    pub testCollision: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMesh,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
    ) -> bool,
    pub testCollision_2: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMesh,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ) -> bool,
    pub render: unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *const gfc__RenderNode),
    pub renderGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *const gfc__RenderNode, _: i32),
    pub renderDepthOnly:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *const gfc__RenderNode),
    pub renderGroupDepthOnly:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *const gfc__RenderNode, _: i32),
    pub renderLitGroup: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMesh,
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
        this: *mut gfc__StaticMesh,
        _: *const gfc__RenderNode,
        _: i32,
        _: u32,
        _: *const gfc__AutoRef_gfc__VertexBuffer_,
    ),
    pub createRenderCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMesh,
        _: i32,
    ) -> *mut gfc__IRenderCallback,
    pub createRenderCallback_2:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh) -> *mut gfc__IRenderCallback,
    pub createLitRenderCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMesh,
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
        this: *mut gfc__StaticMesh,
        _: i32,
        _: u32,
        _: *const gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__IRenderCallback,
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__VertexBuffer
}

impl AsPtr<gfc__IRefObject> for *const gfc__VertexBuffer {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__IndexBuffer
}

impl AsPtr<gfc__IRefObject> for *const gfc__IndexBuffer {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__FogDesc
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

impl AsPtr<gfc__EnvironmentDesc> for *const gfc__FogDesc {
    fn as_ptr(self) -> *const gfc__EnvironmentDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__FogDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__FogDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__AmbientDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(self) -> *const gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_ {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Camera3D
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

impl AsPtr<gfc__IRefObject> for *const gfc__Camera3D {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__DepthOfFieldDesc
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

impl AsPtr<gfc__EnvironmentDesc> for *const gfc__DepthOfFieldDesc {
    fn as_ptr(self) -> *const gfc__EnvironmentDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__DepthOfFieldDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__DepthOfFieldDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__CameraBlurDesc
    pub mScale: f32,
    pub mSampleCount: u32,
    pub mManager: *mut gfc__EnvironmentManager,
}

impl AsPtr<gfc__EnvironmentDesc> for *const gfc__CameraBlurDesc {
    fn as_ptr(self) -> *const gfc__EnvironmentDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__CameraBlurDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__CameraBlurDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Shader
    pub mName: gfc__HString,
    pub mLocked: bool,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Shader {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
}

impl AsPtr<gfc__Object> for *const gfc__WorldObject {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__WorldObject {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct gfc__PixelShaderInstanceData {
    pub objectColor: keen__float4,
    pub preMultiplyAlpha: i32,
    pub colorMapMult: f32,
    pub fadeAmount: f32,
    pub alphaRef: f32,
    pub lightEnable0: i32,
    pub lightEnable1: i32,
    pub lightEnable2: i32,
    pub _pad1: i32,
}

#[repr(C)]
pub struct gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______
    // gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____
}

impl AsPtr<std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______> for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    fn as_ptr(self) -> *const std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
        self as *const _
    }
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0>
    for *const gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____
{
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Image
    pub mType: gfc__ImageType,
    pub mFormat: gfc__ImageFormat,
    pub mSurfaces: std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Image {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__World
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

impl AsPtr<gfc__Object> for *const gfc__World {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__World {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__AmbientDesc
    pub mBlendDuration: f32,
    pub mLowerAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mUpperAmbientColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mRimColor: gfc__TVector4_float_gfc__FloatMath_,
    pub mManager: *mut gfc__EnvironmentManager,
    __pdbindgen_padding: [u8; 12],
}

impl AsPtr<gfc__EnvironmentDesc> for *const gfc__AmbientDesc {
    fn as_ptr(self) -> *const gfc__EnvironmentDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__AmbientDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__AmbientDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Texture
    pub mName: gfc__HString,
}

impl AsPtr<gfc__IRefObject> for *const gfc__Texture {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct gfc__PixelShaderLightData {
    pub light0: gfc__LightData,
    pub light1: gfc__LightData,
    pub light2: gfc__LightData,
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__OutputStream
    pub mEndianess: i32,
    // gfc__ByteOutputStream
    pub mPosition: i32,
    pub mOutput: gfc__Vector_unsigned_char_0_gfc__CAllocator_,
}

impl AsPtr<gfc__OutputStream> for *const gfc__ByteOutputStream {
    fn as_ptr(self) -> *const gfc__OutputStream {
        self as *const _
    }
}

impl AsPtr<gfc__Stream> for *const gfc__ByteOutputStream {
    fn as_ptr(self) -> *const gfc__Stream {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__ByteOutputStream {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__FullScreenEffectDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(
        self,
    ) -> *const gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_ {
        self as *const _
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
pub struct gfc__Mesh {
    pub __vfptr: *const gfc__Mesh____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Mesh
}

impl AsPtr<gfc__IRefObject> for *const gfc__Mesh {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__Mesh____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> gfc__Mesh__Type,
    pub isCompressed: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> bool,
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: i32) -> *mut gfc__Mesh__Group,
    pub getGroupCount: unsafe extern "thiscall" fn(this: *const gfc__Mesh) -> i32,
    pub getGroupIDAt: unsafe extern "thiscall" fn(this: *const gfc__Mesh, _: i32) -> i32,
    pub getGroupNameAt:
        unsafe extern "thiscall" fn(this: *const gfc__Mesh, _: i32) -> *const gfc__String,
    pub beginCreate: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut gfc__MeshBuilder),
    pub create: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut gfc__MeshBuilder),
    pub endCreate: unsafe extern "thiscall" fn(this: *mut gfc__Mesh),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
    ) -> *const gfc__TBox_float_gfc__FloatMath_,
    pub getVertexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub getIndexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__Mesh,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub updateAddress: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: *mut (), _: *mut ()),
}

#[repr(C)]
pub struct gfc__Mesh__Group {
    pub Name: gfc__String,
    pub ID: i32,
    pub PrimType: i32,
    pub mStartLitVert: i32,
    pub VertexStart: i32,
    pub VertexCount: i32,
    pub IndexStart: i32,
    pub IndexCount: i32,
}

#[repr(C)]
pub struct gfc__WorldRegion {
    pub __vfptr: *const gfc__WorldRegion____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldRegion
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

impl AsPtr<gfc__Object> for *const gfc__WorldRegion {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__WorldRegion {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct gfc__PixelShaderGlobalData {
    pub cameraPos: keen__float3,
    pub time: f32,
    pub depthBias: keen__float4,
    pub rimColor: keen__float4,
    pub ambientColor: keen__float4,
    pub ambientColor2: keen__float4,
    pub fogParams: keen__float4,
    pub fogColor: keen__float4,
    pub invProj: keen__GraphicsMatrix44,
    pub depthRange: keen__float4,
    pub viewport: keen__float4,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub spotOffsets: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 256],
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__ObjectReader
}

impl AsPtr<gfc__IRefObject> for *const gfc__ObjectReader {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Material
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

impl AsPtr<gfc__Object> for *const gfc__Material {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Material {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct gfc__KGGraphics {
    pub __vfptr: *const gfc__KGGraphics____vftable,
    // gfc__Graphics
    // gfc__KGGraphics
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_destructors: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding: [u8; 3072],
    pub mDefaultAllocator: *mut keen__MemoryAllocator,
    pub m_pGraphicsSystem: *mut keen__GraphicsSystem,
    pub m_pCommandWriter: *mut keen__GraphicsCommandWriter,
    pub m_pShaderFileSystem: *mut keen__FileSystem,
    pub m_pDepthStates: [*const keen__DepthStencilState; 3],
    pub m_pZStencilDisabled: *const keen__DepthStencilState,
    pub m_pNoCullWithBias: [*const keen__RasterizerState; 10],
    pub m_pBackCull: *const keen__RasterizerState,
    pub m_pBackCullWithBias: [*const keen__RasterizerState; 10],
    pub m_pBackCullReverse: *const keen__RasterizerState,
    pub m_pNoCullReverse: *const keen__RasterizerState,
    pub m_pParamSamplerStates: [*const keen__SamplerState; 16],
    pub m_pLightmapSamplerState: *const keen__SamplerState,
    pub mDeviceThreadID: u32,
    pub mPrevThreadID: u32,
    pub mNumSamplers: u16,
    pub mWaitForVSync: bool,
    pub mFlipMode: u32,
    pub m_depthBias: i32,
    pub m_reverseCulling: bool,
    pub m_hasShaderGlobalConstants: bool,
    pub m_hasInstanceConstants: bool,
    pub m_hasLightConstants: bool,
    pub m_parameterBufferSlot: i32,
    pub mWorldMatrix: gfc__Matrix4,
    pub mViewMatrix: gfc__Matrix4,
    pub mInvViewMatrix: gfc__Matrix4,
    pub mProjMatrix: gfc__Matrix4,
    pub mInvProjMatrix: gfc__Matrix4,
    pub mViewProjMatrix: gfc__Matrix4,
    pub m_viewPort: gfc__Viewport,
    pub m_aspectRatio: f32,
    pub mUseFPRenderTarget: bool,
    pub mUseSceneRenderTarget: bool,
    pub mClearZ: f32,
    pub mClearStencil: u32,
    pub mBlendMode: i32,
    pub mBlendFactor: u32,
    pub mDepthMode: i32,
    pub mGamma: f32,
    pub mBrightness: f32,
    pub mShader: gfc__AutoRef_gfc__Shader_,
    pub mHostSkinMemory: *mut (),
    pub mGcmCommandBuffer: *mut (),
    pub mGcmCommandBufferSize: u32,
    pub mTiledMemorySize: u32,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_vertexBuffers: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_2: [u8; 48],
    pub m_vertexBufferCount: u32,
    pub m_pVertexConstantBuffers: [*mut keen__DynamicConstantBuffer; 4],
    pub m_pPixelShaderConstantBuffer: [*mut keen__DynamicConstantBuffer; 3],
    pub m_pVertexFormats: [*const keen__VertexFormat; 20],
    pub m_pBlendStates: [*const keen__BlendState; 6],
    pub m_pColorWriteDisabledBlendState: *const keen__BlendState,
    pub m_pStencilIncrementDepthStencilState: *const keen__DepthStencilState,
    pub m_pStencilDecrementDepthStencilState: *const keen__DepthStencilState,
    pub m_pDrawMaskedDepthStencilState: *const keen__DepthStencilState,
    pub m_graphicsAllocator: keen__LowOverheadMemoryAllocator,
    pub m_framebufferAllocator: keen__ZoneMemoryAllocator,
    pub m_videoTextureAllocator: keen__TlsfMemoryAllocator,
    __pdbindgen_padding_3: [u8; 4],
    pub m_pixelShaderGlobalConstantBufferCache: gfc__PixelShaderGlobalData,
    pub m_pixelShaderGlobalConstantDataDirty: bool,
    __pdbindgen_padding_4: [u8; 15],
    pub m_pixelShaderInstanceConstantBufferCache: gfc__PixelShaderInstanceData,
    pub m_pixelShaderInstanceConstantDataDirty: bool,
    __pdbindgen_padding_5: [u8; 15],
    pub m_pixelShaderLightConstantBufferCache: gfc__PixelShaderLightData,
    pub m_pixelShaderLightConstantDataDirty: bool,
    pub m_pCurrentFrameBuffer: *const keen__TextureData,
    pub m_pCurrentDepthBuffer: *const keen__TextureData,
    pub m_envProbeDepthBuffer: keen__TextureData,
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_environmentProbeTextures: compile_error!("unimplemented feature: class layout 0x0"),
    __pdbindgen_padding_6: [u8; 8],
    pub m_pEnvironmentProbeRenderTargets: [*const keen__RenderTarget; 2],
    pub m_postProcessEffect: u32,
    pub m_antiAliasingType: u32,
    pub m_textureFilteringMode: u32,
    pub currentTextureFilteringMode: u8,
    pub m_pMaterialConstantBuffers: [*mut keen__DynamicConstantBuffer; 20],
    __pdbindgen_padding_7: [u8; 4],
}

impl AsPtr<gfc__Graphics> for *const gfc__KGGraphics {
    fn as_ptr(self) -> *const gfc__Graphics {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__KGGraphics____vftable {
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
    pub createRenderTargetEx: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub getSlopeScaledDepthBias: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> f32,
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
    // gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__CameraBlurDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(self) -> *const gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_ {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__VertexDeclaration
}

impl AsPtr<gfc__IRefObject> for *const gfc__VertexDeclaration {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__VertexDeclaration____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_ {
    // gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_
    pub mData: *mut gfc__AutoRef_gfc__FogDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
    // gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_
    pub mFixedData: [u8; 40],
}

impl AsPtr<gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_>
    for *const gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_
{
    fn as_ptr(self) -> *const gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
        self as *const _
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
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______
    // gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____
}

impl AsPtr<std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
        self as *const _
    }
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
pub struct gfc__ByteInputStream {
    pub __vfptr: *const gfc__ByteInputStream____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__InputStream
    pub mEndianess: i32,
    pub mBufferAvail: u32,
    pub mBufferPtr: *mut u8,
    // gfc__ByteInputStream
    pub mBytes: *const u8,
    pub mNumBytes: u32,
    pub mMarkIndex: i32,
    pub mOwnsBuffer: bool,
}

impl AsPtr<gfc__InputStream> for *const gfc__ByteInputStream {
    fn as_ptr(self) -> *const gfc__InputStream {
        self as *const _
    }
}

impl AsPtr<gfc__Stream> for *const gfc__ByteInputStream {
    fn as_ptr(self) -> *const gfc__Stream {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__ByteInputStream {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__ByteInputStream____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
    pub available: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> i64,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub markSupported: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> bool,
    pub mark: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__InputStream),
    pub seekSupported: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> i64,
    pub skipBytes: unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: i32),
    pub clone: unsafe extern "thiscall" fn(
        this: *mut gfc__InputStream,
        result: *mut gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__InputStream_,
    pub getBuffer: unsafe extern "thiscall" fn(this: *mut gfc__InputStream) -> *const u8,
    pub read_raw:
        unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: *mut (), _: i32) -> i32,
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___
}

impl AsPtr<std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______> for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    fn as_ptr(self) -> *const std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
        self as *const _
    }
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0>
    for *const gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___
{
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ParticleManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Skeleton3D {
    pub __vfptr: *const gfc__Skeleton3D____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub __vfptr_2: *const gfc__Hierarchical_gfc__Node3D_____vftable,
    // gfc__Hierarchical_gfc__Node3D_
    pub mParent: *mut gfc__Node3D,
    pub mHead: gfc__AutoRef_gfc__Node3D_,
    pub mTail: gfc__AutoRef_gfc__Node3D_,
    pub mNext: gfc__AutoRef_gfc__Node3D_,
    pub mPrev: gfc__AutoRef_gfc__Node3D_,
    // gfc__Node3D
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
    // gfc__Skeleton3D
    pub mNodeHashTable: [*mut gfc__Node3D; 13],
    pub mRefNode: gfc__HString,
}

impl AsPtr<gfc__Node3D> for *const gfc__Skeleton3D {
    fn as_ptr(self) -> *const gfc__Node3D {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__Skeleton3D {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Skeleton3D {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

impl AsPtr<gfc__Hierarchical_gfc__Node3D_> for *const gfc__Skeleton3D {
    fn as_ptr(self) -> *const gfc__Hierarchical_gfc__Node3D_ {
        (self as usize + 0x8) as *const _
    }
}

#[repr(C)]
pub struct gfc__Skeleton3D____vftable {
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub __vfptr_2: *const gfc__Hierarchical_gfc__Node3D_____vftable,
    // gfc__Hierarchical_gfc__Node3D_
    pub mParent: *mut gfc__Node3D,
    pub mHead: gfc__AutoRef_gfc__Node3D_,
    pub mTail: gfc__AutoRef_gfc__Node3D_,
    pub mNext: gfc__AutoRef_gfc__Node3D_,
    pub mPrev: gfc__AutoRef_gfc__Node3D_,
    // gfc__Node3D
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

impl AsPtr<gfc__Object> for *const gfc__Node3D {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__Node3D {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

impl AsPtr<gfc__Hierarchical_gfc__Node3D_> for *const gfc__Node3D {
    fn as_ptr(self) -> *const gfc__Hierarchical_gfc__Node3D_ {
        (self as usize + 0x8) as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__HDRDesc
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

impl AsPtr<gfc__EnvironmentDesc> for *const gfc__HDRDesc {
    fn as_ptr(self) -> *const gfc__EnvironmentDesc {
        self as *const _
    }
}

impl AsPtr<gfc__Object> for *const gfc__HDRDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__HDRDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
}

impl AsPtr<gfc__Object> for *const gfc__EnvironmentDesc {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__EnvironmentDesc {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct keen__ZoneMemoryAllocator {
    pub __vfptr: *const keen__ZoneMemoryAllocator____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__ZoneAllocatorAdapter,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
    // keen__ZoneMemoryAllocator
}

impl AsPtr<keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_>
    for *const keen__ZoneMemoryAllocator
{
    fn as_ptr(self) -> *const keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_ {
        self as *const _
    }
}

impl AsPtr<keen__MemoryAllocator> for *const keen__ZoneMemoryAllocator {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__ZoneMemoryAllocator____vftable {
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
pub struct keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_ {
    pub __vfptr: *const keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__ZoneAllocatorAdapter,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
}

impl AsPtr<keen__MemoryAllocator> for *const keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_ {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_____vftable {
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
pub struct keen__LowOverheadMemoryAllocator {
    pub __vfptr: *const keen__LowOverheadMemoryAllocator____vftable,
    // keen__MemoryAllocator
    // keen__BaseMemoryAllocator_keen__LowOverheadAllocator_
    pub m_mutex: keen__Mutex,
    pub m_name: [i8; 128],
    pub m_allocator: keen__LowOverheadAllocator,
    pub m_memoryBlock: keen__MemoryBlock,
    pub m_allocatedSize: u32,
    pub m_maxAllocatedSize: u32,
    pub m_allocationCount: u32,
    pub m_flags: u32,
    // keen__LowOverheadMemoryAllocator
}

impl AsPtr<keen__BaseMemoryAllocator_keen__LowOverheadAllocator_>
    for *const keen__LowOverheadMemoryAllocator
{
    fn as_ptr(self) -> *const keen__BaseMemoryAllocator_keen__LowOverheadAllocator_ {
        self as *const _
    }
}

impl AsPtr<keen__MemoryAllocator> for *const keen__LowOverheadMemoryAllocator {
    fn as_ptr(self) -> *const keen__MemoryAllocator {
        self as *const _
    }
}

#[repr(C)]
pub struct keen__LowOverheadMemoryAllocator____vftable {
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
pub struct keen__ZoneAllocatorAdapter {
    pub m_allocator: keen__ZoneAllocator,
    pub m_allocationCount: u32,
}

#[repr(C)]
pub struct keen__ZoneAllocator {
    pub m_memory: keen__MemoryBlock,
    pub m_pCurrentAddress: *mut u8,
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__String_const__gfc__String___ {
    // std___Allocator_base_std__pair_gfc__String_const__gfc__String___
    __pdbindgen_padding: [u8; 1],
    // std__allocator_std__pair_gfc__String_const__gfc__String___
}

impl AsPtr<std___Allocator_base_std__pair_gfc__String_const__gfc__String___>
    for *const std__allocator_std__pair_gfc__String_const__gfc__String___
{
    fn as_ptr(self) -> *const std___Allocator_base_std__pair_gfc__String_const__gfc__String___ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

impl AsPtr<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_
    __pdbindgen_padding: [u8; 1],
    /* std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ */
}

impl AsPtr<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_> for *const std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {
    fn as_ptr(self) -> *const std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
{
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
}

impl AsPtr<std___Container_base0> for *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__String_ {
    // std___Pair_base_gfc__String_const__gfc__String_
    pub first: gfc__String,
    pub second: gfc__String,
    // std__pair_gfc__String_const__gfc__String_
}

impl AsPtr<std___Pair_base_gfc__String_const__gfc__String_>
    for *const std__pair_gfc__String_const__gfc__String_
{
    fn as_ptr(self) -> *const std___Pair_base_gfc__String_const__gfc__String_ {
        self as *const _
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
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
    }
}

#[repr(C)]
pub struct std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    // std___Container_base0
    __pdbindgen_padding: [u8; 1],
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub comp: compile_error!("malformed PDB: oops"),
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____
}

impl AsPtr<std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    fn as_ptr(self) -> *const std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    fn as_ptr(self) -> *const std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    fn as_ptr(self) -> *const std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
        self as *const _
    }
}

impl AsPtr<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    fn as_ptr(self) -> *const std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {
        self as *const _
    }
}

impl AsPtr<std___Container_base0> for *const std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    fn as_ptr(self) -> *const std___Container_base0 {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__MeshBuilder
    pub mBounds: gfc__BoundingVolume,
    pub mVertexFormat: gfc__VertexFormat,
    pub mSubMeshes: gfc__Vector_gfc__AutoRef_gfc__MBSubMesh__0_gfc__CAllocator_,
    pub mBones: gfc__Vector_gfc__MBBone_0_gfc__CAllocator_,
    pub mUserData: gfc__Map_gfc__String_gfc__String_std__less_gfc__String___,
    pub mFlags: gfc__TFlags_unsigned_long_,
}

impl AsPtr<gfc__Object> for *const gfc__MeshBuilder {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__MeshBuilder {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
pub struct gfc__MeshCache {
    pub __vfptr: *const gfc__MeshCache____vftable,
    // gfc__ResourceCache
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__MeshCache
    pub mReloadInfo: gfc__Vector_gfc__MeshCache__ReloadInfo_0_gfc__CAllocator_,
}

impl AsPtr<gfc__ResourceCache> for *const gfc__MeshCache {
    fn as_ptr(self) -> *const gfc__ResourceCache {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__MeshCache____vftable {
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
pub struct gfc__MeshCache__ReloadInfo {
    pub PackageID: i32,
    pub Name: gfc__HString,
    pub Buffer: *mut u8,
    pub Length: u32,
}

#[repr(C)]
pub struct gfc__MeshResourceUnopt {
    pub __vfptr: *const gfc__MeshResourceUnopt____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
    // gfc__ResourceType_gfc__Mesh_2_
    pub mResource: gfc__AutoRef_gfc__Mesh_,
    // gfc__MeshResource
    pub mName: gfc__HString,
    // gfc__MeshResourceUnopt
    pub mBuilder: gfc__AutoRef_gfc__MeshBuilder_,
    pub mFinalizeMesh: gfc__AutoRef_gfc__Mesh_,
}

impl AsPtr<gfc__MeshResource> for *const gfc__MeshResourceUnopt {
    fn as_ptr(self) -> *const gfc__MeshResource {
        self as *const _
    }
}

impl AsPtr<gfc__ResourceType_gfc__Mesh_2_> for *const gfc__MeshResourceUnopt {
    fn as_ptr(self) -> *const gfc__ResourceType_gfc__Mesh_2_ {
        self as *const _
    }
}

impl AsPtr<gfc__Resource> for *const gfc__MeshResourceUnopt {
    fn as_ptr(self) -> *const gfc__Resource {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__MeshResourceUnopt {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__MeshResourceUnopt____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Resource) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub isUnoptimized: unsafe extern "thiscall" fn(this: *const gfc__MeshResource) -> bool,
}

#[repr(C)]
pub struct gfc__ResourceType_gfc__Mesh_2_ {
    pub __vfptr: *const gfc__ResourceType_gfc__Mesh_2_____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
    // gfc__ResourceType_gfc__Mesh_2_
    pub mResource: gfc__AutoRef_gfc__Mesh_,
}

impl AsPtr<gfc__Resource> for *const gfc__ResourceType_gfc__Mesh_2_ {
    fn as_ptr(self) -> *const gfc__Resource {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__ResourceType_gfc__Mesh_2_ {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
    }
}

#[repr(C)]
pub struct gfc__ResourceType_gfc__Mesh_2_____vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Resource) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__Resource),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__SkinMesh_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Mesh_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__MeshReader {
    pub __vfptr: *const gfc__MeshReader____vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    /* gfc__ObjectReader
     * gfc__MeshReader */
}

impl AsPtr<gfc__ObjectReader> for *const gfc__MeshReader {
    fn as_ptr(self) -> *const gfc__ObjectReader {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__MeshReader {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__MBSubMesh
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

impl AsPtr<gfc__Object> for *const gfc__MBSubMesh {
    fn as_ptr(self) -> *const gfc__Object {
        self as *const _
    }
}

impl AsPtr<gfc__IRefObject> for *const gfc__MBSubMesh {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__ObjectWriter
}

impl AsPtr<gfc__IRefObject> for *const gfc__ObjectWriter {
    fn as_ptr(self) -> *const gfc__IRefObject {
        self as *const _
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
