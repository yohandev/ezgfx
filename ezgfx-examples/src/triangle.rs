use ezgfx::*;

#[vertex]
pub struct PosColorVertex
{
    pub position: [f32; 3],
    pub color: [f32; 3]
}

pub struct MyGraphicsPipeline;

#[pipeline(render)]
impl MyGraphicsPipeline
{
    type Vertex: Vertex = PosColorVertex;
    type Index: Index   = u16;

    const VERT_PATH: str = "ezgfx-examples/src/assets/shader.vert";
    const FRAG_PATH: str = "ezgfx-examples/src/assets/shader.frag";
}