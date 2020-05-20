## concept usage
```rust
use ezgfx::*;

#[vertex]
pub struct PosTexVertex
{
    // vertex declaration is generated by the macro and can
    // be extended to support more types.
    pub [f32; 3] pos;
    pub [f32; 2] tex;
}

#[pipeline(render)]
pub struct MyPipeline
{
    texture: TextureSet,
    uniform: MyUniform
}

impl RenderPipeline for MyPipeline
{
    // specify what type of input vertices/indices are used
    // by the pipeline.
    type Vertex: Vertex = PosTexVertex;
    type Index: Index = u16;

    // macro will scan GLSL#450 source and determine uniforms,
    // textures, etc. while generating the verbose pipeline
    // code behind-the-scenes.
    const VERT_SRC: str = include_bytes!("assets/shader.vert");
    const FRAG_SRC: str = include_bytes!("assets/shader.frag");
}

fn main()
{
    // the event loop is the core of an application, as it
    // provides events. it's not the intent of ezgfx to take
    // full control of your code, so you have to create the
    // event loop yourself.
    let event_loop = ezgfx::util::event_loop();

    // the render queue is both the factory and command queue
    // used for rendering. don't lose this object! 
    let queue = RenderQueue::create(&event_loop);

    // the create method was generated by the pipeline macro
    // and takes appropriate parameters based off the shader.
    let my_pipeline = MyPipeline::create
    (
        queue,                          // queue is also res factory

        ["assets/tex.png"],             // set = 0 -> texture
                                        // provide an array of paths
                                        // to textures. there can be
                                        // multiple textures per sam-
                                        // pler, hence the array.

        (Matrix4::<f32>::identity())    // set = 1 -> uniform
                                        // this is a union struct
                                        // that matches that defined
                                        // in the shader. a GLSL type
                                        // map can be defined for your
                                        // math library using the
                                        // ShaderUniformType trait.
    );

    my_pipeline.set_trans_uniforms      // the set_trans_uniform was
    (                                   // generated by the pipeline
        (Matrix4::<f32>::random())      // macro, from the TransUniform
    );                                  // uniform in the shader. it's
                                        // snake_cases automatically.

    my_pipeline.set(0, [0u8; 80 * 60]); // you can also set shader
                                        // resources by set index.
                                        // this is less convenient
                                        // because it will need a [u8]
                                        // and is unsafe. in debug
                                        // mode, the method performs
                                        // size assertions.

    queue.frame();                      // next framebuffer
    queue.clear                         // clear with color
    (
        Clear::ColorDepth(0.1, 0.2, 0.3, 1.0)
    );                                  // you can also Clear::None, to
                                        // do things like multiple passes
                                        // to render transparent objects.

    queue.set_pipeline(&my_pipeline);   // set render pipeline
                                        // this will bind all associated
                                        // resources automatically.
    queue.draw(my_geometry);            // sets vertex/index buffers
                                        // and draws indexed.

    // NOTE: this might not actually run because we're not processing
    // event_loop events. see the complete examples in ezgfx-examples/
}
```

```glsl
#version 450

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_uv;
layout(location = 0) out vec2 v_uv;

layout(set = 1, binding = 0) uniform TransUniforms
{
    mat4 u_view_proj;
};

void main()
{
    v_uv = a_uv;
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}
```

```glsl
#version 450

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main()
{
    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_uv);
}
```