use crate::{
    darksiders1::gfc,
    library::dx::{create_staging_texture2d, init_with_hresult},
};
use gfx::{
    format::{Bgra8, B8_G8_R8_A8},
    handle::{RenderTargetView, Texture},
    memory::Typed,
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
            DXGI_FORMAT_B8G8R8A8_UNORM,
            DXGI_FORMAT_R32G32B32A32_FLOAT,
            DXGI_FORMAT_R32G32B32_FLOAT,
            DXGI_FORMAT_R32G32_FLOAT,
        },
        dxgitype::DXGI_SAMPLE_DESC,
    },
    um::{
        d3d11::{
            ID3D11Texture2D,
            D3D11_APPEND_ALIGNED_ELEMENT,
            D3D11_BIND_RENDER_TARGET,
            D3D11_BIND_SHADER_RESOURCE,
            D3D11_BIND_UNORDERED_ACCESS,
            D3D11_BIND_VERTEX_BUFFER,
            D3D11_BUFFER_DESC,
            D3D11_COMPARISON_ALWAYS,
            D3D11_CPU_ACCESS_WRITE,
            D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            D3D11_FLOAT32_MAX,
            D3D11_INPUT_ELEMENT_DESC,
            D3D11_INPUT_PER_VERTEX_DATA,
            D3D11_MAP_READ,
            D3D11_RECT,
            D3D11_RESOURCE_MISC_GENERATE_MIPS,
            D3D11_SAMPLER_DESC,
            D3D11_SUBRESOURCE_DATA,
            D3D11_TEXTURE2D_DESC,
            D3D11_TEXTURE_ADDRESS_BORDER,
            D3D11_USAGE_DEFAULT,
        },
        d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
    },
};

pub struct State {
    gfx_device: gfx_device_dx11::Device,
    gfx_factory: gfx_device_dx11::Factory,
    imgui_texture: Texture<gfx_device_dx11::Resources, B8_G8_R8_A8>,
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
    hq_sampler: d3d11::SamplerState,
    blit_staging_tex: d3d11::Texture2D,
    blit_dest_tex: d3d11::Texture2D,
    blit_dest_view: d3d11::ShaderResourceView,
}

#[allow(clippy::too_many_lines)]
pub fn init(screen_width: u16, screen_height: u16, imgui: &mut Context) -> State {
    let device;
    let context;
    let vs_copy;
    let ps_tex;
    let input_layout;
    let quad_vertex_buffer;
    let hq_sampler;
    let blit_dest_tex;
    let blit_dest_view;
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

        let desc = D3D11_TEXTURE2D_DESC {
            Width: screen_width.into(),
            Height: screen_height.into(),
            MipLevels: 0,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE
                | D3D11_BIND_RENDER_TARGET
                | D3D11_BIND_UNORDERED_ACCESS,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
            MiscFlags: D3D11_RESOURCE_MISC_GENERATE_MIPS,
        };
        blit_dest_tex = init_with_hresult(|p| (*device).CreateTexture2D(&desc, ptr::null(), p))
            .map(|p| d3d11::Texture2D::from_raw(p))
            .unwrap();

        blit_dest_view = init_with_hresult(|p| {
            (*device).CreateShaderResourceView(blit_dest_tex.as_resource().raw(), ptr::null(), p)
        })
        .map(|p| d3d11::ShaderResourceView::from_raw(p))
        .unwrap();
    }

    let (gfx_device, mut gfx_factory) =
        gfx_device_dx11::create_from_existing(device, context).unwrap();
    let (imgui_texture, _view, imgui_render_target) = gfx_factory
        .create_render_target(screen_width, screen_height)
        .unwrap();
    let imgui_renderer = Renderer::init(imgui, &mut gfx_factory, Shaders::HlslSm40).unwrap();
    let imgui_encoder = gfx_factory.create_command_buffer().into();

    let blit_staging_tex;
    unsafe {
        let device = d3d11::Device::from_ptr(device);

        let texture = imgui_texture.raw().resource().as_resource() as *mut ID3D11Texture2D;
        let texture = d3d11::Texture2D::from_ptr(texture);

        blit_staging_tex = create_staging_texture2d(&device, &texture).unwrap();
    }

    State {
        gfx_device,
        gfx_factory,
        imgui_texture,
        imgui_render_target,
        imgui_renderer,
        imgui_encoder,
        vs_copy,
        ps_tex,
        input_layout,
        quad_vertex_buffer,
        hq_sampler,
        blit_staging_tex,
        blit_dest_tex,
        blit_dest_view,
    }
}

pub fn draw(
    state: &mut State,
    draw_data: &DrawData,
    window_left: i32,
    window_top: i32,
    window_width: i32,
    window_height: i32,
) {
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

        // This is pretty inefficient, but I can't figure out how to get `CopyResource`
        // to work with a texture created by `gfx`. So we're going the long way around.
        (*context.raw()).CopyResource(
            state.blit_staging_tex.as_resource().raw(),
            state.imgui_texture.raw().resource().as_resource().cast(),
        );
        let mapped = state
            .blit_staging_tex
            .map(&context, 0, D3D11_MAP_READ, 0)
            .unwrap();
        (*context.raw()).UpdateSubresource(
            state.blit_dest_tex.as_resource().raw(),
            0,
            ptr::null(),
            mapped.data().as_ptr().cast(),
            mapped.row_pitch(),
            0,
        );
        drop(mapped);

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
        // Piggyback off rasterizer state leftover from imgui renderer
        (*context.raw()).RSSetScissorRects(1, &D3D11_RECT {
            left: window_left,
            top: window_top,
            right: window_left + window_width,
            bottom: window_top + window_height,
        });
        (*context.raw()).PSSetShader(state.ps_tex.raw(), ptr::null(), 0);
        (*context.raw()).PSSetSamplers(0, 1, &state.hq_sampler.raw());
        (*context.raw()).PSSetShaderResources(0, 1, &state.blit_dest_view.raw());
        (*context.raw()).OMSetRenderTargets(1, &target, ptr::null_mut());
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
