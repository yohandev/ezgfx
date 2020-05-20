use ezgfx::vertex;

///                ~ vertices ~
/// define the vertex struct as-is in the shader,
/// then add the #[vertex] attribute. That's it! 
///                     ~

#[vertex]
pub struct PosColorVertex
{
    pub position: [f32; 3],
    pub color: [f32; 3]
}