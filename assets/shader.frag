#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 2) uniform texture2D t_Color;
layout(set = 0, binding = 3) uniform sampler s_Color;

void main() {
    vec4 texel = texture(sampler2D(t_Color, s_Color), v_TexCoord);
    if(texel.a < 0.5) {
        discard;
    }
    o_Target = texel;
}
