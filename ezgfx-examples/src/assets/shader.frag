#version 450

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform TransformUniform
{
    mat4 u_view_proj;
    vec3 u_something;
};
//layout(set = 0, binding = 0) uniform texture2D t_diffuse;
//layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main()
{
    f_color = vec4(1.0, 1.0, 1.0, 1.0);
    //texture(sampler2D(t_diffuse, s_diffuse), v_uv);
}