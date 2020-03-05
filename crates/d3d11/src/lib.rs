#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub use wrap::{
    blob::Blob,
    buffer::Buffer,
    compile::compile,
    device::Device,
    device_context::DeviceContext,
    input_layout::InputLayout,
    mapped_subresource::MappedSubresource,
    pixel_shader::PixelShader,
    resource::Resource,
    sampler_state::SamplerState,
    shader_resource_view::ShaderResourceView,
    texture_2d::Texture2D,
    vertex_shader::VertexShader,
};

#[macro_use]
mod macros;

mod utils;
mod wrap;
