use wgpu::*;

use crate::RenderContext;
use crate::Vertex;
use crate::Index;

pub struct Geometry
{
    /// vertex count
    pub v_count: usize,
    /// index count
    pub i_count: usize,

    pub(crate) v_buf: Buffer,
    pub(crate) i_buf: Buffer
}

impl Geometry
{
    pub fn create<V: Vertex, I: Index>(ctx: &RenderContext, vertices: &[V], indices: &[I]) -> Self
    {
        let v_buf = ctx.device.create_buffer_with_data  // vertex buffer
        (
            bytemuck::cast_slice(vertices),
            BufferUsage::VERTEX
        );

        let i_buf = ctx.device.create_buffer_with_data  // index buffer
        (
            bytemuck::cast_slice(indices),
            BufferUsage::INDEX
        );

        Self
        {
            v_count: vertices.len(),
            i_count: indices.len(),
            v_buf,
            i_buf
        }
    }
}