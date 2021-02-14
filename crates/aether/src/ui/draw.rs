use crate::{darksiders1::gfc, library::dx::init_with_hresult};
use gfx::{
    format::Bgra8,
    handle::{RenderTargetView, ShaderResourceView},
    Device,
    Encoder,
    Factory,
};
use imgui::{Context, DrawData};
use imgui_gfx_renderer::{Renderer, Shaders};
use std::{convert::TryInto, mem, ptr};
use winapi::{
    shared::{
        dxgiformat::{
            DXGI_FORMAT_R32G32B32A32_FLOAT,
            DXGI_FORMAT_R32G32B32_FLOAT,
            DXGI_FORMAT_R32G32_FLOAT,
        },
        minwindef::FALSE,
    },
    um::{
        d3d11::{
            D3D11_APPEND_ALIGNED_ELEMENT,
            D3D11_BIND_VERTEX_BUFFER,
            D3D11_BUFFER_DESC,
            D3D11_COMPARISON_ALWAYS,
            D3D11_CULL_NONE,
            D3D11_FILL_SOLID,
            D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            D3D11_FLOAT32_MAX,
            D3D11_INPUT_ELEMENT_DESC,
            D3D11_INPUT_PER_VERTEX_DATA,
            D3D11_RASTERIZER_DESC,
            D3D11_SAMPLER_DESC,
            D3D11_SUBRESOURCE_DATA,
            D3D11_TEXTURE_ADDRESS_BORDER,
            D3D11_USAGE_DEFAULT,
        },
        d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
    },
};

pub struct State {
    gfx_device: gfx_device_dx11::Device,
    gfx_factory: gfx_device_dx11::Factory,
    imgui_shader_resource: ShaderResourceView<gfx_device_dx11::Resources, [f32; 4]>,
    imgui_render_target: RenderTargetView<gfx_device_dx11::Resources, Bgra8>,
    imgui_renderer: Renderer<Bgra8, gfx_device_dx11::Resources>,
    imgui_encoder: Encoder<
        gfx_device_dx11::Resources,
        gfx_device_dx11::CommandBuffer<gfx_device_dx11::CommandList>,
    >,
    vs_copy: d3d11::VertexShader,
    ps_tex: d3d11::PixelShader,
    input_layout: d3d11::InputLayout,
    quad_vertex_buffer: d3d11::Buffer,
    rasterizer_state: d3d11::RasterizerState,
    hq_sampler: d3d11::SamplerState,
}

#[allow(clippy::too_many_lines)]
pub fn init(screen_width: u16, screen_height: u16, imgui: &mut Context) -> State {
    let device;
    let context;
    let vs_copy;
    let ps_tex;
    let input_layout;
    let quad_vertex_buffer;
    let rasterizer_state;
    let hq_sampler;
    let graphics = gfc::KGGraphics::get_instance();
    unsafe {
        device = (*(*graphics.as_ptr()).m_pGraphicsSystem).pDevice
            as *mut winapi::um::d3d11::ID3D11Device;
        context = (*(*graphics.as_ptr()).m_pGraphicsSystem).pImmediateContext
            as *mut winapi::um::d3d11::ID3D11DeviceContext;

        let source = include_str!("shaders/simple.hlsl");
        let vs_copy_code = d3d11::compile(source, cstr!("vs_copy"), cstr!("vs_5_0"))
            .map_err(|e| String::from_utf8_lossy(e.buffer()).into_owned())
            .unwrap();
        let ps_tex_code = d3d11::compile(source, cstr!("ps_tex"), cstr!("ps_5_0"))
            .map_err(|e| String::from_utf8_lossy(e.buffer()).into_owned())
            .unwrap();

        vs_copy = init_with_hresult(|p| {
            (*device).CreateVertexShader(
                vs_copy_code.buffer().as_ptr().cast(),
                vs_copy_code.buffer().len(),
                ptr::null_mut(),
                p,
            )
        })
        .map(|p| d3d11::VertexShader::from_raw(p))
        .unwrap();

        ps_tex = init_with_hresult(|p| {
            (*device).CreatePixelShader(
                ps_tex_code.buffer().as_ptr().cast(),
                ps_tex_code.buffer().len(),
                ptr::null_mut(),
                p,
            )
        })
        .map(|p| d3d11::PixelShader::from_raw(p))
        .unwrap();

        input_layout = init_with_hresult(|p| {
            (*device).CreateInputLayout(
                Vertex::LAYOUT.as_ptr(),
                Vertex::LAYOUT.len().try_into().unwrap(),
                vs_copy_code.buffer().as_ptr().cast(),
                vs_copy_code.buffer().len(),
                p,
            )
        })
        .map(|p| d3d11::InputLayout::from_raw(p))
        .unwrap();

        let desc = D3D11_BUFFER_DESC {
            ByteWidth: mem::size_of_val(TEXTURED_QUAD).try_into().unwrap(),
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let data = D3D11_SUBRESOURCE_DATA {
            pSysMem: TEXTURED_QUAD.as_ptr().cast(),
            SysMemPitch: 0,
            SysMemSlicePitch: 0,
        };
        quad_vertex_buffer = init_with_hresult(|p| (*device).CreateBuffer(&desc, &data, p))
            .map(|p| d3d11::Buffer::from_raw(p))
            .unwrap();

        let desc = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_NONE,
            FrontCounterClockwise: FALSE,
            DepthBias: 0,
            DepthBiasClamp: 0.0,
            SlopeScaledDepthBias: 0.0,
            DepthClipEnable: FALSE,
            ScissorEnable: FALSE,
            MultisampleEnable: FALSE,
            AntialiasedLineEnable: FALSE,
        };
        rasterizer_state = init_with_hresult(|p| (*device).CreateRasterizerState(&desc, p))
            .map(|p| d3d11::RasterizerState::from_raw(p))
            .unwrap();

        let desc = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D11_TEXTURE_ADDRESS_BORDER,
            AddressV: D3D11_TEXTURE_ADDRESS_BORDER,
            AddressW: D3D11_TEXTURE_ADDRESS_BORDER,
            MipLODBias: 0.0,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D11_COMPARISON_ALWAYS,
            BorderColor: [0.0, 0.0, 0.0, 0.0],
            MinLOD: 0.0,
            MaxLOD: D3D11_FLOAT32_MAX,
        };
        hq_sampler = init_with_hresult(|p| (*device).CreateSamplerState(&desc, p))
            .map(|p| d3d11::SamplerState::from_raw(p))
            .unwrap();
    }

    let (gfx_device, mut gfx_factory) =
        gfx_device_dx11::create_from_existing(device, context).unwrap();
    let (_imgui_texture, imgui_shader_resource, imgui_render_target) = gfx_factory
        .create_render_target(screen_width, screen_height)
        .unwrap();
    let imgui_renderer = Renderer::init(imgui, &mut gfx_factory, Shaders::HlslSm40).unwrap();
    let imgui_encoder = gfx_factory.create_command_buffer().into();

    State {
        gfx_device,
        gfx_factory,
        imgui_shader_resource,
        imgui_render_target,
        imgui_renderer,
        imgui_encoder,
        vs_copy,
        ps_tex,
        input_layout,
        quad_vertex_buffer,
        rasterizer_state,
        hq_sampler,
    }
}

pub fn check_screen_resolution_change(state: &mut State, screen_width: u16, screen_height: u16) {
    let (prev_width, prev_height, _, _) = state.imgui_render_target.get_dimensions();
    // If the resolution did not change, there is nothing to do.
    if (screen_width, screen_height) == (prev_width, prev_height) {
        return;
    }

    // Otherwise, recreate the texture
    let (_imgui_texture, imgui_shader_resource, imgui_render_target) = state
        .gfx_factory
        .create_render_target(screen_width, screen_height)
        .unwrap();
    state.imgui_shader_resource = imgui_shader_resource;
    state.imgui_render_target = imgui_render_target;
}

pub fn draw(state: &mut State, draw_data: &DrawData) {
    state
        .imgui_encoder
        .clear(&state.imgui_render_target, [0.0, 0.0, 0.0, 0.0]);
    state
        .imgui_renderer
        .render(
            &mut state.gfx_factory,
            &mut state.imgui_encoder,
            &mut state.imgui_render_target,
            draw_data,
        )
        .unwrap();
    state.imgui_encoder.flush(&mut state.gfx_device);
    state.gfx_device.cleanup();

    let graphics = gfc::KGGraphics::get_instance();
    unsafe {
        let context = (*(*graphics.as_ptr()).m_pGraphicsSystem).pImmediateContext;
        let context = d3d11::DeviceContext::from_ptr(context.cast());
        let target = (*(*(*graphics.as_ptr()).m_pCommandWriter).m_pRenderTarget).renderTargetViews
            [0] as *mut winapi::um::d3d11::ID3D11RenderTargetView;

        (*context.raw()).IASetInputLayout(state.input_layout.raw());
        (*context.raw()).IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        (*context.raw()).IASetVertexBuffers(
            0,
            1,
            &state.quad_vertex_buffer.raw(),
            &mem::size_of::<Vertex>().try_into().unwrap(),
            &0,
        );
        (*context.raw()).VSSetShader(state.vs_copy.raw(), ptr::null(), 0);
        (*context.raw()).RSSetState(state.rasterizer_state.raw());
        (*context.raw()).OMSetRenderTargets(1, &target, ptr::null_mut());
        (*context.raw()).PSSetShader(state.ps_tex.raw(), ptr::null(), 0);
        (*context.raw()).PSSetSamplers(0, 1, &state.hq_sampler.raw());
        (*context.raw()).PSSetShaderResources(0, 1, &state.imgui_shader_resource.raw_view().0);
        (*context.raw()).Draw(6, 0);
    }
}

#[repr(C)]
struct Vertex {
    pos: [f32; 3],
    col: [f32; 4],
    tex: [f32; 2],
}

impl Vertex {
    const LAYOUT: &'static [D3D11_INPUT_ELEMENT_DESC] = &[
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"POSITION\0".as_ptr().cast(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"COLOR\0".as_ptr().cast(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: b"TEXCOORD\0".as_ptr().cast(),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];
}

const TEXTURED_QUAD: &[Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [1.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 0.0],
        col: [0.0, 0.0, 0.0, 1.0],
        tex: [1.0, 0.0],
    },
];
