#[shader("assets/simple.vert")] // TODO: in future, parse shader directly and extract textures
pub struct MyShaderVertModule
{
    #[set(1)]
    albedo: Texture,

    #[set(2)]
    normal: Texture,
}

#[shader("assets/simple.frag")]
pub struct MyShaderFragModule
{
    #[set(0)]
    transform: Uniform,
}

#[pipeline]
pub struct MyPipelineDesc
{
    #[set(0)]
    transform: Uniform,

    #[set(1)]
    albedo: Texture,

    #[set(2)]
    normal: Texture,
}

impl MyPipelineDesc
{
    pub fn load(shaders: &[str])
    {

    }
}