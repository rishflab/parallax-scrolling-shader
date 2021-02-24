#version 450

layout(location=0) in vec4 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 centre;

layout(location=0) out vec2 v_tex_coords;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    mat4 persp;
};

layout(set=0, binding=1)
buffer Instances {
    mat4 s_models[];
};

void main() {

    v_tex_coords = a_tex_coords;

    vec4 p_c = persp * s_models[gl_InstanceIndex] * centre;

    vec4 o_c = ortho * s_models[gl_InstanceIndex] * centre;
    vec4 o_pos = ortho * s_models[gl_InstanceIndex] * a_position;

    vec4 o_c_ndc = o_c/o_c.w;
    vec4 p_c_ndc =  p_c/p_c.w;
    vec4 d = p_c_ndc - o_c_ndc;

    vec4 o_pos_ndc = o_pos/o_pos.w + d;

    gl_Position =  o_pos_ndc * o_pos.w;

}