use wgpu::*;

pub trait Pipeline
{
    /// whether this pipeline is used for rendering or compute operations
    const TYPE: PipelineType;

    /// path to the shader(s) - 2(frag + vert) for render, 1(comp) for compute
    const SHADER_PATHS: &'static [&'static str];

    /// create an instance of the pipeline
    fn create(queue: crate::RenderQueue);
}

pub enum PipelineType
{
    Render,
    Compute
}

pub trait PipelineResource
{
    fn bind_layout(index: usize, stage: ShaderStage);
}