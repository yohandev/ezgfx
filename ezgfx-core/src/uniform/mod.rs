/// implementations of the Uniform trait for common uniform types

mod impl_buffer2;
mod impl_buffer;
mod texture;
mod buffer;

pub use texture::*;
pub use buffer::*;

/// represents a generic uniform
pub trait Uniform
{
    // /// get the bind group layout of this uniform instance
    // fn bind_group_layout(&self, stage: wgpu::ShaderStage, ) -> &wgpu::BindGroupLayout;

    // /// get the bind group of this uniform instance
    // fn bind_group(&self) -> &wgpu::BindGroup;

    /// get this uniform's BindingType, which is then used in BindGroupLayoutEntry
    fn ty(&self) -> wgpu::BindingType;

    /// get this uniform's BindingResource, which is then used in BindGroupDescriptor
    fn resource(&self) -> wgpu::BindingResource;
}