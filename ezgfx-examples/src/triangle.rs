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

// the MyGraphicsPipeline struct isn't used as a pipeline itself.
// it's used as a factory that creates pipelines of a certain type,
// as defined in the #[pipeline(..)] impl
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
    let (win, mut ctx) = RenderContext::create(&evt_loop);
    
    // pipeline resources -- these are all uniforms
    let uni = TransformUniform::create(&ctx, [0.0; 16]);
    let tex = Texture::from_file(&ctx, "assets/texture.png");
    let smp = Sampler::create(&ctx, None);

    // create the pipeline itself
    let pip = MyGraphicsPipeline::create(&ctx, &uni, &tex, &smp);

    // geometry(container for vertices, indices)
    let geo = Geometry::create(&ctx, VERTICES, INDICES);

    evt_loop.run
    (
        move |e, _, ctrl_flow|
        {
            use winit::event::Event;

            *ctrl_flow = ControlFlow::Poll;

            match e
            {
                Event::RedrawRequested(_) =>
                {
                    // -- render frame --
                    ctx.begin_frame();
                    
                    //let pass = ctx.begin_render_pass([0.1, 0.2, 0.3, 1.0]);   // clear frameview
                    // .set_render_pipeline(&pip)                  // set rendering pipeline
                    // .draw_geometry(&geo)                        // draw triangle
                    // .submit(&mut ctx);
                    // pass.submit(&mut ctx);
                },
                Event::MainEventsCleared => win.request_redraw(),
                _ => {}
            }
        }
    );
}

fn render<'a>(ctx: &'a mut RenderContext)
{
    
}

const VERTICES: &[PosTexVertex] = &
[
    PosTexVertex { position: [0.5, 0.5, 0.0], tex: [1.0, 1.0] },
    PosTexVertex { position: [-0.5, 0.5, 0.0], tex: [-1.0, 1.0] },
    PosTexVertex { position: [-0.5, -0.5, 0.0], tex: [-1.0, -1.0] },
    PosTexVertex { position: [0.5, -0.5, 0.0], tex: [1.0, -1.0] },
];

const INDICES: &[u16] = &
[
    0, 1, 2, 0, 2, 3
];