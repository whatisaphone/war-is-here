#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(feature = "strict", deny(warnings))]

pub use blob::Blob;
pub use buffer::Buffer;
pub use compile::compile;
pub use device::Device;
pub use device_context::DeviceContext;
pub use input_layout::InputLayout;
pub use mapped_subresource::MappedSubresource;
pub use pixel_shader::PixelShader;
pub use resource::Resource;
pub use sampler_state::SamplerState;
pub use shader_resource_view::ShaderResourceView;
pub use texture_2d::Texture2D;
pub use vertex_shader::VertexShader;

#[macro_use]
mod macros;

mod blob;
mod buffer;
mod compile;
mod device;
mod device_context;
mod input_layout;
mod mapped_subresource;
mod pixel_shader;
mod resource;
mod sampler_state;
mod shader_resource_view;
mod texture_2d;
mod utils;
mod vertex_shader;
