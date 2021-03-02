#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 1) flat in uint v_tex_id;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform texture2D t_Color[128];
layout(set = 1, binding = 1) uniform sampler s_Color;

void main() {
    vec4 texel = texture(sampler2D(t_Color[v_tex_id], s_Color), v_TexCoord);
    if(texel.a < 0.5) {
        discard;
    }
    o_Target = texel;
}
