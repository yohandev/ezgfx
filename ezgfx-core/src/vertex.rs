//pub use wgpu::{ VertexAttributeDescriptor, VertexFormat};
//pub use bytemuck::{ Zeroable, Pod };
use bytemuck::*;
use wgpu::*;

/// represents a vertex, to be used in meshes
pub trait Vertex: Pod
{
    const DESC: &'static [VertexAttributeDescriptor];
    const SIZE: usize;
}

pub trait VertexAttribute
{
    const FORMAT: VertexFormat;
    const SIZE: usize;
}

impl VertexAttribute for [f32; 4]
{
    const FORMAT: VertexFormat = VertexFormat::Float4;
    const SIZE: usize = std::mem::size_of::<[f32; 4]>();
}

impl VertexAttribute for [f32; 3]
{
    const FORMAT: VertexFormat = VertexFormat::Float3;
    const SIZE: usize = std::mem::size_of::<[f32; 3]>();
}

impl VertexAttribute for [f32; 2]
{
    const FORMAT: VertexFormat = VertexFormat::Float2;
    const SIZE: usize = std::mem::size_of::<[f32; 2]>();
}

impl VertexAttribute for [f32; 1]
{
    const FORMAT: VertexFormat = VertexFormat::Float;
    const SIZE: usize = std::mem::size_of::<[f32; 1]>();
}

impl VertexAttribute for f32
{
    const FORMAT: VertexFormat = VertexFormat::Float;
    const SIZE: usize = std::mem::size_of::<f32>();
}

impl VertexAttribute for [i32; 4]
{
    const FORMAT: VertexFormat = VertexFormat::Int4;
    const SIZE: usize = std::mem::size_of::<[i32; 4]>();
}

impl VertexAttribute for [i32; 3]
{
    const FORMAT: VertexFormat = VertexFormat::Int3;
    const SIZE: usize = std::mem::size_of::<[i32; 3]>();
}

impl VertexAttribute for [i32; 2]
{
    const FORMAT: VertexFormat = VertexFormat::Int2;
    const SIZE: usize = std::mem::size_of::<[i32; 2]>();
}

impl VertexAttribute for [i32; 1]
{
    const FORMAT: VertexFormat = VertexFormat::Int;
    const SIZE: usize = std::mem::size_of::<[i32; 1]>();
}

impl VertexAttribute for i32
{
    const FORMAT: VertexFormat = VertexFormat::Int;
    const SIZE: usize = std::mem::size_of::<i32>();
}

impl VertexAttribute for [u32; 4]
{
    const FORMAT: VertexFormat = VertexFormat::Uint4;
    const SIZE: usize = std::mem::size_of::<[u32; 4]>();
}

impl VertexAttribute for [u32; 3]
{
    const FORMAT: VertexFormat = VertexFormat::Uint3;
    const SIZE: usize = std::mem::size_of::<[u32; 3]>();
}

impl VertexAttribute for [u32; 2]
{
    const FORMAT: VertexFormat = VertexFormat::Uint2;
    const SIZE: usize = std::mem::size_of::<[u32; 2]>();
}

impl VertexAttribute for [u32; 1]
{
    const FORMAT: VertexFormat = VertexFormat::Uint;
    const SIZE: usize = std::mem::size_of::<[u32; 1]>();
}

impl VertexAttribute for u32
{
    const FORMAT: VertexFormat = VertexFormat::Uint;
    const SIZE: usize = std::mem::size_of::<u32>();
}