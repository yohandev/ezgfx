use wgpu::*;

use crate::Uniform;

pub struct Texture
{
    buf: Buffer   
}

impl Uniform for Texture
{
    fn ty(&self) -> BindingType
    {
        todo!()
    }

    fn resource(&self) -> BindingResource
    {
        todo!()
    }

    fn matches(&self, shader_repr: &spirv_reflect::types::ReflectDescriptorBinding) -> bool
    {
        todo!()
    }
}