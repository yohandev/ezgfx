use crate::*;

pub struct Pipeline
{
    pub pipeline: wgpu::RenderPipeline,
    pub bindings: Vec<(u32, wgpu::BindGroup)>
}

/// describes a pipeline used to render things
pub trait RenderPipeline
{
    /// type of vertex used in this pipeline
    type Vertex: Vertex;
    /// type of index used in this pipeline
    type Index: Index;

    /// path to the vertex shader
    const VERT_PATH: str;
    /// path to the fragment shader
    const FRAG_PATH: str;
}