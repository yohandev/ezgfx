use bytemuck::Pod;
use wgpu::*;

pub trait Index: Pod
{
    const FORMAT: IndexFormat;
}

impl Index for u16
{
    const FORMAT: IndexFormat = IndexFormat::Uint16;
}

impl Index for u32
{
    const FORMAT: IndexFormat = IndexFormat::Uint32;
}