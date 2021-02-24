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
//    v_tex_coords = a_tex_coords;
//
//    vec4 persp_centre = persp * s_models[gl_InstanceIndex] * centre;
//
//    vec4 ortho_centre = ortho * s_models[gl_InstanceIndex] * centre;
//    vec4 ortho_pos = ortho * s_models[gl_InstanceIndex] * a_position;
//
//
//    vec4 bad =  persp * s_models[gl_InstanceIndex] * a_position;
//
//    vec4 good =  persp_centre +  (ortho_pos - ortho_centre);
//
//
//    gl_Position =  good;

    v_tex_coords = a_tex_coords;

    vec4 p_c = persp * s_models[gl_InstanceIndex] * centre;

    vec4 o_c = ortho * s_models[gl_InstanceIndex] * centre;
    //vec4 o_pos = ortho * s_models[gl_InstanceIndex] * a_position;

    vec4 o_c_ndc = o_c/o_c.w;
    vec4 p_c_ndc =  p_c/p_c.w;
    vec4 d = o_c_ndc - p_c_ndc;

    mat4 shift = mat4(
        vec4(1.0, 0.0, 0.0 ,0.0),
        vec4(0.0, 1.0, 0.0 ,0.0),
        vec4(0.0, 0.0, 1.0 ,0.0),
        vec4(d.xyz, 1.0)
    );

    gl_Position =  shift * ortho * s_models[gl_InstanceIndex] * a_position;
}