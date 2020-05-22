use wgpu::*;

/// represents a vertex, to be used in meshes
pub trait Vertex: crate::BufMember
{
    /// description of this shader, as represented in shaders
    const DESC: &'static [VertexAttributeDescriptor];
}

/// implementors can be used in vertex descriptions 
pub trait VertexAttribute: crate::BufMember
{
    const FORMAT: VertexFormat;
}

impl VertexAttribute for [f32; 4] { const FORMAT: VertexFormat = VertexFormat::Float4; }
impl VertexAttribute for [f32; 3] { const FORMAT: VertexFormat = VertexFormat::Float3; }
impl VertexAttribute for [f32; 2] { const FORMAT: VertexFormat = VertexFormat::Float2; }
impl VertexAttribute for [f32; 1] { const FORMAT: VertexFormat = VertexFormat::Float; }
impl VertexAttribute for f32 { const FORMAT: VertexFormat = VertexFormat::Float; }

impl VertexAttribute for [i32; 4] { const FORMAT: VertexFormat = VertexFormat::Int4; }
impl VertexAttribute for [i32; 3] { const FORMAT: VertexFormat = VertexFormat::Int3; }
impl VertexAttribute for [i32; 2] { const FORMAT: VertexFormat = VertexFormat::Int2; }
impl VertexAttribute for [i32; 1] { const FORMAT: VertexFormat = VertexFormat::Int; }
impl VertexAttribute for i32 { const FORMAT: VertexFormat = VertexFormat::Int; }

impl VertexAttribute for [u32; 4] { const FORMAT: VertexFormat = VertexFormat::Uint4; }
impl VertexAttribute for [u32; 3] { const FORMAT: VertexFormat = VertexFormat::Uint3; }
impl VertexAttribute for [u32; 2] { const FORMAT: VertexFormat = VertexFormat::Uint2; }
impl VertexAttribute for [u32; 1] { const FORMAT: VertexFormat = VertexFormat::Uint; }
impl VertexAttribute for u32 { const FORMAT: VertexFormat = VertexFormat::Uint; }