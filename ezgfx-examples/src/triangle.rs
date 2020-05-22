use ezgfx::*;

#[vertex]
pub struct PosColorVertex
{
    pub position: [f32; 3],
    pub color: [f32; 3]
}

#[uniform]
pub struct TransformUniform
{
    pub view_proj: [f32; 16],
    pub model: [f32; 16]
}

pub struct MyGraphicsPipeline;

#[pipeline(render)]
impl MyGraphicsPipeline
{
    type Vertex: Vertex = PosColorVertex;
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

    // create a window and render queue from the event loop
    let (win, ren) = RenderQueue::create(&evt_loop);
    
    let uni = TransformUniform::create(&ren, [0.0; 16], [0.0; 16]);
    let pip = MyGraphicsPipeline::create(&ren, &uni);

    evt_loop.run
    (
        move |e, _, ctrl_flow|
        {
            *ctrl_flow = ControlFlow::Poll;
        }
    );
}