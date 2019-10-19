#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types::*, types2::*, types4::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

#[repr(C)]
pub struct gfc__AutoRef_gfc__Visual_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__WorldLight {
    pub vfptr: *const gfc__WorldLight__vftable,
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
    // gfc__WorldLight
    pub mObject: gfc__AutoRef_gfc__Object3D_,
    pub mEnabled: bool,
    pub mJitterAmount: f32,
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__WorldLight {}

unsafe impl UpcastToNop<gfc__Object> for gfc__WorldLight {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldLight {}

impl gfc__WorldLight {
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

    pub unsafe extern "thiscall" fn getStatic(&self) -> bool {
        ((*self.vfptr).getStatic)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__WorldLight__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldLight) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WorldLight) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldLight,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldLight, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldLight,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldLight),
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
    pub vfptr: *const gfc__PhantomBody__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub vfptr_2: *const gfc__CollisionObject__vftable,
    // gfc__CollisionObject
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
    pub vfptr: *const gfc__RagdollMapper__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const hkpCollisionAgent__vftable,
    // hkBaseObject
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
pub struct gfc__CShapeTerrainChunk {
    pub vfptr: *const gfc__CShapeTerrainChunk__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeTerrainChunk
    pub mChunk: *mut gfc__TerrainChunk,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeTerrainChunk {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeTerrainChunk {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeTerrainChunk {}

impl gfc__CShapeTerrainChunk {
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
pub struct gfc__CShapeTerrainChunk__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainChunk) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainChunk) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainChunk) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk) -> bool,
    pub containsType:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainChunk, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainChunk,
        _: f32,
        _: i32,
    ) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainChunk, _: i32),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__TerrainChunkLayer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__CShapeTerrainNode {
    pub vfptr: *const gfc__CShapeTerrainNode__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeTerrainNode
    pub mNode: *mut gfc__TerrainNode,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeTerrainNode {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeTerrainNode {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeTerrainNode {}

impl gfc__CShapeTerrainNode {
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
pub struct gfc__CShapeTerrainNode__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainNode) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainNode) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainNode) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode) -> bool,
    pub containsType:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTerrainNode, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTerrainNode,
        _: f32,
        _: i32,
    ) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTerrainNode, _: i32),
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
    pub vfptr: *const hkpNullBroadPhaseListener__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub vfptr_2: *const hkpBroadPhaseListener__vftable,
    /* hkpBroadPhaseListener
     * hkpNullBroadPhaseListener */
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
    pub vfptr: *const hkpConvexVerticesShape__vftable,
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
pub struct hkpAction {
    pub vfptr: *const hkpAction__vftable,
    // hkBaseObject
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
pub struct gfc__CShapeTransform {
    pub vfptr: *const gfc__CShapeTransform__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeTransform
    pub mShape: gfc__AutoRef_gfc__CShape_,
    pub mTransform: gfc__Matrix4,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeTransform {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeTransform {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeTransform {}

impl gfc__CShapeTransform {
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
pub struct gfc__CShapeTransform__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeTransform) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeTransform) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeTransform) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform) -> bool,
    pub containsType:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeTransform, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeTransform,
        _: f32,
        _: i32,
    ) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeTransform, _: i32),
}

#[repr(C)]
pub struct gfc__CShapeCapsule {
    pub vfptr: *const gfc__CShapeCapsule__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeCapsule
    pub mRadius: f32,
    pub mLength: f32,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeCapsule {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeCapsule {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeCapsule {}

impl gfc__CShapeCapsule {
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
pub struct gfc__CShapeCapsule__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeCapsule) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeCapsule) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeCapsule) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShapeCapsule, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCapsule,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule, _: f32, _: i32) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCapsule, _: i32),
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhysMeshCache__ReloadInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysMeshCache__ReloadInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__PhysMeshCache {
    pub vfptr: *const gfc__PhysMeshCache__vftable,
    // gfc__ResourceCache
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
    pub vfptr: *const gfc__CShapeBox__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const gfc__CShapeMesh__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const gfc__DebrisManager__vftable,
    // gfc__ResourceListener
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
pub struct gfc__CShapeSphere {
    pub vfptr: *const gfc__CShapeSphere__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeSphere
    pub mRadius: f32,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeSphere {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeSphere {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeSphere {}

impl gfc__CShapeSphere {
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
pub struct gfc__CShapeSphere__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeSphere) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeSphere) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeSphere) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShapeSphere, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeSphere,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere, _: f32, _: i32) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeSphere, _: i32),
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__DebrisManagerDesc_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__CShapeCylinder {
    pub vfptr: *const gfc__CShapeCylinder__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeCylinder
    pub mRadius: f32,
    pub mLength: f32,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeCylinder {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeCylinder {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeCylinder {}

impl gfc__CShapeCylinder {
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
pub struct gfc__CShapeCylinder__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeCylinder) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeCylinder) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeCylinder) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder) -> bool,
    pub containsType: unsafe extern "thiscall" fn(this: *const gfc__CShapeCylinder, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeCylinder,
        _: f32,
        _: i32,
    ) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeCylinder, _: i32),
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
    // hkpBroadPhaseHandlePair
    pub m_a: *mut hkpBroadPhaseHandle,
    pub m_b: *mut hkpBroadPhaseHandle,
    // hkpTypedBroadPhaseHandlePair
}

unsafe impl UpcastToNop<hkpBroadPhaseHandlePair> for hkpTypedBroadPhaseHandlePair {}

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
    pub vfptr: *const gfc__PhysicsDetectRegion__vftable,
    // hkpWorldPostSimulationListener
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
    pub vfptr: *const gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy__vftable,
    /* hkpPhantomOverlapListener
     * gfc__PhysicsDetectRegion__PhantomOverlapListenerProxy */
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
    pub vfptr: *const gfc__TraversalWaypoint__vftable,
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
    pub vfptr: *const gfc__TraversalWaypoint__TraversalWaypointGizmo__vftable,
    // gfc__SceneObject
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub vfptr_2: *const gfc__IRenderCallback__vftable,
    // gfc__IRenderCallback
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
pub struct gfc__Vector_gfc__AutoRef_gfc__CShape__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CShape_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__CShapeComposite {
    pub vfptr: *const gfc__CShapeComposite__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__CShape
    pub mMaterialID: i32,
    // gfc__CShapeComposite
    pub mShapes: gfc__Vector_gfc__AutoRef_gfc__CShape__0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__CShape> for gfc__CShapeComposite {}

unsafe impl UpcastToNop<gfc__Object> for gfc__CShapeComposite {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__CShapeComposite {}

impl gfc__CShapeComposite {
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
pub struct gfc__CShapeComposite__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__CShapeComposite) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__CShapeComposite) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__CShapeComposite) -> i32,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *const gfc__Matrix4,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub getBoundingBox_2: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub collideRay: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut gfc__CInfo,
    ) -> bool,
    pub isComposite: unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite) -> bool,
    pub containsType:
        unsafe extern "thiscall" fn(this: *const gfc__CShapeComposite, _: i32) -> bool,
    pub queryShapes: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: *const gfc__TBox_float_gfc__FloatMath_,
        _: *mut gfc__Vector_gfc__CShape___0_gfc__CAllocator_,
    ),
    pub createHavokShape: unsafe extern "thiscall" fn(
        this: *mut gfc__CShapeComposite,
        _: f32,
        _: i32,
    ) -> *mut hkpShape,
    pub setMaterialID: unsafe extern "thiscall" fn(this: *mut gfc__CShapeComposite, _: i32),
}

#[repr(C)]
pub struct gfc__CollisionManager {
    pub vfptr: *const gfc__CollisionManager__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const gfc__ModSysContainerModule__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__VisScriptEntity
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    // gfc__VisScriptModule
    pub mEventLinks: gfc__Vector_gfc__ModuleEventLink_0_gfc__CAllocator_,
    pub mInputLinks: gfc__Vector_gfc__ModuleInputLink_0_gfc__CAllocator_,
    pub mVariableLinks: gfc__Vector_gfc__ModuleVariableLink_0_gfc__CAllocator_,
    // gfc__ModSysContainerModule
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
    pub vfptr: *const gfc__OOObjectWriter__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsReceived_t_0___vftable,
    // CCallbackBase
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
    pub vfptr: *const CCallbackImpl_16___vftable,
    // CCallbackBase
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
    pub vfptr: *const keen__SteamStats__vftable,
    // keen__ISteamStatsCallback
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
    pub vfptr: *const keen__SteamAchievements__vftable,
    // keen__ISteamAchievementsCallback
    pub m_onUserAchievementStored:
        CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0_,
    pub vfptr_2: *const keen__ISteamStatsCallback__vftable,
    // keen__ISteamStatsCallback
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
    pub vfptr: *const CCallbackImpl_152___vftable,
    // CCallbackBase
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
    pub vfptr: *const CCallback_keen__ISteamStatsCallback_UserStatsStored_t_0___vftable,
    // CCallbackBase
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
    pub vfptr:
        *const CCallback_keen__ISteamAchievementsCallback_UserAchievementStored_t_0___vftable,
    // CCallbackBase
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
    pub vfptr: *const CCallbackImpl_24___vftable,
    // CCallbackBase
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
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_pRasterizerStates: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_pBlendStates: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_pDepthStencilState: compile_error!("unimplemented feature: sizeof array 0x0"),
    #[cfg(pdb_issue = "unimplemented feature: sizeof array 0x0")]
    pub m_pSamplerState: compile_error!("unimplemented feature: sizeof array 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub m_renderPassCameraData: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "unimplemented feature: class layout 0x0")]
    pub vertexData: compile_error!("unimplemented feature: class layout 0x0"),
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
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub deferredCommandBuffer: keen__GraphicsCommandBuffer,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub emptyFragmentShader: keen__FragmentShader,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pDefaultSwapChain: *mut keen__RenderSwapChain,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pCurrentSwapChain: *mut keen__RenderSwapChain,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub currentFrameNumber: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub pScreenCapture: *mut keen__ScreenCapture,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub previousFullscreenMode: keen__graphics__WindowMode,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub fullscreenMode: keen__graphics__WindowMode,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeWidth: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeHeight: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
    pub exclusiveModeRefreshRateNumerator: u32,
    #[cfg(pdb_issue = "can\'t lay out field accurately")]
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
    pub vfptr: *const gfc__Character__vftable,
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
    pub vfptr: *const gfc__Actor__vftable,
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
    pub vfptr: *const gfc__GameCamera__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const gfc__TriggerRegion__vftable,
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
    pub vfptr: *const gfc__Player__vftable,
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
    pub vfptr_2: *const gfc__ResourceListener__vftable,
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
    pub vfptr: *const gfc__CharacterDoInteractiveDesc__vftable,
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
    pub vfptr: *const gfc__Mount__vftable,
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
    pub vfptr: *const gfc__KinematicActor__vftable,
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
    pub vfptr: *const gfc__CharMoveStateDesc__vftable,
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
    pub vfptr: *const gfc__HitInfo__vftable,
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
    pub vfptr: *const gfc__DetectorRegion__vftable,
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
    pub vfptr: *const gfc__Inventory__vftable,
    // gfc__IRefObject
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
    pub vfptr: *const gfc__Container__vftable,
    // gfc__IRefObject
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
pub struct gfc__Vector_gfc__AutoRef_gfc__Item__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__Item_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__WindowHelper {
    pub vfptr: *const gfc__WindowHelper__vftable,
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
    pub vfptr: *const gfc__EquippableItem__vftable,
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
    // gfc__Item
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
    pub vfptr: *const gfc__TeleportHelper__vftable,
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
    pub vfptr: *const gfc__Hud__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub vfptr_2: *const gfc__Hierarchical_gfc___UIControl___vftable,
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
    // gfc__UIScriptControl
    // gfc__Hud
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
    pub vfptr: *const gfc__DraggableActor__vftable,
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
pub struct gfc__Vector_gfc__AutoRef_gfc__WeaponEnhancement__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__WeaponEnhancement_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__DSUIManager {
    pub vfptr: *const gfc__DSUIManager__vftable,
    // gfc___UIManager
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
    __pdbindgen_padding_2: [u8; 6],
    // gfc__DSUIManager
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
pub struct gfc__Weapon {
    pub vfptr: *const gfc__Weapon__vftable,
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
    // gfc__Item
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
    pub vfptr: *const gfc__Item__vftable,
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
    // gfc__Item
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
pub struct gfc__UIScriptControl {
    pub vfptr: *const gfc__UIScriptControl__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub vfptr_2: *const gfc__Hierarchical_gfc___UIControl___vftable,
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
    pub vfptr: *const gfc__PhysicsShapeGizmo__vftable,
    // gfc__SceneObject
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    pub vfptr_2: *const gfc__IRenderCallback__vftable,
    // gfc__IRenderCallback
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
    pub vfptr: *const gfc__LoadMapMenu__vftable,
    // gfc__IRefObject
    pub ReferenceCount: i32,
    // gfc__Object
    pub vfptr_2: *const gfc__Hierarchical_gfc___UIControl___vftable,
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
pub struct gfc__KGMeshCache {
    pub vfptr: *const gfc__KGMeshCache__vftable,
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
    pub vfptr: *const hkMemoryAllocator__vftable,
}

impl hkMemoryAllocator {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn blockAlloc(&self, a1: i32) -> *mut () {
        ((*self.vfptr).blockAlloc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn blockFree(&self, a1: *mut (), a2: i32) {
        ((*self.vfptr).blockFree)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn bufAlloc(&self, a1: *mut i32) -> *mut () {
        ((*self.vfptr).bufAlloc)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn bufFree(&self, a1: *mut (), a2: i32) {
        ((*self.vfptr).bufFree)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn bufRealloc(
        &self,
        a1: *mut (),
        a2: i32,
        a3: *mut i32,
    ) -> *mut () {
        ((*self.vfptr).bufRealloc)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn blockAllocBatch(&self, a1: *mut *mut (), a2: i32, a3: i32) {
        ((*self.vfptr).blockAllocBatch)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn blockFreeBatch(&self, a1: *mut *mut (), a2: i32, a3: i32) {
        ((*self.vfptr).blockFreeBatch)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getMemoryStatistics(
        &self,
        a1: *mut hkMemoryAllocator__MemoryStatistics,
    ) {
        ((*self.vfptr).getMemoryStatistics)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAllocatedSize(&self, a1: *const (), a2: i32) -> i32 {
        ((*self.vfptr).getAllocatedSize)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn resetPeakMemoryStatistics(&self) {
        ((*self.vfptr).resetPeakMemoryStatistics)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getExtendedInterface(
        &self,
    ) -> *mut hkMemoryAllocator__ExtendedInterface {
        ((*self.vfptr).getExtendedInterface)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkMemoryAllocator__vftable {
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

unsafe impl UpcastToNop<hkArrayBase_hkRefCountedProperties__Entry_>
    for hkArray_hkRefCountedProperties__Entry_hkContainerHeapAllocator_
{
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

unsafe impl UpcastToNop<hkVector4f> for hkcdVertex {}

#[repr(C)]
pub struct hkRotationf {
    // hkMatrix3f
    pub m_col0: hkVector4f,
    pub m_col1: hkVector4f,
    pub m_col2: hkVector4f,
    // hkRotationf
}

unsafe impl UpcastToNop<hkMatrix3f> for hkRotationf {}

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
    pub vfptr: *const hkcdShape__vftable,
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

unsafe impl UpcastToNop<hkReferencedObject> for hkcdShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkcdShape {}

impl hkcdShape {
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
pub struct hkcdShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkcdShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkcdShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkcdShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkcdShape),
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
    pub vfptr: *const hkReferencedObject__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
}

unsafe impl UpcastToNop<hkBaseObject> for hkReferencedObject {}

impl hkReferencedObject {
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
pub struct hkReferencedObject__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkReferencedObject, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkReferencedObject),
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
    pub vfptr: *const hkpConstraintData__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintData
    pub m_userData: u32,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConstraintData {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConstraintData {}

impl hkpConstraintData {
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

    pub unsafe extern "thiscall" fn getType(&self) -> i32 {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getConstraintInfo(
        &self,
        a1: *mut hkpConstraintData__ConstraintInfo,
    ) {
        ((*self.vfptr).getConstraintInfo)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isValid(&self, result: *mut hkBool) -> *mut hkBool {
        ((*self.vfptr).isValid)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setMaximumLinearImpulse(&self, a1: f32) {
        ((*self.vfptr).setMaximumLinearImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMaximumAngularImpulse(&self, a1: f32) {
        ((*self.vfptr).setMaximumAngularImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setBreachImpulse(&self, a1: f32) {
        ((*self.vfptr).setBreachImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getMaximumLinearImpulse(&self) -> f32 {
        ((*self.vfptr).getMaximumLinearImpulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaximumAngularImpulse(&self) -> f32 {
        ((*self.vfptr).getMaximumAngularImpulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getBreachImpulse(&self) -> f32 {
        ((*self.vfptr).getBreachImpulse)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setBodyToNotify(&self, a1: i32) {
        ((*self.vfptr).setBodyToNotify)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getNotifiedBodyIndex(&self) -> u8 {
        ((*self.vfptr).getNotifiedBodyIndex)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setSolvingMethod(&self, a1: hkpConstraintAtom__SolvingMethod) {
        ((*self.vfptr).setSolvingMethod)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaStabilizationFactor(
        &self,
        result: *mut hkResult,
        a2: f32,
    ) -> *mut hkResult {
        ((*self.vfptr).setInertiaStabilizationFactor)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getInertiaStabilizationFactor(
        &self,
        result: *mut hkResult,
        a2: *mut f32,
    ) -> *mut hkResult {
        ((*self.vfptr).getInertiaStabilizationFactor)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn getAppliedLinearImpulse(
        &self,
        a1: *const hkTransformf,
        a2: *const hkTransformf,
        a3: *const hkpConstraintRuntime,
        a4: *mut hkVector4f,
    ) {
        ((*self.vfptr).getAppliedLinearImpulse)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getRuntimeInfo(
        &self,
        a1: hkBool,
        a2: *mut hkpConstraintData__RuntimeInfo,
    ) {
        ((*self.vfptr).getRuntimeInfo)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getSolverResults(
        &self,
        a1: *mut hkpConstraintRuntime,
    ) -> *mut hkpSolverResults {
        ((*self.vfptr).getSolverResults)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addInstance(&self, a1: *mut hkpConstraintRuntime, a2: i32) {
        ((*self.vfptr).addInstance)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateDirtyAtoms(
        &self,
    ) -> hkpConstraintData__UpdateAtomsResult__Enum {
        ((*self.vfptr).updateDirtyAtoms)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn buildJacobian(
        &self,
        a1: *const hkpConstraintQueryIn,
        a2: *mut hkpConstraintQueryOut,
    ) {
        ((*self.vfptr).buildJacobian)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn isBuildJacobianCallbackRequired(
        &self,
        result: *mut hkBool,
    ) -> *mut hkBool {
        ((*self.vfptr).isBuildJacobianCallbackRequired)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn buildJacobianCallback(
        &self,
        a1: *const hkpConstraintQueryIn,
        a2: *const hkpConstraintQueryOut,
    ) {
        ((*self.vfptr).buildJacobianCallback)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpConstraintData__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpConstraintData, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpConstraintData),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpConstraintData) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConstraintData),
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
    pub vfptr: *const hkpShapeContainer__vftable,
}

impl hkpShapeContainer {
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
}

#[repr(C)]
pub struct hkpShapeContainer__vftable {
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
    pub vfptr: *const hkpBvTreeShape__vftable,
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

unsafe impl UpcastToNop<hkpShape> for hkpBvTreeShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpBvTreeShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpBvTreeShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBvTreeShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBvTreeShape {}

impl hkpBvTreeShape {
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
}

#[repr(C)]
pub struct hkpBvTreeShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpBvTreeShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpBvTreeShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpBvTreeShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBvTreeShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpBvTreeShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpBvTreeShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpBvTreeShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpBvTreeShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpBvTreeShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpBvTreeShape,
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
pub struct hkpLinearCastInput {
    pub m_to: hkVector4f,
    pub m_maxExtraPenetration: f32,
    pub m_startPointTolerance: f32,
    __pdbindgen_padding: [u8; 8],
}

#[repr(C)]
pub struct hkpBroadPhase {
    pub vfptr: *const hkpBroadPhase__vftable,
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

unsafe impl UpcastToNop<hkReferencedObject> for hkpBroadPhase {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBroadPhase {}

impl hkpBroadPhase {
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

    pub unsafe extern "thiscall" fn getType(&self) -> hkpBroadPhase__BroadPhaseType {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCapabilityDelegate(
        &self,
        a1: hkpBroadPhase__Capabilities,
    ) -> *const hkpBroadPhase {
        ((*self.vfptr).getCapabilityDelegate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addObject(
        &self,
        a1: *mut hkpBroadPhaseHandle,
        a2: *const hkAabb,
        a3: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        a4: bool,
    ) {
        ((*self.vfptr).addObject)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn addObject_2(
        &self,
        a1: *mut hkpBroadPhaseHandle,
        a2: *const hkAabbUint32,
        a3: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        a4: bool,
    ) {
        ((*self.vfptr).addObject_2)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn addObjectBatch(
        &self,
        a1: *const hkArrayBase_hkpBroadPhaseHandle___,
        a2: *const hkArrayBase_hkAabb_,
        a3: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).addObjectBatch)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn removeObject(
        &self,
        a1: *mut hkpBroadPhaseHandle,
        a2: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).removeObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn removeObjectBatch(
        &self,
        a1: *const hkArrayBase_hkpBroadPhaseHandle___,
        a2: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).removeObjectBatch)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getNumObjects(&self) -> i32 {
        ((*self.vfptr).getNumObjects)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateAabbs(
        &self,
        a1: *mut *mut hkpBroadPhaseHandle,
        a2: *const hkAabb,
        a3: i32,
        a4: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        a5: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).updateAabbs)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn updateAabbsUint32(
        &self,
        a1: *mut *mut hkpBroadPhaseHandle,
        a2: *const hkAabbUint32,
        a3: i32,
        a4: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
        a5: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).updateAabbsUint32)(self as *const _ as *mut _, a1, a2, a3, a4, a5)
    }

    pub unsafe extern "thiscall" fn defragment(&self) {
        ((*self.vfptr).defragment)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn checkDeterminism(&self) {
        ((*self.vfptr).checkDeterminism)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getAllAabbs(
        &self,
        a1: *mut hkArray_hkAabb_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getAllAabbs)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAabb(
        &self,
        a1: *const hkpBroadPhaseHandle,
        a2: *mut hkAabb,
    ) {
        ((*self.vfptr).getAabb)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getExtents(&self, a1: *mut hkVector4f, a2: *mut hkVector4f) {
        ((*self.vfptr).getExtents)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn querySingleAabb(
        &self,
        a1: *const hkAabb,
        a2: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).querySingleAabb)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn reQuerySingleObject(
        &self,
        a1: *const hkpBroadPhaseHandle,
        a2: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).reQuerySingleObject)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn querySingleAabbWithCollector(
        &self,
        a1: *const hkAabb,
        a2: *mut hkpBroadPhaseCastCollector,
    ) {
        ((*self.vfptr).querySingleAabbWithCollector)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn areAabbsOverlapping(
        &self,
        a1: *const hkpBroadPhaseHandle,
        a2: *const hkpBroadPhaseHandle,
    ) -> bool {
        ((*self.vfptr).areAabbsOverlapping)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn shiftAllObjects(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkVector4f,
        a3: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).shiftAllObjects)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn shiftBroadPhase(
        &self,
        a1: *const hkVector4f,
        a2: *mut hkVector4f,
        a3: *mut hkArray_hkpBroadPhaseHandlePair_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).shiftBroadPhase)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getOffsetLowHigh32bit(
        &self,
        a1: *mut hkVector4f,
        a2: *mut hkVector4f,
    ) {
        ((*self.vfptr).getOffsetLowHigh32bit)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn castRay(
        &self,
        a1: *const hkpBroadPhase__hkpCastRayInput,
        a2: *mut hkpBroadPhaseCastCollector,
        a3: i32,
    ) {
        ((*self.vfptr).castRay)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getAabbCacheSize(&self) -> i32 {
        ((*self.vfptr).getAabbCacheSize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn calcAabbCache(
        &self,
        a1: *const hkArrayBase_hkpCollidable___,
        a2: *mut i8,
    ) {
        ((*self.vfptr).calcAabbCache)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn calcAabbCache_2(&self, a1: *const hkAabb, a2: *mut i8) {
        ((*self.vfptr).calcAabbCache_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn castAabb(
        &self,
        a1: *const hkpBroadPhase__hkpCastAabbInput,
        a2: *mut hkpBroadPhaseCastCollector,
    ) {
        ((*self.vfptr).castAabb)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn set32BitOffsetAndScale(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *const hkVector4f,
    ) {
        ((*self.vfptr).set32BitOffsetAndScale)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpBroadPhase__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpBroadPhase, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpBroadPhase),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpBroadPhase) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBroadPhase),
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
    pub vfptr: *const hkpBroadPhaseCastCollector__vftable,
}

impl hkpBroadPhaseCastCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addBroadPhaseHandle(
        &self,
        a1: *const hkpBroadPhaseHandle,
        a2: i32,
    ) -> f32 {
        ((*self.vfptr).addBroadPhaseHandle)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpBroadPhaseCastCollector__vftable {
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
    pub vfptr: *const hkpCollisionFilter__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    pub vfptr_2: *const hkpCollidableCollidableFilter__vftable,
    // hkpCollidableCollidableFilter
    pub vfptr_3: *const hkpShapeCollectionFilter__vftable,
    // hkpShapeCollectionFilter
    pub vfptr_4: *const hkpRayShapeCollectionFilter__vftable,
    // hkpRayShapeCollectionFilter
    pub vfptr_5: *const hkpRayCollidableFilter__vftable,
    // hkpRayCollidableFilter
    // hkpCollisionFilter
    pub m_prepad: [u32; 2],
    pub m_type: hkEnum_enum_hkpCollisionFilter__hkpFilterType_unsigned_int_,
    pub m_postpad: [u32; 3],
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpCollisionFilter {}

unsafe impl UpcastToNop<hkBaseObject> for hkpCollisionFilter {}

unsafe impl UpcastTo<hkpCollidableCollidableFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpCollidableCollidableFilter {
        (p as usize + 0x8) as *const _
    }
}

unsafe impl UpcastTo<hkpShapeCollectionFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpShapeCollectionFilter {
        (p as usize + 0xc) as *const _
    }
}

unsafe impl UpcastTo<hkpRayShapeCollectionFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpRayShapeCollectionFilter {
        (p as usize + 0x10) as *const _
    }
}

unsafe impl UpcastTo<hkpRayCollidableFilter> for hkpCollisionFilter {
    fn upcast_to(p: *const Self) -> *const hkpRayCollidableFilter {
        (p as usize + 0x14) as *const _
    }
}

impl hkpCollisionFilter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isCollisionEnabled(
        &self,
        result: *mut hkBool,
        a2: *const hkpWorldRayCastInput,
        a3: *const hkpCollidable,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn numShapeKeyHitsLimitBreached(
        &self,
        a1: *const hkpCollisionInput,
        a2: *const hkpCdBody,
        a3: *const hkpCdBody,
        a4: *const hkpBvTreeShape,
        a5: *mut hkAabb,
        a6: *mut u32,
        a7: i32,
    ) -> i32 {
        ((*self.vfptr).numShapeKeyHitsLimitBreached)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
        )
    }

    pub unsafe extern "thiscall" fn __vecDelDtor_2(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn init(&self, a1: *mut hkpWorld) {
        ((*self.vfptr).init)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpCollisionFilter__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: u32) -> *mut (),
    pub isCollisionEnabled: unsafe extern "thiscall" fn(
        this: *const hkpCollisionFilter,
        result: *mut hkBool,
        _: *const hkpWorldRayCastInput,
        _: *const hkpCollidable,
    ) -> *mut hkBool,
    pub numShapeKeyHitsLimitBreached: unsafe extern "thiscall" fn(
        this: *const hkpCollisionFilter,
        _: *const hkpCollisionInput,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpBvTreeShape,
        _: *mut hkAabb,
        _: *mut u32,
        _: i32,
    ) -> i32,
    pub __vecDelDtor_2:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionFilter, _: u32) -> *mut (),
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
    pub vfptr: *const hkpSphereRepShape__vftable,
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

unsafe impl UpcastToNop<hkpShape> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSphereRepShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSphereRepShape {}

impl hkpSphereRepShape {
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
pub struct hkpSphereRepShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpSphereRepShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpSphereRepShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpSphereRepShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpSphereRepShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpSphereRepShape,
        _: *const hkpShape__CalcSizeForSpuInput,
        _: i32,
    ) -> i32,
}

#[repr(C)]
pub struct hkpConvexShape {
    pub vfptr: *const hkpConvexShape__vftable,
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

unsafe impl UpcastToNop<hkpSphereRepShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkpShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkpShapeBase> for hkpConvexShape {}

unsafe impl UpcastToNop<hkcdShape> for hkpConvexShape {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexShape {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexShape {}

impl hkpConvexShape {
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
pub struct hkpConvexShape__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpConvexShape, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpConvexShape),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConvexShape),
    pub isConvex: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> bool,
    pub getAabb: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkTransformf,
        _: f32,
        _: *mut hkAabb,
    ),
    pub castRay: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        result: *mut hkBool,
        _: *const hkpShapeRayCastInput,
        _: *mut hkpShapeRayCastOutput,
    ) -> *mut hkBool,
    pub castRayWithCollector: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkpShapeRayCastInput,
        _: *const hkpCdBody,
        _: *mut hkpRayHitCollector,
    ),
    pub castRayBundle: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        result: *mut hkVector4fComparison,
        _: *const hkpShapeRayBundleCastInput,
        _: *mut hkpShapeRayBundleCastOutput,
        _: *const hkVector4fComparison,
    ) -> *mut hkVector4fComparison,
    pub getSupportingVertex: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const hkVector4f,
        _: *mut hkcdVertex,
    ),
    pub convertVertexIdsToVertices: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *const u16,
        _: i32,
        _: *mut hkcdVertex,
    ),
    pub getCentre: unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *mut hkVector4f),
    pub getNumCollisionSpheres: unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> i32,
    pub getCollisionSpheres: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *mut hkSphere,
    ) -> *const hkSphere,
    pub weldContactPoint: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
        _: *mut u16,
        _: *mut u8,
        _: *mut hkVector4f,
        _: *const hkTransformf,
        _: *const hkpConvexShape,
        _: *const hkTransformf,
        _: *mut hkVector4f,
    ) -> i32,
    pub getContainer:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape) -> *const hkpShapeContainer,
    pub getMaximumProjection:
        unsafe extern "thiscall" fn(this: *const hkpConvexShape, _: *const hkVector4f) -> f32,
    pub calcSizeForSpu: unsafe extern "thiscall" fn(
        this: *const hkpConvexShape,
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
    pub vfptr: *const hkpAabbCastCollector__vftable,
    __pdbindgen_padding: [u8; 12],
    pub m_earlyOutFraction: hkSimdFloat32,
}

impl hkpAabbCastCollector {
    pub unsafe extern "thiscall" fn addHit(&self, a1: u32) {
        ((*self.vfptr).addHit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpAabbCastCollector__vftable {
    pub addHit: unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpAabbCastCollector, _: u32) -> *mut (),
}

#[repr(C)]
pub struct hkpConvexVerticesConnectivity {
    pub vfptr: *const hkpConvexVerticesConnectivity__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConvexVerticesConnectivity
    pub m_vertexIndices: hkArray_unsigned_short_hkContainerHeapAllocator_,
    pub m_numVerticesPerFace: hkArray_unsigned_char_hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexVerticesConnectivity {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexVerticesConnectivity {}

impl hkpConvexVerticesConnectivity {
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

    pub unsafe extern "thiscall" fn clear(&self) {
        ((*self.vfptr).clear)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpConvexVerticesConnectivity__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesConnectivity) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpConvexVerticesConnectivity),
    pub clear: unsafe extern "thiscall" fn(this: *mut hkpConvexVerticesConnectivity),
}

#[repr(C)]
pub struct hkpConvexListFilter {
    pub vfptr: *const hkpConvexListFilter__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConvexListFilter
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConvexListFilter {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConvexListFilter {}

impl hkpConvexListFilter {
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

    pub unsafe extern "thiscall" fn getConvexListCollisionType(
        &self,
        a1: *const hkpCdBody,
        a2: *const hkpCdBody,
        a3: *const hkpCollisionInput,
    ) -> hkpConvexListFilter__ConvexListCollisionType {
        ((*self.vfptr).getConvexListCollisionType)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpConvexListFilter__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpConvexListFilter, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConvexListFilter),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConvexListFilter) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConvexListFilter),
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
    pub vfptr: *const hkpPhantom__vftable,
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

unsafe impl UpcastToNop<hkpWorldObject> for hkpPhantom {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpPhantom {}

unsafe impl UpcastToNop<hkBaseObject> for hkpPhantom {}

impl hkpPhantom {
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
}

#[repr(C)]
pub struct hkpPhantom__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpPhantom, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpPhantom),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpPhantom) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpPhantom),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpPhantom,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpPhantom,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState: unsafe extern "thiscall" fn(this: *mut hkpPhantom) -> *mut hkMotionState,
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
    pub vfptr: *const hkpWorld__vftable,
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

unsafe impl UpcastToNop<hkReferencedObject> for hkpWorld {}

unsafe impl UpcastToNop<hkBaseObject> for hkpWorld {}

impl hkpWorld {
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
pub struct hkpWorld__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpWorld, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpWorld),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpWorld) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpWorld),
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
    pub vfptr: *const hkpRigidBody__vftable,
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

unsafe impl UpcastToNop<hkpEntity> for hkpRigidBody {}

unsafe impl UpcastToNop<hkpWorldObject> for hkpRigidBody {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpRigidBody {}

unsafe impl UpcastToNop<hkBaseObject> for hkpRigidBody {}

impl hkpRigidBody {
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

    pub unsafe extern "thiscall" fn clone(&self) -> *mut hkpRigidBody {
        ((*self.vfptr).clone)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpRigidBody__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpRigidBody, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpRigidBody),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpRigidBody) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpRigidBody),
    pub setShape: unsafe extern "thiscall" fn(
        this: *mut hkpRigidBody,
        _: *const hkpShape,
    ) -> hkWorldOperation__Result,
    pub updateShape: unsafe extern "thiscall" fn(
        this: *mut hkpRigidBody,
        _: *mut hkpShapeModifier,
    ) -> hkWorldOperation__Result,
    pub getMotionState: unsafe extern "thiscall" fn(this: *mut hkpRigidBody) -> *mut hkMotionState,
    pub deallocateInternalArrays: unsafe extern "thiscall" fn(this: *mut hkpRigidBody),
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
    pub vfptr: *const hkpSimulationIsland__vftable,
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

unsafe impl UpcastToNop<hkpConstraintOwner> for hkpSimulationIsland {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimulationIsland {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimulationIsland {}

impl hkpSimulationIsland {
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

    pub unsafe extern "thiscall" fn addConstraintToCriticalLockedIsland(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).addConstraintToCriticalLockedIsland)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeConstraintFromCriticalLockedIsland(
        &self,
        a1: *mut hkpConstraintInstance,
    ) {
        ((*self.vfptr).removeConstraintFromCriticalLockedIsland)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addCallbackRequest(
        &self,
        a1: *mut hkpConstraintInstance,
        a2: i32,
    ) {
        ((*self.vfptr).addCallbackRequest)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn checkAccessRw(&self) {
        ((*self.vfptr).checkAccessRw)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpSimulationIsland__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpSimulationIsland) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpSimulationIsland),
    pub addConstraintToCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: *mut hkpConstraintInstance),
    pub removeConstraintFromCriticalLockedIsland:
        unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland, _: *mut hkpConstraintInstance),
    pub addCallbackRequest: unsafe extern "thiscall" fn(
        this: *mut hkpSimulationIsland,
        _: *mut hkpConstraintInstance,
        _: i32,
    ),
    pub checkAccessRw: unsafe extern "thiscall" fn(this: *mut hkpSimulationIsland),
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

unsafe impl UpcastToNop<hkpCollisionEvent> for hkpContactPointEvent {}

#[repr(C)]
pub struct hkpDynamicsContactMgr {
    pub vfptr: *const hkpDynamicsContactMgr__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpContactMgr
    pub m_type: hkpContactMgr__Type,
    // hkpDynamicsContactMgr
    pub m_world: *mut hkpWorld,
}

unsafe impl UpcastToNop<hkpContactMgr> for hkpDynamicsContactMgr {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpDynamicsContactMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpDynamicsContactMgr {}

impl hkpDynamicsContactMgr {
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

    pub unsafe extern "thiscall" fn getContactPointProperties(
        &self,
        a1: u16,
    ) -> *mut hkpContactPointProperties {
        ((*self.vfptr).getContactPointProperties)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContactPoint(&self, a1: u16) -> *mut hkContactPoint {
        ((*self.vfptr).getContactPoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAllContactPointIds(
        &self,
        a1: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getAllContactPointIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpContactMgr__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseBeginCallback(
        &self,
        a1: *const hkContactPoint,
        a2: *mut hkpSimpleConstraintInfoInitInput,
        a3: *mut hkpBodyVelocity,
        a4: *mut hkpSimpleConstraintInfoInitInput,
        a5: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseBeginCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseEndCallback(
        &self,
        a1: *const hkContactPoint,
        a2: f32,
        a3: *mut hkpSimpleConstraintInfoInitInput,
        a4: *mut hkpBodyVelocity,
        a5: *mut hkpSimpleConstraintInfoInitInput,
        a6: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseEndCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
        )
    }

    pub unsafe extern "thiscall" fn getConstraintInstance(&self) -> *mut hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn fireCallbacksForEarliestToi(
        &self,
        result: *mut hkBool,
        a2: *mut hkpToiEvent,
        a3: *mut f32,
    ) -> *mut hkBool {
        ((*self.vfptr).fireCallbacksForEarliestToi)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn confirmToi(
        &self,
        a1: *mut hkpToiEvent,
        a2: f32,
        a3: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).confirmToi)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpDynamicsContactMgr__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpDynamicsContactMgr) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpDynamicsContactMgr),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        result: *mut hkResult,
        _: i32,
    ) -> *mut hkResult,
    pub removeContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: u16,
        _: *mut hkpConstraintOwner,
    ),
    pub processContactImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionData,
    ),
    pub addToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpDynamicsContactMgr,
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
        this: *mut hkpDynamicsContactMgr,
        _: *mut hkpConstraintOwner,
        _: *mut hkpContactPointProperties,
    ),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut hkpDynamicsContactMgr),
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

unsafe impl UpcastToNop<hkpConstraintAtom> for hkpModifierConstraintAtom {}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr {
    pub vfptr: *const hkpSimpleConstraintContactMgr__vftable,
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

unsafe impl UpcastToNop<hkpDynamicsContactMgr> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkpContactMgr> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimpleConstraintContactMgr {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimpleConstraintContactMgr {}

impl hkpSimpleConstraintContactMgr {
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

    pub unsafe extern "thiscall" fn getContactPointProperties(
        &self,
        a1: u16,
    ) -> *mut hkpContactPointProperties {
        ((*self.vfptr).getContactPointProperties)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getContactPoint(&self, a1: u16) -> *mut hkContactPoint {
        ((*self.vfptr).getContactPoint)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAllContactPointIds(
        &self,
        a1: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getAllContactPointIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getType(&self) -> hkpContactMgr__Type {
        ((*self.vfptr).getType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseBeginCallback(
        &self,
        a1: *const hkContactPoint,
        a2: *mut hkpSimpleConstraintInfoInitInput,
        a3: *mut hkpBodyVelocity,
        a4: *mut hkpSimpleConstraintInfoInitInput,
        a5: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseBeginCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn toiCollisionResponseEndCallback(
        &self,
        a1: *const hkContactPoint,
        a2: f32,
        a3: *mut hkpSimpleConstraintInfoInitInput,
        a4: *mut hkpBodyVelocity,
        a5: *mut hkpSimpleConstraintInfoInitInput,
        a6: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).toiCollisionResponseEndCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
        )
    }

    pub unsafe extern "thiscall" fn getConstraintInstance(&self) -> *mut hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn fireCallbacksForEarliestToi(
        &self,
        result: *mut hkBool,
        a2: *mut hkpToiEvent,
        a3: *mut f32,
    ) -> *mut hkBool {
        ((*self.vfptr).fireCallbacksForEarliestToi)(self as *const _ as *mut _, result, a2, a3)
    }

    pub unsafe extern "thiscall" fn confirmToi(
        &self,
        a1: *mut hkpToiEvent,
        a2: f32,
        a3: *mut hkArray_hkpEntity___hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).confirmToi)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getConstraintInstance_2(&self) -> *const hkpConstraintInstance {
        ((*self.vfptr).getConstraintInstance_2)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkpSimpleConstraintContactMgr__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpSimpleConstraintContactMgr) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpSimpleConstraintContactMgr),
    pub addContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkpCdBody,
        _: *const hkpCdBody,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionOutput,
        _: *const hkpGskCache,
        _: *mut hkContactPoint,
    ) -> u16,
    pub reserveContactPointsImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        result: *mut hkResult,
        _: i32,
    ) -> *mut hkResult,
    pub removeContactPointImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
        _: *mut hkpConstraintOwner,
    ),
    pub processContactImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkpCollidable,
        _: *const hkpCollidable,
        _: *const hkpProcessCollisionInput,
        _: *mut hkpProcessCollisionData,
    ),
    pub addToiImpl: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
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
        this: *mut hkpSimpleConstraintContactMgr,
        _: *mut hkpConstraintOwner,
        _: *mut hkpContactPointProperties,
    ),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut hkpSimpleConstraintContactMgr),
    pub getContactPointProperties: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
    )
        -> *mut hkpContactPointProperties,
    pub getContactPoint: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: u16,
    ) -> *mut hkContactPoint,
    pub getAllContactPointIds: unsafe extern "thiscall" fn(
        this: *const hkpSimpleConstraintContactMgr,
        _: *mut hkArray_unsigned_short_hkContainerHeapAllocator_,
    ),
    pub getType: unsafe extern "thiscall" fn(
        this: *const hkpSimpleConstraintContactMgr,
    ) -> hkpContactMgr__Type,
    pub toiCollisionResponseBeginCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkContactPoint,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub toiCollisionResponseEndCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        _: *const hkContactPoint,
        _: f32,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
        _: *mut hkpSimpleConstraintInfoInitInput,
        _: *mut hkpBodyVelocity,
    ),
    pub getConstraintInstance: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
    ) -> *mut hkpConstraintInstance,
    pub fireCallbacksForEarliestToi: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
        result: *mut hkBool,
        _: *mut hkpToiEvent,
        _: *mut f32,
    ) -> *mut hkBool,
    pub confirmToi: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleConstraintContactMgr,
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
pub struct hkpContactPointConfirmedEvent {
    pub m_collidableA: *const hkpCollidable,
    pub m_collidableB: *const hkpCollidable,
    pub m_callbackFiredFrom: *mut hkpEntity,
    pub m_contactPoint: *mut hkContactPoint,
    pub m_contactPointProperties: *mut hkpContactPointProperties,
    pub m_rotateNormal: f32,
    pub m_projectedVelocity: f32,
    pub m_type: hkEnum_enum_hkpContactPointAddedEvent__Type_int_,
    pub m_contactData: *const hkpSimpleContactConstraintData,
}

#[repr(C)]
pub struct hkEnum_enum_hkpContactPointAddedEvent__Type_int_ {
    pub m_storage: i32,
}

#[repr(C)]
pub struct hkpCollisionListener {
    pub vfptr: *const hkpCollisionListener__vftable,
    /* hkpContactListener
     * hkpCollisionListener */
}

unsafe impl UpcastToNop<hkpContactListener> for hkpCollisionListener {}

impl hkpCollisionListener {
    pub unsafe extern "thiscall" fn contactPointCallback(&self, a1: *const hkpContactPointEvent) {
        ((*self.vfptr).contactPointCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collisionAddedCallback(&self, a1: *const hkpCollisionEvent) {
        ((*self.vfptr).collisionAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn collisionRemovedCallback(&self, a1: *const hkpCollisionEvent) {
        ((*self.vfptr).collisionRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointAddedCallback(
        &self,
        a1: *mut hkpContactPointAddedEvent,
    ) {
        ((*self.vfptr).contactPointAddedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointRemovedCallback(
        &self,
        a1: *mut hkpContactPointRemovedEvent,
    ) {
        ((*self.vfptr).contactPointRemovedCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactProcessCallback(&self, a1: *mut hkpContactProcessEvent) {
        ((*self.vfptr).contactProcessCallback)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn contactPointConfirmedCallback(
        &self,
        a1: *mut hkpContactPointConfirmedEvent,
    ) {
        ((*self.vfptr).contactPointConfirmedCallback)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpCollisionListener__vftable {
    pub contactPointCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *const hkpContactPointEvent,
    ),
    pub collisionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: *const hkpCollisionEvent),
    pub collisionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: *const hkpCollisionEvent),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpCollisionListener, _: u32) -> *mut (),
    pub contactPointAddedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointAddedEvent,
    ),
    pub contactPointRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointRemovedEvent,
    ),
    pub contactProcessCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactProcessEvent,
    ),
    pub contactPointConfirmedCallback: unsafe extern "thiscall" fn(
        this: *mut hkpCollisionListener,
        _: *mut hkpContactPointConfirmedEvent,
    ),
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

unsafe impl UpcastToNop<hkpCollisionInput> for hkpLinearCastCollisionInput {}

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

unsafe impl UpcastToNop<hkpShapeRayCastCollectorOutput> for hkpShapeRayCastOutput {}

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
    pub vfptr: *const hkThreadPool__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkThreadPool
}

unsafe impl UpcastToNop<hkReferencedObject> for hkThreadPool {}

unsafe impl UpcastToNop<hkBaseObject> for hkThreadPool {}

impl hkThreadPool {
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

    pub unsafe extern "thiscall" fn processJobQueue(&self, a1: *mut hkJobQueue, a2: hkJobType) {
        ((*self.vfptr).processJobQueue)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn processTaskQueue(&self, a1: *mut hkDefaultTaskQueue) {
        ((*self.vfptr).processTaskQueue)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isProcessing(&self) -> bool {
        ((*self.vfptr).isProcessing)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn waitForCompletion(&self) {
        ((*self.vfptr).waitForCompletion)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNumThreads(&self) -> i32 {
        ((*self.vfptr).getNumThreads)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setNumThreads(&self, a1: i32) {
        ((*self.vfptr).setNumThreads)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn appendTimerData(
        &self,
        a1: *mut hkArrayBase_hkTimerData_,
        a2: *mut hkMemoryAllocator,
    ) {
        ((*self.vfptr).appendTimerData)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn clearTimerData(&self) {
        ((*self.vfptr).clearTimerData)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn gcThreadMemoryOnNextCompletion(&self) {
        ((*self.vfptr).gcThreadMemoryOnNextCompletion)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct hkThreadPool__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkThreadPool, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkThreadPool),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkThreadPool) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkThreadPool),
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

unsafe impl UpcastToNop<hkpProcessCollisionData> for hkpProcessCollisionOutput {}

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

unsafe impl UpcastToNop<hkpCdBody> for hkpCollidable {}

#[repr(C)]
pub struct hkpCollisionDispatcher__ShapeInheritance {
    pub m_primaryType: hkcdShapeType__ShapeTypeEnum,
    pub m_alternateType: hkcdShapeType__ShapeTypeEnum,
}

#[repr(C)]
pub struct hkpBreakableMaterial {
    pub vfptr: *const hkpBreakableMaterial__vftable,
    // hkBaseObject
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpBreakableMaterial
    pub m_strength: f32,
    pub m_typeAndFlags: i32,
    pub m_properties: *mut hkRefCountedProperties,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpBreakableMaterial {}

unsafe impl UpcastToNop<hkBaseObject> for hkpBreakableMaterial {}

impl hkpBreakableMaterial {
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

    pub unsafe extern "thiscall" fn createInverseMapping(&self, a1: *const hkcdShape) {
        ((*self.vfptr).createInverseMapping)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn duplicate(&self) -> *mut hkpBreakableMaterial {
        ((*self.vfptr).duplicate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setDefaultMapping(&self) {
        ((*self.vfptr).setDefaultMapping)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getSubShapeMaterialIndex(
        &self,
        a1: *const hkcdShape,
        a2: u32,
    ) -> i16 {
        ((*self.vfptr).getSubShapeMaterialIndex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getShapeKeyMaterial(
        &self,
        a1: *const hkcdShape,
        a2: u32,
    ) -> *mut hkpBreakableMaterial {
        ((*self.vfptr).getShapeKeyMaterial)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertShapeKeyToSubShapeId(&self, a1: u32) -> u32 {
        ((*self.vfptr).convertShapeKeyToSubShapeId)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn convertShapeKeysToSubShapeIds(
        &self,
        a1: *mut hkArray_unsigned_int_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).convertShapeKeysToSubShapeIds)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn disableSubShapes(
        &self,
        a1: *mut hkcdShape,
        a2: *const i16,
        a3: i32,
    ) {
        ((*self.vfptr).disableSubShapes)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn getNumSubMaterials(&self) -> i32 {
        ((*self.vfptr).getNumSubMaterials)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getShapeKeysForSubShapes(
        &self,
        a1: *const hkcdShape,
        a2: *const u32,
        a3: i32,
        a4: *mut hkpBreakableMaterial__ShapeKeyCollector,
    ) {
        ((*self.vfptr).getShapeKeysForSubShapes)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSubShapeMaterialIndices(
        &self,
        a1: *const hkcdShape,
        a2: *const hkArray_unsigned_int_hkContainerHeapAllocator_,
        a3: *mut hkArray_short_hkContainerHeapAllocator_,
    ) {
        ((*self.vfptr).getSubShapeMaterialIndices)(self as *const _ as *mut _, a1, a2, a3)
    }
}

#[repr(C)]
pub struct hkpBreakableMaterial__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpBreakableMaterial),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpBreakableMaterial),
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
    pub vfptr: *const hkpBreakableMaterial__ShapeKeyCollector__vftable,
}

impl hkpBreakableMaterial__ShapeKeyCollector {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addShapeKey(&self, a1: u32) {
        ((*self.vfptr).addShapeKey)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn addShapeKeyBatch(&self, a1: *const u32, a2: i32) {
        ((*self.vfptr).addShapeKeyBatch)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn addContiguousShapeKeyRange(&self, a1: u32, a2: i32) {
        ((*self.vfptr).addContiguousShapeKeyRange)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkpBreakableMaterial__ShapeKeyCollector__vftable {
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
