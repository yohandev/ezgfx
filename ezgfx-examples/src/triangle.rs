use ezgfx::*;

#[vertex]
pub struct PosTexVertex
{
    pub position: [f32; 3],
    pub tex: [f32; 2]
}

#[uniform]
pub struct TransformUniform
{
    pub view_proj: [f32; 16]
}

pub struct MyGraphicsPipeline;

#[pipeline(render)]
impl MyGraphicsPipeline
{
    type Vertex: Vertex = PosTexVertex;
    type Index: Index   = u16;

    const VERT_PATH: str = "assets/shader.vert";
    const FRAG_PATH: str = "assets/shader.frag";
}

pub fn run()
{
    // winit is bundled in ezgfx
    use winit::event_loop::*;

    // make the event loop yourself. it's used for much more than
    // just graphics, which is beyond the scope of ezgfx
    let evt_loop = EventLoop::new();

    // create a window and render context from the event loop
    let (win, ctx) = RenderContext::create(&evt_loop);
    
    // pipeline resources -- these are all uniforms
    let uni = TransformUniform::create(&ctx, [0.0; 16]);
    let tex = Texture::from_file(&ctx, "assets/texture.png");
    let smp = Sampler::create(&ctx, None);

    // create the pipeline itself
    let pip = MyGraphicsPipeline::create(&ctx, &uni, &tex, &smp);

    evt_loop.run
    (
        move |e, _, ctrl_flow|
        {
            *ctrl_flow = ControlFlow::Poll;
        }
    );
}