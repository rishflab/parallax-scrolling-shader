#version 450

layout(location=0) in vec4 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 centre;

layout(location=0) out vec2 v_tex_coords;

layout(set=0, binding=0)
uniform Uniforms {
//    mat4 ortho_near;
//    mat4 ortho_far;
    mat4 persp;
};

layout(set=0, binding=1)
buffer Instances {
    mat4 s_models[];
};

void main() {
    v_tex_coords = a_tex_coords;
//    vec4 ortho_near = ortho_near * s_models[gl_InstanceIndex] * a_position;
//    vec4 ortho_far_pos = ortho_far * s_models[gl_InstanceIndex] * centre;
//    vec4 ortho_far_pos = ortho_far * s_models[gl_InstanceIndex] * a_position;

    vec4 persp_pos = persp * s_models[gl_InstanceIndex] * a_position;
//    vec4 d = ortho_far_pos - persp_pos;

//    mat4 translate = mat4(
//        vec4(1.0, 0.0, 0.0, 0.0),
//        vec4(0.0, 1.0, 0.0, 0.0),
//        vec4(0.0, 0.0, 1.0, 0.0),
//        vec4(d.xyz, 1.0)
//    );

    gl_Position =  persp_pos;
}