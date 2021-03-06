#![allow(non_camel_case_types, non_snake_case, unused_imports)]
#![allow(clippy::use_self)]

use super::{types::*, types3::*, types4::*};
use pdbindgen_runtime::{UpcastTo, UpcastToNop};

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
    // gfc__IRefObject
    pub vfptr: *const gfc__Material__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__Material {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Material {}

impl gfc__Material {
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
pub struct gfc__Material__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Material, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Material) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Material, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Material) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Material) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Material,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Material) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Material, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Material,
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
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Camera: *mut gfc__Camera3D,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Shader: *mut gfc__Shader,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Material: *mut gfc__Material,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub RenderCallback: *mut gfc__IRenderCallback,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub MeshInstance: *mut gfc__MeshInstance,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Context: i32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub SceneContext: *mut (),
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub ObjectContext: *mut (),
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Transform: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub World: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub WorldView: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub WorldViewProj: gfc__Matrix4,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub MatrixArray: *mut gfc__MatrixArray,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub BVolume: gfc__BoundingVolume,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub Lights: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_2: [u8; 64],
    pub DynMeshNode: *mut gfc__DynMeshNode,
    pub ObjectColor: gfc__TVector4_float_gfc__FloatMath_,
    pub FadeAmount: gfc__TVector4_float_gfc__FloatMath_,
    __pdbindgen_padding_3: [u8; 8],
}

#[repr(C)]
pub struct gfc__KGGraphics {
    // gfc__Graphics
    pub vfptr: *const gfc__KGGraphics__vftable,
    // gfc__KGGraphics
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_destructors: compile_error!("unimplemented class layout"),
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_vertexBuffers: compile_error!("unimplemented class layout"),
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_environmentProbeTextures: compile_error!("unimplemented class layout"),
    __pdbindgen_padding_6: [u8; 8],
    pub m_pEnvironmentProbeRenderTargets: [*const keen__RenderTarget; 2],
    pub m_postProcessEffect: u32,
    pub m_antiAliasingType: u32,
    pub m_textureFilteringMode: u32,
    pub currentTextureFilteringMode: u8,
    pub m_pMaterialConstantBuffers: [*mut keen__DynamicConstantBuffer; 20],
    __pdbindgen_padding_7: [u8; 4],
}

unsafe impl UpcastToNop<gfc__Graphics> for gfc__KGGraphics {}

impl gfc__KGGraphics {
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

    pub unsafe extern "thiscall" fn createRenderTargetEx(
        &self,
        result: *mut gfc__AutoRef_gfc__Texture_,
        a2: u16,
        a3: u16,
        a4: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_ {
        ((*self.vfptr).createRenderTargetEx)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getSlopeScaledDepthBias(&self) -> f32 {
        ((*self.vfptr).getSlopeScaledDepthBias)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__KGGraphics__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: u32) -> *mut (),
    pub init: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: u16,
        _: u16,
        _: u8,
        _: bool,
    ) -> bool,
    pub isInitialized: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> bool,
    pub release: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub getSceneBackBufferSize:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics, _: *mut u16, _: *mut u16),
    pub getScreenBackBufferSize:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics, _: *mut u16, _: *mut u16),
    pub getAspectRatio: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> f32,
    pub acquireThreadOwnership: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub releaseThreadOwnership: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics) -> bool,
    pub begin3D: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics) -> bool,
    pub end3D: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub getWaitForVSync: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> bool,
    pub setWaitForVSync: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: bool),
    pub unsetSamplers: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub getWorldMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics, _: *mut gfc__Matrix4),
    pub setWorldMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *const gfc__Matrix4),
    pub commitWorldMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *const gfc__Matrix4),
    pub getViewMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> *const gfc__Matrix4,
    pub setViewMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *const gfc__Matrix4),
    pub getProjectionMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> *const gfc__Matrix4,
    pub setProjectionMatrix:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *const gfc__Matrix4),
    pub getViewProjMatrix:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> *const gfc__Matrix4,
    pub setViewport:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *const gfc__Viewport),
    pub getViewport: unsafe extern "thiscall" fn(
        this: *const gfc__KGGraphics,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
        _: *mut i32,
    ),
    pub clear: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: bool,
        _: bool,
        _: bool,
        _: u32,
        _: f32,
        _: u32,
    ),
    pub clear_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub pushClip: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub popClip: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub enableClip: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub disableClip: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub setDepthTestMode: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub setCullMode: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub setBlendMode: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32),
    pub getBlendMode: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> i32,
    pub setBlendFactor: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
    ),
    pub setGamma: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: f32),
    pub setPostProcessEffect: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: u32),
    pub setAntiAliasingType: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: u32),
    pub setTextureFilteringMode: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: u32),
    pub setShader: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *mut gfc__Shader),
    pub getShader: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> *mut gfc__Shader,
    pub setVertexDeclaration:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *mut gfc__VertexDeclaration),
    pub setIndexBuffer:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *mut gfc__IndexBuffer),
    pub setVertexBuffer:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *mut gfc__VertexBuffer),
    pub setVertexBufferMasked:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: *mut gfc__VertexBuffer, _: u32),
    pub drawFullScreenQuad: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
    pub drawRect:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: i32, _: i32, _: i32, _: i32),
    pub drawPrimitiveUP: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: u32,
        _: u32,
        _: *const (),
        _: u32,
    ),
    pub drawPrimitiveRetained: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: u32,
        _: u32,
        _: *const (),
        _: u32,
    ),
    pub drawIndexedPrimitiveUP: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: u32,
        _: u32,
        _: u32,
        _: *const (),
        _: *const (),
        _: u32,
    ),
    pub drawPrimitives:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics, _: u32, _: u32, _: u32),
    pub drawPrimitivesIndexed: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: u32,
        _: u32,
        _: u32,
        _: u32,
        _: u32,
    ),
    pub setRenderTarget: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        _: *mut gfc__Texture,
        _: *mut gfc__Texture,
        _: i32,
    ),
    pub setDefaultRenderTarget: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics),
    pub setDefaultDepthStencilTarget: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics),
    pub createVertexDeclaration:
        unsafe extern "thiscall" fn(
            this: *mut gfc__KGGraphics,
            result: *mut gfc__AutoRef_gfc__VertexDeclaration_,
            _: *const gfc__VertexFormat,
        ) -> *mut gfc__AutoRef_gfc__VertexDeclaration_,
    pub createVertexBuffer: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
        _: i32,
        _: u32,
    )
        -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub createVertexBuffer_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__VertexBuffer_,
    )
        -> *mut gfc__AutoRef_gfc__VertexBuffer_,
    pub createIndexBuffer: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__IndexBuffer_,
        _: u32,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__IndexBuffer_,
    pub createMeshBuilder: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__MeshBuilder_,
    ) -> *mut gfc__AutoRef_gfc__MeshBuilder_,
    pub createStaticMesh: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
        _: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_,
    pub createStaticMesh_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__StaticMesh_,
    ) -> *mut gfc__AutoRef_gfc__StaticMesh_,
    pub createSkinMesh: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
        _: *mut gfc__MeshBuilder,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_,
    pub createSkinMesh_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__SkinMesh_,
    ) -> *mut gfc__AutoRef_gfc__SkinMesh_,
    pub createRenderTexture: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__RenderTexture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    )
        -> *mut gfc__AutoRef_gfc__RenderTexture_,
    pub createRenderTarget: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderTarget_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderDepth: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createRenderCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: u16,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *mut (),
        _: u32,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_2: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *const gfc__String,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_3: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: *mut gfc__Image,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createTexture_4: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: i32,
        _: gfc__ImageFormat,
        _: bool,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createCubemap: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: gfc__ImageFormat,
        _: bool,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createLightmap: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Texture_,
        _: i32,
        _: i32,
        _: gfc__ImageFormat,
    ) -> *mut gfc__AutoRef_gfc__Texture_,
    pub createShader: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Shader_,
    ) -> *mut gfc__AutoRef_gfc__Shader_,
    pub createShaderCompiler: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__ShaderCompiler_,
    )
        -> *mut gfc__AutoRef_gfc__ShaderCompiler_,
    pub createQuery: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Query_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Query_,
    pub createCamera: unsafe extern "thiscall" fn(
        this: *mut gfc__KGGraphics,
        result: *mut gfc__AutoRef_gfc__Camera3D_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Camera3D_,
    pub createRenderer:
        unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics) -> *mut gfc__Renderer,
    pub getPlatform: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> i32,
    pub getDepthBias: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> f32,
    pub getDecalDepthBias: unsafe extern "thiscall" fn(this: *const gfc__KGGraphics) -> f32,
    pub recover: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics) -> bool,
    pub isMeshFormatSupported:
        unsafe extern "thiscall" fn(this: *const gfc__KGGraphics, _: i32) -> bool,
    pub blockUntilDefragged: unsafe extern "thiscall" fn(this: *mut gfc__KGGraphics),
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
pub struct gfc__Vector_gfc__HashTable_unsigned___int64_gfc__HStringManager__StringRef_gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_ {
    pub mData: *mut gfc__HashTable_unsigned___int64_gfc__HStringManager__StringRef_gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
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
pub struct gfc__Vector4Parameter {
    // gfc__IRefObject
    pub vfptr: *const gfc__Vector4Parameter__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Parameter
    pub mName: gfc__HString,
    // gfc__Vector4Parameter
    pub mValue: gfc__TVector4_float_gfc__FloatMath_,
}

unsafe impl UpcastToNop<gfc__Parameter> for gfc__Vector4Parameter {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Vector4Parameter {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Vector4Parameter {}

impl gfc__Vector4Parameter {
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
pub struct gfc__Vector4Parameter__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter) -> i32,
    pub setBool: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: bool, _: f32),
    pub getBool: unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter, _: f32) -> bool,
    pub setFloat: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: f32, _: f32),
    pub getFloat: unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter, _: f32) -> f32,
    pub setUInt32: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: u32, _: f32),
    pub getUInt32: unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter, _: f32) -> u32,
    pub setVector2: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *const gfc__TVector2_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector2: unsafe extern "thiscall" fn(
        this: *const gfc__Vector4Parameter,
        result: *mut gfc__TVector2_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector2_float_gfc__FloatMath_,
    pub setVector3: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector3: unsafe extern "thiscall" fn(
        this: *const gfc__Vector4Parameter,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setVector4: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *const gfc__TVector4_float_gfc__FloatMath_,
        _: f32,
    ),
    pub getVector4: unsafe extern "thiscall" fn(
        this: *const gfc__Vector4Parameter,
        result: *mut gfc__TVector4_float_gfc__FloatMath_,
        _: f32,
    ) -> *mut gfc__TVector4_float_gfc__FloatMath_,
    pub setMatrix4: unsafe extern "thiscall" fn(
        this: *mut gfc__Vector4Parameter,
        _: *const gfc__Matrix4,
        _: f32,
    ),
    pub getMatrix4: unsafe extern "thiscall" fn(
        this: *const gfc__Vector4Parameter,
        result: *mut gfc__Matrix4,
        _: f32,
    ) -> *mut gfc__Matrix4,
    pub setTexture:
        unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: *mut gfc__Texture, _: f32),
    pub getTexture:
        unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: f32) -> *mut gfc__Texture,
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Vector4Parameter, _: i32),
    pub clone:
        unsafe extern "thiscall" fn(this: *const gfc__Vector4Parameter) -> *mut gfc__Parameter,
}

#[repr(C)]
pub struct gfc__HashTable_unsigned___int64_gfc__HStringManager__StringRef_gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator_ {
    pub mHashSize: u32,
    pub mpHashTable: *mut u32,
    pub mPairs: gfc__Vector_gfc__HashTable_unsigned___int64_gfc__HStringManager__StringRef_gfc__Hash_unsigned_long_unsigned___int64__gfc__CAllocator___KeyValuePair_0_gfc__CAllocator_,
    pub mNextAvailable: u32,
    pub mCount: u32,
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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__CameraBlurDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__CameraBlurDesc__10_0_gfc__CAllocator_
{
}

#[repr(C)]
pub struct gfc__Vector_gfc__DynMeshNode___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__DynMeshNode,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VertexDeclaration {
    // gfc__IRefObject
    pub vfptr: *const gfc__VertexDeclaration__vftable,
    pub ReferenceCount: i32,
    // gfc__VertexDeclaration
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__VertexDeclaration {}

impl gfc__VertexDeclaration {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__VertexDeclaration__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__VertexDeclaration, _: u32) -> *mut (),
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

unsafe impl UpcastToNop<gfc__Vector_gfc__AutoRef_gfc__FogDesc__0_gfc__CAllocator_>
    for gfc__FixedVector_gfc__AutoRef_gfc__FogDesc__10_0_gfc__CAllocator_
{
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
    // gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____
}

unsafe impl UpcastToNop<std__map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object_______> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0___> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object____std__allocator_std__pair_gfc__AutoRef_gfc__Object__const__gfc__AutoRef_gfc__Object______0_> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

unsafe impl UpcastToNop<std___Container_base0> for gfc__Map_gfc__AutoRef_gfc__Object__gfc__AutoRef_gfc__Object__std__less_gfc__AutoRef_gfc__Object_____ {}

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
    // gfc__IRefObject
    pub vfptr: *const gfc__ByteInputStream__vftable,
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

unsafe impl UpcastToNop<gfc__InputStream> for gfc__ByteInputStream {}

unsafe impl UpcastToNop<gfc__Stream> for gfc__ByteInputStream {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ByteInputStream {}

impl gfc__ByteInputStream {
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
pub struct gfc__ByteInputStream__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: u32) -> *mut (),
    pub getType:
        unsafe extern "thiscall" fn(this: *const gfc__ByteInputStream) -> gfc__Stream__StreamType,
    pub available: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) -> i64,
    pub close: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream),
    pub markSupported: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) -> bool,
    pub mark: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream),
    pub reset: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream),
    pub seekSupported: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) -> bool,
    pub seek: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: u64, _: i32),
    pub tell: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) -> i64,
    pub skipBytes: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: i32),
    pub clone: unsafe extern "thiscall" fn(
        this: *mut gfc__ByteInputStream,
        result: *mut gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__InputStream_,
    pub getBuffer: unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream) -> *const u8,
    pub read_raw:
        unsafe extern "thiscall" fn(this: *mut gfc__ByteInputStream, _: *mut (), _: i32) -> i32,
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {
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
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter_______> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Parameter______0_> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Parameter__std__less_gfc__HString___
{
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ParticleManager_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Skeleton3D {
    // gfc__IRefObject
    pub vfptr: *const gfc__Skeleton3D__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc__Node3D_
    pub vfptr_2: *const gfc__Skeleton3D__vftable,
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

unsafe impl UpcastToNop<gfc__Node3D> for gfc__Skeleton3D {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Skeleton3D {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Skeleton3D {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc__Node3D_> for gfc__Skeleton3D {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc__Node3D_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__Skeleton3D {
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
}

#[repr(C)]
pub struct gfc__Skeleton3D__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: u32) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
    pub addBack: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D),
    pub remove_2: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D),
    pub added: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
    pub removed: unsafe extern "thiscall" fn(this: *mut gfc__Skeleton3D, _: *mut gfc__Node3D),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__Node3D__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc__Node3D_
    pub vfptr_2: *const gfc__Node3D__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__Node3D {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Node3D {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc__Node3D_> for gfc__Node3D {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc__Node3D_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc__Node3D {
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
}

#[repr(C)]
pub struct gfc__Node3D__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: u32) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
    pub addBack: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc__Node3D),
    pub remove_2: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc__Node3D),
    pub added: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
    pub removed: unsafe extern "thiscall" fn(this: *mut gfc__Node3D, _: *mut gfc__Node3D),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__HDRDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
    // gfc__HDRDesc
    __pdbindgen_padding: [u8; 3],
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

unsafe impl UpcastToNop<gfc__EnvironmentDesc> for gfc__HDRDesc {}

unsafe impl UpcastToNop<gfc__Object> for gfc__HDRDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__HDRDesc {}

impl gfc__HDRDesc {
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
pub struct gfc__HDRDesc__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__HDRDesc) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__HDRDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__HDRDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__HDRDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc, _: *mut gfc__Renderer),
    pub restore: unsafe extern "thiscall" fn(this: *mut gfc__HDRDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct gfc__EnvironmentDesc {
    // gfc__IRefObject
    pub vfptr: *const gfc__EnvironmentDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__EnvironmentDesc
    pub mApplied: bool,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__EnvironmentDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__EnvironmentDesc {}

impl gfc__EnvironmentDesc {
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
pub struct gfc__EnvironmentDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__EnvironmentDesc) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__EnvironmentDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__EnvironmentDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__EnvironmentDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EnvironmentDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub apply: unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
    pub restore:
        unsafe extern "thiscall" fn(this: *mut gfc__EnvironmentDesc, _: *mut gfc__Renderer),
}

#[repr(C)]
pub struct keen__ZoneMemoryAllocator {
    // keen__MemoryAllocator
    pub vfptr: *const keen__ZoneMemoryAllocator__vftable,
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

unsafe impl UpcastToNop<keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_>
    for keen__ZoneMemoryAllocator
{
}

unsafe impl UpcastToNop<keen__MemoryAllocator> for keen__ZoneMemoryAllocator {}

impl keen__ZoneMemoryAllocator {
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
pub struct keen__ZoneMemoryAllocator__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__ZoneMemoryAllocator, _: u32) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__ZoneMemoryAllocator,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(this: *mut keen__ZoneMemoryAllocator, _: *mut ()),
    pub getName: unsafe extern "thiscall" fn(this: *const keen__ZoneMemoryAllocator) -> *const i8,
}

#[repr(C)]
pub struct keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_ {
    // keen__MemoryAllocator
    pub vfptr: *const keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter___vftable,
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

unsafe impl UpcastToNop<keen__MemoryAllocator>
    for keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_
{
}

impl keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_ {
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
pub struct keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter___vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_,
        _: u32,
    ) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(
        this: *mut keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_,
        _: *mut (),
    ),
    pub getName: unsafe extern "thiscall" fn(
        this: *const keen__BaseMemoryAllocator_keen__ZoneAllocatorAdapter_,
    ) -> *const i8,
}

#[repr(C)]
pub struct keen__LowOverheadMemoryAllocator {
    // keen__MemoryAllocator
    pub vfptr: *const keen__LowOverheadMemoryAllocator__vftable,
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

unsafe impl UpcastToNop<keen__BaseMemoryAllocator_keen__LowOverheadAllocator_>
    for keen__LowOverheadMemoryAllocator
{
}

unsafe impl UpcastToNop<keen__MemoryAllocator> for keen__LowOverheadMemoryAllocator {}

impl keen__LowOverheadMemoryAllocator {
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
pub struct keen__LowOverheadMemoryAllocator__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut keen__LowOverheadMemoryAllocator, _: u32) -> *mut (),
    pub allocate: unsafe extern "thiscall" fn(
        this: *mut keen__LowOverheadMemoryAllocator,
        _: u32,
        _: u32,
        _: u32,
        _: *const i8,
    ) -> *mut (),
    pub free: unsafe extern "thiscall" fn(this: *mut keen__LowOverheadMemoryAllocator, _: *mut ()),
    pub getName:
        unsafe extern "thiscall" fn(this: *const keen__LowOverheadMemoryAllocator) -> *const i8,
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
    // std__allocator_std__pair_gfc__String_const__gfc__String___
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__String_const__gfc__String___>
    for std__allocator_std__pair_gfc__String_const__gfc__String___
{
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

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
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_ {}

#[repr(C)]
pub struct std__pair_gfc__String_const__gfc__String_ {
    // std___Pair_base_gfc__String_const__gfc__String_
    pub first: gfc__String,
    pub second: gfc__String,
    // std__pair_gfc__String_const__gfc__String_
}

unsafe impl UpcastToNop<std___Pair_base_gfc__String_const__gfc__String_>
    for std__pair_gfc__String_const__gfc__String_
{
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
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___ {}

#[repr(C)]
pub struct std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____ {}

#[repr(C)]
pub struct gfc__ResourcePackage {
    pub vfptr: *const gfc__ResourcePackage__vftable,
    pub mPercentLoaded: i32,
    pub mOriginalNames: *mut gfc__Map_gfc__HString_gfc__Vector_gfc__HString_0_gfc__CAllocator__std__less_gfc__HString___,
    pub mInfo: *mut gfc__ResourcePackageInfo,
    pub mOutstandingLoads: [i32; 18],
    pub mReadSpeed: f32,
    pub mVideoMemoryUseByType: [u32; 18],
    pub mSystemMemoryUseByType: [u32; 18],
    pub mLoadTimeByType: [u32; 18],
    pub mRead: *mut gfc__ResourcePackageRead,
    pub mAllocQueue: *mut gfc__LockFreeQueue_gfc__Resource___,
    pub mMainThreadQueue: *mut gfc__LockFreeQueue_gfc__Resource___,
    pub mLoadThreadQueue: *mut gfc__LockFreeQueue_gfc__Resource___,
    pub mShaderNames: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mReffedShaders: bool,
    pub mOwnsOriginalResourceNames: bool,
    pub mOwnsQueues: bool,
    pub mBufferState: u8,
    pub mVersion: i32,
}

impl gfc__ResourcePackage {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getOriginalFile(
        &self,
        result: *mut gfc__AutoRef_gfc__File_,
        a2: *const gfc__String,
    ) -> *mut gfc__AutoRef_gfc__File_ {
        ((*self.vfptr).getOriginalFile)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn isUnpacked(&self) -> bool {
        ((*self.vfptr).isUnpacked)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getResourceIndex(
        &self,
        a1: *mut gfc__ResourceIndex,
        a2: *const gfc__ResourceCacheMap,
    ) {
        ((*self.vfptr).getResourceIndex)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn read(
        &self,
        a1: *const gfc__ResourceCacheMap,
        a2: gfc__AutoRef_gfc__OverrideResources_,
    ) -> u32 {
        ((*self.vfptr).read)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn reloadResource(
        &self,
        a1: gfc__AutoRef_gfc__File_,
        a2: *mut gfc__ResourceCache,
    ) {
        ((*self.vfptr).reloadResource)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__ResourcePackage__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourcePackage, _: u32) -> *mut (),
    pub getOriginalFile: unsafe extern "thiscall" fn(
        this: *const gfc__ResourcePackage,
        result: *mut gfc__AutoRef_gfc__File_,
        _: *const gfc__String,
    ) -> *mut gfc__AutoRef_gfc__File_,
    pub isUnpacked: unsafe extern "thiscall" fn(this: *const gfc__ResourcePackage) -> bool,
    pub getResourceIndex: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackage,
        _: *mut gfc__ResourceIndex,
        _: *const gfc__ResourceCacheMap,
    ),
    pub read: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackage,
        _: *const gfc__ResourceCacheMap,
        _: gfc__AutoRef_gfc__OverrideResources_,
    ) -> u32,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackage,
        _: gfc__AutoRef_gfc__File_,
        _: *mut gfc__ResourceCache,
    ),
}

#[repr(C)]
pub struct gfc__MBWeights {
    pub Weight0: gfc__MBWeight,
    pub Weight1: gfc__MBWeight,
    pub Weight2: gfc__MBWeight,
    pub Weight3: gfc__MBWeight,
}

#[repr(C)]
pub struct gfc__ResourcePackageRead {
    pub CurrentExt: i32,
    pub CurrentFile: i32,
    pub CurrentBuffer: i32,
    pub Exts: gfc__Vector_gfc__ResourcePackageRead__ExtInfo_0_gfc__CAllocator_,
    pub Buffers: gfc__Vector_gfc__ResourceBuffer_0_gfc__CAllocator_,
    pub Analysis: gfc__AutoRef_gfc__InputStream_,
    pub OverrideResourceList: gfc__AutoRef_gfc__OverrideResources_,
    pub CacheMap: *const gfc__ResourceCacheMap,
    pub IsFreeingBuffers: bool,
}

#[repr(C)]
pub struct gfc__ResourcePackageRead__ExtInfo {
    pub Ext: gfc__HString,
    pub Files: gfc__Vector_gfc__ResourcePackageRead__FileInfo_0_gfc__CAllocator_,
}

#[repr(C)]
pub struct gfc__ResourcePackageRead__FileInfo {
    pub Name: gfc__HString,
    pub UserData: u32,
    pub NumBuffers: i32,
    pub Offset: u32,
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
pub struct gfc__Vector_gfc__ResourcePackageRead__FileInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ResourcePackageRead__FileInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__MeshBuilder {
    // gfc__IRefObject
    pub vfptr: *const gfc__MeshBuilder__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__MeshBuilder {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MeshBuilder {}

impl gfc__MeshBuilder {
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

    pub unsafe extern "thiscall" fn computeTangentVectors(&self, a1: *mut gfc__MBSubMesh) {
        ((*self.vfptr).computeTangentVectors)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn optimize(&self, a1: *mut gfc__MBSubMesh) -> bool {
        ((*self.vfptr).optimize)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__MeshBuilder__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__MeshBuilder) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__MeshBuilder) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshBuilder,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshBuilder, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshBuilder,
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
    // gfc__ResourceCache
    pub vfptr: *const gfc__MeshCache__vftable,
    pub mExtensions: gfc__Vector_gfc__HString_0_gfc__CAllocator_,
    pub mType: i32,
    pub mPackages: gfc__Vector_gfc__ResourceCache__PackageInfo___0_gfc__CAllocator_,
    // gfc__MeshCache
    pub mReloadInfo: gfc__Vector_gfc__MeshCache__ReloadInfo_0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__ResourceCache> for gfc__MeshCache {}

impl gfc__MeshCache {
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
pub struct gfc__MeshCache__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: u32) -> *mut (),
    pub loadDefaultResource:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: gfc__AutoRef_gfc__File_),
    pub initThread: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache),
    pub shutdownThread: unsafe extern "thiscall" fn(this: *mut gfc__MeshCache),
    pub analyzeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshCache,
        _: *mut gfc__ResourceAnalyzeInfo,
    ) -> bool,
    pub canCreateBuffersInThread:
        unsafe extern "thiscall" fn(this: *const gfc__MeshCache, _: i32) -> bool,
    pub createBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: *mut gfc__ResourceBufferInfo),
    pub freeBuffers:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: *mut gfc__ResourceLoadInfo),
    pub loadResource:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshCache, _: *mut gfc__ResourceLoadInfo),
    pub canReloadResources: unsafe extern "thiscall" fn(this: *const gfc__MeshCache) -> bool,
    pub reloadsQueued: unsafe extern "thiscall" fn(this: *const gfc__MeshCache) -> bool,
    pub reloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshCache,
        _: *mut gfc__ResourceLoadInfo,
    ) -> bool,
    pub needUnlinkResource: unsafe extern "thiscall" fn(this: *const gfc__MeshCache) -> bool,
    pub unlinkResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshCache,
        _: *mut (),
        _: *const gfc__HString,
        _: *const gfc__HString,
    ),
    pub needUnloadResource: unsafe extern "thiscall" fn(this: *const gfc__MeshCache) -> bool,
    pub unloadResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshCache,
        _: *mut (),
        _: *mut gfc__ResourceLoadInfo,
    ),
    pub freeResource: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshCache,
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
    // gfc__IRefObject
    pub vfptr: *const gfc__MeshResourceUnopt__vftable,
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

unsafe impl UpcastToNop<gfc__MeshResource> for gfc__MeshResourceUnopt {}

unsafe impl UpcastToNop<gfc__ResourceType_gfc__Mesh_2_> for gfc__MeshResourceUnopt {}

unsafe impl UpcastToNop<gfc__Resource> for gfc__MeshResourceUnopt {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MeshResourceUnopt {}

impl gfc__MeshResourceUnopt {
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

    pub unsafe extern "thiscall" fn isUnoptimized(&self) -> bool {
        ((*self.vfptr).isUnoptimized)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__MeshResourceUnopt__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__MeshResourceUnopt, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__MeshResourceUnopt) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__MeshResourceUnopt),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__MeshResourceUnopt),
    pub isUnoptimized: unsafe extern "thiscall" fn(this: *const gfc__MeshResourceUnopt) -> bool,
}

#[repr(C)]
pub struct gfc__Vector_gfc__ResourcePackageRead__ExtInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__ResourcePackageRead__ExtInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ResourceType_gfc__Mesh_2_ {
    // gfc__IRefObject
    pub vfptr: *const gfc__ResourceType_gfc__Mesh_2___vftable,
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
    // gfc__ResourceType_gfc__Mesh_2_
    pub mResource: gfc__AutoRef_gfc__Mesh_,
}

unsafe impl UpcastToNop<gfc__Resource> for gfc__ResourceType_gfc__Mesh_2_ {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ResourceType_gfc__Mesh_2_ {}

impl gfc__ResourceType_gfc__Mesh_2_ {
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
pub struct gfc__ResourceType_gfc__Mesh_2___vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceType_gfc__Mesh_2_, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__ResourceType_gfc__Mesh_2_) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__ResourceType_gfc__Mesh_2_),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__ResourceType_gfc__Mesh_2_),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__MeshReader__vftable,
    pub ReferenceCount: i32,
    /* gfc__ObjectReader
     * gfc__MeshReader */
}

unsafe impl UpcastToNop<gfc__ObjectReader> for gfc__MeshReader {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MeshReader {}

impl gfc__MeshReader {
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
pub struct gfc__MeshReader__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MeshReader, _: u32) -> *mut (),
    pub readObject: unsafe extern "thiscall" fn(
        this: *mut gfc__MeshReader,
        result: *mut gfc__AutoRef_gfc__Object_,
        _: gfc__AutoRef_gfc__InputStream_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
}

#[repr(C)]
pub struct gfc__ResourceCacheMap {
    pub vfptr: *const gfc__ResourceCacheMap__vftable,
}

impl gfc__ResourceCacheMap {
    pub unsafe extern "thiscall" fn getCache(
        &self,
        a1: *const gfc__HString,
    ) -> *mut gfc__ResourceCache {
        ((*self.vfptr).getCache)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__ResourceCacheMap__vftable {
    pub getCache: unsafe extern "thiscall" fn(
        this: *const gfc__ResourceCacheMap,
        _: *const gfc__HString,
    ) -> *mut gfc__ResourceCache,
}

#[repr(C)]
pub struct gfc__MBBone {
    pub Offset: gfc__Matrix4,
    pub ID: i32,
    pub Name: gfc__String,
}

#[repr(C)]
pub struct gfc__MBSubMesh {
    // gfc__IRefObject
    pub vfptr: *const gfc__MBSubMesh__vftable,
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

unsafe impl UpcastToNop<gfc__Object> for gfc__MBSubMesh {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MBSubMesh {}

impl gfc__MBSubMesh {
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
pub struct gfc__MBSubMesh__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__MBSubMesh) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__MBSubMesh) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__MBSubMesh,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__MBSubMesh, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__MBSubMesh,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__ResourcePackageInfo {
    // gfc__IRefObject
    pub vfptr: *const gfc__ResourcePackageInfo__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__ResourcePackageInfo
    pub mNameUser: gfc__HString,
    pub mPath: gfc__HString,
    pub mPermanentID: i32,
    pub mLoadTime: i32,
    pub mCategory: i32,
    pub mPermanentDependencies: gfc__Vector_int_0_gfc__CAllocator_,
    pub mRuntimeID: i32,
    pub mRuntimeName: gfc__HString,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub mRefAndState: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
    pub mPackage: *mut gfc__ResourcePackage,
    pub mFlattenedDependencies: gfc__Vector_int_0_gfc__CAllocator_,
    pub mReverseDependencies: gfc__Vector_int_0_gfc__CAllocator_,
    pub mUnloadWaitFrames: i32,
    pub mDefault: bool,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__ResourcePackageInfo {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ResourcePackageInfo {}

impl gfc__ResourcePackageInfo {
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
pub struct gfc__ResourcePackageInfo__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourcePackageInfo, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__ResourcePackageInfo) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourcePackageInfo, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__ResourcePackageInfo) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourcePackageInfo) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackageInfo,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourcePackageInfo) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackageInfo,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ResourcePackageInfo,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__ObjectWriter {
    // gfc__IRefObject
    pub vfptr: *const gfc__ObjectWriter__vftable,
    pub ReferenceCount: i32,
    // gfc__ObjectWriter
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ObjectWriter {}

impl gfc__ObjectWriter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__ObjectWriter__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ObjectWriter, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__MeshResource__vftable,
    pub ReferenceCount: i32,
    // gfc__Resource
    pub mState: i32,
    pub mPackageID: i32,
    // gfc__ResourceType_gfc__Mesh_2_
    pub mResource: gfc__AutoRef_gfc__Mesh_,
    // gfc__MeshResource
    pub mName: gfc__HString,
}

unsafe impl UpcastToNop<gfc__ResourceType_gfc__Mesh_2_> for gfc__MeshResource {}

unsafe impl UpcastToNop<gfc__Resource> for gfc__MeshResource {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__MeshResource {}

impl gfc__MeshResource {
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

    pub unsafe extern "thiscall" fn isUnoptimized(&self) -> bool {
        ((*self.vfptr).isUnoptimized)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__MeshResource__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__MeshResource, _: u32) -> *mut (),
    pub getType: unsafe extern "thiscall" fn(this: *const gfc__MeshResource) -> i32,
    pub finalize: unsafe extern "thiscall" fn(this: *mut gfc__MeshResource),
    pub unload: unsafe extern "thiscall" fn(this: *mut gfc__MeshResource),
    pub isUnoptimized: unsafe extern "thiscall" fn(this: *const gfc__MeshResource) -> bool,
}

#[repr(C)]
pub struct gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__String___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___
    // std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____
    // gfc__Map_gfc__String_gfc__String_std__less_gfc__String___
}

unsafe impl UpcastToNop<std__map_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String_____> for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0___> for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__String_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__String____0_> for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__String_gfc__String_std__less_gfc__String___
{
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__Font_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Font___ {
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Font_,
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_ {}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc___UIControl___ {
    // std___Pair_base_gfc__HString_const__gfc___UIControl___
    pub first: gfc__HString,
    pub second: *mut gfc___UIControl,
    // std__pair_gfc__HString_const__gfc___UIControl___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc___UIControl___>
    for std__pair_gfc__HString_const__gfc___UIControl___
{
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<
        std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
    > for std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____
{
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_> for std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_ {}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_ {}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_> for std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______ {}

#[repr(C)]
pub struct std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font___ {
    // std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Font___
    pub first: gfc__HString,
    pub second: gfc__AutoRef_gfc__Font_,
    // std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__HString_const__gfc__AutoRef_gfc__Font___>
    for std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font___
{
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    // std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_> for std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______ {}

#[repr(C)]
pub struct std___Pair_base_gfc__HString_const__gfc___UIControl___ {
    pub first: gfc__HString,
    pub second: *mut gfc___UIControl,
}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc___UIControl_____
    // std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc___UIControl_____>
    for std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____
{
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc___UIControl_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_ {}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_> for std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_ {}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____>
    for std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____
{
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node {
    pub _Left: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Parent: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Right: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Myval: std__pair_gfc__HString_const__gfc___UIControl___,
    pub _Color: i8,
    pub _Isnil: i8,
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____>
    for std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____
{
}

#[repr(C)]
pub struct gfc__AutoRef_gfc___UIControl_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Darksiders {
    // gfc__OblivionGame
    pub vfptr: *const gfc__Darksiders__vftable,
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
    // gfc__Darksiders
    __pdbindgen_padding_2: [u8; 7],
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

unsafe impl UpcastToNop<gfc__OblivionGame> for gfc__Darksiders {}

impl gfc__Darksiders {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onDraw(&self) {
        ((*self.vfptr).onDraw)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDrawUI(&self) {
        ((*self.vfptr).onDrawUI)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPostDraw(&self) {
        ((*self.vfptr).onPostDraw)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreVideo(&self) {
        ((*self.vfptr).onPreVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPostVideo(&self) {
        ((*self.vfptr).onPostVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onStartup(&self) {
        ((*self.vfptr).onStartup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onShutdown(&self) {
        ((*self.vfptr).onShutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoss(&self) {
        ((*self.vfptr).onLoss)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onRecovery(&self) {
        ((*self.vfptr).onRecovery)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreWorldDestroy(&self) {
        ((*self.vfptr).onPreWorldDestroy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onFinalizeDeoverrideWorld(&self) {
        ((*self.vfptr).onFinalizeDeoverrideWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onFinalizeWorld(&self) {
        ((*self.vfptr).onFinalizeWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreUpdate(&self, a1: f32) {
        ((*self.vfptr).onPreUpdate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onPreUpdateInterval(&self, a1: f32) {
        ((*self.vfptr).onPreUpdateInterval)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUpdateWorld(&self, a1: f32, a2: f32) {
        ((*self.vfptr).onUpdateWorld)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onPostUpdateInterval(&self, a1: f32) {
        ((*self.vfptr).onPostUpdateInterval)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onPostUpdate(&self, a1: f32) {
        ((*self.vfptr).onPostUpdate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCharacterClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getCharacterClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn startParticleMultithreadUpdate(&self) {
        ((*self.vfptr).startParticleMultithreadUpdate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn waitParticleMultithreadUpdate(&self) {
        ((*self.vfptr).waitParticleMultithreadUpdate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCorePackages(
        &self,
        a1: *mut gfc__Vector_int_0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).getCorePackages)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPaused(&self, a1: bool) {
        ((*self.vfptr).setPaused)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMinimized(&self, a1: bool) {
        ((*self.vfptr).setMinimized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFOV(&self) -> f32 {
        ((*self.vfptr).getFOV)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNearDistance(&self) -> f32 {
        ((*self.vfptr).getNearDistance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFarDistance(&self) -> f32 {
        ((*self.vfptr).getFarDistance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCameraMatrix(
        &self,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4 {
        ((*self.vfptr).getCameraMatrix)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPlayerPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        a2: bool,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPlayerPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setResolution(&self, a1: u16, a2: u16, a3: u16, a4: u16) {
        ((*self.vfptr).setResolution)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn hasOverSamplingChanged(&self) -> bool {
        ((*self.vfptr).hasOverSamplingChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFullscreen(&self, a1: bool) {
        ((*self.vfptr).setFullscreen)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unloadWorldManagers(&self) {
        ((*self.vfptr).unloadWorldManagers)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadRegions(
        &self,
        a1: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a2: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a3: bool,
    ) {
        ((*self.vfptr).loadRegions)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn playVideo(
        &self,
        a1: *const gfc__HString,
        a2: *mut gfc__Object,
    ) -> bool {
        ((*self.vfptr).playVideo)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopVideo(&self) {
        ((*self.vfptr).stopVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn abortVideo(&self) {
        ((*self.vfptr).abortVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onCinematicStart(&self, a1: *mut gfc__Cinematic) {
        ((*self.vfptr).onCinematicStart)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onCinematicStop(&self, a1: *mut gfc__Cinematic) {
        ((*self.vfptr).onCinematicStop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateUI(&self, a1: f32) {
        ((*self.vfptr).updateUI)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateProfile(&self) {
        ((*self.vfptr).updateProfile)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateInputDevices(&self, a1: f32) {
        ((*self.vfptr).updateInputDevices)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cleanup(&self, a1: bool) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn changeResolution(&self, a1: u16, a2: u16, a3: bool) {
        ((*self.vfptr).changeResolution)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn createWorldManager(&self) -> *mut gfc__WorldManager {
        ((*self.vfptr).createWorldManager)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initWorldManager(&self) {
        ((*self.vfptr).initWorldManager)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn finalizeWorld(&self) {
        ((*self.vfptr).finalizeWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadConfigSettings(&self) {
        ((*self.vfptr).loadConfigSettings)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupEnvSettings(&self) {
        ((*self.vfptr).setupEnvSettings)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onFinalizePreviewWorld(&self) {
        ((*self.vfptr).onFinalizePreviewWorld)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Darksiders__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: u32) -> *mut (),
    pub onDraw: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onDrawUI: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onPostDraw: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onPreVideo: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onPostVideo: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onStartup: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onShutdown: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onLoss: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onRecovery: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onPreWorldDestroy: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onFinalizeDeoverrideWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onFinalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onPreUpdate: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub onPreUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub onUpdateWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32, _: f32),
    pub onPostUpdateInterval: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub onPostUpdate: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub getCharacterClass:
        unsafe extern "thiscall" fn(this: *mut gfc__Darksiders) -> *mut gfc__Class,
    pub startParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub waitParticleMultithreadUpdate: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub getCorePackages: unsafe extern "thiscall" fn(
        this: *mut gfc__Darksiders,
        _: *mut gfc__Vector_int_0_gfc__CAllocator_,
    ),
    pub setPaused: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: bool),
    pub setMinimized: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: bool),
    pub getFOV: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders) -> f32,
    pub getNearDistance: unsafe extern "thiscall" fn(this: *const gfc__Darksiders) -> f32,
    pub getFarDistance: unsafe extern "thiscall" fn(this: *const gfc__Darksiders) -> f32,
    pub getCameraMatrix: unsafe extern "thiscall" fn(
        this: *mut gfc__Darksiders,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4,
    pub getPlayerPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Darksiders,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        _: bool,
    )
        -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: u16, _: u16, _: u16, _: u16),
    pub hasOverSamplingChanged: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders) -> bool,
    pub setFullscreen: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: bool),
    pub unloadWorldManagers: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub loadRegions: unsafe extern "thiscall" fn(
        this: *mut gfc__Darksiders,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        _: bool,
    ),
    pub playVideo: unsafe extern "thiscall" fn(
        this: *mut gfc__Darksiders,
        _: *const gfc__HString,
        _: *mut gfc__Object,
    ) -> bool,
    pub stopVideo: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub abortVideo: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onCinematicStart:
        unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *mut gfc__Cinematic),
    pub onCinematicStop:
        unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: *mut gfc__Cinematic),
    pub updateUI: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub updateProfile: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub updateInputDevices: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: f32),
    pub cleanup: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: bool),
    pub changeResolution:
        unsafe extern "thiscall" fn(this: *mut gfc__Darksiders, _: u16, _: u16, _: bool),
    pub createWorldManager:
        unsafe extern "thiscall" fn(this: *const gfc__Darksiders) -> *mut gfc__WorldManager,
    pub initWorldManager: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub finalizeWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub loadConfigSettings: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub setupEnvSettings: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
    pub onFinalizePreviewWorld: unsafe extern "thiscall" fn(this: *mut gfc__Darksiders),
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value_______> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Value______0_> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___
{
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
    // gfc__IRefObject
    pub vfptr: *const gfc__WorldManager__vftable,
    pub ReferenceCount: i32,
    // gfc__WorldManager
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

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldManager {}

impl gfc__WorldManager {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn shutdown(&self) {
        ((*self.vfptr).shutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadRegions(
        &self,
        a1: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a2: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a3: bool,
    ) {
        ((*self.vfptr).loadRegions)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn onRestore(&self) {
        ((*self.vfptr).onRestore)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preAddToWorld(&self) {
        ((*self.vfptr).preAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn postAddToWorld(&self) {
        ((*self.vfptr).postAddToWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preRemoveFromWorld(&self) {
        ((*self.vfptr).preRemoveFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupInitialLoad(
        &self,
        a1: *mut gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a2: i32,
    ) {
        ((*self.vfptr).setupInitialLoad)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__WorldManager__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WorldManager, _: u32) -> *mut (),
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
    // gfc___UIEvent
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
    // gfc__KeyboardEvent
    pub mKeyCode: i32,
    pub mChar: i8,
    pub mShiftPressed: bool,
    pub mCtrlPressed: bool,
    pub mAltPressed: bool,
    pub mStateChange: bool,
}

unsafe impl UpcastToNop<gfc___UIEvent> for gfc__KeyboardEvent {}

#[repr(C)]
pub struct gfc__Hierarchical_gfc___UIControl_ {
    pub vfptr: *const gfc__Hierarchical_gfc___UIControl___vftable,
    pub mParent: *mut gfc___UIControl,
    pub mHead: gfc__AutoRef_gfc___UIControl_,
    pub mTail: gfc__AutoRef_gfc___UIControl_,
    pub mNext: gfc__AutoRef_gfc___UIControl_,
    pub mPrev: gfc__AutoRef_gfc___UIControl_,
}

impl gfc__Hierarchical_gfc___UIControl_ {
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
}

#[repr(C)]
pub struct gfc__Hierarchical_gfc___UIControl___vftable {
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
    // gfc___UIEvent
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
    // gfc__FocusEvent
}

unsafe impl UpcastToNop<gfc___UIEvent> for gfc__FocusEvent {}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font_______> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Font______0_> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Font__std__less_gfc__HString___
{
}

#[repr(C)]
pub struct gfc__ScriptClass {
    // gfc__IRefObject
    pub vfptr: *const gfc__ScriptClass__vftable,
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
    // gfc__ScriptClass
    pub mPackageID: i32,
    pub mDefaultValues: gfc__Map_gfc__HString_gfc__AutoRef_gfc__Value__std__less_gfc__HString___,
    pub mStates: gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___,
}

unsafe impl UpcastToNop<gfc__Class> for gfc__ScriptClass {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ScriptClass {}

impl gfc__ScriptClass {
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
pub struct gfc__ScriptClass__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ScriptClass, _: u32) -> *mut (),
    pub getTypeID: unsafe extern "thiscall" fn(this: *mut gfc__ScriptClass) -> i32,
    pub getName: unsafe extern "thiscall" fn(this: *const gfc__ScriptClass) -> *const gfc__HString,
    pub getParent: unsafe extern "thiscall" fn(this: *const gfc__ScriptClass) -> *mut gfc__Class,
    pub instanceof:
        unsafe extern "thiscall" fn(this: *const gfc__ScriptClass, _: *const gfc__Class) -> bool,
    pub isAbstract: unsafe extern "thiscall" fn(this: *const gfc__ScriptClass) -> bool,
    pub newInstance: unsafe extern "thiscall" fn(
        this: *mut gfc__ScriptClass,
        result: *mut gfc__AutoRef_gfc__Object_,
    ) -> *mut gfc__AutoRef_gfc__Object_,
    pub getPropertyByName: unsafe extern "thiscall" fn(
        this: *const gfc__ScriptClass,
        _: *const gfc__HString,
        _: *mut *const gfc__Class,
    ) -> *mut gfc__Property,
    pub getPropertyByID: unsafe extern "thiscall" fn(
        this: *const gfc__ScriptClass,
        _: *const u64,
        _: *mut *const gfc__Class,
    ) -> *mut gfc__Property,
    pub getMethodByName: unsafe extern "thiscall" fn(
        this: *mut gfc__ScriptClass,
        _: *const gfc__HString,
    ) -> *mut gfc__Method,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__ScriptClass, _: *const u64) -> *mut gfc__Method,
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
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState_______> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__ScriptState______0_> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__AutoRef_gfc__ScriptState__std__less_gfc__HString___
{
}

#[repr(C)]
pub struct gfc__MouseEvent {
    // gfc___UIEvent
    pub mEventID: i32,
    pub mContext: i32,
    pub mSource: *mut gfc___UIControl,
    // gfc__MouseEvent
    pub mLocation: gfc__TVector2_int_gfc__FloatMath_,
    pub mButton: u8,
    pub mClickCount: u8,
    pub mScrollDelta: i32,
    pub mShiftPressed: bool,
    pub mCtrlPressed: bool,
    pub mAltPressed: bool,
}

unsafe impl UpcastToNop<gfc___UIEvent> for gfc__MouseEvent {}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ScriptState_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__VisScriptModule___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__VisScriptModule,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Helper {
    // gfc__IRefObject
    pub vfptr: *const gfc__Helper__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Helper
    pub mIsIterating: bool,
    pub mListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mSystemListeners: gfc__Vector_gfc__AutoRef_gfc__Object__0_gfc__CAllocator_,
    pub mQueuedListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
    pub mQueuedSystemListeners: gfc__Vector_gfc__Helper__QueuedListenerInfo_0_gfc__CAllocator_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Helper {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Helper {}

impl gfc__Helper {
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
pub struct gfc__Helper__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Helper, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Helper) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Helper, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Helper) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Helper) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Helper,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Helper) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Helper, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Helper,
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
    pub vfptr: *const gfc__OblivionGame__vftable,
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

impl gfc__OblivionGame {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onDraw(&self) {
        ((*self.vfptr).onDraw)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onDrawUI(&self) {
        ((*self.vfptr).onDrawUI)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPostDraw(&self) {
        ((*self.vfptr).onPostDraw)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreVideo(&self) {
        ((*self.vfptr).onPreVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPostVideo(&self) {
        ((*self.vfptr).onPostVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onStartup(&self) {
        ((*self.vfptr).onStartup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onShutdown(&self) {
        ((*self.vfptr).onShutdown)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onLoss(&self) {
        ((*self.vfptr).onLoss)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onRecovery(&self) {
        ((*self.vfptr).onRecovery)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreWorldDestroy(&self) {
        ((*self.vfptr).onPreWorldDestroy)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onFinalizeDeoverrideWorld(&self) {
        ((*self.vfptr).onFinalizeDeoverrideWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onFinalizeWorld(&self) {
        ((*self.vfptr).onFinalizeWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onPreUpdate(&self, a1: f32) {
        ((*self.vfptr).onPreUpdate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onPreUpdateInterval(&self, a1: f32) {
        ((*self.vfptr).onPreUpdateInterval)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onUpdateWorld(&self, a1: f32, a2: f32) {
        ((*self.vfptr).onUpdateWorld)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onPostUpdateInterval(&self, a1: f32) {
        ((*self.vfptr).onPostUpdateInterval)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onPostUpdate(&self, a1: f32) {
        ((*self.vfptr).onPostUpdate)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCharacterClass(&self) -> *mut gfc__Class {
        ((*self.vfptr).getCharacterClass)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn startParticleMultithreadUpdate(&self) {
        ((*self.vfptr).startParticleMultithreadUpdate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn waitParticleMultithreadUpdate(&self) {
        ((*self.vfptr).waitParticleMultithreadUpdate)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCorePackages(
        &self,
        a1: *mut gfc__Vector_int_0_gfc__CAllocator_,
    ) {
        ((*self.vfptr).getCorePackages)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPaused(&self, a1: bool) {
        ((*self.vfptr).setPaused)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMinimized(&self, a1: bool) {
        ((*self.vfptr).setMinimized)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getFOV(&self) -> f32 {
        ((*self.vfptr).getFOV)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getNearDistance(&self) -> f32 {
        ((*self.vfptr).getNearDistance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getFarDistance(&self) -> f32 {
        ((*self.vfptr).getFarDistance)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getCameraMatrix(
        &self,
        result: *mut gfc__Matrix4,
    ) -> *mut gfc__Matrix4 {
        ((*self.vfptr).getCameraMatrix)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getPlayerPosition(
        &self,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
        a2: bool,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getPlayerPosition)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setResolution(&self, a1: u16, a2: u16, a3: u16, a4: u16) {
        ((*self.vfptr).setResolution)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn hasOverSamplingChanged(&self) -> bool {
        ((*self.vfptr).hasOverSamplingChanged)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setFullscreen(&self, a1: bool) {
        ((*self.vfptr).setFullscreen)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn unloadWorldManagers(&self) {
        ((*self.vfptr).unloadWorldManagers)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadRegions(
        &self,
        a1: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a2: *const gfc__Vector_gfc__RegionLoadInfo_0_gfc__CAllocator_,
        a3: bool,
    ) {
        ((*self.vfptr).loadRegions)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn playVideo(
        &self,
        a1: *const gfc__HString,
        a2: *mut gfc__Object,
    ) -> bool {
        ((*self.vfptr).playVideo)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn stopVideo(&self) {
        ((*self.vfptr).stopVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn abortVideo(&self) {
        ((*self.vfptr).abortVideo)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn onCinematicStart(&self, a1: *mut gfc__Cinematic) {
        ((*self.vfptr).onCinematicStart)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn onCinematicStop(&self, a1: *mut gfc__Cinematic) {
        ((*self.vfptr).onCinematicStop)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateUI(&self, a1: f32) {
        ((*self.vfptr).updateUI)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateProfile(&self) {
        ((*self.vfptr).updateProfile)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn updateInputDevices(&self, a1: f32) {
        ((*self.vfptr).updateInputDevices)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cleanup(&self, a1: bool) {
        ((*self.vfptr).cleanup)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn changeResolution(&self, a1: u16, a2: u16, a3: bool) {
        ((*self.vfptr).changeResolution)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn createWorldManager(&self) -> *mut gfc__WorldManager {
        ((*self.vfptr).createWorldManager)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn initWorldManager(&self) {
        ((*self.vfptr).initWorldManager)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn finalizeWorld(&self) {
        ((*self.vfptr).finalizeWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn loadConfigSettings(&self) {
        ((*self.vfptr).loadConfigSettings)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setupEnvSettings(&self) {
        ((*self.vfptr).setupEnvSettings)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__OblivionGame__vftable {
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
    // gfc__IRefObject
    pub vfptr: *const gfc___UIControl__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Hierarchical_gfc___UIControl_
    pub vfptr_2: *const gfc___UIControl__vftable,
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
}

unsafe impl UpcastToNop<gfc__Object> for gfc___UIControl {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc___UIControl {}

unsafe impl UpcastTo<gfc__Hierarchical_gfc___UIControl_> for gfc___UIControl {
    fn upcast_to(p: *const Self) -> *const gfc__Hierarchical_gfc___UIControl_ {
        (p as usize + 0x8) as *const _
    }
}

impl gfc___UIControl {
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
pub struct gfc___UIControl__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: u32) -> *mut (),
    pub addFront: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub addBack: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub add: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub remove: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub remove_2: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub clear: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
    pub added: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub removed: unsafe extern "thiscall" fn(this: *mut gfc___UIControl, _: *mut gfc___UIControl),
    pub invalidateHierarchy: unsafe extern "thiscall" fn(this: *mut gfc___UIControl),
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
pub struct gfc___UIManager {
    pub vfptr: *const gfc___UIManager__vftable,
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
}

impl gfc___UIManager {
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
}

#[repr(C)]
pub struct gfc___UIManager__vftable {
    pub init: unsafe extern "thiscall" fn(this: *mut gfc___UIManager),
    pub shutdown: unsafe extern "thiscall" fn(this: *mut gfc___UIManager),
    pub onLoss: unsafe extern "thiscall" fn(this: *mut gfc___UIManager),
    pub onRecovery: unsafe extern "thiscall" fn(this: *mut gfc___UIManager),
    pub addControl:
        unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *mut gfc___UIControl, _: bool),
    pub removeControl:
        unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *mut gfc___UIControl),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: f32),
    pub playSound: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *const gfc__String),
    pub setHideUI: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: bool),
    pub isUIHidden: unsafe extern "thiscall" fn(this: *const gfc___UIManager) -> bool,
    pub drawControlsInternal:
        unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: *mut gfc__UIRenderer),
    pub updateInternal: unsafe extern "thiscall" fn(this: *mut gfc___UIManager, _: f32),
}

#[repr(C)]
pub struct gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc___UIControl_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___
    // std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______
    // gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl_______> for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0___> for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc___UIControl___std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc___UIControl______0_> for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc___UIControl___std__less_gfc__HString___
{
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
pub struct gfc__AutoRef_gfc__UITimer_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__GameCamera_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct List_gfc__AutoRef_gfc__Object___ {
    pub mList: *mut List_gfc__AutoRef_gfc__Object_____ListNode,
    pub mTail: *mut List_gfc__AutoRef_gfc__Object_____ListNode,
    pub mSize: i32,
}

#[repr(C)]
pub struct List_gfc__AutoRef_gfc__Object_____ListNode {
    pub next: *mut List_gfc__AutoRef_gfc__Object_____ListNode,
    pub value: gfc__AutoRef_gfc__Object_,
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
    pub vfptr: *const unit4__SaveDataDescriptionInterface__vftable,
}

impl unit4__SaveDataDescriptionInterface {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getSaveDataSize(&self) -> u32 {
        ((*self.vfptr).getSaveDataSize)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isSaveDataValid(&self, a1: *const ()) -> bool {
        ((*self.vfptr).isSaveDataValid)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn initializeSaveDataToDefault(&self, a1: *mut ()) {
        ((*self.vfptr).initializeSaveDataToDefault)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct unit4__SaveDataDescriptionInterface__vftable {
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
pub struct List_gfc__AutoRef_gfc___UIControl___ {
    pub mList: *mut List_gfc__AutoRef_gfc___UIControl_____ListNode,
    pub mTail: *mut List_gfc__AutoRef_gfc___UIControl_____ListNode,
    pub mSize: i32,
}

#[repr(C)]
pub struct List_gfc__AutoRef_gfc___UIControl_____ListNode {
    pub next: *mut List_gfc__AutoRef_gfc___UIControl_____ListNode,
    pub value: gfc__AutoRef_gfc___UIControl_,
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
    pub vfptr: *const hkpPhantomOverlapListener__vftable,
}

impl hkpPhantomOverlapListener {
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
pub struct hkpPhantomOverlapListener__vftable {
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
    pub vfptr: *const hkpShapeCollectionFilter__vftable,
}

impl hkpShapeCollectionFilter {
    pub unsafe extern "thiscall" fn isCollisionEnabled(
        &self,
        result: *mut hkBool,
        a2: *const hkpCollisionInput,
        a3: *const hkpCdBody,
        a4: *const hkpCdBody,
        a5: *const hkpShapeContainer,
        a6: *const hkpShapeContainer,
        a7: u32,
        a8: u32,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled)(
            self as *const _ as *mut _,
            result,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8,
        )
    }

    pub unsafe extern "thiscall" fn isCollisionEnabled_2(
        &self,
        result: *mut hkBool,
        a2: *const hkpCollisionInput,
        a3: *const hkpCdBody,
        a4: *const hkpCdBody,
        a5: *const hkpShapeContainer,
        a6: u32,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled_2)(self as *const _ as *mut _, result, a2, a3, a4, a5, a6)
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

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpShapeCollectionFilter__vftable {
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
    // hkArrayBase_hkpWorldExtension___
    pub m_data: *mut *mut hkpWorldExtension,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpWorldExtension___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpWorldExtension___>
    for hkArray_hkpWorldExtension___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_> for std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_
    // std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_> for std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_ {}

#[repr(C)]
pub struct std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
    pub comp: std__less_gfc__String_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_ {}

#[repr(C)]
pub struct std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______ {
    // std___Allocator_base_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______
    // std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______>
    for std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______
{
}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___ {
    // std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___
    // std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std__pair_gfc__String_const__gfc__StateMapValue___>
    for std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___
{
}

#[repr(C)]
pub struct std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____ {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
{
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

#[repr(C)]
pub struct std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
{
    // std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
    // std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_
    __pdbindgen_padding: [u8; 1],
}

unsafe impl UpcastToNop<std___Allocator_base_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_> for std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_ {}

#[repr(C)]
pub struct std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
{
    // std___Container_base0
    // std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
    pub comp: gfc__PhysicsEffects__ContactCmp,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_ {}

#[repr(C)]
pub struct std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {
    // std___Container_base0
    // std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
    pub comp: gfc__PhysicsEffects__ContactCmp,
    // std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    pub _Myhead: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_,
    pub _Alval: std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______,
    // std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    // std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_> for std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

#[repr(C)]
pub struct std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_> for std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

#[repr(C)]
pub struct std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {
    // std___Container_base0
    // std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
    pub comp: gfc__PhysicsEffects__ContactCmp,
    // std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    pub _Myhead: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_,
    pub _Alval: std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______,
}

unsafe impl UpcastToNop<std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_> for std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

#[repr(C)]
pub struct std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {
    // std___Container_base0
    // std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
    pub comp: gfc__PhysicsEffects__ContactCmp,
    // std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    pub _Myhead: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_,
    pub _Alval: std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______,
    // std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
}

unsafe impl UpcastToNop<std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_> for std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___ {}

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
pub struct std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____,
}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

unsafe impl UpcastToNop<std___Container_base0> for std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___ {}

#[repr(C)]
pub struct std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {
    // std___Container_base0
    // std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_
    pub comp: std__less_gfc__String_,
    // std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__String_const__gfc__StateMapValue___,
    // std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    // std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___
    // std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0___> for std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue____0_> for std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__String_gfc__StateMapValue_std__less_gfc__String__std__allocator_std__pair_gfc__String_const__gfc__StateMapValue_____ {}

#[repr(C)]
pub struct std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____ {
    // std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____
    // std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____
    __pdbindgen_padding: [u8; 1],
}

unsafe impl
    UpcastToNop<std___Allocator_base_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____>
    for std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____
{
}

#[repr(C)]
pub struct std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
    pub first: *mut gfc__Class,
    pub second: gfc__AutoRef_gfc__WorldComponent_,
}

#[repr(C)]
pub struct std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
{
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
}

unsafe impl UpcastToNop<std___Container_base0> for std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_ {}

#[repr(C)]
pub struct std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {
    // std___Container_base0
    // std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_
    pub comp: gfc__PhysicsEffects__ContactCmp,
    // std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    pub _Myhead: *mut std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0______Node_,
    pub _Alval: std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______,
    // std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    // std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___
    // std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________
}

unsafe impl UpcastToNop<std___Tree_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0___> for std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {}

unsafe impl UpcastToNop<std___Tset_traits_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo_______0_> for std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {}

unsafe impl UpcastToNop<std___Container_base0> for std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________ {}

#[repr(C)]
pub struct std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___ {
    // std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___
    pub first: *mut gfc__Class,
    pub second: gfc__AutoRef_gfc__WorldComponent_,
    // std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___
}

unsafe impl UpcastToNop<std___Pair_base_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___>
    for std__pair_gfc__Class___const_gfc__AutoRef_gfc__WorldComponent___
{
}

#[repr(C)]
pub struct std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______
}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_> for std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {}

unsafe impl UpcastToNop<std___Container_base0> for std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______ {}

#[repr(C)]
pub struct hkSmallArray_hkConstraintInternal_ {
    pub m_data: *mut hkConstraintInternal,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpWorldPostIntegrateListener___
    pub m_data: *mut *mut hkpWorldPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpWorldPostIntegrateListener___>
    for hkArray_hkpWorldPostIntegrateListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpWorldPostSimulationListener___
    pub m_data: *mut *mut hkpWorldPostSimulationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpWorldPostSimulationListener___>
    for hkArray_hkpWorldPostSimulationListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpSimpleContactConstraintDataInfo {
    pub m_flags: u16,
    pub m_biNormalAxis: u16,
    pub m_rollingFrictionMultiplier: hkHalf,
    pub m_internalData1: hkHalf,
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_rhsRolling: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 4],
    pub m_contactRadius: f32,
    pub m_data: [f32; 4],
}

#[repr(C)]
pub struct hkpSimpleContactConstraintAtom {
    // hkpConstraintAtom
    pub m_type: hkEnum_enum_hkpConstraintAtom__AtomType_unsigned_short_,
    // hkpSimpleContactConstraintAtom
    pub m_sizeOfAllAtoms: u16,
    pub m_numContactPoints: u16,
    pub m_numReservedContactPoints: u16,
    pub m_numUserDatasForBodyA: u8,
    pub m_numUserDatasForBodyB: u8,
    pub m_contactPointPropertiesStriding: u8,
    pub m_maxNumContactPoints: u16,
    pub m_info: hkpSimpleContactConstraintDataInfo,
}

unsafe impl UpcastToNop<hkpConstraintAtom> for hkpSimpleContactConstraintAtom {}

#[repr(C)]
pub struct hkpRayShapeCollectionFilter {
    pub vfptr: *const hkpRayShapeCollectionFilter__vftable,
}

impl hkpRayShapeCollectionFilter {
    pub unsafe extern "thiscall" fn isCollisionEnabled(
        &self,
        result: *mut hkBool,
        a2: *const hkpShapeRayCastInput,
        a3: *const hkpShapeContainer,
        a4: u32,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled)(self as *const _ as *mut _, result, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpRayShapeCollectionFilter__vftable {
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
    // hkArrayBase_hkpPhantomOverlapListener___
    pub m_data: *mut *mut hkpPhantomOverlapListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpPhantomOverlapListener___>
    for hkArray_hkpPhantomOverlapListener___hkContainerHeapAllocator_
{
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
    // hkpLinkedCollidable
    pub m_collisionEntries: hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_,
}

unsafe impl UpcastToNop<hkpCollidable> for hkpLinkedCollidable {}

unsafe impl UpcastToNop<hkpCdBody> for hkpLinkedCollidable {}

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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_potentialContacts: compile_error!("unimplemented class layout"),
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
    // hkArrayBase_hkpConstraintListener___
    pub m_data: *mut *mut hkpConstraintListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpConstraintListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpConstraintListener___>
    for hkArray_hkpConstraintListener___hkContainerHeapAllocator_
{
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
    // hkArrayBase_hkpContactImpulseLimitBreachedListener___
    pub m_data: *mut *mut hkpContactImpulseLimitBreachedListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpContactImpulseLimitBreachedListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpContactImpulseLimitBreachedListener___>
    for hkArray_hkpContactImpulseLimitBreachedListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpMaxSizeMotion {
    // hkBaseObject
    pub vfptr: *const hkpMaxSizeMotion__vftable,
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_deactivationRefPosition: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 32],
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
    /* hkpKeyframedRigidMotion
     * hkpMaxSizeMotion */
}

unsafe impl UpcastToNop<hkpKeyframedRigidMotion> for hkpMaxSizeMotion {}

unsafe impl UpcastToNop<hkpMotion> for hkpMaxSizeMotion {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpMaxSizeMotion {}

unsafe impl UpcastToNop<hkBaseObject> for hkpMaxSizeMotion {}

impl hkpMaxSizeMotion {
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

    pub unsafe extern "thiscall" fn setMass(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMass)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMass_2(&self, a1: f32) {
        ((*self.vfptr).setMass_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMassInv)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv_2(&self, a1: f32) {
        ((*self.vfptr).setMassInv_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaInvLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setCenterOfMassInLocal(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setCenterOfMassInLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: *const hkQuaternionf) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPositionAndRotation(
        &self,
        a1: *const hkVector4f,
        a2: *const hkQuaternionf,
    ) {
        ((*self.vfptr).setPositionAndRotation)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setTransform(&self, a1: *const hkTransformf) {
        ((*self.vfptr).setTransform)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLinearVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setLinearVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setAngularVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setAngularVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocity(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut f32,
        a4: *mut f32,
    ) {
        ((*self.vfptr).getProjectedPointVelocity)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocitySimd(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut hkSimdFloat32,
        a4: *mut hkSimdFloat32,
    ) {
        ((*self.vfptr).getProjectedPointVelocitySimd)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn applyLinearImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyLinearImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyPointImpulse(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
    ) {
        ((*self.vfptr).applyPointImpulse)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyAngularImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyAngularImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyForce(
        &self,
        a1: f32,
        a2: *const hkVector4f,
        a3: *const hkVector4f,
    ) {
        ((*self.vfptr).applyForce)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn applyForce_2(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyForce_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyTorque(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyTorque)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMotionStateAndVelocitiesAndDeactivationType(
        &self,
        a1: *mut hkpMotion,
    ) {
        ((*self.vfptr).getMotionStateAndVelocitiesAndDeactivationType)(
            self as *const _ as *mut _,
            a1,
        )
    }

    pub unsafe extern "thiscall" fn setStepPosition(&self, a1: f32, a2: f32) {
        ((*self.vfptr).setStepPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setStoredMotion(&self, a1: *mut hkpMaxSizeMotion) {
        ((*self.vfptr).setStoredMotion)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpMaxSizeMotion__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: u32) -> *mut (),
    pub __first_virtual_table_function__: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion),
    pub getClassType: unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion),
    pub setMass: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkSimdFloat32),
    pub setMass_2: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: f32),
    pub setMassInv:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkSimdFloat32),
    pub setMassInv_2: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: f32),
    pub getInertiaLocal:
        unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion, _: *mut hkMatrix3f),
    pub getInertiaWorld:
        unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion, _: *mut hkMatrix3f),
    pub setInertiaLocal:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkMatrix3f),
    pub setInertiaInvLocal:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkMatrix3f),
    pub getInertiaInvLocal:
        unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion, _: *mut hkMatrix3f),
    pub getInertiaInvWorld:
        unsafe extern "thiscall" fn(this: *const hkpMaxSizeMotion, _: *mut hkMatrix3f),
    pub setCenterOfMassInLocal:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub setPosition: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub setRotation:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkQuaternionf),
    pub setPositionAndRotation: unsafe extern "thiscall" fn(
        this: *mut hkpMaxSizeMotion,
        _: *const hkVector4f,
        _: *const hkQuaternionf,
    ),
    pub setTransform:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkTransformf),
    pub setLinearVelocity:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub setAngularVelocity:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub getProjectedPointVelocity: unsafe extern "thiscall" fn(
        this: *const hkpMaxSizeMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut f32,
        _: *mut f32,
    ),
    pub getProjectedPointVelocitySimd: unsafe extern "thiscall" fn(
        this: *const hkpMaxSizeMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut hkSimdFloat32,
        _: *mut hkSimdFloat32,
    ),
    pub applyLinearImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub applyPointImpulse: unsafe extern "thiscall" fn(
        this: *mut hkpMaxSizeMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyAngularImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *const hkVector4f),
    pub applyForce: unsafe extern "thiscall" fn(
        this: *mut hkpMaxSizeMotion,
        _: f32,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyForce_2:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: f32, _: *const hkVector4f),
    pub applyTorque:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: f32, _: *const hkVector4f),
    pub getMotionStateAndVelocitiesAndDeactivationType:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *mut hkpMotion),
    pub setStepPosition: unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: f32, _: f32),
    pub setStoredMotion:
        unsafe extern "thiscall" fn(this: *mut hkpMaxSizeMotion, _: *mut hkpMaxSizeMotion),
}

#[repr(C)]
pub struct hkpTypedBroadPhaseHandle {
    // hkpBroadPhaseHandle
    pub m_id: u32,
    // hkpTypedBroadPhaseHandle
    pub m_type: i8,
    pub m_ownerOffset: i8,
    pub m_objectQualityType: i8,
    pub m_collisionFilterInfo: u32,
}

unsafe impl UpcastToNop<hkpBroadPhaseHandle> for hkpTypedBroadPhaseHandle {}

#[repr(C)]
pub struct hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_ {
    // hkArrayBase_unsigned_char_
    pub m_data: *mut u8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_unsigned_char_hkContainerHeapAllocator_
    // hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_
    pub m_storage: [u8; 8],
}

unsafe impl UpcastToNop<hkArray_unsigned_char_hkContainerHeapAllocator_>
    for hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_
{
}

unsafe impl UpcastToNop<hkArrayBase_unsigned_char_>
    for hkInplaceArray_unsigned_char_8_hkContainerHeapAllocator_
{
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
    pub vfptr: *const hkpContactListener__vftable,
}

impl hkpContactListener {
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
}

#[repr(C)]
pub struct hkpContactListener__vftable {
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
    // hkJob
    pub m_jobSubType: u8,
    pub m_jobType: hkEnum_enum_hkJobType_unsigned_char_,
    pub m_jobSpuType: hkEnum_enum_hkJobSpuType_unsigned_char_,
    pub m_size: u16,
    pub m_threadAffinity: i16,
    // hkJobQueue__JobQueueEntry
    __pdbindgen_padding: [u8; 8],
    pub m_data: [u8; 112],
}

unsafe impl UpcastToNop<hkJob> for hkJobQueue__JobQueueEntry {}

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
    // hkArrayBase_hkpContactListener___
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpContactListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpContactListener___>
    for hkArray_hkpContactListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkpContactListener___ {
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkGskCache16 {
    // hkpGskCache
    pub m_vertices: [u16; 4],
    pub m_dimA: u8,
    pub m_dimB: u8,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_maxDimA: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub m_maxDimB: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 1],
    pub m_gskFlags: u8,
    // hkGskCache16
    __pdbindgen_padding_2: [u8; 4],
}

unsafe impl UpcastToNop<hkpGskCache> for hkGskCache16 {}

#[repr(C)]
pub struct hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_ {
    // hkArrayBase_hkJobQueue__CustomJobTypeSetup_
    pub m_data: *mut hkJobQueue__CustomJobTypeSetup,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkJobQueue__CustomJobTypeSetup_>
    for hkArray_hkJobQueue__CustomJobTypeSetup_hkContainerHeapAllocator_
{
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
    // hkBaseObject
    pub vfptr: *const hkpConstraintOwner__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintOwner
    pub m_constraintInfo: hkpConstraintInfo,
}

unsafe impl UpcastToNop<hkReferencedObject> for hkpConstraintOwner {}

unsafe impl UpcastToNop<hkBaseObject> for hkpConstraintOwner {}

impl hkpConstraintOwner {
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
pub struct hkpConstraintOwner__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut hkpConstraintOwner, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpConstraintOwner),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpConstraintOwner) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkpConstraintOwner),
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
    // hkArrayBase_char_
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_char_hkContainerTempAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_char_> for hkArray_char_hkContainerTempAllocator_ {}

#[repr(C)]
pub struct hkpCdBody {
    pub m_shape: *const hkpShape,
    pub m_shapeKey: u32,
    pub m_motion: *const (),
    pub m_parent: *const hkpCdBody,
}

#[repr(C)]
pub struct hkArray_hkpActionListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpActionListener___
    pub m_data: *mut *mut hkpActionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpActionListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpActionListener___>
    for hkArray_hkpActionListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkpPhantomListener___ {
    pub m_data: *mut *mut hkpPhantomListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArrayBase_hkVector4f_ {
    pub m_data: *mut hkVector4f,
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
    pub vfptr: *const hkpRayCollidableFilter__vftable,
}

impl hkpRayCollidableFilter {
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
}

#[repr(C)]
pub struct hkpRayCollidableFilter__vftable {
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
    pub vfptr: *const hkpBroadPhaseListener__vftable,
}

impl hkpBroadPhaseListener {
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
}

#[repr(C)]
pub struct hkpBroadPhaseListener__vftable {
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
    // hkArrayBase_hkViewPtr_hkpConstraintInstance___
    pub m_data: *mut hkViewPtr_hkpConstraintInstance_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkViewPtr_hkpConstraintInstance___>
    for hkArray_hkViewPtr_hkpConstraintInstance__hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct gfc__AutoRef_gfc__ScriptFunctions_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__EmitterInstance {
    pub Transform: gfc__Matrix4,
    pub Offset: gfc__Matrix4,
    pub RaycastTo: gfc__TVector3_float_gfc__FloatMath_,
    pub LastPosition: [f32; 3],
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub LinearVelocity: [f32; 3],
    pub LastOrientation: [f32; 4],
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub AngularVelocity: [f32; 3],
    pub ChannelId: i32,
    pub SoundChannel: i32,
    pub PoolSeed: u32,
    pub TotalAge: f32,
    pub EffectDuration: f32,
    pub TimeSinceSpawn: f32,
    pub EyeDistanceSqr: f32,
    pub Fade: f32,
    pub TrailLength: f32,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub Power: f32,
    pub RefNode: *mut gfc__Node3D,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub RefObject: *mut gfc__Object3D,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub RefParticle: gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub AttachTo: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub WorldRelRotation: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub SortType: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub DoRaycast: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub CalcAngularVelocity: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub ParticleType: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlaggedForDelete: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub Stopping: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub DisableEmit: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub DrawFailed: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub UseAlternateTime: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 8],
    pub ParticleList: gfc__EmitterInstance___unnamed_type_ParticleList_,
    pub WorldPtr: *mut gfc__World,
    pub SceneObjectPtr: *mut gfc__EmitterSceneObject,
    pub EmitterPtr: *mut gfc__Emitter,
    __pdbindgen_padding_2: [u8; 8],
}

#[repr(C)]
pub struct gfc__EmitterInstance___unnamed_type_ParticleList_ {
    pub SpriteHead: gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub MeshHead: gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub TrailHead: gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_,
    pub SpriteTail: gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub MeshTail: gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_,
    #[cfg(pdb_issue = "can\'t lay out type accurately")]
    pub TrailTail: gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_,
    pub Size: u32,
}

#[repr(C)]
pub struct gfc__LockFreePoolMarker_gfc__EmitterInstance_ {
    pub Object: *mut gfc__EmitterInstance,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub State: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___ {
    pub Value: *mut gfc__PhysicsEffects__ContactInfo,
    pub Next: *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub planes: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 1024],
}

#[repr(C)]
pub struct gfc__VisScriptVariable {
    // gfc__IRefObject
    pub vfptr: *const gfc__VisScriptVariable__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__VisScriptEntity
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
    // gfc__VisScriptVariable
    pub mName: gfc__HString,
    pub mRead: bool,
    pub mWrite: bool,
    pub mExternal: bool,
    pub mModContainerModule: *mut gfc__ModSysContainerModule,
    pub mConnectionID: u32,
}

unsafe impl UpcastToNop<gfc__VisScriptEntity> for gfc__VisScriptVariable {}

unsafe impl UpcastToNop<gfc__Object> for gfc__VisScriptVariable {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__VisScriptVariable {}

impl gfc__VisScriptVariable {
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

    pub unsafe extern "thiscall" fn getColor(
        &self,
        result: *mut gfc__TVector4_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector4_float_gfc__FloatMath_ {
        ((*self.vfptr).getColor)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn getVariableType(&self) -> i32 {
        ((*self.vfptr).getVariableType)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn isArray(&self) -> bool {
        ((*self.vfptr).isArray)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getVariableValue(
        &self,
        result: *mut gfc__AutoRef_gfc__Value_,
    ) -> *mut gfc__AutoRef_gfc__Value_ {
        ((*self.vfptr).getVariableValue)(self as *const _ as *mut _, result)
    }

    pub unsafe extern "thiscall" fn setVariableValue(&self, a1: gfc__AutoRef_gfc__Value_) {
        ((*self.vfptr).setVariableValue)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__VisScriptVariable__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__VisScriptVariable) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__VisScriptVariable) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptVariable,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptVariable,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptVariable,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptVariable) -> *const i8,
    pub compile:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable),
    pub clearDeadLinks:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptVariable, _: *mut gfc__ModuleSystem),
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
pub struct gfc__AutoRef_gfc__PhysicsEffectsDesc_ {
    pub p: *mut gfc__IRefObject,
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
pub struct gfc__LockFreePoolMarker_gfc__TrailParticle_ {
    pub Object: *mut gfc__TrailParticle,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub State: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__EmitterInstance_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__Occluder {
    pub Center: gfc__TVector3_float_gfc__FloatMath_,
    pub Planes: *mut gfc__Plane,
    pub PlaneCount: i32,
}

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__MeshParticle_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__FullScreenFXGroup__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__FullScreenFXGroup_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance__0_gfc__CAllocator_
{
    pub mData: *mut gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhysicsEffects__FXParticleManager__ParticleInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysicsEffects__FXParticleManager__ParticleInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__ModuleSystem {
    // gfc__IRefObject
    pub vfptr: *const gfc__ModuleSystem__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__ModuleSystem
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

unsafe impl UpcastToNop<gfc__Object> for gfc__ModuleSystem {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__ModuleSystem {}

impl gfc__ModuleSystem {
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

    pub unsafe extern "thiscall" fn start(&self) {
        ((*self.vfptr).start)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn compile(&self) {
        ((*self.vfptr).compile)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__ModuleSystem__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__ModuleSystem) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__ModuleSystem) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__ModuleSystem,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__ModuleSystem,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__ModuleSystem,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub start: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem),
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__ModuleSystem),
}

#[repr(C)]
pub struct gfc__MeshParticle {
    // gfc__Particle
    pub Position: [f32; 3],
    pub Velocity: [f32; 3],
    pub SpinRate: f32,
    pub SpinAngle: f32,
    pub Age: f32,
    pub Life: f32,
    pub Size: f32,
    pub Mass: f32,
    pub EmitterInst: *mut gfc__EmitterInstance,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipX: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipY: compile_error!("unimplemented type kind"),
    // gfc__MeshParticle
    __pdbindgen_padding: [u8; 4],
    pub EmitOrientation: gfc__Quaternion,
    pub SizeOverLife: f32,
    pub StretchOverLife: f32,
    pub xv: [f32; 3],
    pub yv: [f32; 3],
    pub zv: [f32; 3],
    pub wv: [f32; 3],
    pub Prev: gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_,
    pub Next: gfc__LockFreePoolHandle_gfc__MeshParticle_gfc__MeshParticle_,
}

unsafe impl UpcastToNop<gfc__Particle> for gfc__MeshParticle {}

#[repr(C)]
pub struct gfc__AutoRef_gfc__PSystem_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__Vector_gfc__AutoRef_gfc__CameraCinematicGroup__0_gfc__CAllocator_ {
    pub mData: *mut gfc__AutoRef_gfc__CameraCinematicGroup_,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VisScriptModule {
    // gfc__IRefObject
    pub vfptr: *const gfc__VisScriptModule__vftable,
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
}

unsafe impl UpcastToNop<gfc__VisScriptEntity> for gfc__VisScriptModule {}

unsafe impl UpcastToNop<gfc__Object> for gfc__VisScriptModule {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__VisScriptModule {}

impl gfc__VisScriptModule {
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
pub struct gfc__VisScriptModule__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, result: *mut gfc__HString) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *mut gfc__ObjectCloner, _: gfc__AutoRef_gfc__Object_),
    pub getUILabel: unsafe extern "thiscall" fn(this: *const gfc__VisScriptModule) -> *const i8,
    pub compile: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *mut gfc__ModuleSystem),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *mut gfc__Object),
    pub end: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule),
    pub clearDeadLinks: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptModule, _: *mut gfc__ModuleSystem),
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
pub struct gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__RigidBody,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhysicsEffects__FXSoundManager__SoundInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysicsEffects__FXSoundManager__SoundInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__VisScriptEntity {
    // gfc__IRefObject
    pub vfptr: *const gfc__VisScriptEntity__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__VisScriptEntity
    pub mID: u32,
    pub mComment: gfc__HString,
    pub mLocationX: i32,
    pub mLocationY: i32,
    pub mModuleSystem: *mut gfc__ModuleSystem,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__VisScriptEntity {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__VisScriptEntity {}

impl gfc__VisScriptEntity {
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
}

#[repr(C)]
pub struct gfc__VisScriptEntity__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__VisScriptEntity) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptEntity,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__VisScriptEntity) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptEntity,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__VisScriptEntity,
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
pub struct gfc__AutoRef_gfc__ScriptFunction_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LockFreePoolMarker_gfc__SpriteParticle_ {
    pub Object: *mut gfc__SpriteParticle,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub State: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct gfc__SpriteParticle {
    // gfc__Particle
    pub Position: [f32; 3],
    pub Velocity: [f32; 3],
    pub SpinRate: f32,
    pub SpinAngle: f32,
    pub Age: f32,
    pub Life: f32,
    pub Size: f32,
    pub Mass: f32,
    pub EmitterInst: *mut gfc__EmitterInstance,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipX: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipY: compile_error!("unimplemented type kind"),
    // gfc__SpriteParticle
    __pdbindgen_padding: [u8; 4],
    pub OldPosition: [f32; 3],
    pub SpinCenter: [f32; 2],
    pub Frame: f32,
    pub Prev: gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_,
    pub Next: gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_,
}

unsafe impl UpcastToNop<gfc__Particle> for gfc__SpriteParticle {}

#[repr(C)]
pub struct gfc__AutoRef_gfc__VisScriptEntity_ {
    pub p: *mut gfc__IRefObject,
}

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__TrailParticle_gfc__TrailParticle_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__TrailParticle_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__PhysicsMaterialInteraction {
    // gfc__IRefObject
    pub vfptr: *const gfc__PhysicsMaterialInteraction__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__PhysicsMaterialInteraction
    pub mSmallHitSoundID: i32,
    pub mHitSoundID: i32,
    pub mSlideSoundID: i32,
    pub mRollSoundID: i32,
    pub mHitParticle: gfc__HString,
    pub mHitPSystem: gfc__AutoRef_gfc__PSystem_,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__PhysicsMaterialInteraction {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PhysicsMaterialInteraction {}

impl gfc__PhysicsMaterialInteraction {
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
pub struct gfc__PhysicsMaterialInteraction__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsMaterialInteraction, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(
        this: *const gfc__PhysicsMaterialInteraction,
    ) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsMaterialInteraction,
        _: *const gfc__HString,
    ),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsMaterialInteraction) -> *const (),
    pub getScriptData_2:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsMaterialInteraction) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsMaterialInteraction,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsMaterialInteraction,
    ) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsMaterialInteraction,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsMaterialInteraction,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
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
pub struct gfc__Vector_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__PhysicsManager {
    // gfc__IRefObject
    pub vfptr: *const gfc__PhysicsManager__vftable,
    pub ReferenceCount: i32,
    // gfc__PhysicsManager
    pub mRigidBodies: gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_,
    pub mRagdolls: gfc__Vector_gfc__RagdollMapper___0_gfc__CAllocator_,
    pub mKeyframedBodies: gfc__Vector_gfc__PhysicsManager__KeyframedBodyInfo_0_gfc__CAllocator_,
    pub mDynamicBodies: gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_,
    pub mPhantomBodies: gfc__Vector_gfc__PhantomBody___0_gfc__CAllocator_,
    pub mCachedAdds: gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_,
    pub mCachedAddsConstraints: gfc__Vector_hkpConstraintInstance___0_gfc__CAllocator_,
    pub mCachedRemoves: gfc__Vector_gfc__RigidBody___0_gfc__CAllocator_,
    pub mWorld: *mut hkpWorld,
    pub mTimeStep: f32,
    pub mSteps: i32,
    pub mEffects: *mut gfc__PhysicsEffects,
    pub mDebrisMgr: *mut gfc__DebrisManager,
    pub mNextSystem: i32,
    pub mJobQueue: *mut hkJobQueue,
    pub mJobThreadPool: *mut hkThreadPool,
    pub mJobSemaphore: *mut (),
    pub mVisualDebuggerPort: i32,
    pub mHavokVisualDebugger: *mut gfc__HavokVisualDebugger,
    pub mHavokWorldExtents: gfc__TBox_float_gfc__FloatMath_,
    pub mUseSharedVisualDebugger: bool,
    pub mCacheAddRemoves: bool,
    pub mUseStatistics: bool,
}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PhysicsManager {}

impl gfc__PhysicsManager {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__PhysicsManager__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsManager, _: u32) -> *mut (),
}

#[repr(C)]
pub struct gfc__PhysicsManager__KeyframedBodyInfo {
    pub mBody: *mut gfc__RigidBody,
    pub mCachedPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mCachedRotation: gfc__Quaternion,
    pub mHasCached: bool,
}

#[repr(C)]
pub struct gfc__LockFreePoolHandle_gfc__SpriteParticle_gfc__SpriteParticle_ {
    pub mMarker: *const gfc__LockFreePoolMarker_gfc__SpriteParticle_,
    pub mVersion: u32,
}

#[repr(C)]
pub struct gfc__Vector_gfc__RagdollMapper___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__RagdollMapper,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

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
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipX: compile_error!("unimplemented type kind"),
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub FlipY: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct gfc__Visual {
    // gfc__IRefObject
    pub vfptr: *const gfc__Visual__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Visual
    pub mFadeStartDistance: f32,
    pub mFadeEndDistance: f32,
    pub mInvertFade: bool,
    pub mLightGroup: u32,
    pub mCharacterShadow: bool,
    pub mForceFade: f32,
    pub mObjectColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mObject: *mut gfc__Object3D,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__Visual {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Visual {}

impl gfc__Visual {
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

    pub unsafe extern "thiscall" fn isStatic(&self) -> bool {
        ((*self.vfptr).isStatic)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn preload(&self, a1: i32) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMaterialOverride(
        &self,
        a1: *const gfc__HString,
        a2: gfc__AutoRef_gfc__Material_,
    ) {
        ((*self.vfptr).setMaterialOverride)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setMaterial(
        &self,
        a1: *const gfc__HString,
        a2: gfc__AutoRef_gfc__Material_,
    ) {
        ((*self.vfptr).setMaterial)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn clearMaterialOverride(&self) {
        ((*self.vfptr).clearMaterialOverride)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaterialCount(&self) -> i32 {
        ((*self.vfptr).getMaterialCount)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn getMaterialAt(
        &self,
        result: *mut gfc__AutoRef_gfc__Material_,
        a2: i32,
    ) -> *mut gfc__AutoRef_gfc__Material_ {
        ((*self.vfptr).getMaterialAt)(self as *const _ as *mut _, result, a2)
    }

    pub unsafe extern "thiscall" fn setMaterialAt(&self, a1: i32, a2: gfc__AutoRef_gfc__Material_) {
        ((*self.vfptr).setMaterialAt)(self as *const _ as *mut _, a1, a2)
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

    pub unsafe extern "thiscall" fn getForceFade(&self) -> f32 {
        ((*self.vfptr).getForceFade)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setForceFade(&self, a1: f32) {
        ((*self.vfptr).setForceFade)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getObjectColor(
        &self,
    ) -> *const gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getObjectColor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObjectColor(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setObjectColor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRenderLightGroup(&self) -> u32 {
        ((*self.vfptr).getRenderLightGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setCastShadows(&self, a1: bool) {
        ((*self.vfptr).setCastShadows)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCastShadows(&self) -> bool {
        ((*self.vfptr).getCastShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAlphaShadows(&self, a1: bool) {
        ((*self.vfptr).setAlphaShadows)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAlphaShadows(&self) -> bool {
        ((*self.vfptr).getAlphaShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self, a1: bool) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Visual__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Visual) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Visual) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Visual) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Visual, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Visual,
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
pub struct gfc__Emitter {
    // gfc__IRefObject
    pub vfptr: *const gfc__Emitter__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Emitter
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

unsafe impl UpcastToNop<gfc__Object> for gfc__Emitter {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Emitter {}

impl gfc__Emitter {
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

    pub unsafe extern "thiscall" fn preload(&self, a1: i32, a2: *mut gfc__PSystem) {
        ((*self.vfptr).preload)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn unload(&self) {
        ((*self.vfptr).unload)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn createSceneObject(
        &self,
        a1: gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_,
    ) -> *mut gfc__EmitterSceneObject {
        ((*self.vfptr).createSceneObject)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setupEmitter(&self, a1: *mut gfc__EmitterInstance) {
        ((*self.vfptr).setupEmitter)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn updateEmitterST(&self, a1: *mut gfc__EmitterInstance, a2: f32) {
        ((*self.vfptr).updateEmitterST)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn updateEmitter(&self, a1: *mut gfc__EmitterInstance, a2: f32) {
        ((*self.vfptr).updateEmitter)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn submit(
        &self,
        a1: *mut gfc__EmitterInstance,
        a2: *mut gfc__Camera3D,
        a3: bool,
        a4: u32,
    ) {
        ((*self.vfptr).submit)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn render(
        &self,
        a1: *mut gfc__EmitterInstance,
        a2: *mut gfc__Camera3D,
        a3: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).render)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn prepGeometry(
        &self,
        a1: *mut gfc__EmitterInstance,
        a2: *mut gfc__RenderNode,
    ) {
        ((*self.vfptr).prepGeometry)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn onDestroy(&self, a1: *mut gfc__EmitterInstance, a2: bool) {
        ((*self.vfptr).onDestroy)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn convertEmitter(&self) -> bool {
        ((*self.vfptr).convertEmitter)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__Emitter__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Emitter) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Emitter) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Emitter) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Emitter, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Emitter,
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
pub struct gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {
    // std___Container_base0
    // std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_
    pub comp: std__less_gfc__HString_,
    // std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    pub _Myhead: *mut std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node,
    pub _Mysize: u32,
    pub _Alnod: std__allocator_std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0______Node_,
    pub _Alval: std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_____,
    // std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    // std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___
    // std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______
    // gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___
}

unsafe impl UpcastToNop<std__map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample_______> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_val_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tree_nod_std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0___> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Tmap_traits_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString__std__allocator_std__pair_gfc__HString_const__gfc__AutoRef_gfc__Sample______0_> for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___ {}

unsafe impl UpcastToNop<std___Container_base0>
    for gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___
{
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
pub struct gfc__HavokVisualDebugger {
    pub mVisualDebugger: *mut hkVisualDebugger,
    pub mVisualDebuggerContext: *mut hkpPhysicsContext,
}

#[repr(C)]
pub struct gfc__ResourceListener {
    pub vfptr: *const gfc__ResourceListener__vftable,
}

impl gfc__ResourceListener {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn packageUnloading(&self, a1: i32) {
        ((*self.vfptr).packageUnloading)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct gfc__ResourceListener__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: u32) -> *mut (),
    pub packageUnloading: unsafe extern "thiscall" fn(this: *mut gfc__ResourceListener, _: i32),
}

#[repr(C)]
pub struct gfc__StaticMeshVisual {
    // gfc__IRefObject
    pub vfptr: *const gfc__StaticMeshVisual__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__Visual
    pub mFadeStartDistance: f32,
    pub mFadeEndDistance: f32,
    pub mInvertFade: bool,
    pub mLightGroup: u32,
    pub mCharacterShadow: bool,
    pub mForceFade: f32,
    pub mObjectColor: gfc__TVector3_float_gfc__FloatMath_,
    pub mObject: *mut gfc__Object3D,
    // gfc__SceneObject
    pub vfptr_2: *const gfc__StaticMeshVisual__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_3: *const gfc__StaticMeshVisual__vftable,
    pub mLocked: bool,
    // gfc__StaticMeshVisual
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

unsafe impl UpcastToNop<gfc__Visual> for gfc__StaticMeshVisual {}

unsafe impl UpcastToNop<gfc__Object> for gfc__StaticMeshVisual {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__StaticMeshVisual {}

unsafe impl UpcastTo<gfc__SceneObject> for gfc__StaticMeshVisual {
    fn upcast_to(p: *const Self) -> *const gfc__SceneObject {
        (p as usize + 0x30) as *const _
    }
}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__StaticMeshVisual {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x80) as *const _
    }
}

impl gfc__StaticMeshVisual {
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

    pub unsafe extern "thiscall" fn getForceFade(&self) -> f32 {
        ((*self.vfptr).getForceFade)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setForceFade(&self, a1: f32) {
        ((*self.vfptr).setForceFade)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getObjectColor(
        &self,
    ) -> *const gfc__TVector3_float_gfc__FloatMath_ {
        ((*self.vfptr).getObjectColor)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setObjectColor(
        &self,
        a1: *const gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).setObjectColor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getRenderLightGroup(&self) -> u32 {
        ((*self.vfptr).getRenderLightGroup)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setCastShadows(&self, a1: bool) {
        ((*self.vfptr).setCastShadows)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getCastShadows(&self) -> bool {
        ((*self.vfptr).getCastShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setAlphaShadows(&self, a1: bool) {
        ((*self.vfptr).setAlphaShadows)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getAlphaShadows(&self) -> bool {
        ((*self.vfptr).getAlphaShadows)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn attach(&self, a1: *mut gfc__Object3D) {
        ((*self.vfptr).attach)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn detach(&self) {
        ((*self.vfptr).detach)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn addToWorld(&self, a1: bool) {
        ((*self.vfptr).addToWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFromWorld(&self) {
        ((*self.vfptr).removeFromWorld)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn setHide(&self, a1: bool) {
        ((*self.vfptr).setHide)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setDoNotSubmit(&self, a1: bool) {
        ((*self.vfptr).setDoNotSubmit)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getDoNotSubmit(&self) -> bool {
        ((*self.vfptr).getDoNotSubmit)(self as *const _ as *mut _)
    }
}

#[repr(C)]
pub struct gfc__StaticMeshVisual__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub getContext:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> bool,
    pub getForceFade: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> f32,
    pub setForceFade: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: f32),
    pub getObjectColor: unsafe extern "thiscall" fn(
        this: *const gfc__StaticMeshVisual,
    )
        -> *const gfc__TVector3_float_gfc__FloatMath_,
    pub setObjectColor: unsafe extern "thiscall" fn(
        this: *mut gfc__StaticMeshVisual,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRenderLightGroup: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual) -> u32,
    pub setCastShadows: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub getCastShadows: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
    pub setAlphaShadows: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub getAlphaShadows: unsafe extern "thiscall" fn(this: *const gfc__StaticMeshVisual) -> bool,
    pub attach:
        unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: *mut gfc__Object3D),
    pub detach: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual),
    pub addToWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual, _: bool),
    pub removeFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__StaticMeshVisual),
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
pub struct gfc__Vector_hkpConstraintInstance___0_gfc__CAllocator_ {
    pub mData: *mut *mut hkpConstraintInstance,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
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
pub struct gfc__LockFreeQueue_gfc__PhysicsEffects__ContactInfo___ {
    pub mHead: *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
    pub mPops: u32,
    pub mTail: *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
    pub mPushes: u32,
    pub mDummyNode: gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
}

#[repr(C)]
pub struct gfc__SceneObject {
    pub vfptr: *const gfc__SceneObject__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
}

impl gfc__SceneObject {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn removeFromScene(&self) {
        ((*self.vfptr).removeFromScene)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn cacheBoundingVolume(&self) {
        ((*self.vfptr).cacheBoundingVolume)(self as *const _ as *mut _)
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

    pub unsafe extern "thiscall" fn cull(&self, a1: *const gfc__Clipper) -> bool {
        ((*self.vfptr).cull)(self as *const _ as *mut _, a1)
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
pub struct gfc__SceneObject__vftable {
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
pub struct gfc__Vector_gfc__PhysicsManager__KeyframedBodyInfo_0_gfc__CAllocator_ {
    pub mData: *mut gfc__PhysicsManager__KeyframedBodyInfo,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct gfc__DetectorObject {
    // gfc__IRefObject
    pub vfptr: *const gfc__DetectorObject__vftable,
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
}

unsafe impl UpcastToNop<gfc__PhysicsShapeObject> for gfc__DetectorObject {}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__DetectorObject {}

unsafe impl UpcastToNop<gfc__Object> for gfc__DetectorObject {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__DetectorObject {}

impl gfc__DetectorObject {
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
pub struct gfc__DetectorObject__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__DetectorObject) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__DetectorObject) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__DetectorObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject),
    pub getGizmoColor: unsafe extern "thiscall" fn(
        this: *const gfc__DetectorObject,
    )
        -> *const gfc__TVector4_float_gfc__FloatMath_,
    pub getGizmoIcon:
        unsafe extern "thiscall" fn(this: *const gfc__DetectorObject) -> *const gfc__HString,
    pub getPhantomBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__DetectorObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub onEnter: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__Actor),
    pub onExit: unsafe extern "thiscall" fn(this: *mut gfc__DetectorObject, _: *mut gfc__Actor),
}

#[repr(C)]
pub struct gfc__Vector_gfc__PhantomBody___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__PhantomBody,
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
pub struct gfc__PhysicsEffectsDesc {
    // gfc__IRefObject
    pub vfptr: *const gfc__PhysicsEffectsDesc__vftable,
    pub ReferenceCount: i32,
    // gfc__Object
    // gfc__PhysicsEffectsDesc
    pub mMaxPlayingParticles: i32,
    pub mMaxPlayingSounds: i32,
    pub mMaxPlayingExternalParticles: i32,
    pub mMaxPlayingExternalSounds: i32,
    pub mMinTimeBetweenHits: f32,
    pub mStepsBeforeStartSlide: i32,
    pub mStepsBeforeStopSlide: i32,
    pub mHitMinVelocity: f32,
    pub mHitMaxVelocity: f32,
    pub mSmallHitVolume: f32,
    pub mNormalSeparationVelocity: f32,
    pub mSlidingThreshold: f32,
    pub mRollingContactMaxVelocity: f32,
    pub mOffNormalCOMRelativeVelocityThreshold: f32,
}

unsafe impl UpcastToNop<gfc__Object> for gfc__PhysicsEffectsDesc {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PhysicsEffectsDesc {}

impl gfc__PhysicsEffectsDesc {
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
pub struct gfc__PhysicsEffectsDesc__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffectsDesc, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsEffectsDesc) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffectsDesc, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsEffectsDesc) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffectsDesc) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffectsDesc,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffectsDesc) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffectsDesc,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffectsDesc,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
}

#[repr(C)]
pub struct gfc__SceneManager {
    // gfc__IRefObject
    pub vfptr: *const gfc__SceneManager__vftable,
    pub ReferenceCount: i32,
    // gfc__SceneManager
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

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__SceneManager {}

impl gfc__SceneManager {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn cull(
        &self,
        a1: *const gfc__Matrix4,
        a2: *const gfc__Frustum,
        a3: *mut gfc__Camera3D,
        a4: bool,
        a5: *mut gfc__SceneObserver,
        a6: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) {
        ((*self.vfptr).cull)(self as *const _ as *mut _, a1, a2, a3, a4, a5, a6)
    }

    pub unsafe extern "thiscall" fn startCullingJobs(
        &self,
        a1: *mut gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
        a2: *const gfc__Clipper,
        a3: i32,
    ) {
        ((*self.vfptr).startCullingJobs)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn waitForCullingJobs(&self) {
        ((*self.vfptr).waitForCullingJobs)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn cacheBoundingVolumes(&self) {
        ((*self.vfptr).cacheBoundingVolumes)(self as *const _ as *mut _)
    }

    pub unsafe extern "thiscall" fn cacheBoundingVolumes_2(
        &self,
        a1: *mut gfc__Vector_gfc__SceneObject___0_gfc__CAllocator_,
        a2: i32,
    ) {
        ((*self.vfptr).cacheBoundingVolumes_2)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__SceneManager__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__SceneManager, _: u32) -> *mut (),
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
    // gfc__IRefObject
    pub vfptr: *const gfc__PhysicsShapeObject__vftable,
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
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__PhysicsShapeObject {}

unsafe impl UpcastToNop<gfc__Object> for gfc__PhysicsShapeObject {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__PhysicsShapeObject {}

impl gfc__PhysicsShapeObject {
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
}

#[repr(C)]
pub struct gfc__PhysicsShapeObject__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: u32) -> *mut (),
    pub getClass:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeObject) -> *mut gfc__Class,
    pub setState:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *const gfc__HString),
    pub getScriptData:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeObject) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> *mut gfc__Environment,
    pub getMethodByID: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *const u64,
    ) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> bool,
    pub addObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub removeObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: gfc__AutoRef_gfc__WorldObject_,
    ),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub getGroup:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> *mut gfc__WorldObject,
    pub getObject:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> *mut gfc__Object3D,
    pub setObject:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__PhysicsShapeObject,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub removeFromPathFinding: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__TraversalWaypoint,
    ),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject, _: *mut gfc__WorldObject),
    pub overrideHitEffect: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: f32,
        _: *mut gfc__Body,
    ) -> bool,
    pub supportsStaticLighting:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> bool,
    pub staticLightingIsDynamic:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsShapeObject,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__PhysicsShapeObject),
    pub getGizmoColor: unsafe extern "thiscall" fn(
        this: *const gfc__PhysicsShapeObject,
    )
        -> *const gfc__TVector4_float_gfc__FloatMath_,
    pub getGizmoIcon:
        unsafe extern "thiscall" fn(this: *const gfc__PhysicsShapeObject) -> *const gfc__HString,
}

#[repr(C)]
pub struct gfc__PhysicsEffects {
    // hkpContactListener
    pub vfptr: *const gfc__PhysicsEffects__vftable,
    // hkpCollisionListener
    // gfc__PhysicsEffects
    __pdbindgen_padding: [u8; 4],
    pub mAllocContact: gfc__LockFreeQueue_gfc__PhysicsEffects__ContactInfo___,
    pub mPendingContact: gfc__LockFreeQueue_gfc__PhysicsEffects__ContactInfo___,
    pub mContacts: std__set_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______gfc__PhysicsEffects__ContactCmp_std__allocator_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo________,
    pub mExternalContacts: gfc__Vector_gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo______0_gfc__CAllocator_,
    pub mSoundMgr: gfc__PhysicsEffects__FXSoundManager,
    pub mParticleMgr: gfc__PhysicsEffects__FXParticleManager,
    pub mExternalSoundMgr: gfc__PhysicsEffects__FXSoundManager,
    pub mExternalParticleMgr: gfc__PhysicsEffects__FXParticleManager,
    pub mWorld: *mut hkpWorld,
    pub mDesc: gfc__AutoRef_gfc__PhysicsEffectsDesc_,
    pub mCurTime: f32,
    pub mAllInfo: *mut gfc__PhysicsEffects__ContactInfo,
    pub mAllEffects: *mut gfc__LockFreeNode_gfc__PhysicsEffects__ContactInfo___,
}

unsafe impl UpcastToNop<hkpCollisionListener> for gfc__PhysicsEffects {}

unsafe impl UpcastToNop<hkpContactListener> for gfc__PhysicsEffects {}

impl gfc__PhysicsEffects {
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
pub struct gfc__PhysicsEffects__vftable {
    pub contactPointCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffects, _: *const hkpContactPointEvent),
    pub collisionAddedCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffects, _: *const hkpCollisionEvent),
    pub collisionRemovedCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffects, _: *const hkpCollisionEvent),
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffects, _: u32) -> *mut (),
    pub contactPointAddedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffects,
        _: *mut hkpContactPointAddedEvent,
    ),
    pub contactPointRemovedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffects,
        _: *mut hkpContactPointRemovedEvent,
    ),
    pub contactProcessCallback:
        unsafe extern "thiscall" fn(this: *mut gfc__PhysicsEffects, _: *mut hkpContactProcessEvent),
    pub contactPointConfirmedCallback: unsafe extern "thiscall" fn(
        this: *mut gfc__PhysicsEffects,
        _: *mut hkpContactPointConfirmedEvent,
    ),
}

#[repr(C)]
pub struct gfc__PhysicsEffects__FXParticleManager {
    pub mParticles:
        gfc__Vector_gfc__PhysicsEffects__FXParticleManager__ParticleInfo_0_gfc__CAllocator_,
    pub mWorld: *mut gfc__World,
}

#[repr(C)]
pub struct gfc__PhysicsEffects__FXParticleManager__ParticleInfo {
    pub Contact: *mut gfc__PhysicsEffects__ContactInfo,
    pub Particle: gfc__AutoRef_gfc__PSystem_,
    pub Emitters: gfc__Vector_gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance__0_gfc__CAllocator_,
    pub Type: u8,
    pub Looping: bool,
    pub Used: bool,
}

#[repr(C)]
pub struct gfc__PhysicsEffects__FXSoundManager {
    pub mCurTime: f32,
    pub mDesc: *const gfc__PhysicsEffectsDesc,
    pub mSounds: gfc__Vector_gfc__PhysicsEffects__FXSoundManager__SoundInfo_0_gfc__CAllocator_,
    pub mSamples: gfc__Map_gfc__HString_gfc__AutoRef_gfc__Sample__std__less_gfc__HString___,
}

#[repr(C)]
pub struct gfc__PhysicsEffects__FXSoundManager__SoundInfo {
    pub Contact: *mut gfc__PhysicsEffects__ContactInfo,
    pub Channel: i32,
    pub Volume: f32,
    pub Type: u8,
    pub Looping: bool,
    pub Used: bool,
}

#[repr(C)]
pub struct gfc__PhysicsEffects__ContactCmp {
    __pdbindgen_padding: [u8; 1],
}

#[repr(C)]
pub struct gfc__PhysicsEffects__ContactInfo {
    pub Body: gfc__AutoRef_gfc__RigidBody_,
    pub Velocity: f32,
    pub Position: gfc__TVector3_float_gfc__FloatMath_,
    pub Normal: gfc__TVector3_float_gfc__FloatMath_,
    pub PrevType: u8,
    pub CurType: u8,
    pub StepsThisType: u8,
    pub MissedSteps: u8,
    pub PlayingEffects: i32,
    pub EffectStartTime: f32,
    pub Interaction: *mut gfc__PhysicsMaterialInteraction,
    pub Matrix: gfc__Matrix4,
    pub UseMatrix: bool,
    pub UseAlternateTime: bool,
    __pdbindgen_padding: [u8; 14],
}

#[repr(C)]
pub struct gfc__EmitterSceneObject {
    // gfc__SceneObject
    pub vfptr: *const gfc__EmitterSceneObject__vftable,
    pub mType: gfc__SceneObject__Type,
    pub mDrawCounter: u32,
    pub mCachedBoundingVolume: gfc__BoundingVolume,
    pub mFlags: gfc__TFlags_unsigned_long_,
    pub mSceneManager: *mut gfc__SceneManager,
    pub mCells: gfc__Vector_gfc__SceneCell___0_gfc__CAllocator_,
    pub mHashID: u32,
    // gfc__IRenderCallback
    pub vfptr_2: *const gfc__EmitterSceneObject__vftable,
    pub mLocked: bool,
    // gfc__EmitterSceneObject
    pub mBVolume: gfc__BoundingVolume,
    pub mEmitter: *mut gfc__Emitter,
    pub mEmitterHandle: gfc__LockFreePoolHandle_gfc__EmitterInstance_gfc__EmitterInstance_,
    pub mEmitterInstance: *mut gfc__EmitterInstance,
    pub mHide: bool,
}

unsafe impl UpcastToNop<gfc__SceneObject> for gfc__EmitterSceneObject {}

unsafe impl UpcastTo<gfc__IRenderCallback> for gfc__EmitterSceneObject {
    fn upcast_to(p: *const Self) -> *const gfc__IRenderCallback {
        (p as usize + 0x50) as *const _
    }
}

impl gfc__EmitterSceneObject {
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
pub struct gfc__EmitterSceneObject__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject, _: u32) -> *mut (),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub renderDepthOnly: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *mut gfc__Camera3D,
        _: *mut gfc__RenderNode,
    ),
    pub startGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject, _: *mut gfc__RenderNode),
    pub finishGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject, _: *mut gfc__RenderNode),
    pub prepGeometry:
        unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject, _: *mut gfc__RenderNode),
    pub cullAndSubmit: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *const gfc__Clipper,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub submit: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *mut gfc__Camera3D,
        _: bool,
        _: u32,
    ),
    pub submitHidden: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *mut gfc__Camera3D,
        _: u32,
    ),
    pub getContext:
        unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject) -> *mut gfc__Object,
    pub pickObject: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *mut f32,
        _: bool,
    ) -> bool,
    pub isStatic: unsafe extern "thiscall" fn(this: *const gfc__EmitterSceneObject) -> bool,
    pub isHighPriority: unsafe extern "thiscall" fn(this: *const gfc__EmitterSceneObject) -> bool,
    pub writeText: unsafe extern "thiscall" fn(
        this: *mut gfc__EmitterSceneObject,
        _: *mut gfc__AutoRef_gfc__OutputStream_,
    ),
    pub setIsSky: unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject, _: bool),
    pub getIsSky: unsafe extern "thiscall" fn(this: *const gfc__EmitterSceneObject) -> bool,
    pub getHide: unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject) -> bool,
    pub getFreeze: unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject) -> bool,
    pub getLocked: unsafe extern "thiscall" fn(this: *mut gfc__EmitterSceneObject) -> bool,
}

#[repr(C)]
pub struct gfc__Cinematic {
    // gfc__IRefObject
    pub vfptr: *const gfc__Cinematic__vftable,
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
    // gfc__Cinematic
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

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__Cinematic {}

unsafe impl UpcastToNop<gfc__Object> for gfc__Cinematic {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__Cinematic {}

impl gfc__Cinematic {
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
pub struct gfc__Cinematic__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__Cinematic) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__Cinematic) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__Cinematic,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__Cinematic, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__Cinematic,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__Cinematic),
}

#[repr(C)]
pub struct gfc__WorldGroup {
    // gfc__IRefObject
    pub vfptr: *const gfc__WorldGroup__vftable,
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
    // gfc__WorldGroup
    pub mPosition: gfc__TVector3_float_gfc__FloatMath_,
    pub mRotation: gfc__TVector3_float_gfc__FloatMath_,
    pub mObjects: List_gfc__AutoRef_gfc__WorldObject___,
}

unsafe impl UpcastToNop<gfc__WorldObject> for gfc__WorldGroup {}

unsafe impl UpcastToNop<gfc__Object> for gfc__WorldGroup {}

unsafe impl UpcastToNop<gfc__IRefObject> for gfc__WorldGroup {}

impl gfc__WorldGroup {
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

    pub unsafe extern "thiscall" fn setPropertyValue(
        &self,
        a1: *const gfc__String,
        a2: gfc__AutoRef_gfc__Value_,
    ) {
        ((*self.vfptr).setPropertyValue)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct gfc__WorldGroup__vftable {
    pub __vecDelDtor: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: u32) -> *mut (),
    pub getClass: unsafe extern "thiscall" fn(this: *const gfc__WorldGroup) -> *mut gfc__Class,
    pub setState: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *const gfc__HString),
    pub getScriptData: unsafe extern "thiscall" fn(this: *const gfc__WorldGroup) -> *const (),
    pub getScriptData_2: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> *mut (),
    pub getScriptState: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        result: *mut gfc__HString,
    ) -> *mut gfc__HString,
    pub getScriptEnvironment:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> *mut gfc__Environment,
    pub getMethodByID:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *const u64) -> *mut gfc__Method,
    pub cloneObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__ObjectCloner,
        _: gfc__AutoRef_gfc__Object_,
    ),
    pub setPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getPosition: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getRotation: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub setScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub getScale: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        result: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> *mut gfc__TVector3_float_gfc__FloatMath_,
    pub getBoundingBox: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__TBox_float_gfc__FloatMath_,
    ),
    pub doAddToWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub doRemoveFromWorld: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub placedInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub droppedToGroundInEditor: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub preload: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub begin: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub update: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: f32),
    pub updatePost: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: f32),
    pub render: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__Renderer,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub drawDebug: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub getPackageID: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> i32,
    pub playSound: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__SoundDesc,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub playSound_2: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: i32,
        _: *mut gfc__TVector3_float_gfc__FloatMath_,
    ) -> i32,
    pub stopSound: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: i32),
    pub setHide: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setFreeze: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setLocked: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setSelected: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setDisabled: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setDisplayNames: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setError: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub setSettled: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: bool),
    pub isGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> bool,
    pub addObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: gfc__AutoRef_gfc__WorldObject_),
    pub removeObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: gfc__AutoRef_gfc__WorldObject_),
    pub inGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> bool,
    pub isRoot: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> bool,
    pub removeFromGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub getGroup: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> *mut gfc__WorldObject,
    pub getObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> *mut gfc__Object3D,
    pub setObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *mut gfc__Object3D),
    pub getAnimation: unsafe extern "thiscall" fn(
        this: *const gfc__WorldGroup,
        result: *mut gfc__AutoRef_gfc__Animation_,
        _: i32,
    ) -> *mut gfc__AutoRef_gfc__Animation_,
    pub addObjectToWorld:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *mut gfc__World),
    pub addToLayer: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub removeFromPathFinding:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *mut gfc__TraversalWaypoint),
    pub detachFromObject: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
    pub onAttachedToObject: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__WorldObject,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
        _: *const gfc__TVector3_float_gfc__FloatMath_,
    ),
    pub onDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *mut gfc__WorldObject),
    pub onChildDetachedFromObject:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: *mut gfc__WorldObject),
    pub overrideHitEffect:
        unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup, _: f32, _: *mut gfc__Body) -> bool,
    pub supportsStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> bool,
    pub staticLightingIsDynamic: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> bool,
    pub getAORayLength: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup) -> i32,
    pub initStaticLighting: unsafe extern "thiscall" fn(
        this: *mut gfc__WorldGroup,
        _: *mut gfc__StaticLightingObjectOpt,
    ) -> bool,
    pub clearStaticLighting: unsafe extern "thiscall" fn(this: *mut gfc__WorldGroup),
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
    pub Object: *mut gfc__MeshParticle,
    #[cfg(pdb_issue = "unimplemented type kind")]
    pub State: compile_error!("unimplemented type kind"),
    __pdbindgen_padding: [u8; 4],
}

#[repr(C)]
pub struct gfc__Vector_gfc__ScenePortal___0_gfc__CAllocator_ {
    pub mData: *mut *mut gfc__ScenePortal,
    pub mSize: i32,
    pub mCapacityAndFlags: i32,
}

#[repr(C)]
pub struct hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpAgentNnSector___
    pub m_data: *mut *mut hkpAgentNnSector,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpAgentNnSector___hkContainerHeapAllocator_
    // hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_
    pub m_storage: [*mut hkpAgentNnSector; 1],
}

unsafe impl UpcastToNop<hkArray_hkpAgentNnSector___hkContainerHeapAllocator_>
    for hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_
{
}

unsafe impl UpcastToNop<hkArrayBase_hkpAgentNnSector___>
    for hkInplaceArray_hkpAgentNnSector___1_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkInplaceArray_char_128_hkContainerTempAllocator_ {
    // hkArrayBase_char_
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_char_hkContainerTempAllocator_
    // hkInplaceArray_char_128_hkContainerTempAllocator_
    pub m_storage: [i8; 128],
}

unsafe impl UpcastToNop<hkArray_char_hkContainerTempAllocator_>
    for hkInplaceArray_char_128_hkContainerTempAllocator_
{
}

unsafe impl UpcastToNop<hkArrayBase_char_> for hkInplaceArray_char_128_hkContainerTempAllocator_ {}

#[repr(C)]
pub struct hkContactPointPropertiesWithExtendedUserData16 {
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
    // hkContactPointPropertiesWithExtendedUserData16
    pub m_extendedUserDatas: [u32; 7],
}

unsafe impl UpcastToNop<hkpContactPointProperties>
    for hkContactPointPropertiesWithExtendedUserData16
{
}

unsafe impl UpcastToNop<hkpSolverResults> for hkContactPointPropertiesWithExtendedUserData16 {}

unsafe impl UpcastTo<hkContactPointMaterial> for hkContactPointPropertiesWithExtendedUserData16 {
    fn upcast_to(p: *const Self) -> *const hkContactPointMaterial {
        (p as usize + 0x8) as *const _
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
    // hkArrayBase_hkpIslandActivationListener___
    pub m_data: *mut *mut hkpIslandActivationListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpIslandActivationListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpIslandActivationListener___>
    for hkArray_hkpIslandActivationListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpAction___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpAction___
    pub m_data: *mut *mut hkpAction,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpAction___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpAction___>
    for hkArray_hkpAction___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_int_hkContainerHeapAllocator_ {
    // hkArrayBase_int_
    pub m_data: *mut i32,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_int_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_int_> for hkArray_int_hkContainerHeapAllocator_ {}

#[repr(C)]
pub struct hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_ {
    // hkArrayBase_hkArray_int_hkContainerHeapAllocator___
    pub m_data: *mut hkArray_int_hkContainerHeapAllocator_,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkArray_int_hkContainerHeapAllocator___>
    for hkArray_hkArray_int_hkContainerHeapAllocator__hkContainerHeapAllocator_
{
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
    // hkArrayBase_hkpIslandPostCollideListener___
    pub m_data: *mut *mut hkpIslandPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpIslandPostCollideListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpIslandPostCollideListener___>
    for hkArray_hkpIslandPostCollideListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkLocalFrameCollector {
    // hkBaseObject
    pub vfptr: *const hkLocalFrameCollector__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkLocalFrameCollector
}

unsafe impl UpcastToNop<hkReferencedObject> for hkLocalFrameCollector {}

unsafe impl UpcastToNop<hkBaseObject> for hkLocalFrameCollector {}

impl hkLocalFrameCollector {
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

    pub unsafe extern "thiscall" fn addFrame(&self, a1: *const hkLocalFrame, a2: f32) {
        ((*self.vfptr).addFrame)(self as *const _ as *mut _, a1, a2)
    }
}

#[repr(C)]
pub struct hkLocalFrameCollector__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkLocalFrameCollector, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkLocalFrameCollector),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkLocalFrameCollector) -> *const hkClass,
    pub deleteThisReferencedObject: unsafe extern "thiscall" fn(this: *const hkLocalFrameCollector),
    pub addFrame: unsafe extern "thiscall" fn(
        this: *mut hkLocalFrameCollector,
        _: *const hkLocalFrame,
        _: f32,
    ),
}

#[repr(C)]
pub struct hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_ {
    // hkArrayBase_hkpEntity___
    pub m_data: *mut *mut hkpEntity,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpEntity___hkContainerHeapAllocator_
    // hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_
    pub m_storage: [*mut hkpEntity; 1],
}

unsafe impl UpcastToNop<hkArray_hkpEntity___hkContainerHeapAllocator_>
    for hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_
{
}

unsafe impl UpcastToNop<hkArrayBase_hkpEntity___>
    for hkInplaceArray_hkpEntity___1_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_char_ {
    pub m_data: *mut i8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_unsigned_char_hkContainerHeapAllocator_ {
    // hkArrayBase_unsigned_char_
    pub m_data: *mut u8,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_unsigned_char_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_unsigned_char_>
    for hkArray_unsigned_char_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpIslandPostIntegrateListener___
    pub m_data: *mut *mut hkpIslandPostIntegrateListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpIslandPostIntegrateListener___>
    for hkArray_hkpIslandPostIntegrateListener___hkContainerHeapAllocator_
{
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
    // hkArrayBase_hkpWorldPostCollideListener___
    pub m_data: *mut *mut hkpWorldPostCollideListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpWorldPostCollideListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpWorldPostCollideListener___>
    for hkArray_hkpWorldPostCollideListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkpCollidableCollidableFilter {
    pub vfptr: *const hkpCollidableCollidableFilter__vftable,
}

impl hkpCollidableCollidableFilter {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn isCollisionEnabled(
        &self,
        result: *mut hkBool,
        a2: *const hkpCollidable,
        a3: *const hkpCollidable,
    ) -> *mut hkBool {
        ((*self.vfptr).isCollisionEnabled)(self as *const _ as *mut _, result, a2, a3)
    }
}

#[repr(C)]
pub struct hkpCollidableCollidableFilter__vftable {
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
    // hkArrayBase_hkpEntity___
    pub m_data: *mut *mut hkpEntity,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpEntity___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpEntity___>
    for hkArray_hkpEntity___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkPadSpu_hkpConstraintInstance___ {
    pub m_storage: *mut hkpConstraintInstance,
}

#[repr(C)]
pub struct hkpShapeModifier {
    pub vfptr: *const hkpShapeModifier__vftable,
}

impl hkpShapeModifier {
    pub unsafe extern "thiscall" fn __vecDelDtor(&self, a1: u32) -> *mut () {
        ((*self.vfptr).__vecDelDtor)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn modifyShape(&self, a1: *mut hkpShape) {
        ((*self.vfptr).modifyShape)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpShapeModifier__vftable {
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
    // hkBaseObject
    pub vfptr: *const hkpKeyframedRigidMotion__vftable,
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
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_deactivationRefPosition: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 32],
    pub m_deactivationRefOrientation: [u32; 2],
    pub m_savedMotion: *mut hkpMaxSizeMotion,
    pub m_savedQualityTypeIndex: u16,
    pub m_gravityFactor: hkHalf,
    // hkpKeyframedRigidMotion
}

unsafe impl UpcastToNop<hkpMotion> for hkpKeyframedRigidMotion {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpKeyframedRigidMotion {}

unsafe impl UpcastToNop<hkBaseObject> for hkpKeyframedRigidMotion {}

impl hkpKeyframedRigidMotion {
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

    pub unsafe extern "thiscall" fn setMass(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMass)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMass_2(&self, a1: f32) {
        ((*self.vfptr).setMass_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv(&self, a1: *const hkSimdFloat32) {
        ((*self.vfptr).setMassInv)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setMassInv_2(&self, a1: f32) {
        ((*self.vfptr).setMassInv_2)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setInertiaInvLocal(&self, a1: *const hkMatrix3f) {
        ((*self.vfptr).setInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvLocal(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getInertiaInvWorld(&self, a1: *mut hkMatrix3f) {
        ((*self.vfptr).getInertiaInvWorld)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setCenterOfMassInLocal(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setCenterOfMassInLocal)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPosition(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setPosition)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setRotation(&self, a1: *const hkQuaternionf) {
        ((*self.vfptr).setRotation)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setPositionAndRotation(
        &self,
        a1: *const hkVector4f,
        a2: *const hkQuaternionf,
    ) {
        ((*self.vfptr).setPositionAndRotation)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setTransform(&self, a1: *const hkTransformf) {
        ((*self.vfptr).setTransform)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setLinearVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setLinearVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn setAngularVelocity(&self, a1: *const hkVector4f) {
        ((*self.vfptr).setAngularVelocity)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocity(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut f32,
        a4: *mut f32,
    ) {
        ((*self.vfptr).getProjectedPointVelocity)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn getProjectedPointVelocitySimd(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
        a3: *mut hkSimdFloat32,
        a4: *mut hkSimdFloat32,
    ) {
        ((*self.vfptr).getProjectedPointVelocitySimd)(self as *const _ as *mut _, a1, a2, a3, a4)
    }

    pub unsafe extern "thiscall" fn applyLinearImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyLinearImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyPointImpulse(
        &self,
        a1: *const hkVector4f,
        a2: *const hkVector4f,
    ) {
        ((*self.vfptr).applyPointImpulse)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyAngularImpulse(&self, a1: *const hkVector4f) {
        ((*self.vfptr).applyAngularImpulse)(self as *const _ as *mut _, a1)
    }

    pub unsafe extern "thiscall" fn applyForce(
        &self,
        a1: f32,
        a2: *const hkVector4f,
        a3: *const hkVector4f,
    ) {
        ((*self.vfptr).applyForce)(self as *const _ as *mut _, a1, a2, a3)
    }

    pub unsafe extern "thiscall" fn applyForce_2(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyForce_2)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn applyTorque(&self, a1: f32, a2: *const hkVector4f) {
        ((*self.vfptr).applyTorque)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn getMotionStateAndVelocitiesAndDeactivationType(
        &self,
        a1: *mut hkpMotion,
    ) {
        ((*self.vfptr).getMotionStateAndVelocitiesAndDeactivationType)(
            self as *const _ as *mut _,
            a1,
        )
    }

    pub unsafe extern "thiscall" fn setStepPosition(&self, a1: f32, a2: f32) {
        ((*self.vfptr).setStepPosition)(self as *const _ as *mut _, a1, a2)
    }

    pub unsafe extern "thiscall" fn setStoredMotion(&self, a1: *mut hkpMaxSizeMotion) {
        ((*self.vfptr).setStoredMotion)(self as *const _ as *mut _, a1)
    }
}

#[repr(C)]
pub struct hkpKeyframedRigidMotion__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion),
    pub setMass:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkSimdFloat32),
    pub setMass_2: unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: f32),
    pub setMassInv:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkSimdFloat32),
    pub setMassInv_2: unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: f32),
    pub getInertiaLocal:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion, _: *mut hkMatrix3f),
    pub getInertiaWorld:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion, _: *mut hkMatrix3f),
    pub setInertiaLocal:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkMatrix3f),
    pub setInertiaInvLocal:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkMatrix3f),
    pub getInertiaInvLocal:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion, _: *mut hkMatrix3f),
    pub getInertiaInvWorld:
        unsafe extern "thiscall" fn(this: *const hkpKeyframedRigidMotion, _: *mut hkMatrix3f),
    pub setCenterOfMassInLocal:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub setPosition:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub setRotation:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkQuaternionf),
    pub setPositionAndRotation: unsafe extern "thiscall" fn(
        this: *mut hkpKeyframedRigidMotion,
        _: *const hkVector4f,
        _: *const hkQuaternionf,
    ),
    pub setTransform:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkTransformf),
    pub setLinearVelocity:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub setAngularVelocity:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub getProjectedPointVelocity: unsafe extern "thiscall" fn(
        this: *const hkpKeyframedRigidMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut f32,
        _: *mut f32,
    ),
    pub getProjectedPointVelocitySimd: unsafe extern "thiscall" fn(
        this: *const hkpKeyframedRigidMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
        _: *mut hkSimdFloat32,
        _: *mut hkSimdFloat32,
    ),
    pub applyLinearImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub applyPointImpulse: unsafe extern "thiscall" fn(
        this: *mut hkpKeyframedRigidMotion,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyAngularImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *const hkVector4f),
    pub applyForce: unsafe extern "thiscall" fn(
        this: *mut hkpKeyframedRigidMotion,
        _: f32,
        _: *const hkVector4f,
        _: *const hkVector4f,
    ),
    pub applyForce_2: unsafe extern "thiscall" fn(
        this: *mut hkpKeyframedRigidMotion,
        _: f32,
        _: *const hkVector4f,
    ),
    pub applyTorque: unsafe extern "thiscall" fn(
        this: *mut hkpKeyframedRigidMotion,
        _: f32,
        _: *const hkVector4f,
    ),
    pub getMotionStateAndVelocitiesAndDeactivationType:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *mut hkpMotion),
    pub setStepPosition:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: f32, _: f32),
    pub setStoredMotion:
        unsafe extern "thiscall" fn(this: *mut hkpKeyframedRigidMotion, _: *mut hkpMaxSizeMotion),
}

#[repr(C)]
pub struct hkArray_hkpPhantomListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpPhantomListener___
    pub m_data: *mut *mut hkpPhantomListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpPhantomListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpPhantomListener___>
    for hkArray_hkpPhantomListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct keen__FixedSizedArray_gfc__SceneManager__DistributedCulling_1024_ {
    #[cfg(pdb_issue = "unimplemented class layout")]
    pub m_data: compile_error!("unimplemented class layout"),
    __pdbindgen_padding: [u8; 24576],
    pub m_size: u32,
}

#[repr(C)]
pub struct hkCriticalSection {
    pub m_section: _RTL_CRITICAL_SECTION,
}

#[repr(C)]
pub struct hkpAgentNnEntry {
    // hkpAgentEntry
    pub m_streamCommand: u8,
    pub m_agentType: u8,
    pub m_numContactPoints: u8,
    pub m_size: u8,
    // hkpAgentNnEntry
    pub m_agentIndexOnCollidable: [u16; 2],
    pub m_contactMgr: *mut hkpContactMgr,
    pub m_collisionQualityIndex: u8,
    pub m_forceCollideOntoPpu: u8,
    pub m_nnTrackType: hkEnum_enum_hkpAgentNnTrackType_unsigned_char_,
    pub m_padding: u8,
    pub m_collidable: [*mut hkpLinkedCollidable; 2],
}

unsafe impl UpcastToNop<hkpAgentEntry> for hkpAgentNnEntry {}

#[repr(C)]
pub struct hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpWorldDeletionListener___
    pub m_data: *mut *mut hkpWorldDeletionListener,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpWorldDeletionListener___>
    for hkArray_hkpWorldDeletionListener___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArray_hkSimpleProperty_hkContainerHeapAllocator_ {
    // hkArrayBase_hkSimpleProperty_
    pub m_data: *mut hkSimpleProperty,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkSimpleProperty_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkSimpleProperty_>
    for hkArray_hkSimpleProperty_hkContainerHeapAllocator_
{
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
    // hkBaseObject
    pub vfptr: *const hkpSimpleContactConstraintData__vftable,
    // hkReferencedObject
    pub m_memSizeAndRefCount: u32,
    // hkpConstraintData
    pub m_userData: u32,
    // hkpSimpleContactConstraintData
    pub m_idMgrA: hkpDynamicsCpIdMgr,
    pub m_clientData: *mut (),
    pub m_constraint: *mut hkpConstraintInstance,
    pub m_atom: *mut hkpSimpleContactConstraintAtom,
    pub m_atomSize: i32,
}

unsafe impl UpcastToNop<hkpConstraintData> for hkpSimpleContactConstraintData {}

unsafe impl UpcastToNop<hkReferencedObject> for hkpSimpleContactConstraintData {}

unsafe impl UpcastToNop<hkBaseObject> for hkpSimpleContactConstraintData {}

impl hkpSimpleContactConstraintData {
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

    pub unsafe extern "thiscall" fn collisionResponseBeginCallback(
        &self,
        a1: *const hkContactPoint,
        a2: *mut hkpSimpleConstraintInfoInitInput,
        a3: *mut hkpBodyVelocity,
        a4: *mut hkpSimpleConstraintInfoInitInput,
        a5: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).collisionResponseBeginCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
        )
    }

    pub unsafe extern "thiscall" fn collisionResponseEndCallback(
        &self,
        a1: *const hkContactPoint,
        a2: f32,
        a3: *mut hkpSimpleConstraintInfoInitInput,
        a4: *mut hkpBodyVelocity,
        a5: *mut hkpSimpleConstraintInfoInitInput,
        a6: *mut hkpBodyVelocity,
    ) {
        ((*self.vfptr).collisionResponseEndCallback)(
            self as *const _ as *mut _,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
        )
    }
}

#[repr(C)]
pub struct hkpSimpleContactConstraintData__vftable {
    pub __vecDelDtor:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData, _: u32) -> *mut (),
    pub __first_virtual_table_function__:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData),
    pub getClassType:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> *const hkClass,
    pub deleteThisReferencedObject:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData),
    pub getType: unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> i32,
    pub getConstraintInfo: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        _: *mut hkpConstraintData__ConstraintInfo,
    ),
    pub isValid: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        result: *mut hkBool,
    ) -> *mut hkBool,
    pub setMaximumLinearImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData, _: f32),
    pub setMaximumAngularImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData, _: f32),
    pub setBreachImpulse:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData, _: f32),
    pub getMaximumLinearImpulse:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> f32,
    pub getMaximumAngularImpulse:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> f32,
    pub getBreachImpulse:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> f32,
    pub setBodyToNotify:
        unsafe extern "thiscall" fn(this: *mut hkpSimpleContactConstraintData, _: i32),
    pub getNotifiedBodyIndex:
        unsafe extern "thiscall" fn(this: *const hkpSimpleContactConstraintData) -> u8,
    pub setSolvingMethod: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
        _: hkpConstraintAtom__SolvingMethod,
    ),
    pub setInertiaStabilizationFactor: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
        result: *mut hkResult,
        _: f32,
    ) -> *mut hkResult,
    pub getInertiaStabilizationFactor: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        result: *mut hkResult,
        _: *mut f32,
    ) -> *mut hkResult,
    pub getAppliedLinearImpulse: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        _: *const hkTransformf,
        _: *const hkTransformf,
        _: *const hkpConstraintRuntime,
        _: *mut hkVector4f,
    ),
    pub getRuntimeInfo: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        _: hkBool,
        _: *mut hkpConstraintData__RuntimeInfo,
    ),
    pub getSolverResults: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        _: *mut hkpConstraintRuntime,
    ) -> *mut hkpSolverResults,
    pub addInstance: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        _: *mut hkpConstraintRuntime,
        _: i32,
    ),
    pub updateDirtyAtoms: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
    )
        -> hkpConstraintData__UpdateAtomsResult__Enum,
    pub buildJacobian: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
        _: *const hkpConstraintQueryIn,
        _: *mut hkpConstraintQueryOut,
    ),
    pub isBuildJacobianCallbackRequired: unsafe extern "thiscall" fn(
        this: *const hkpSimpleContactConstraintData,
        result: *mut hkBool,
    ) -> *mut hkBool,
    pub buildJacobianCallback: unsafe extern "thiscall" fn(
        this: *mut hkpSimpleContactConstraintData,
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
pub struct hkArray_hkVector4f_hkContainerHeapAllocator_ {
    // hkArrayBase_hkVector4f_
    pub m_data: *mut hkVector4f,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkVector4f_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkVector4f_> for hkArray_hkVector4f_hkContainerHeapAllocator_ {}

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
    // hkArrayBase_hkpLinkedCollidable__CollisionEntry_
    pub m_data: *mut hkpLinkedCollidable__CollisionEntry,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpLinkedCollidable__CollisionEntry_>
    for hkArray_hkpLinkedCollidable__CollisionEntry_hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkArrayBase_hkLocalFrame_const___ {
    pub m_data: *mut *const hkLocalFrame,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
}

#[repr(C)]
pub struct hkArray_hkpPhantom___hkContainerHeapAllocator_ {
    // hkArrayBase_hkpPhantom___
    pub m_data: *mut *mut hkpPhantom,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_hkpPhantom___hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_hkpPhantom___>
    for hkArray_hkpPhantom___hkContainerHeapAllocator_
{
}

#[repr(C)]
pub struct hkSmallArray_hkpContactListener___ {
    pub m_data: *mut *mut hkpContactListener,
    pub m_size: u16,
    pub m_capacityAndFlags: u16,
}

#[repr(C)]
pub struct hkArray_unsigned_short_hkContainerHeapAllocator_ {
    // hkArrayBase_unsigned_short_
    pub m_data: *mut u16,
    pub m_size: i32,
    pub m_capacityAndFlags: i32,
    // hkArray_unsigned_short_hkContainerHeapAllocator_
}

unsafe impl UpcastToNop<hkArrayBase_unsigned_short_>
    for hkArray_unsigned_short_hkContainerHeapAllocator_
{
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
