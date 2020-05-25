/// implementations of the Uniform trait for common uniform types

mod texture;
mod sampler;

pub use texture::*;
pub use sampler::*;

/// represents a generic uniform
pub trait Uniform
{
    /// get this uniform's BindingType, which is then used in BindGroupLayoutEntry
    fn ty(&self) -> wgpu::BindingType;

    /// get this uniform's BindingResource, which is then used in BindGroupDescriptor
    fn resource(&self) -> wgpu::BindingResource;

    /// does this rust uniform struct match the shader uniform?
    fn matches(&self, shader_repr: &spirv_reflect::types::ReflectDescriptorBinding) -> bool;
}