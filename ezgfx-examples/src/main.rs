use ezgfx::winit::event_loop::*;
use ezgfx::*;

mod pipeline;
mod vertex;

fn main()
{
    let evt_loop = EventLoop::new();

    let (win, ren) = RenderQueue::create(&evt_loop);

    let h = vertex::PosColorVertex::DESC;
    let p = pipeline::MyGraphicsPipeline::create(ren);

    println!("{:?}", p);

    evt_loop.run
    (
        move |e, _, ctrl_flow|
        {
            *ctrl_flow = ControlFlow::Poll;
        }
    );
}
