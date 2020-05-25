use spirv_reflect::types::*;
use wgpu::*;

use crate::{ Uniform, RenderQueue };

pub struct Sampler
{
    sampler: wgpu::Sampler
}

impl Uniform for Sampler
{
    fn ty(&self) -> BindingType
    {
        BindingType::Sampler { comparison: false }
    }

    fn resource(&self) -> BindingResource
    {
        BindingResource::Sampler(&self.sampler)
    }

    fn matches(&self, shader_repr: &ReflectDescriptorBinding) -> bool
    {
        shader_repr.resource_type == ReflectResourceType::ShaderResourceView
    }
}

impl Sampler
{
    pub fn create(render: &RenderQueue, opt: Option<SamplerDescriptor>) -> Self
    {
        let sampler = render.device.create_sampler
        (
            &opt.unwrap_or(SamplerDescriptor
            {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Linear, // change to nearest for pixel textures
                min_filter: FilterMode::Nearest,
                mipmap_filter: FilterMode::Nearest,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: CompareFunction::Always
            })
        );

        Self { sampler }
    }
}