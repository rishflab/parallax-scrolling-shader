#version 450
// INPUTS:
// The vertex position
layout(location=0) in vec4 a_position;
// The uv texture coordinate
layout(location=1) in vec2 a_tex_coords;
// The model matrix columns
layout(location=2) in vec4 model_matrix_0;
layout(location=3) in vec4 model_matrix_1;
layout(location=4) in vec4 model_matrix_2;
layout(location=5) in vec4 model_matrix_3;
// The frame id of the sprite it in a texture array (used for animated sprites)
layout(location=6) in uint frame_id;

// OUTPUTS TO FRAGMENT SHADER
// The uv texture coordinate
layout(location=0) out vec2 v_tex_coords;
// The frame id of the sprite it in a texture array (used for animated sprites)
layout(location=1) flat out uint v_tex_id;


layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
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

    gl_Position =  ortho * model_matrix * a_position;

}