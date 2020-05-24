#version 450

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_uv;

layout(location = 0) out vec2 v_uv;

layout(set = 5, binding = 0) uniform TransformUniform
{
    mat4 u_view_proj;
    vec3 u_something;
};
// layout(set = 2, binding = 0) uniform PooPoo
// {
//     float pee;
// };
//layout(set = 0, binding = 0) uniform texture2D MyTex;
//layout(set = 0, binding = 1) uniform texture2D YourTex;
//layout(set = 0, binding = 2) uniform sampler MySampler;

void main()
{
    v_uv = a_uv;
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}