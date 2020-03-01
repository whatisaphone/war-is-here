#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types2::*, types3::*, types4::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

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
    pub vfptr: *const keen__MemoryAllocator__vftable,
}

impl keen__MemoryAllocator {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn allocate(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: *const i8,
    ) -> *mut () {
        ((*self.vfptr).allocate)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn free(&self, a1: *mut ()) {
        ((*self.vfptr).free)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const i8 {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct keen__MemoryAllocator__vftable {
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
pub struct keen__Matrix43 {
    pub rot: keen__Matrix33,
    pub pos: keen__Vector3,
}

#[repr(C)]
pub struct keen__Matrix44 {
    pub x: keen__Vector4,
    pub y: keen__Vector4,
    pub z: keen__Vector4,
    pub w: keen__Vector4,
}

#[repr(C)]
pub struct keen__Vector3 {
    // keen__float3
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // keen__Vector3
}

unsafe impl UpcastToNop<keen__float3> for keen__Vector3 {}

#[repr(C)]
pub struct keen__Projection {
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub data: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 24],
    pub r#type: keen__ProjectionType,
    pub rightHanded: bool,
}

#[repr(C)]
pub struct keen__Frustum {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_planes: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 96],
}

#[repr(C)]
pub struct keen__Matrix33 {
    pub x: keen__Vector3,
    pub y: keen__Vector3,
    pub z: keen__Vector3,
}

#[repr(C)]
pub struct keen__Vector4 {
    // keen__float4
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    // keen__Vector4
}

unsafe impl UpcastToNop<keen__float4> for keen__Vector4 {}

#[repr(C)]
pub struct keen__Camera {
    pub m_worldMatrix: keen__Matrix43,
    pub m_projection: keen__Projection,
    pub m_viewMatrix: keen__Matrix43,
    pub m_frustum: keen__Frustum,
    pub m_viewDirty: bool,
    pub m_frustumDirty: bool,
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
    // keen__MemoryAllocator
    pub vfptr: *const keen__BaseMemoryAllocator_keen__TlsfAllocator___vftable,
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

unsafe impl UpcastToNop<keen__MemoryAllocator> for keen__BaseMemoryAllocator_keen__TlsfAllocator_ {}

impl keen__BaseMemoryAllocator_keen__TlsfAllocator_ {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn allocate(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: *const i8,
    ) -> *mut () {
        ((*self.vfptr).allocate)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn free(&self, a1: *mut ()) {
        ((*self.vfptr).free)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const i8 {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__TlsfAllocator___vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__TlsfAllocator_,
        _: u32,
    ) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__TlsfAllocator_,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__TlsfAllocator_,
        _: *mut (),
    ),
    pub getName: unsafe extern "thiscall" fn(
        this: *const keen__BaseMemoryAllocator_keen__TlsfAllocator_,
    ) -> *const i8,
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
    // keen__MemoryAllocator
    pub vfptr: *const keen__TlsfMemoryAllocator__vftable,
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

unsafe impl UpcastToNop<keen__BaseMemoryAllocator_keen__TlsfAllocator_>
    for keen__TlsfMemoryAllocator
{
}

unsafe impl UpcastToNop<keen__MemoryAllocator> for keen__TlsfMemoryAllocator {}

impl keen__TlsfMemoryAllocator {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn allocate(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: *const i8,
    ) -> *mut () {
        ((*self.vfptr).allocate)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn free(&self, a1: *mut ()) {
        ((*self.vfptr).free)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const i8 {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct keen__TlsfMemoryAllocator__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__TlsfMemoryAllocator, _: u32) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__TlsfMemoryAllocator,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(this: *mut keen__TlsfMemoryAllocator, _: *mut ()),
    pub getName: unsafe extern "thiscall" fn(this: *const keen__TlsfMemoryAllocator) -> *const i8,
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
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_steamid: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct CCallbackBase {
    pub vfptr: *const CCallbackBase__vftable,
    pub m_nCallbackFlags: u8,
    pub m_iCallback: i32,
}

impl CCallbackBase {
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
pub struct CCallbackBase__vftable {
    pub Run: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut (), _: bool, _: u64),
    pub Run_2: unsafe extern "thiscall" fn(this: *mut CCallbackBase, _: *mut ()),
    pub GetCallbackSizeBytes: unsafe extern "thiscall" fn(this: *mut CCallbackBase) -> i32,
}

#[repr(C)]
pub struct keen__UserAccountSystemBase {
    pub creationParameters: keen__UserAccountSystemCreationParameters,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub runningOperations: compile_error!("unimplemented class layout"),
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub runningOperations: compile_error!("unimplemented class layout"),
    // keen__UserAccountSystem
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub accounts: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 4280],
    pub accountCount: u32,
}

unsafe impl UpcastToNop<keen__UserAccountSystemBase> for keen__UserAccountSystem {}

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
    pub vfptr: *const ISteamUserStats__vftable,
}

impl ISteamUserStats {
    pub unsafe extern "thiscall" fn RequestCurrentStats(&self) -> bool {
        ((*self.vfptr).RequestCurrentStats)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn GetStat(&self, a1: *const i8, a2: *mut f32) -> bool {
        ((*self.vfptr).GetStat)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn GetStat_2(&self, a1: *const i8, a2: *mut i32) -> bool {
        ((*self.vfptr).GetStat_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn SetStat(&self, a1: *const i8, a2: f32) -> bool {
        ((*self.vfptr).SetStat)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn SetStat_2(&self, a1: *const i8, a2: i32) -> bool {
        ((*self.vfptr).SetStat_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn UpdateAvgRateStat(
        &self,
        a1: *const i8,
        a2: f32,
        a3: f64,
    ) -> bool {
        ((*self.vfptr).UpdateAvgRateStat)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetAchievement(&self, a1: *const i8, a2: *mut bool) -> bool {
        ((*self.vfptr).GetAchievement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn SetAchievement(&self, a1: *const i8) -> bool {
        ((*self.vfptr).SetAchievement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn ClearAchievement(&self, a1: *const i8) -> bool {
        ((*self.vfptr).ClearAchievement)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetAchievementAndUnlockTime(
        &self,
        a1: *const i8,
        a2: *mut bool,
        a3: *mut u32,
    ) -> bool {
        ((*self.vfptr).GetAchievementAndUnlockTime)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn StoreStats(&self) -> bool {
        ((*self.vfptr).StoreStats)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn GetAchievementIcon(&self, a1: *const i8) -> i32 {
        ((*self.vfptr).GetAchievementIcon)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetAchievementDisplayAttribute(
        &self,
        a1: *const i8,
        a2: *const i8,
    ) -> *const i8 {
        ((*self.vfptr).GetAchievementDisplayAttribute)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn IndicateAchievementProgress(
        &self,
        a1: *const i8,
        a2: u32,
        a3: u32,
    ) -> bool {
        ((*self.vfptr).IndicateAchievementProgress)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetNumAchievements(&self) -> u32 {
        ((*self.vfptr).GetNumAchievements)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn GetAchievementName(&self, a1: u32) -> *const i8 {
        ((*self.vfptr).GetAchievementName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn RequestUserStats(&self, a1: CSteamID) -> u64 {
        ((*self.vfptr).RequestUserStats)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetUserStat(
        &self,
        a1: CSteamID,
        a2: *const i8,
        a3: *mut f32,
    ) -> bool {
        ((*self.vfptr).GetUserStat)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetUserStat_2(
        &self,
        a1: CSteamID,
        a2: *const i8,
        a3: *mut i32,
    ) -> bool {
        ((*self.vfptr).GetUserStat_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetUserAchievement(
        &self,
        a1: CSteamID,
        a2: *const i8,
        a3: *mut bool,
    ) -> bool {
        ((*self.vfptr).GetUserAchievement)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetUserAchievementAndUnlockTime(
        &self,
        a1: CSteamID,
        a2: *const i8,
        a3: *mut bool,
        a4: *mut u32,
    ) -> bool {
        ((*self.vfptr).GetUserAchievementAndUnlockTime)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn ResetAllStats(&self, a1: bool) -> bool {
        ((*self.vfptr).ResetAllStats)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn FindOrCreateLeaderboard(
        &self,
        a1: *const i8,
        a2: ELeaderboardSortMethod,
        a3: ELeaderboardDisplayType,
    ) -> u64 {
        ((*self.vfptr).FindOrCreateLeaderboard)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn FindLeaderboard(&self, a1: *const i8) -> u64 {
        ((*self.vfptr).FindLeaderboard)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetLeaderboardName(&self, a1: u64) -> *const i8 {
        ((*self.vfptr).GetLeaderboardName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetLeaderboardEntryCount(&self, a1: u64) -> i32 {
        ((*self.vfptr).GetLeaderboardEntryCount)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetLeaderboardSortMethod(
        &self,
        a1: u64,
    ) -> ELeaderboardSortMethod {
        ((*self.vfptr).GetLeaderboardSortMethod)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetLeaderboardDisplayType(
        &self,
        a1: u64,
    ) -> ELeaderboardDisplayType {
        ((*self.vfptr).GetLeaderboardDisplayType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn DownloadLeaderboardEntries(
        &self,
        a1: u64,
        a2: ELeaderboardDataRequest,
        a3: i32,
        a4: i32,
    ) -> u64 {
        ((*self.vfptr).DownloadLeaderboardEntries)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn DownloadLeaderboardEntriesForUsers(
        &self,
        a1: u64,
        a2: *mut CSteamID,
        a3: i32,
    ) -> u64 {
        ((*self.vfptr).DownloadLeaderboardEntriesForUsers)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetDownloadedLeaderboardEntry(
        &self,
        a1: u64,
        a2: i32,
        a3: *mut LeaderboardEntry_t,
        a4: *mut i32,
        a5: i32,
    ) -> bool {
        ((*self.vfptr).GetDownloadedLeaderboardEntry)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn UploadLeaderboardScore(
        &self,
        a1: u64,
        a2: ELeaderboardUploadScoreMethod,
        a3: i32,
        a4: *const i32,
        a5: i32,
    ) -> u64 {
        ((*self.vfptr).UploadLeaderboardScore)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn AttachLeaderboardUGC(&self, a1: u64, a2: u64) -> u64 {
        ((*self.vfptr).AttachLeaderboardUGC)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn GetNumberOfCurrentPlayers(&self) -> u64 {
        ((*self.vfptr).GetNumberOfCurrentPlayers)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn RequestGlobalAchievementPercentages(&self) -> u64 {
        ((*self.vfptr).RequestGlobalAchievementPercentages)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn GetMostAchievedAchievementInfo(
        &self,
        a1: *mut i8,
        a2: u32,
        a3: *mut f32,
        a4: *mut bool,
    ) -> i32 {
        ((*self.vfptr).GetMostAchievedAchievementInfo)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn GetNextMostAchievedAchievementInfo(
        &self,
        a1: i32,
        a2: *mut i8,
        a3: u32,
        a4: *mut f32,
        a5: *mut bool,
    ) -> i32 {
        ((*self.vfptr).GetNextMostAchievedAchievementInfo)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn GetAchievementAchievedPercent(
        &self,
        a1: *const i8,
        a2: *mut f32,
    ) -> bool {
        ((*self.vfptr).GetAchievementAchievedPercent)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn RequestGlobalStats(&self, a1: i32) -> u64 {
        ((*self.vfptr).RequestGlobalStats)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn GetGlobalStat(&self, a1: *const i8, a2: *mut f64) -> bool {
        ((*self.vfptr).GetGlobalStat)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn GetGlobalStat_2(&self, a1: *const i8, a2: *mut i64) -> bool {
        ((*self.vfptr).GetGlobalStat_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn GetGlobalStatHistory(
        &self,
        a1: *const i8,
        a2: *mut f64,
        a3: u32,
    ) -> i32 {
        ((*self.vfptr).GetGlobalStatHistory)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn GetGlobalStatHistory_2(
        &self,
        a1: *const i8,
        a2: *mut i64,
        a3: u32,
    ) -> i32 {
        ((*self.vfptr).GetGlobalStatHistory_2)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct ISteamUserStats__vftable {
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

unsafe impl UpcastToNop<ID3D11View> for ID3D11ShaderResourceView {}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11ShaderResourceView {}

unsafe impl UpcastToNop<IUnknown> for ID3D11ShaderResourceView {}

#[repr(C)]
pub struct ID3D11View {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11View {}

unsafe impl UpcastToNop<IUnknown> for ID3D11View {}

#[repr(C)]
pub struct ID3D11InputLayout {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11InputLayout */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11InputLayout {}

unsafe impl UpcastToNop<IUnknown> for ID3D11InputLayout {}

#[repr(C)]
pub struct ID3D11DeviceChild {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // ID3D11DeviceChild
}

unsafe impl UpcastToNop<IUnknown> for ID3D11DeviceChild {}

#[repr(C)]
pub struct ID3D11DepthStencilState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11DepthStencilState */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11DepthStencilState {}

unsafe impl UpcastToNop<IUnknown> for ID3D11DepthStencilState {}

#[repr(C)]
pub struct ID3D11SamplerState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11SamplerState */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11SamplerState {}

unsafe impl UpcastToNop<IUnknown> for ID3D11SamplerState {}

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

unsafe impl UpcastToNop<ID3D11View> for ID3D11RenderTargetView {}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11RenderTargetView {}

unsafe impl UpcastToNop<IUnknown> for ID3D11RenderTargetView {}

#[repr(C)]
pub struct ID3D11Device {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    // ID3D11Device
}

unsafe impl UpcastToNop<IUnknown> for ID3D11Device {}

#[repr(C)]
pub struct ID3D11DepthStencilView {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11View
     * ID3D11DepthStencilView */
}

unsafe impl UpcastToNop<ID3D11View> for ID3D11DepthStencilView {}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11DepthStencilView {}

unsafe impl UpcastToNop<IUnknown> for ID3D11DepthStencilView {}

#[repr(C)]
pub struct ID3D11VertexShader {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11VertexShader */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11VertexShader {}

unsafe impl UpcastToNop<IUnknown> for ID3D11VertexShader {}

#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}

#[repr(C)]
pub struct ID3D11BlendState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11BlendState */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11BlendState {}

unsafe impl UpcastToNop<IUnknown> for ID3D11BlendState {}

#[repr(C)]
pub struct ID3D11PixelShader {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11PixelShader */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11PixelShader {}

unsafe impl UpcastToNop<IUnknown> for ID3D11PixelShader {}

#[repr(C)]
pub struct keen__DepthStencilState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__DepthStencilState
    pub pState: *mut ID3D11DepthStencilState,
}

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__DepthStencilState {}

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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_instances: compile_error!("unimplemented class layout"),
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

unsafe impl UpcastToNop<keen__InternalListBase> for keen__InternalList_keen__HashMap_unsigned_int_keen__GraphicsStateObject___keen__DefaultHashmapTraits_unsigned_int_keen__GraphicsStateObject_______Entry_ {}

#[repr(C)]
pub struct keen__RasterizerState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__RasterizerState
    pub pState: *mut ID3D11RasterizerState,
}

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__RasterizerState {}

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

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__SamplerState {}

#[repr(C)]
pub struct keen__BlendState {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__BlendState
    pub pState: *mut ID3D11BlendState,
}

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__BlendState {}

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

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__VertexInputBinding {}

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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub colorBuffers: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 64],
    pub depthBuffer: keen__RenderTargetBuffer,
    pub colorBufferCount: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct keen__SoftwareSkinningJointMatrixData {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub jointMatrices: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 16384],
}

#[repr(C)]
pub struct keen__VertexFormat {
    // keen__GraphicsStateObject
    pub hash: u32,
    pub refCount: u32,
    // keen__VertexFormat
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub attributes: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 68],
    pub attributeOffsets: [u32; 17],
    pub attributeCount: u32,
    pub attributeIndices: [u32; 17],
    pub streamStride: [u32; 3],
}

unsafe impl UpcastToNop<keen__GraphicsStateObject> for keen__VertexFormat {}

#[repr(C)]
pub struct ID3D11Buffer {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11Resource
     * ID3D11Buffer */
}

unsafe impl UpcastToNop<ID3D11Resource> for ID3D11Buffer {}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11Buffer {}

unsafe impl UpcastToNop<IUnknown> for ID3D11Buffer {}

#[repr(C)]
pub struct ID3D11Resource {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11Resource */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11Resource {}

unsafe impl UpcastToNop<IUnknown> for ID3D11Resource {}

#[repr(C)]
pub struct ID3D11RasterizerState {
    // IUnknown
    pub lpVtbl: *mut IUnknownVtbl,
    /* ID3D11DeviceChild
     * ID3D11RasterizerState */
}

unsafe impl UpcastToNop<ID3D11DeviceChild> for ID3D11RasterizerState {}

unsafe impl UpcastToNop<IUnknown> for ID3D11RasterizerState {}

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
    #[cfg(pdb_issue = "unimplemented enum layout")]
    pub responseOptions: compile_error!("unimplemented enum layout"),
    __pdbindgen_padding: [u8; 8],
    pub responseOptionCount: u32,
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface {
    pub vfptr: *const keen__SaveData__SaveDataProviderWin32Interface__vftable,
}

impl keen__SaveData__SaveDataProviderWin32Interface {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn create(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *const keen__SaveData__SaveDataSystemCreationParameters,
    ) -> bool {
        ((*self.vfptr).create)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn destroy(&self, a1: *mut keen__MemoryAllocator) {
        ((*self.vfptr).destroy)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn enumerateProfiles(
        &self,
        a1: *mut keen__SizedArray_keen__SaveData__Profile_,
    ) -> bool {
        ((*self.vfptr).enumerateProfiles)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn enumerateSaveData(
        &self,
        a1: *mut keen__SizedArray_keen__SaveData__SaveData_,
        a2: u32,
        a3: u32,
    ) -> bool {
        ((*self.vfptr).enumerateSaveData)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn connectProfileWithUser(
        &self,
        a1: u32,
        a2: keen__UserAccountId,
    ) -> bool {
        ((*self.vfptr).connectProfileWithUser)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn disconnectProfile(&self, a1: u32) -> bool {
        ((*self.vfptr).disconnectProfile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isProfileConnected(&self, a1: u32) -> bool {
        ((*self.vfptr).isProfileConnected)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateCreateProfile(
        &self,
        a1: u32,
        a2: *const keen__SaveData__Profile,
    ) {
        ((*self.vfptr).updateCreateProfile)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateUpdateProfile(
        &self,
        a1: u32,
        a2: *const keen__SaveData__Profile,
    ) {
        ((*self.vfptr).updateUpdateProfile)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateEraseProfile(&self, a1: u32) {
        ((*self.vfptr).updateEraseProfile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateConnectProfile(
        &self,
        a1: u32,
        a2: *const keen__SaveData__Profile,
    ) {
        ((*self.vfptr).updateConnectProfile)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateDisconnectProfile(
        &self,
        a1: u32,
        a2: *const keen__SaveData__Profile,
    ) {
        ((*self.vfptr).updateDisconnectProfile)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateCheckSaveDataSet(&self, a1: u32) {
        ((*self.vfptr).updateCheckSaveDataSet)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateCreateSaveDataSet(&self, a1: u32) {
        ((*self.vfptr).updateCreateSaveDataSet)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateEraseSaveDataSet(&self, a1: u32) {
        ((*self.vfptr).updateEraseSaveDataSet)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateCreateSaveData(
        &self,
        a1: *mut keen__SaveData__SaveData,
        a2: u32,
        a3: *const keen__SaveData__StorageOperation,
    ) {
        ((*self.vfptr).updateCreateSaveData)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn updateWriteSaveData(
        &self,
        a1: u32,
        a2: *const keen__SaveData__SaveData,
        a3: *const keen__SaveData__StorageOperation,
    ) {
        ((*self.vfptr).updateWriteSaveData)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn updateReadSaveData(
        &self,
        a1: *mut keen__SaveData__StorageOperation,
        a2: u32,
        a3: *const keen__SaveData__SaveData,
    ) {
        ((*self.vfptr).updateReadSaveData)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn updateEraseSaveData(
        &self,
        a1: u32,
        a2: *const keen__SaveData__SaveData,
    ) {
        ((*self.vfptr).updateEraseSaveData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn hasFinishedOperation(&self) -> bool {
        ((*self.vfptr).hasFinishedOperation)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getOperationResult(&self) -> keen__SaveData__OperationResult {
        ((*self.vfptr).getOperationResult)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSegmentCount(&self, a1: u32) -> u32 {
        ((*self.vfptr).getSegmentCount)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct keen__SaveData__SaveDataProviderWin32Interface__vftable {
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
    pub vfptr: *const keen__FileDeviceInterface__vftable,
}

impl keen__FileDeviceInterface {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn openFile(
        &self,
        result: *mut keen__IoResult_unsigned_short_,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_ {
        ((*self.vfptr).openFile)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn closeFile(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).closeFile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeMountData(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *mut keen__FileDeviceMountData,
    ) -> keen__IoError {
        ((*self.vfptr).freeMountData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *mut (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).read)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn write(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *const (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).write)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn flushWriteBuffer(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).flushWriteBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: u16, a2: u64) -> keen__IoError {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setFilePermissionByName(
        &self,
        a1: *const keen__FileDeviceMountData,
        a2: *const i8,
        a3: bool,
    ) -> keen__IoError {
        ((*self.vfptr).setFilePermissionByName)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getFileStatusByName(
        &self,
        a1: *mut keen__FileStatus,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: u32,
    ) -> keen__IoError {
        ((*self.vfptr).getFileStatusByName)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct keen__FileDeviceInterface__vftable {
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
    // keen__FileDeviceInterface
    pub vfptr: *const keen__AliasPathFileDevice__vftable,
    // keen__AliasPathFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__AliasPathFileDevice__StreamEntry_,
}

unsafe impl UpcastToNop<keen__FileDeviceInterface> for keen__AliasPathFileDevice {}

impl keen__AliasPathFileDevice {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn openFile(
        &self,
        result: *mut keen__IoResult_unsigned_short_,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_ {
        ((*self.vfptr).openFile)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn closeFile(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).closeFile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeMountData(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *mut keen__FileDeviceMountData,
    ) -> keen__IoError {
        ((*self.vfptr).freeMountData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *mut (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).read)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn write(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *const (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).write)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn flushWriteBuffer(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).flushWriteBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: u16, a2: u64) -> keen__IoError {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setFilePermissionByName(
        &self,
        a1: *const keen__FileDeviceMountData,
        a2: *const i8,
        a3: bool,
    ) -> keen__IoError {
        ((*self.vfptr).setFilePermissionByName)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getFileStatusByName(
        &self,
        a1: *mut keen__FileStatus,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: u32,
    ) -> keen__IoError {
        ((*self.vfptr).getFileStatusByName)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct keen__AliasPathFileDevice__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__AliasPathFileDevice, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__AliasPathFileDevice, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__AliasPathFileDevice, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__AliasPathFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__AliasPathFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__AliasPathFileDevice,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__AliasPathFileDevice,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__NativeFileDevice {
    // keen__FileDeviceInterface
    pub vfptr: *const keen__NativeFileDevice__vftable,
    // keen__NativeFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__NativeFileDevice__FileStreamData_,
    pub m_readThreadContexts: keen__Array_keen__NativeFileDevice__ReadThreadContext_,
    pub m_mutex: keen__Mutex,
    pub m_useReadThread: bool,
}

unsafe impl UpcastToNop<keen__FileDeviceInterface> for keen__NativeFileDevice {}

impl keen__NativeFileDevice {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn openFile(
        &self,
        result: *mut keen__IoResult_unsigned_short_,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_ {
        ((*self.vfptr).openFile)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn closeFile(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).closeFile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeMountData(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *mut keen__FileDeviceMountData,
    ) -> keen__IoError {
        ((*self.vfptr).freeMountData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *mut (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).read)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn write(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *const (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).write)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn flushWriteBuffer(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).flushWriteBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: u16, a2: u64) -> keen__IoError {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setFilePermissionByName(
        &self,
        a1: *const keen__FileDeviceMountData,
        a2: *const i8,
        a3: bool,
    ) -> keen__IoError {
        ((*self.vfptr).setFilePermissionByName)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getFileStatusByName(
        &self,
        a1: *mut keen__FileStatus,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: u32,
    ) -> keen__IoError {
        ((*self.vfptr).getFileStatusByName)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct keen__NativeFileDevice__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__NativeFileDevice, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__NativeFileDevice, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__NativeFileDevice, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__NativeFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__NativeFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__NativeFileDevice,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__NativeFileDevice,
        _: *mut keen__FileStatus,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: u32,
    ) -> keen__IoError,
}

#[repr(C)]
pub struct keen__NativeFileDevice__ReadThreadContext {
    pub thread: keen__Thread,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub buffers: compile_error!("unimplemented class layout"),
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
    // keen__FileDeviceInterface
    pub vfptr: *const keen__PakFileDevice__vftable,
    // keen__PakFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__PakFileDevice__PakFileStream_,
    pub m_chunkAllocator: keen__TlsfMemoryAllocator,
}

unsafe impl UpcastToNop<keen__FileDeviceInterface> for keen__PakFileDevice {}

impl keen__PakFileDevice {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn openFile(
        &self,
        result: *mut keen__IoResult_unsigned_short_,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_ {
        ((*self.vfptr).openFile)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn closeFile(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).closeFile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeMountData(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *mut keen__FileDeviceMountData,
    ) -> keen__IoError {
        ((*self.vfptr).freeMountData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *mut (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).read)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn write(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *const (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).write)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn flushWriteBuffer(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).flushWriteBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: u16, a2: u64) -> keen__IoError {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setFilePermissionByName(
        &self,
        a1: *const keen__FileDeviceMountData,
        a2: *const i8,
        a3: bool,
    ) -> keen__IoError {
        ((*self.vfptr).setFilePermissionByName)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getFileStatusByName(
        &self,
        a1: *mut keen__FileStatus,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: u32,
    ) -> keen__IoError {
        ((*self.vfptr).getFileStatusByName)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct keen__PakFileDevice__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__PakFileDevice, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__PakFileDevice, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__PakFileDevice, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__PakFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__PakFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__PakFileDevice,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__PakFileDevice,
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

unsafe impl UpcastToNop<keen__ListEntry_keen__FileSystemDeviceEntry_>
    for keen__FileSystemDeviceEntry
{
}

#[repr(C)]
pub struct keen__IoResult_unsigned_short_ {
    pub result: u16,
    pub error: keen__IoError,
}

#[repr(C)]
pub struct keen__MemoryFileDevice {
    // keen__FileDeviceInterface
    pub vfptr: *const keen__MemoryFileDevice__vftable,
    // keen__MemoryFileDevice
    pub m_streams: keen__FileStreamAllocator_keen__MemoryFileDevice__StreamEntry_,
    pub m_files: keen__Array_keen__MemoryFileDevice__FileEntry_,
    pub m_fileIndices: keen__IndexArray,
}

unsafe impl UpcastToNop<keen__FileDeviceInterface> for keen__MemoryFileDevice {}

impl keen__MemoryFileDevice {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn openFile(
        &self,
        result: *mut keen__IoResult_unsigned_short_,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_ {
        ((*self.vfptr).openFile)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn closeFile(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).closeFile)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn freeMountData(
        &self,
        a1: *mut keen__MemoryAllocator,
        a2: *mut keen__FileDeviceMountData,
    ) -> keen__IoError {
        ((*self.vfptr).freeMountData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *mut (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).read)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn write(
        &self,
        result: *mut keen__IoResult_unsigned_int_,
        a2: u16,
        a3: *const (),
        a4: u32,
    ) -> *mut keen__IoResult_unsigned_int_ {
        ((*self.vfptr).write)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn flushWriteBuffer(&self, a1: u16) -> keen__IoError {
        ((*self.vfptr).flushWriteBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSize(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getSize)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: u16, a2: u64) -> keen__IoError {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPosition(
        &self,
        result: *mut keen__IoResult_unsigned___int64_,
        a2: u16,
    ) -> *mut keen__IoResult_unsigned___int64_ {
        ((*self.vfptr).getPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setFilePermissionByName(
        &self,
        a1: *const keen__FileDeviceMountData,
        a2: *const i8,
        a3: bool,
    ) -> keen__IoError {
        ((*self.vfptr).setFilePermissionByName)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getFileStatusByName(
        &self,
        a1: *mut keen__FileStatus,
        a2: *const keen__FileDeviceMountData,
        a3: *const i8,
        a4: u32,
    ) -> keen__IoError {
        ((*self.vfptr).getFileStatusByName)(self as *const _ as *mut _, a1, a2, a3, a4)
    }
}

#[repr(C)]
pub struct keen__MemoryFileDevice__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__MemoryFileDevice, _: u32) -> *mut (),
    pub openFile: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        result: *mut keen__IoResult_unsigned_short_,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: keen__FileAccessMode,
    ) -> *mut keen__IoResult_unsigned_short_,
    pub closeFile:
        unsafe extern "thiscall" fn(this: *mut keen__MemoryFileDevice, _: u16) -> keen__IoError,
    pub freeMountData: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        _: *mut keen__MemoryAllocator,
        _: *mut keen__FileDeviceMountData,
    ) -> keen__IoError,
    pub read: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *mut (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub write: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        result: *mut keen__IoResult_unsigned_int_,
        _: u16,
        _: *const (),
        _: u32,
    ) -> *mut keen__IoResult_unsigned_int_,
    pub flushWriteBuffer:
        unsafe extern "thiscall" fn(this: *mut keen__MemoryFileDevice, _: u16) -> keen__IoError,
    pub getSize: unsafe extern "thiscall" fn(
        this: *const keen__MemoryFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        _: u16,
        _: u64,
    ) -> keen__IoError,
    pub getPosition: unsafe extern "thiscall" fn(
        this: *const keen__MemoryFileDevice,
        result: *mut keen__IoResult_unsigned___int64_,
        _: u16,
    ) -> *mut keen__IoResult_unsigned___int64_,
    pub setFilePermissionByName: unsafe extern "thiscall" fn(
        this: *mut keen__MemoryFileDevice,
        _: *const keen__FileDeviceMountData,
        _: *const i8,
        _: bool,
    ) -> keen__IoError,
    pub getFileStatusByName: unsafe extern "thiscall" fn(
        this: *const keen__MemoryFileDevice,
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
    // keen__MemoryFileDeviceMountData
    pub pAllocator: *mut keen__MemoryAllocator,
    pub fileHandle: u32,
}

unsafe impl UpcastToNop<keen__FileDeviceMountData> for keen__MemoryFileDeviceMountData {}

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

unsafe impl UpcastToNop<keen__TreeNode_keen__FileSystemMountPoint_> for keen__FileSystemMountPoint {}

#[repr(C)]
pub struct keen__IndexArray {
    pub m_indices: keen__SizedArray_unsigned_int_,
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__LowOverheadAllocator_ {
    // keen__MemoryAllocator
    pub vfptr: *const keen__BaseMemoryAllocator_keen__LowOverheadAllocator___vftable,
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

unsafe impl UpcastToNop<keen__MemoryAllocator>
    for keen__BaseMemoryAllocator_keen__LowOverheadAllocator_
{
}

impl keen__BaseMemoryAllocator_keen__LowOverheadAllocator_ {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn allocate(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: *const i8,
    ) -> *mut () {
        ((*self.vfptr).allocate)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn free(&self, a1: *mut ()) {
        ((*self.vfptr).free)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const i8 {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__LowOverheadAllocator___vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__LowOverheadAllocator_,
        _: u32,
    ) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__LowOverheadAllocator_,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__LowOverheadAllocator_,
        _: *mut (),
    ),
    pub getName: unsafe extern "thiscall" fn(
        this: *const keen__BaseMemoryAllocator_keen__LowOverheadAllocator_,
    ) -> *const i8,
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

unsafe impl UpcastToNop<IUnknown> for IDirectInput8A {}

#[repr(C)]
pub struct keen__InputSystem {
    pub eventQueue: keen__Queue_keen__InputEvent_,
    __pdbindgen_padding: [u8; 4],
    pub controllerState: keen__PlatformControllerState,
    pub storedEvents: keen__SizedArray_keen__InputEvent_,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub controllerInfos: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 384],
    pub autoCatchType: keen__InputSystemControllerAutoCatchType,
    pub autoCatchPlayerId: *const keen__LocalPlayerIdStructureType,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub mappedAxisButtonStates: compile_error!("unimplemented class layout"),
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
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub data: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 16],
}

#[repr(C)]
pub struct keen__PlatformControllerState {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub controllers: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 48],
    pub mouseVisible: bool,
    pub mousePositionRelative: bool,
    pub lastMousePosition: keen__float2,
    pub pDirect8: *mut IDirectInput8A,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub directInputControllers: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 64],
    pub directInputControllerCount: u32,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub lastState: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_3: [u8; 1796],
    pub mouseWheelButtonFlags: u32,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub steamController: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_4: [u8; 68],
    pub steamControllerCount: u32,
    __pdbindgen_padding_5: [u8; 4],
}

#[repr(C)]
pub struct keen__MouseEventData {
    pub position: keen__float2,
}

#[repr(C)]
pub struct keen__MouseWheelEventData {
    pub wheelDelta: i32,
}

#[repr(C)]
pub struct keen__StaticArray_keen__GestureHelper__MoveData_ {
    pub m_pData: *mut keen__GestureHelper__MoveData,
    pub m_size: u32,
}

#[repr(C)]
pub struct keen__KeyEventData {
    pub keyCode: u32,
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
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_ {}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    // std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_> for std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
    // std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_,
    pub _Alval: std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_> for std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
    // std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_,
    pub _Alval: std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____,
    // std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    // std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_> for std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

#[repr(C)]
pub struct std__allocator_char_ {
    // std___Allocator_base_char_
    // std__allocator_char_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_char_> for std__allocator_char_ {}

#[repr(C)]
pub struct std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
{
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_ {}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    // std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_> for std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
    // std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_,
    pub _Alval: std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____,
    // std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_> for std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___ {}

#[repr(C)]
pub struct std___Container_base0 {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___String_val_char_std__allocator_char___ {
    // std___Container_base0
    // std___String_val_char_std__allocator_char___
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub _Bx: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 16],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___String_val_char_std__allocator_char___ {}

#[repr(C)]
pub struct std__binary_function_gfc__HString_gfc__HString_bool_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__binary_function_int_int_bool_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

#[repr(C)]
pub struct std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
    // std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_,
    pub _Alval: std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____,
    // std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    // std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    // std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_> for std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___ {}

#[repr(C)]
pub struct std__less_int_ {
    // std__binary_function_int_int_bool_
    // std__less_int_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std__binary_function_int_int_bool_> for std__less_int_ {}

#[repr(C)]
pub struct std__less_gfc__HString_ {
    // std__binary_function_gfc__HString_gfc__HString_bool_
    // std__less_gfc__HString_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std__binary_function_gfc__HString_gfc__HString_bool_>
    for std__less_gfc__HString_
{
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____
{
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____
    // std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____> for std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

#[repr(C)]
pub struct std___Allocator_base_char_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_ {}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__ResourceCache_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___ {}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__ResourceCache_____
    // std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__ResourceCache_____>
    for std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____
{
}

#[repr(C)]
pub struct std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____ {
    // std___Allocator_base_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____
    // std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____>
    for std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____
{
}

#[repr(C)]
pub struct std__basic_string_char_std__char_traits_char__std__allocator_char___ {
    // std___Container_base0
    // std___String_val_char_std__allocator_char___
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub _Bx: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 16],
    pub _Mysize: u32,
    pub _Myres: u32,
    pub _Alval: std__allocator_char_,
    // std__basic_string_char_std__char_traits_char__std__allocator_char___
}

unsafe impl UpcastToNop<std___String_val_char_std__allocator_char___>
    for std__basic_string_char_std__char_traits_char__std__allocator_char___
{
}

unsafe impl UpcastToNop<std___Container_base0>
    for std__basic_string_char_std__char_traits_char__std__allocator_char___
{
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub r: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub g: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub b: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub u: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub v: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub w: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
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
    pub vfptr: *const gfc__ResourceCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
}

impl gfc__ResourceCache {
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
pub struct gfc__ResourceCache__vftable {
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub r: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub g: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub b: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub a: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
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
pub struct gfc__Vector_gfc__ResourceCache___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ResourceCache,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ValueStack {
    pub mSize: i32,
    pub mStack: *mut gfc__ValueStack___Stack,
}

#[repr(C)]
pub struct gfc__ValueStack___Stack {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub Stack: compile_error!("unimplemented class layout"),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__InputStream__vftable,
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__InputStream
    pub mEndianess: i32,
    pub mBufferAvail: u32,
    pub mBufferPtr: *mut u8,
}

unsafe impl UpcastToNop<gfc__Stream> for gfc__InputStream {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__InputStream {}

impl gfc__InputStream {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Stream__StreamType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn available(&self) -> i64 {
        ((*self.vfptr).available)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn close(&self) {
        ((*self.vfptr).close)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn markSupported(&self) -> bool {
        ((*self.vfptr).markSupported)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn mark(&self) {
        ((*self.vfptr).mark)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn seekSupported(&self) -> bool {
        ((*self.vfptr).seekSupported)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn seek(&self, a1: u64, a2: i32) {
        ((*self.vfptr).seek)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn tell(&self) -> i64 {
        ((*self.vfptr).tell)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn skipBytes(&self, a1: i32) {
        ((*self.vfptr).skipBytes)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clone(
        &self,
        result: *mut gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__InputStream_ {
        ((*self.vfptr).clone)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getBuffer(&self) -> *const u8 {
        ((*self.vfptr).getBuffer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn read_raw(&self, a1: *mut (), a2: i32) -> i32 {
        ((*self.vfptr).read_raw)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__InputStream__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__InputStream, _: u32) -> *mut (),
    pub getType:
        unsafe extern "thiscall" fn(this: *const gfc__InputStream) -> gfc__Stream__StreamType,
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
pub struct gfc__Lock {
    pub mMutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourceListener___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ResourceListener,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
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
pub struct gfc__Vector_gfc__AutoRef_gfc__PackageMarker__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__PackageMarker_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
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
pub struct gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {
    // std___Container_base0
    // std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_
    pub comp: std__less_int_,
    // std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0______Node_,
    pub _Alval: std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_____,
    // std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    // std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___
    // std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______
    // gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___
}

unsafe impl UpcastToNop<std__map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources_______> for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0___> for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {}

unsafe impl UpcastToNop<std___Tmap_traits_int_gfc__AutoRef_gfc__OverrideResources__std__less_int__std__allocator_std__pair_int_const__gfc__AutoRef_gfc__OverrideResources______0_> for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___
{
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
    // gfc__IRefObject
    pub vfptr: *const gfc__SoundDesc__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__SoundDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__SoundDesc {}

impl gfc__SoundDesc {
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
pub struct gfc__SoundDesc__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SoundDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__SoundDesc) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__SoundDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__SoundDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__SoundDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__SoundDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__SoundDesc) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__SoundDesc, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__SoundDesc,
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Resource__vftable,
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Resource {}

impl gfc__Resource {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn finalize(&self) {
        ((*self.vfptr).finalize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn unload(&self) {
        ((*self.vfptr).unload)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Resource__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Resource, _: u32) -> *mut (),
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub x: gfc__Matrix4Row,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub y: gfc__Matrix4Row,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub z: gfc__Matrix4Row,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub w: gfc__Matrix4Row,
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub m: compile_error!("unimplemented sizeof array"),
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ResourceCache__PackageInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
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
pub struct gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___
    // std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______
    // gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator_______> for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0___> for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__Vector_gfc__HString_0_gfc__CAllocator______0_> for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___
{
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub u: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub v: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
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
pub struct gfc__Vector_gfc__WorldManager___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__WorldManager,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__IRefObject {
    pub vfptr: *const gfc__IRefObject__vftable,
    pub ReferenceCount: i32,
}

impl gfc__IRefObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__IRefObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IRefObject, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Sample_ {
    pub p: *mut gfc__IRefObject,
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
pub struct gfc__Vector_gfc__ResourcePackageInfo___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ResourcePackageInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Object__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Object {}

impl gfc__Object {
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
pub struct gfc__Object__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Object, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__OutputStream__vftable,
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__OutputStream
    pub mEndianess: i32,
}

unsafe impl UpcastToNop<gfc__Stream> for gfc__OutputStream {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__OutputStream {}

impl gfc__OutputStream {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Stream__StreamType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn close(&self) {
        ((*self.vfptr).close)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn write(&self, a1: *const (), a2: i32) {
        ((*self.vfptr).write)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn flush(&self) {
        ((*self.vfptr).flush)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isSeekable(&self) -> bool {
        ((*self.vfptr).isSeekable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn seek(&self, a1: u64, a2: i32) {
        ((*self.vfptr).seek)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn tell(&self) -> i64 {
        ((*self.vfptr).tell)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__OutputStream__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: u32) -> *mut (),
    pub getType:
        unsafe extern "thiscall" fn(this: *const gfc__OutputStream) -> gfc__Stream__StreamType,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub write: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: *const (), _: i32),
    pub flush: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream),
    pub isSeekable: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__OutputStream) -> i64,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__OverrideResources_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Mutex {
    pub mMutex: keen__Mutex,
}

#[repr(C)]
pub struct gfc__Property {
    // gfc__IRefObject
    pub vfptr: *const gfc__Property__vftable,
    pub ReferenceCount: i32,
    // gfc__Property
    pub mName: gfc__HString,
    pub mAnnotation: gfc__HString,
    pub mContextFlags: u8,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Property {}

impl gfc__Property {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getTypeClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getTypeClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getElementType(&self) -> i32 {
        ((*self.vfptr).getElementType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isReadOnly(&self) -> bool {
        ((*self.vfptr).isReadOnly)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isTransient(&self) -> bool {
        ((*self.vfptr).isTransient)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isEditable(&self) -> bool {
        ((*self.vfptr).isEditable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setValue(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).setValue)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getValue(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getValue)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getElementCount(&self, a1: *mut gfc__Object) -> i32 {
        ((*self.vfptr).getElementCount)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn reserveElements(&self, a1: *mut gfc__Object, a2: i32) {
        ((*self.vfptr).reserveElements)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn addElement(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).addElement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn removeElement(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).removeElement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn containsElement(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) -> bool {
        ((*self.vfptr).containsElement)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setElementAt(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
        a3: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).setElementAt)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getElementAt(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: *mut gfc__Object,
        a3: gfc__AutoRef_gfc__Value_,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getElementAt)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn addElementAt(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
        a3: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).addElementAt)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn removeElementAt(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).removeElementAt)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn containsElementAt(
        &self,
        a1: *mut gfc__Object,
        a2: gfc__AutoRef_gfc__Value_,
    ) -> bool {
        ((*self.vfptr).containsElementAt)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn removeAllElements(&self, a1: *mut gfc__Object) {
        ((*self.vfptr).removeAllElements)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getKeys(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getKeys)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getValues(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
        a2: *mut gfc__Object,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getValues)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn clone(&self, a1: *mut gfc__Object, a2: *mut gfc__Object) {
        ((*self.vfptr).clone)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Property__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Property, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Stream__vftable,
    pub ReferenceCount: i32,
    // gfc__Stream
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Stream {}

impl gfc__Stream {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Stream__StreamType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Stream__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Stream, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Stream) -> gfc__Stream__StreamType,
}

#[repr(C)]
pub struct gfc__Vector_unsigned_long_0_gfc__CAllocator_ {
    pub mData: *mut u32,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ResourcePackageList_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ResourceManager {
    pub mPackageList: gfc__AutoRef_gfc__ResourcePackageList_,
    pub mCaches: gfc__Vector_gfc__ResourceCache___0_gfc__CAllocator_,
    pub mCachesByExt: gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___,
    pub mNodes: *mut gfc__LockFreeNode_gfc__Resource___,
    __pdbindgen_padding: [u8; 4],
    pub mAllocQueue: gfc__LockFreeQueue_gfc__Resource___,
    pub mPackageQueueLock: gfc__Mutex,
    pub mQueuedPackages: gfc__Vector_gfc__ResourcePackageInfo___0_gfc__CAllocator_,
    pub mPackageMarkerLock: gfc__Mutex,
    pub mPackageMarkers: gfc__Vector_gfc__AutoRef_gfc__PackageMarker__0_gfc__CAllocator_,
    pub mOverrideResources: gfc__Map_int_gfc__AutoRef_gfc__OverrideResources__std__less_int___,
    pub mListeners: gfc__Vector_gfc__ResourceListener___0_gfc__CAllocator_,
    pub mNeedsBufferProcessing: i32,
    pub mBufferPackage: *mut gfc__ResourcePackage,
    pub mWaitForBuffers: gfc__Event,
    pub mLoadThread: gfc__Thread,
    pub mHaveLoadTasks: gfc__Event,
    pub mQueuedLoads: i32,
    pub mQueueLocks: i32,
    pub mThreadMarkers: gfc__Vector_gfc__AutoRef_gfc__PackageMarker__0_gfc__CAllocator_,
    pub mUsedShaders: gfc__HashTable_gfc__HString_gfc__ResourceManager__ShaderState_gfc__Hash_unsigned___int64_gfc__HString__gfc__CAllocator_,
    pub mWorldManagerLock: gfc__Mutex,
    pub mWorldManagers: gfc__Vector_gfc__WorldManager___0_gfc__CAllocator_,
    pub mThreadExit: bool,
    pub mInitialized: bool,
    pub mTrackResources: bool,
    pub mForTool: bool,
    pub mLoadScripts: bool,
    pub mLoadedDefaultResources: bool,
    pub mReading: bool,
    pub mInsideDirtyLoad: bool,
    pub mCheckChunkAvailableArg: *mut (),
    pub mCheckChunkAvailableFunc: *mut unsafe extern "C" fn(_: *mut (), _: *const gfc__HString) -> bool,
    __pdbindgen_padding_2: [u8; 4],
}

#[repr(C)]
pub struct gfc__ResourceManager__ShaderState {
    pub Refs: u32,
    pub Loaded: bool,
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Class__vftable,
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

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Class {}

impl gfc__Class {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getTypeID(&self) -> i32 {
        ((*self.vfptr).getTypeID)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getName(&self) -> *const gfc__HString {
        ((*self.vfptr).getName)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getParent(&self) -> *mut gfc__Class {
        ((*self.vfptr).getParent)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn instanceof(&self, a1: *const gfc__Class) -> bool {
        ((*self.vfptr).instanceof)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isAbstract(&self) -> bool {
        ((*self.vfptr).isAbstract)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn newInstance(
        &self,
        result: *mut gfc__AutoRef_gfc__Object_,
    ) -> *mut gfc__AutoRef_gfc__Object_ {
        ((*self.vfptr).newInstance)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPropertyByName(
        &self,
        a1: *const gfc__HString,
        a2: *mut *const gfc__Class,
    ) -> *mut gfc__Property {
        ((*self.vfptr).getPropertyByName)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getPropertyByID(
        &self,
        a1: *const u64,
        a2: *mut *const gfc__Class,
    ) -> *mut gfc__Property {
        ((*self.vfptr).getPropertyByID)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMethodByName(
        &self,
        a1: *const gfc__HString,
    ) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByName)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMethodByID(&self, a1: *const u64) -> *mut gfc__Method {
        ((*self.vfptr).getMethodByID)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Class__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Class, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Environment__vftable,
    pub ReferenceCount: i32,
    // gfc__Environment
    pub mParent: gfc__AutoRef_gfc__Environment_,
    pub mSymbols: *mut gfc__HashTable_unsigned___int64_gfc__AutoRef_gfc__Variable__gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_,
    pub mContextFlags: u8,
    pub mHasRun: bool,
    pub mTempEnv: bool,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Environment {}

impl gfc__Environment {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Environment__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Environment, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Method__vftable,
    pub ReferenceCount: i32,
    // gfc__Method
    pub mName: gfc__HString,
    pub mContextFlags: u8,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Method {}

impl gfc__Method {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getParamCount(&self) -> i32 {
        ((*self.vfptr).getParamCount)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getParamTypeAt(&self, a1: i32) -> i32 {
        ((*self.vfptr).getParamTypeAt)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getReturnType(&self) -> i32 {
        ((*self.vfptr).getReturnType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getParamClassAt(&self, a1: i32) -> *mut gfc__Class {
        ((*self.vfptr).getParamClassAt)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getReturnClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getReturnClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isScriptMethod(&self) -> bool {
        ((*self.vfptr).isScriptMethod)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn execute(
        &self,
        a1: *mut gfc__Object,
        a2: *mut gfc__ValueStack,
        a3: *mut gfc__Environment,
    ) -> i32 {
        ((*self.vfptr).execute)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__Method__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Method, _: u32) -> *mut (),
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
pub struct gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___
    // std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______
    // gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache_______> for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0___> for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__ResourceCache___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__ResourceCache______0_> for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__ResourceCache___std__less_gfc__HString___
{
}

#[repr(C)]
pub struct gfc__ISynchronized {
    pub mLock: gfc__Lock,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourceBuffer_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ResourceBuffer,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ResourceIndex {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub Resources: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 216],
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__binary_function_gfc__Class___gfc__Class___bool_ {
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
    // std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
    /* std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ */
}

unsafe impl UpcastToNop<std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____> for std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {}

unsafe impl UpcastToNop<std___Container_base0> for std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

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
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____>
    for std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____
{
}

#[repr(C)]
pub struct std__less_gfc__AutoRef_gfc__Object___ {
    // std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_
    // std__less_gfc__AutoRef_gfc__Object___
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_>
    for std__less_gfc__AutoRef_gfc__Object___
{
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_ {}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_ {}

#[repr(C)]
pub struct std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___ {
    // std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
    pub first: gfc__AutoRef_gfc__Object_,
    pub second: gfc__AutoRef_gfc__Object_,
    // std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
}

unsafe impl
    UpcastToNop<std___Pair_base_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___>
    for std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object___
{
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

#[repr(C)]
pub struct std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______ {}

#[repr(C)]
pub struct std__binary_function_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__bool_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___>
    for std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter___
{
}

#[repr(C)]
pub struct std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    // std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

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
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_ {}

#[repr(C)]
pub struct std__allocator_gfc__AutoRef_gfc__ImageSurface___ {
    // std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___
    // std__allocator_gfc__AutoRef_gfc__ImageSurface___
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___>
    for std__allocator_gfc__AutoRef_gfc__ImageSurface___
{
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
    // std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____,
    // std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___ {}

#[repr(C)]
pub struct std__less_gfc__Class___ {
    // std__binary_function_gfc__Class___gfc__Class___bool_
    // std__less_gfc__Class___
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std__binary_function_gfc__Class___gfc__Class___bool_>
    for std__less_gfc__Class___
{
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
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_ {}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____ {
    // std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
    // std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<
        std___Allocator_base_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    > for std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____
{
}

#[repr(C)]
pub struct std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
{
    // std___Container_base0
    // std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____
    pub _Myfirst: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Mylast: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Myend: *mut gfc__AutoRef_gfc__ImageSurface_,
    pub _Alval: std__allocator_gfc__AutoRef_gfc__ImageSurface___,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Vector_val_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____ {}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__less_gfc__String_ {
    // std__binary_function_gfc__String_gfc__String_bool_
    // std__less_gfc__String_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std__binary_function_gfc__String_gfc__String_bool_>
    for std__less_gfc__String_
{
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    // std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

#[repr(C)]
pub struct std___Allocator_base_gfc__AutoRef_gfc__ImageSurface___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__binary_function_gfc__String_gfc__String_bool_ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
    // std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_____,
    // std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0______Node_ {}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Parameter___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Parameter_,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {
    // std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
    // std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____> for std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_____ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_
    pub comp: std__less_gfc__AutoRef_gfc__Object___,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_ {}

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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__HDRDesc__10_0_gfc__CAllocator_
{
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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__DepthOfFieldDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__DepthOfFieldDesc__10_0_gfc__CAllocator_
{
}

#[repr(C)]
pub struct gfc__StaticMesh {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticMesh__vftable,
    pub ReferenceCount: i32,
    /* gfc__Mesh
     * gfc__StaticMesh */
}

unsafe impl UpcastToNop<gfc__Mesh> for gfc__StaticMesh {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticMesh {}

impl gfc__StaticMesh {
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
pub struct gfc__StaticMesh__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__StaticMesh) -> gfc__Mesh__Type,
    pub isCompressed: unsafe extern "thiscall" fn(this: *const gfc__StaticMesh) -> bool,
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: i32) -> *mut gfc__Mesh__Group,
    pub getGroupCount: unsafe extern "thiscall" fn(this: *const gfc__StaticMesh) -> i32,
    pub getGroupIDAt: unsafe extern "thiscall" fn(this: *const gfc__StaticMesh, _: i32) -> i32,
    pub getGroupNameAt:
        unsafe extern "thiscall" fn(this: *const gfc__StaticMesh, _: i32) -> *const gfc__String,
    pub beginCreate:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *mut gfc__MeshBuilder),
    pub create: unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *mut gfc__MeshBuilder),
    pub endCreate: unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh),
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *const gfc__StaticMesh,
    ) -> *const gfc__TBox_float_gfc__FloatMath_,
    pub getVertexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__StaticMesh,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub getIndexBuffer: unsafe extern "thiscall" fn(
        this: *const gfc__StaticMesh,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub updateAddress:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMesh, _: *mut (), _: *mut ()),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__VertexBuffer__vftable,
    pub ReferenceCount: i32,
    // gfc__VertexBuffer
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__VertexBuffer {}

impl gfc__VertexBuffer {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn release(&self) {
        ((*self.vfptr).release)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSize(&self) -> u32 {
        ((*self.vfptr).getSize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVertexCount(&self) -> u32 {
        ((*self.vfptr).getVertexCount)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVertexStride(&self) -> u32 {
        ((*self.vfptr).getVertexStride)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVertexFormat(&self) -> *const gfc__VertexFormat {
        ((*self.vfptr).getVertexFormat)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getElementData(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: i32,
    ) -> *mut u8 {
        ((*self.vfptr).getElementData)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn lockall(&self, a1: i32) {
        ((*self.vfptr).lockall)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn finish(&self) {
        ((*self.vfptr).finish)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn write(&self, a1: gfc__AutoRef_gfc__OutputStream_) -> bool {
        ((*self.vfptr).write)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn read(&self, a1: gfc__AutoRef_gfc__InputStream_) -> bool {
        ((*self.vfptr).read)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateAddress(&self, a1: *mut (), a2: *mut ()) {
        ((*self.vfptr).updateAddress)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__VertexBuffer__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__VertexBuffer, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__IndexBuffer__vftable,
    pub ReferenceCount: i32,
    // gfc__IndexBuffer
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__IndexBuffer {}

impl gfc__IndexBuffer {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn lock(&self, a1: i32) {
        ((*self.vfptr).lock)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unlock(&self) {
        ((*self.vfptr).unlock)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn locked(&self) -> bool {
        ((*self.vfptr).locked)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getIndexCount(&self) -> i32 {
        ((*self.vfptr).getIndexCount)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getData(&self) -> *mut u8 {
        ((*self.vfptr).getData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getStride(&self) -> i32 {
        ((*self.vfptr).getStride)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateAddress(&self, a1: *mut (), a2: *mut ()) {
        ((*self.vfptr).updateAddress)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__IndexBuffer__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__IndexBuffer, _: u32) -> *mut (),
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub u: i32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub v: i32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
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
    pub vfptr: *const gfc__Hierarchical_gfc__Node3D___vftable,
    pub mParent: *mut gfc__Node3D,
    pub mHead: gfc__AutoRef_gfc__Node3D_,
    pub mTail: gfc__AutoRef_gfc__Node3D_,
    pub mNext: gfc__AutoRef_gfc__Node3D_,
    pub mPrev: gfc__AutoRef_gfc__Node3D_,
}

impl gfc__Hierarchical_gfc__Node3D_ {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addFront(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).addFront)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBack(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).addBack)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn add(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).add)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn remove(&self) {
        ((*self.vfptr).remove)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn remove_2(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).remove_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn added(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).added)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removed(&self, a1: *mut gfc__Node3D) {
        ((*self.vfptr).removed)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn invalidateHierarchy(&self) {
        ((*self.vfptr).invalidateHierarchy)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc__Node3D___vftable {
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
    // gfc__IRefObject
    pub vfptr: *const gfc__FogDesc__vftable,
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

unsafe impl UpcastToNop<gfc__EnvironmentDesc> for gfc__FogDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__FogDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__FogDesc {}

impl gfc__FogDesc {
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

    pub unsafe extern "thiscall" fn apply(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).apply)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn restore(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).restore)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__FogDesc__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__FogDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__FogDesc) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__FogDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__FogDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__FogDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__FogDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__FogDesc) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__FogDesc, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__FogDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__FogDesc, _: *mut gfc__Renderer),
    pub restore: unsafe extern "thiscall" fn(this: *mut gfc__FogDesc, _: *mut gfc__Renderer),
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
    pub vfptr: *const gfc__Graphics__vftable,
}

impl gfc__Graphics {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn init(&self, a1: u16, a2: u16, a3: u8, a4: bool) -> bool {
        ((*self.vfptr).init)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn isInitialized(&self) -> bool {
        ((*self.vfptr).isInitialized)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn release(&self) {
        ((*self.vfptr).release)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSceneBackBufferSize(&self, a1: *mut u16, a2: *mut u16) {
        ((*self.vfptr).getSceneBackBufferSize)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getScreenBackBufferSize(&self, a1: *mut u16, a2: *mut u16) {
        ((*self.vfptr).getScreenBackBufferSize)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getAspectRatio(&self) -> f32 {
        ((*self.vfptr).getAspectRatio)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn acquireThreadOwnership(&self) {
        ((*self.vfptr).acquireThreadOwnership)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn releaseThreadOwnership(&self) {
        ((*self.vfptr).releaseThreadOwnership)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn update(&self) -> bool {
        ((*self.vfptr).update)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn begin3D(&self) -> bool {
        ((*self.vfptr).begin3D)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn end3D(&self) {
        ((*self.vfptr).end3D)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getWaitForVSync(&self) -> bool {
        ((*self.vfptr).getWaitForVSync)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setWaitForVSync(&self, a1: bool) {
        ((*self.vfptr).setWaitForVSync)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unsetSamplers(&self) {
        ((*self.vfptr).unsetSamplers)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getWorldMatrix(&self, a1: *mut gfc__Matrix4) {
        ((*self.vfptr).getWorldMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setWorldMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setWorldMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn commitWorldMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).commitWorldMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getViewMatrix(&self) -> *const gfc__Matrix4 {
        ((*self.vfptr).getViewMatrix)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setViewMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setViewMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getProjectionMatrix(&self) -> *const gfc__Matrix4 {
        ((*self.vfptr).getProjectionMatrix)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setProjectionMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setProjectionMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getViewProjMatrix(&self) -> *const gfc__Matrix4 {
        ((*self.vfptr).getViewProjMatrix)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setViewport(&self, a1: *const gfc__Viewport) {
        ((*self.vfptr).setViewport)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getViewport(
        &self,
        a1: *mut i32,
        a2: *mut i32,
        a3: *mut i32,
        a4: *mut i32,
    ) {
        ((*self.vfptr).getViewport)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn clear(
        &self,
        a1: bool,
        a2: bool,
        a3: bool,
        a4: u32,
        a5: f32,
        a6: u32,
    ) {
        ((*self.vfptr).clear)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn clear_2(&self, a1: *const gfc__TVector3_float_gfc__FloatMath_) {
        ((*self.vfptr).clear_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn pushClip(&self, a1: i32) {
        ((*self.vfptr).pushClip)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn popClip(&self, a1: i32) {
        ((*self.vfptr).popClip)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn enableClip(&self, a1: i32) {
        ((*self.vfptr).enableClip)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn disableClip(&self) {
        ((*self.vfptr).disableClip)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDepthTestMode(&self, a1: i32) {
        ((*self.vfptr).setDepthTestMode)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setCullMode(&self, a1: i32) {
        ((*self.vfptr).setCullMode)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setBlendMode(&self, a1: i32) {
        ((*self.vfptr).setBlendMode)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getBlendMode(&self) -> i32 {
        ((*self.vfptr).getBlendMode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setBlendFactor(
        &self,
        a1: *const gfc__TVector4_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setBlendFactor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setGamma(&self, a1: f32) {
        ((*self.vfptr).setGamma)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPostProcessEffect(&self, a1: u32) {
        ((*self.vfptr).setPostProcessEffect)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setAntiAliasingType(&self, a1: u32) {
        ((*self.vfptr).setAntiAliasingType)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setTextureFilteringMode(&self, a1: u32) {
        ((*self.vfptr).setTextureFilteringMode)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setShader(&self, a1: *mut gfc__Shader) {
        ((*self.vfptr).setShader)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getShader(&self) -> *mut gfc__Shader {
        ((*self.vfptr).getShader)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setVertexDeclaration(&self, a1: *mut gfc__VertexDeclaration) {
        ((*self.vfptr).setVertexDeclaration)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setIndexBuffer(&self, a1: *mut gfc__IndexBuffer) {
        ((*self.vfptr).setIndexBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setVertexBuffer(&self, a1: *mut gfc__VertexBuffer) {
        ((*self.vfptr).setVertexBuffer)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setVertexBufferMasked(
        &self,
        a1: *mut gfc__VertexBuffer,
        a2: u32,
    ) {
        ((*self.vfptr).setVertexBufferMasked)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn drawFullScreenQuad(&self) {
        ((*self.vfptr).drawFullScreenQuad)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn drawRect(&self, a1: i32, a2: i32, a3: i32, a4: i32) {
        ((*self.vfptr).drawRect)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn drawPrimitiveUP(
        &self,
        a1: u32,
        a2: u32,
        a3: *const (),
        a4: u32,
    ) {
        ((*self.vfptr).drawPrimitiveUP)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn drawPrimitiveRetained(
        &self,
        a1: u32,
        a2: u32,
        a3: *const (),
        a4: u32,
    ) {
        ((*self.vfptr).drawPrimitiveRetained)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn drawIndexedPrimitiveUP(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: *const (),
        a5: *const (),
        a6: u32,
    ) {
        ((*self.vfptr).drawIndexedPrimitiveUP)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn drawPrimitives(&self, a1: u32, a2: u32, a3: u32) {
        ((*self.vfptr).drawPrimitives)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn drawPrimitivesIndexed(
        &self,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
    ) {
        ((*self.vfptr).drawPrimitivesIndexed)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn setRenderTarget(
        &self,
        a1: *mut gfc__Texture,
        a2: *mut gfc__Texture,
        a3: i32,
    ) {
        ((*self.vfptr).setRenderTarget)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn setDefaultRenderTarget(&self) {
        ((*self.vfptr).setDefaultRenderTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDefaultDepthStencilTarget(&self) {
        ((*self.vfptr).setDefaultDepthStencilTarget)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn createVertexDeclaration(
        &self,
        result: *mut gfc__AutoRef_gfc__VertexDeclaration_,
        a2: *const gfc__VertexFormat,
    ) -> *mut gfc__AutoRef_gfc__VertexDeclaration_ {
        ((*self.vfptr).createVertexDeclaration)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn createVertexBuffer(
        &self,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
        a2: i32,
        a3: u32,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_ {
        ((*self.vfptr).createVertexBuffer)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn createVertexBuffer_2(
        &self,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    ) -> *mut gfc__AutoRef_gfc__VertexBuffer_ {
        ((*self.vfptr).createVertexBuffer_2)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createIndexBuffer(
        &self,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
        a2: u32,
        a3: bool,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_ {
        ((*self.vfptr).createIndexBuffer)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn createMeshBuilder(
        &self,
        result: *mut gfc__AutoRef_gfc__MeshBuilder_,
    ) -> *mut gfc__AutoRef_gfc__MeshBuilder_ {
        ((*self.vfptr).createMeshBuilder)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createStaticMesh(
        &self,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
        a2: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_ {
        ((*self.vfptr).createStaticMesh)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn createStaticMesh_2(
        &self,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_ {
        ((*self.vfptr).createStaticMesh_2)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createSkinMesh(
        &self,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
        a2: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_ {
        ((*self.vfptr).createSkinMesh)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn createSkinMesh_2(
        &self,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_ {
        ((*self.vfptr).createSkinMesh_2)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createRenderTexture(
        &self,
        result: *mut gfc__AutoRef_gfc__RenderTexture_,
        a2: u16,
        a3: u16,
        a4: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__RenderTexture_ {
        ((*self.vfptr).createRenderTexture)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createRenderTarget(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createRenderTarget)(self as *const _ as *mut _, result, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn createRenderTarget_2(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: u16,
        a3: u16,
        a4: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createRenderTarget_2)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createRenderDepth(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: u16,
        a3: u16,
        a4: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createRenderDepth)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createRenderCubemap(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: u16,
        a3: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createRenderCubemap)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn createTexture(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: *mut (),
        a3: u32,
        a4: bool,
        a5: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createTexture)(self as *const _ as *mut _, result, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn createTexture_2(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: *const gfc__String,
        a3: bool,
        a4: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createTexture_2)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createTexture_3(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: *mut gfc__Image,
        a3: bool,
        a4: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createTexture_3)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createTexture_4(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: i32,
        a3: i32,
        a4: gfc__ImageFormat,
        a5: bool,
        a6: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createTexture_4)(self as *const _ as *mut _, result, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn createCubemap(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: i32,
        a3: gfc__ImageFormat,
        a4: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createCubemap)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createLightmap(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: i32,
        a3: i32,
        a4: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createLightmap)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn createShader(
        &self,
        result: *mut gfc__AutoRef_gfc__Shader_,
    ) -> *mut gfc__AutoRef_gfc__Shader_ {
        ((*self.vfptr).createShader)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createShaderCompiler(
        &self,
        result: *mut gfc__AutoRef_gfc__ShaderCompiler_,
    ) -> *mut gfc__AutoRef_gfc__ShaderCompiler_ {
        ((*self.vfptr).createShaderCompiler)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn createQuery(
        &self,
        result: *mut gfc__AutoRef_gfc__Query_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Query_ {
        ((*self.vfptr).createQuery)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn createCamera(
        &self,
        result: *mut gfc__AutoRef_gfc__Camera3D_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Camera3D_ {
        ((*self.vfptr).createCamera)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn createRenderer(&self) -> *mut gfc__Renderer {
        ((*self.vfptr).createRenderer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getPlatform(&self) -> i32 {
        ((*self.vfptr).getPlatform)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDepthBias(&self) -> f32 {
        ((*self.vfptr).getDepthBias)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDecalDepthBias(&self) -> f32 {
        ((*self.vfptr).getDecalDepthBias)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn recover(&self) -> bool {
        ((*self.vfptr).recover)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isMeshFormatSupported(&self, a1: i32) -> bool {
        ((*self.vfptr).isMeshFormatSupported)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn blockUntilDefragged(&self) {
        ((*self.vfptr).blockUntilDefragged)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Graphics__vftable {
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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__AmbientDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__AmbientDesc__10_0_gfc__CAllocator_
{
}

#[repr(C)]
pub struct gfc__ThreadSafeVector_gfc__LightNode___ {
    pub m_vector: gfc__Vector_gfc__LightNode___0_gfc__CAllocator_,
    pub m_mutex: gfc__Mutex,
}

#[repr(C)]
pub struct gfc__Matrix3 {
    pub xx: f32,
    pub xy: f32,
    pub xz: f32,
    pub yx: f32,
    pub yy: f32,
    pub yz: f32,
    pub zx: f32,
    pub zy: f32,
    pub zz: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub x: gfc__Matrix3Row,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub y: gfc__Matrix3Row,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub z: gfc__Matrix3Row,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__AmbientDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Camera3D {
    // gfc__IRefObject
    pub vfptr: *const gfc__Camera3D__vftable,
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

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Camera3D {}

impl gfc__Camera3D {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn sortRenderNodes(&self) {
        ((*self.vfptr).sortRenderNodes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setLightInfluences(&self) {
        ((*self.vfptr).setLightInfluences)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeMatrices(&self, a1: *const gfc__Matrix4, a2: bool) {
        ((*self.vfptr).computeMatrices)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn computeLightVis(&self) {
        ((*self.vfptr).computeLightVis)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn startGeoPrep(&self) {
        ((*self.vfptr).startGeoPrep)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn computeGeoPrep(&self) {
        ((*self.vfptr).computeGeoPrep)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn finishGeoPrep(&self) {
        ((*self.vfptr).finishGeoPrep)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Camera3D__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Camera3D, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__DepthOfFieldDesc__vftable,
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

unsafe impl UpcastToNop<gfc__EnvironmentDesc> for gfc__DepthOfFieldDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__DepthOfFieldDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__DepthOfFieldDesc {}

impl gfc__DepthOfFieldDesc {
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

    pub unsafe extern "thiscall" fn apply(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).apply)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn restore(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).restore)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__DepthOfFieldDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__DepthOfFieldDesc) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__DepthOfFieldDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__DepthOfFieldDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__DepthOfFieldDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DepthOfFieldDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__DepthOfFieldDesc, _: *mut gfc__Renderer),
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
    pub vfptr: *const gfc__Renderer__vftable,
}

impl gfc__Renderer {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn init(&self, a1: u16, a2: u16) {
        ((*self.vfptr).init)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn reinit(&self) {
        ((*self.vfptr).reinit)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn reset(&self) {
        ((*self.vfptr).reset)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadDefaultResources(&self) {
        ((*self.vfptr).loadDefaultResources)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn releaseDefaultResources(&self) {
        ((*self.vfptr).releaseDefaultResources)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setBackgroundColor(
        &self,
        a1: *const gfc__TVector4_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setBackgroundColor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFrameBufferAlpha(&self, a1: f32) {
        ((*self.vfptr).setFrameBufferAlpha)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFogDesc(&self, a1: *const gfc__FogDesc, a2: bool) {
        ((*self.vfptr).setFogDesc)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getFogDesc(&self) -> *const gfc__FogDesc {
        ((*self.vfptr).getFogDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAmbientDesc(&self, a1: *const gfc__AmbientDesc, a2: bool) {
        ((*self.vfptr).setAmbientDesc)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getAmbientDesc(&self) -> *const gfc__AmbientDesc {
        ((*self.vfptr).getAmbientDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setCameraBlurDesc(
        &self,
        a1: *const gfc__CameraBlurDesc,
        a2: bool,
    ) {
        ((*self.vfptr).setCameraBlurDesc)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getCameraBlurDesc(&self) -> *const gfc__CameraBlurDesc {
        ((*self.vfptr).getCameraBlurDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDepthOfFieldDesc(
        &self,
        a1: *const gfc__DepthOfFieldDesc,
        a2: bool,
    ) {
        ((*self.vfptr).setDepthOfFieldDesc)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getDepthOfFieldDesc(&self) -> *const gfc__DepthOfFieldDesc {
        ((*self.vfptr).getDepthOfFieldDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHDRDesc(&self, a1: *const gfc__HDRDesc, a2: bool) {
        ((*self.vfptr).setHDRDesc)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getHDRDesc(&self) -> *const gfc__HDRDesc {
        ((*self.vfptr).getHDRDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHDRRenderer(&self) -> *mut gfc__HDRBase {
        ((*self.vfptr).getHDRRenderer)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getViewport(&self) -> *const gfc__Viewport {
        ((*self.vfptr).getViewport)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setViewport(&self, a1: *const gfc__Viewport) {
        ((*self.vfptr).setViewport)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setNearDistance(&self, a1: f32) {
        ((*self.vfptr).setNearDistance)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFarDistance(&self, a1: f32) {
        ((*self.vfptr).setFarDistance)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getViewMatrix(&self) -> *const gfc__Matrix4 {
        ((*self.vfptr).getViewMatrix)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setViewMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setViewMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getProjectionMatrix(&self) -> *const gfc__Matrix4 {
        ((*self.vfptr).getProjectionMatrix)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setProjectionMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setProjectionMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDoLighting(&self, a1: bool) {
        ((*self.vfptr).setDoLighting)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoLighting(&self) -> bool {
        ((*self.vfptr).getDoLighting)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDoShadows(&self, a1: bool) {
        ((*self.vfptr).setDoShadows)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoShadows(&self) -> bool {
        ((*self.vfptr).getDoShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDoFogging(&self, a1: bool) {
        ((*self.vfptr).setDoFogging)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoFogging(&self) -> bool {
        ((*self.vfptr).getDoFogging)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDoPostRender(&self, a1: bool) {
        ((*self.vfptr).setDoPostRender)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoPostRender(&self) -> bool {
        ((*self.vfptr).getDoPostRender)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDoDepthOfField(&self, a1: bool) {
        ((*self.vfptr).setDoDepthOfField)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoDepthOfField(&self) -> bool {
        ((*self.vfptr).getDoDepthOfField)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setBlendEnvironment(&self, a1: bool) {
        ((*self.vfptr).setBlendEnvironment)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getBlendEnvironment(&self) -> bool {
        ((*self.vfptr).getBlendEnvironment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setBlendFocusDistance(&self, a1: bool) {
        ((*self.vfptr).setBlendFocusDistance)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getBlendFocusDistance(&self) -> bool {
        ((*self.vfptr).getBlendFocusDistance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn forceDynamicLights(&self, a1: bool) {
        ((*self.vfptr).forceDynamicLights)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn forceRenderAllLights(&self, a1: bool) {
        ((*self.vfptr).forceRenderAllLights)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDoZPass(&self, a1: bool) {
        ((*self.vfptr).setDoZPass)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setSkyAmbientDesc(&self, a1: *const gfc__AmbientDesc) {
        ((*self.vfptr).setSkyAmbientDesc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSkyAmbientDesc(&self) -> *const gfc__AmbientDesc {
        ((*self.vfptr).getSkyAmbientDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setSkyFogDesc(&self, a1: *const gfc__FogDesc) {
        ((*self.vfptr).setSkyFogDesc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSkyFogDesc(&self) -> *const gfc__FogDesc {
        ((*self.vfptr).getSkyFogDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addFullScreenEffect(
        &self,
        a1: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ) {
        ((*self.vfptr).addFullScreenEffect)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFullScreenEffect(
        &self,
        a1: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ) {
        ((*self.vfptr).removeFullScreenEffect)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeAllFullScreenEffects(&self) {
        ((*self.vfptr).removeAllFullScreenEffects)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn hasFullScreenEffect(
        &self,
        a1: gfc__AutoRef_gfc__FullScreenEffectDesc_,
    ) -> bool {
        ((*self.vfptr).hasFullScreenEffect)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupFrame(&self, a1: *mut gfc__Camera3D, a2: f32) {
        ((*self.vfptr).setupFrame)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn render(&self) {
        ((*self.vfptr).render)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupView(&self) {
        ((*self.vfptr).setupView)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn renderShadows(&self) {
        ((*self.vfptr).renderShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn renderFrame(&self) {
        ((*self.vfptr).renderFrame)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Renderer__vftable {
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
    // gfc__IRefObject
    pub vfptr: *const gfc__CameraBlurDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__CameraBlurDesc
    pub mScale: f32,
    pub mSampleCount: u32,
    pub mManager: *mut gfc__EnvironmentManager,
}

unsafe impl UpcastToNop<gfc__EnvironmentDesc> for gfc__CameraBlurDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CameraBlurDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CameraBlurDesc {}

impl gfc__CameraBlurDesc {
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

    pub unsafe extern "thiscall" fn apply(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).apply)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn restore(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).restore)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__CameraBlurDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CameraBlurDesc) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CameraBlurDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CameraBlurDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CameraBlurDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CameraBlurDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc, _: *mut gfc__Renderer),
    pub restore: unsafe extern "thiscall" fn(this: *mut gfc__CameraBlurDesc, _: *mut gfc__Renderer),
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
pub struct gfc__Matrix3Row {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct gfc__Shader {
    // gfc__IRefObject
    pub vfptr: *const gfc__Shader__vftable,
    pub ReferenceCount: i32,
    // gfc__Shader
    pub mName: gfc__HString,
    pub mLocked: bool,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Shader {}

impl gfc__Shader {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn createFromDesc(
        &self,
        a1: gfc__AutoRef_gfc__ShaderDesc_,
    ) -> bool {
        ((*self.vfptr).createFromDesc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getVersion(&self) -> u32 {
        ((*self.vfptr).getVersion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getLightingModel(&self) -> u8 {
        ((*self.vfptr).getLightingModel)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBlendMode(&self) -> u8 {
        ((*self.vfptr).getBlendMode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCullMode(&self) -> u8 {
        ((*self.vfptr).getCullMode)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSize(&self) -> u32 {
        ((*self.vfptr).getSize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDesc(
        &self,
        result: *mut gfc__AutoRef_gfc__ShaderDesc_,
    ) -> *mut gfc__AutoRef_gfc__ShaderDesc_ {
        ((*self.vfptr).getDesc)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn isUsed(&self) -> bool {
        ((*self.vfptr).isUsed)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn hasPass(&self, a1: i32) -> bool {
        ((*self.vfptr).hasPass)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn beginPass(&self, a1: i32, a2: i32) {
        ((*self.vfptr).beginPass)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn endPass(&self) {
        ((*self.vfptr).endPass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setWorldMatrix(&self, a1: *const gfc__Matrix4) {
        ((*self.vfptr).setWorldMatrix)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDefaultValues(&self) {
        ((*self.vfptr).setDefaultValues)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setMaterialParams(&self, a1: *mut gfc__Material, a2: f32) {
        ((*self.vfptr).setMaterialParams)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getParameterHandle(&self, a1: *const i8, a2: i32) -> u32 {
        ((*self.vfptr).getParameterHandle)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getParameterHandle_2(&self, a1: *const i8) -> u32 {
        ((*self.vfptr).getParameterHandle_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setConstant1f(&self, a1: *const i8, a2: f32) {
        ((*self.vfptr).setConstant1f)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setConstant1f_2(&self, a1: u32, a2: f32) {
        ((*self.vfptr).setConstant1f_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setConstant4f(
        &self,
        a1: *const i8,
        a2: *const gfc__TVector4_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setConstant4f)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setConstant4f_2(
        &self,
        a1: u32,
        a2: *const gfc__TVector4_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setConstant4f_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setConstantArray1f(
        &self,
        a1: *const i8,
        a2: *const f32,
        a3: u32,
    ) {
        ((*self.vfptr).setConstantArray1f)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn setConstantArray1f_2(&self, a1: u32, a2: *const f32, a3: u32) {
        ((*self.vfptr).setConstantArray1f_2)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn setMatrix(&self, a1: *const i8, a2: *const gfc__Matrix4) {
        ((*self.vfptr).setMatrix)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setMatrix_2(&self, a1: u32, a2: *const gfc__Matrix4) {
        ((*self.vfptr).setMatrix_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setTexture(&self, a1: *const i8, a2: *const gfc__Texture) {
        ((*self.vfptr).setTexture)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setTexture_2(&self, a1: u32, a2: *const gfc__Texture) {
        ((*self.vfptr).setTexture_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setSamplerState(&self, a1: *const i8, a2: u32, a3: u32) {
        ((*self.vfptr).setSamplerState)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn setSamplerState_2(&self, a1: u32, a2: u32, a3: u32) {
        ((*self.vfptr).setSamplerState_2)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct gfc__Shader__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Shader, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__WorldObject__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__WorldObject {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldObject {}

impl gfc__WorldObject {
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
pub struct gfc__WorldObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WorldObject) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldObject, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldObject,
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
    pub vfptr: *const gfc__IRenderCallback__vftable,
    pub mLocked: bool,
}

impl gfc__IRenderCallback {
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
}

#[repr(C)]
pub struct gfc__IRenderCallback__vftable {
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
    pub vfptr: *const gfc__HDRBase__vftable,
    pub mDesc: gfc__HDRDesc,
}

impl gfc__HDRBase {
    pub unsafe extern "thiscall" fn setDesc(&self, a1: *const gfc__HDRDesc) {
        ((*self.vfptr).setDesc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDesc(&self) -> *const gfc__HDRDesc {
        ((*self.vfptr).getDesc)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initializeLuminance(&self) {
        ((*self.vfptr).initializeLuminance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn clearBloom(&self) {
        ((*self.vfptr).clearBloom)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__HDRBase__vftable {
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
    // std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_
    pub comp: std__less_gfc__Class___,
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

unsafe impl UpcastToNop<std__map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent_______> for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0___> for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class____std__allocator_std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent______0_> for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__Class___gfc__AutoRef_gfc__WorldComponent__std__less_gfc__Class_____
{
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
    #[cfg(pdb_issue = "unimplemented sizeof array")]
    pub Elements: compile_error!("unimplemented sizeof array"),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Image__vftable,
    pub ReferenceCount: i32,
    // gfc__Image
    pub mType: gfc__ImageType,
    pub mFormat: gfc__ImageFormat,
    pub mSurfaces: std__vector_gfc__AutoRef_gfc__ImageSurface__std__allocator_gfc__AutoRef_gfc__ImageSurface_____,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Image {}

impl gfc__Image {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__Image__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Image, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__World__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__World {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__World {}

impl gfc__World {
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
pub struct gfc__World__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__World, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__World) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__World, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__World) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__World) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__World,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__World) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__World, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__World,
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
    // gfc__IRefObject
    pub vfptr: *const gfc__AmbientDesc__vftable,
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

unsafe impl UpcastToNop<gfc__EnvironmentDesc> for gfc__AmbientDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__AmbientDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__AmbientDesc {}

impl gfc__AmbientDesc {
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

    pub unsafe extern "thiscall" fn apply(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).apply)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn restore(&self, a1: *mut gfc__Renderer) {
        ((*self.vfptr).restore)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__AmbientDesc__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__AmbientDesc) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__AmbientDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__AmbientDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__AmbientDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc, _: *mut gfc__Renderer),
    pub restore: unsafe extern "thiscall" fn(this: *mut gfc__AmbientDesc, _: *mut gfc__Renderer),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Texture__vftable,
    pub ReferenceCount: i32,
    // gfc__Texture
    pub mName: gfc__HString,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Texture {}

impl gfc__Texture {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Texture__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFormat(&self) -> gfc__ImageFormat {
        ((*self.vfptr).getFormat)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getWidth(&self) -> u16 {
        ((*self.vfptr).getWidth)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getHeight(&self) -> u16 {
        ((*self.vfptr).getHeight)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getDepth(&self) -> u16 {
        ((*self.vfptr).getDepth)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSize(&self) -> u32 {
        ((*self.vfptr).getSize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateAddress(&self, a1: *mut (), a2: *mut ()) {
        ((*self.vfptr).updateAddress)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__Texture__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Texture, _: u32) -> *mut (),
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
pub struct gfc__MatrixArray {
    pub mCount: u32,
    pub mPad: [u32; 3],
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub mFirstMatrix: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 64],
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Animation_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldRegionData {
    // gfc__IRefObject
    pub vfptr: *const gfc__WorldRegionData__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__WorldRegionData
    pub mID: i32,
    pub mName: gfc__HString,
    pub mRegion: gfc__AutoRef_gfc__WorldRegion_,
    pub mLayers: gfc__Vector_gfc__AutoRef_gfc__RegionLayerData__0_gfc__CAllocator_,
    pub mCanLoad: bool,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__WorldRegionData {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldRegionData {}

impl gfc__WorldRegionData {
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
pub struct gfc__WorldRegionData__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldRegionData, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldRegionData) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldRegionData, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WorldRegionData) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegionData) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldRegionData,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldRegionData) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldRegionData,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldRegionData,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VertexBuffer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Frustum {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub planes: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 96],
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__CameraBlurDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__ByteOutputStream {
    // gfc__IRefObject
    pub vfptr: *const gfc__ByteOutputStream__vftable,
    pub ReferenceCount: i32,
    // gfc__Stream
    // gfc__OutputStream
    pub mEndianess: i32,
    // gfc__ByteOutputStream
    pub mPosition: i32,
    pub mOutput: gfc__Vector_unsigned_char_0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__OutputStream> for gfc__ByteOutputStream {}

unsafe impl UpcastToNop<gfc__Stream> for gfc__ByteOutputStream {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ByteOutputStream {}

impl gfc__ByteOutputStream {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> gfc__Stream__StreamType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn close(&self) {
        ((*self.vfptr).close)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn write(&self, a1: *const (), a2: i32) {
        ((*self.vfptr).write)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn flush(&self) {
        ((*self.vfptr).flush)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isSeekable(&self) -> bool {
        ((*self.vfptr).isSeekable)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn seek(&self, a1: u64, a2: i32) {
        ((*self.vfptr).seek)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn tell(&self) -> i64 {
        ((*self.vfptr).tell)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__ByteOutputStream__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream, _: u32) -> *mut (),
    pub getType:
        unsafe extern "thiscall" fn(this: *const gfc__ByteOutputStream) -> gfc__Stream__StreamType,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream),
    pub write: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream, _: *const (), _: i32),
    pub flush: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream),
    pub isSeekable: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__ByteOutputStream) -> i64,
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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__FullScreenEffectDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__FullScreenEffectDesc__10_0_gfc__CAllocator_
{
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Mesh__vftable,
    pub ReferenceCount: i32,
    // gfc__Mesh
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Mesh {}

impl gfc__Mesh {
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
}

#[repr(C)]
pub struct gfc__Mesh__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Mesh, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__WorldRegion__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__WorldRegion {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldRegion {}

impl gfc__WorldRegion {
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
pub struct gfc__WorldRegion__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldRegion) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WorldRegion) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldRegion,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldRegion, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldRegion,
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
pub struct gfc__Parameter {
    // gfc__IRefObject
    pub vfptr: *const gfc__Parameter__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Parameter
    pub mName: gfc__HString,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Parameter {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Parameter {}

impl gfc__Parameter {
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

    pub unsafe extern "thiscall" fn setBool(&self, a1: bool, a2: f32) {
        ((*self.vfptr).setBool)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getBool(&self, a1: f32) -> bool {
        ((*self.vfptr).getBool)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setFloat(&self, a1: f32, a2: f32) {
        ((*self.vfptr).setFloat)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getFloat(&self, a1: f32) -> f32 {
        ((*self.vfptr).getFloat)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setUInt32(&self, a1: u32, a2: f32) {
        ((*self.vfptr).setUInt32)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getUInt32(&self, a1: f32) -> u32 {
        ((*self.vfptr).getUInt32)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setVector2(
        &self,
        a1: *const gfc__TVector2_float_gfc__FloatMath_,
        a2: f32,
    ) {
        ((*self.vfptr).setVector2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getVector2(
        &self,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        a2: f32,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_ {
        ((*self.vfptr).getVector2)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setVector3(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
        a2: f32,
    ) {
        ((*self.vfptr).setVector3)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getVector3(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        a2: f32,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getVector3)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setVector4(
        &self,
        a1: *const gfc__TVector4_float_gfc__FloatMath_,
        a2: f32,
    ) {
        ((*self.vfptr).setVector4)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getVector4(
        &self,
        result: *mut gfc__TVector4_float_gfc__FloatMath_,
        a2: f32,
    ) -> *mut gfc__TVector4_float_gfc__FloatMath_ {
        ((*self.vfptr).getVector4)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setMatrix4(&self, a1: *const gfc__Matrix4, a2: f32) {
        ((*self.vfptr).setMatrix4)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMatrix4(
        &self,
        result: *mut gfc__Matrix4,
        a2: f32,
    ) -> *mut gfc__Matrix4 {
        ((*self.vfptr).getMatrix4)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setTexture(&self, a1: *mut gfc__Texture, a2: f32) {
        ((*self.vfptr).setTexture)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getTexture(&self, a1: f32) -> *mut gfc__Texture {
        ((*self.vfptr).getTexture)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn preload(&self, a1: i32) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn clone(&self) -> *mut gfc__Parameter {
        ((*self.vfptr).clone)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Parameter__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Parameter) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Parameter) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Parameter) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Parameter,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Parameter) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Parameter,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Parameter) -> i32,
    pub setBool: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: bool, _: f32),
    pub getBool: unsafe extern "thiscall" fn(this: *const gfc__Parameter, _: f32) -> bool,
    pub setFloat: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: f32, _: f32),
    pub getFloat: unsafe extern "thiscall" fn(this: *const gfc__Parameter, _: f32) -> f32,
    pub setUInt32: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: u32, _: f32),
    pub getUInt32: unsafe extern "thiscall" fn(this: *const gfc__Parameter, _: f32) -> u32,
    pub setVector2: unsafe extern "thiscall" fn(
        this: *mut gfc__Parameter,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector2: unsafe extern "thiscall" fn(
        this: *const gfc__Parameter,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setVector3: unsafe extern "thiscall" fn(
        this: *mut gfc__Parameter,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector3: unsafe extern "thiscall" fn(
        this: *const gfc__Parameter,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setVector4: unsafe extern "thiscall" fn(
        this: *mut gfc__Parameter,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector4: unsafe extern "thiscall" fn(
        this: *const gfc__Parameter,
        result: *mut gfc__TVector4_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector4_float_gfc__FloatMath_,
    pub setMatrix4:
        unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: *const gfc__Matrix4, _: f32),
    pub getMatrix4: unsafe extern "thiscall" fn(
        this: *const gfc__Parameter,
        result: *mut gfc__Matrix4,
        _: f32,
    ) -> *mut gfc__Matrix4,
    pub setTexture:
        unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: *mut gfc__Texture, _: f32),
    pub getTexture:
        unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: f32) -> *mut gfc__Texture,
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Parameter, _: i32),
    pub clone: unsafe extern "thiscall" fn(this: *const gfc__Parameter) -> *mut gfc__Parameter,
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub spotOffsets: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 256],
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__HDRDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__HDRDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__RegionLayerData {
    // gfc__IRefObject
    pub vfptr: *const gfc__RegionLayerData__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__RegionLayerData
    pub mID: i32,
    pub mName: gfc__HString,
    pub mBoundingBox: gfc__TBox_float_gfc__FloatMath_,
    pub mPackageIDs: gfc__Vector_int_0_gfc__CAllocator_,
    pub mLayer: gfc__AutoRef_gfc__RegionLayer_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__RegionLayerData {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__RegionLayerData {}

impl gfc__RegionLayerData {
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
pub struct gfc__RegionLayerData__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__RegionLayerData, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__RegionLayerData) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__RegionLayerData, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__RegionLayerData) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__RegionLayerData) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__RegionLayerData,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__RegionLayerData) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__RegionLayerData,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__RegionLayerData,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__ObjectReader__vftable,
    pub ReferenceCount: i32,
    // gfc__ObjectReader
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ObjectReader {}

impl gfc__ObjectReader {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn readObject(
        &self,
        result: *mut gfc__AutoRef_gfc__Object_,
        a2: gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__Object_ {
        ((*self.vfptr).readObject)(self as *const _ as *mut _, result, a2)
    }
}

#[repr(C)]
pub struct gfc__ObjectReader__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ObjectReader, _: u32) -> *mut (),
    pub readObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ObjectReader,
        result: *mut gfc__AutoRef_gfc__Object_,
        _: gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct gfc__MaterialCache {
    // gfc__ResourceCache
    pub vfptr: *const gfc__MaterialCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__MaterialCache
    pub mErrorMaterial: gfc__AutoRef_gfc__Material_,
    pub mReleaseQueue: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__ResourceCache> for gfc__MaterialCache {}

impl gfc__MaterialCache {
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
pub struct gfc__MaterialCache__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MaterialCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__MaterialCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__MaterialCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__MaterialCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__MaterialCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MaterialCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__MaterialCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MaterialCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__MaterialCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MaterialCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MaterialCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FogDesc_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__MeshInstance {
    // gfc__IRefObject
    pub vfptr: *const gfc__MeshInstance__vftable,
    pub ReferenceCount: i32,
    // gfc__MeshInstance
    pub mLocked: bool,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MeshInstance {}

impl gfc__MeshInstance {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn lockMesh(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).lockMesh)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn compute(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).compute)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unlockMesh(&self, a1: *mut gfc__RenderNode) {
        ((*self.vfptr).unlockMesh)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__MeshInstance__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: u32) -> *mut (),
    pub lockMesh:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
    pub compute: unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
    pub unlockMesh:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshInstance, _: *mut gfc__RenderNode),
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
