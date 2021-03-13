#version 450

// INPUTS:
// The vertex position
layout(location=0) in vec4 vertex;
// The uv texture coordinate input
layout(location=1) in vec2 uv_in;
// The model matrix columns
layout(location=2) in vec4 model_matrix_0;
layout(location=3) in vec4 model_matrix_1;
layout(location=4) in vec4 model_matrix_2;
layout(location=5) in vec4 model_matrix_3;
// The frame id of the sprite it in a texture array (or atlas)
layout(location=6) in uint tex_id_in;

// OUTPUTS TO FRAGMENT SHADER
// The uv texture coordinate
layout(location=0) out vec2 uv_out;
// The frame id of the sprite it in a texture array (used for animated sprites)
layout(location=1) flat out uint tex_id_out;

// We pass both orthographic and perspective projections to the Unifo
layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    mat4 persp;
};

void main() {

    mat4 model = mat4(
    model_matrix_0,
    model_matrix_1,
    model_matrix_2,
    model_matrix_3
    );

    // Pass the uv texture coordinate and animation frame id through unchanged
    uv_out = uv_in;
    tex_id_out = tex_id_out;

    // 1. Calculate o_pos, the orthographic projection of the vertex
    vec4 o_pos = ortho * model * vertex;

    gl_Position = o_pos;

}