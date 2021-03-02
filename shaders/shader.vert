#version 450

layout(location=0) in vec4 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 model_matrix_0;
layout(location=3) in vec4 model_matrix_1;
layout(location=4) in vec4 model_matrix_2;
layout(location=5) in vec4 model_matrix_3;
layout(location=6) in uint frame_id;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) flat out uint v_tex_id;


layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    mat4 persp;
};

void main() {

    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );

    v_tex_coords = a_tex_coords;
    v_tex_id = frame_id;

    vec4 centre = vec4(vec3(0.0), 1.0);

    vec4 p_c = persp * model_matrix * centre;

    vec4 o_c = ortho * model_matrix * centre;
    vec4 o_pos = ortho * model_matrix * a_position;

    vec4 o_c_ndc = o_c/o_c.w;
    vec4 p_c_ndc =  p_c/p_c.w;
    vec4 d = p_c_ndc - o_c_ndc;

    vec4 o_pos_ndc = o_pos/o_pos.w + d;

    gl_Position =  o_pos_ndc * o_pos.w;

}