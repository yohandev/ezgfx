pub enum RenderAction
{
    /// begin a renderpass with optional clear colour
    RenderPassBegin(Option<[f32; 4]>),

    /// submit the current render pass to the queue
    RenderPassSubmit,
}