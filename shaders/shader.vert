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

    vec4 ortho_centre = ortho * s_models[gl_InstanceIndex] * centre;
    vec4 ortho_pos = ortho * s_models[gl_InstanceIndex] * a_position;

    vec4 persp_centre = persp * s_models[gl_InstanceIndex] * centre;

    gl_Position =  persp_centre + 50.0 * (ortho_pos - ortho_centre);

    //vec4 persp_pos = persp * s_models[gl_InstanceIndex] * a_position;
    //gl_Position =  persp_pos;
}