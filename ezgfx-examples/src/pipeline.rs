use ezgfx::pipeline;

use crate::vertex::PosColorVertex;

#[pipeline("assets/shader.vert", "assets/shader.frag")]
pub struct MyGraphicsPipeline
{
    VERTEX:     PosColorVertex,
    INDEX:      u16,

    //#[pipeline_attr(set = 0, stage = vertex)]
    //transform:  Uniform, // set 0

    //#[pipeline_attr(set = 1, stage = fragment)]
    //albedo:     Texture, // set 1

    //#[pipeline_attr(set = 2, stage = fragment)]
    //normal:     Texture, // set 2
}

// #[pipeline("assets/shader.comp")]
// struct MyComputePipeline
// {

// }