# Parallax Scrolling Vertex Shader

There are many tutorial and articles on the internet describing how to implement parallax scrolling using layers.
Sprites are assigned to a layers based on their depth and the layers are translated at different speeds to create the parallax scrolling effect.
This method is unnecessarily tedious and fine tuning layer speeds to achieve a realistic parallax effect can be a time consuming task.

This article describes a layerless approach to the parallax scrolling technique where the parallax shift is computed
from the depth of the sprite. This method allows the artist or game developer to specify the 3D coordinates of the
sprite and the field-of-view of the camera as they would in a 3D scene. The artist or game-developer can
think of their game world naturally, in three dimensions, instead of in layers with varying and somewhat arbitrary
translation speeds.

This approach uses both perspective and orthographic cameras to create the parallax scrolling effect without the use
of layers. We take advantage of 3D dimensional nature of the perspective camera to compute the position of the
sprite centre in the final scene. The sprite is rendered at this position using an orthographic camera. By combining 
orthographic and perspective projections we achieve parallax scrolling and pixel perfect sprite rendering without the
use of layers.

Build and run the example using `cargo run`. Note: you will need to [install the Rust programming language](https://rustup.rs) to compile the example. 

#### Why not use a perspective camera?
Perspective cameras intrinsically render objects with parallax scrolling effect. Far away objects appear to move
slower than close objects. The problem with using a perspective camera to render 2D sprites is that far away objects also 
appear to be smaller that close objects. Resizing of sprites textures can ruin certain art styles, notably pixel art.

## Implementation 

This technique can be implemented purely in the vertex shader stage allowing us to leverage the power of the GPU.
Some minor changes are required to a standard vertex shader. First let us consider a fairly [standard vertex shader](shaders/standard_shader.vert) for a 2D game.
We are simply transforming the sprite vertex to clip space using the orthographic projection and model matrices. 

```glsl
layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
};

void main() {
    ...
    gl_Position =  ortho * model_matrix * a_position;
}
```

To create a [parallax scrolling shader](shaders/shader.vert) we need to add a perspective camera to our uniforms.

```glsl
layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    mat4 persp;
};
```

We use the perspective and orthographic cameras to calculate where the sprite will be
in normalised device coordinates(ndc) if it were rendered using a perspective camera and with an 
orthographic camera. We calculate the distance between these two centres in ndc space and shift
the orthographic projection of the sprite vertex by this distance. We are rendering the sprite
with a shifted orthographic camera. This shifting is what causes the parallax scrolling effect.


```glsl

void main() {
    ...

    // 0. Pass the uv texture coordinate and animation frame id through unchanged
    v_tex_coords = a_tex_coords;
    v_tex_id = frame_id;

    // 1. We assume our sprite quads are always centered at (0.0, 0.0, 0.0)
    vec4 centre = vec4(vec3(0.0), 1.0);

    // 2. Calculate p_c, the perspective projection of the sprite quad centre
    vec4 p_c = persp * model_matrix * centre;

    // 3. Calculate o_c, the orthographic projection of the sprite quad centre
    vec4 o_c = ortho * model_matrix * centre;

    // 4. Calculate o_pos, the orthographic projection of the vertex
    vec4 o_pos = ortho * model_matrix * a_position;

    // 5. Convert p_c, o_c and o_pos to normalised device coordinates
    vec4 o_c_ndc = o_c/o_c.w;
    vec4 p_c_ndc =  p_c/p_c.w;
    vec4 o_pos_ndc = o_pos/o_pos.w;

    // 6. Calculate d_ndc, the distance between the perspective and orthographic centres in ndc coordinates
    vec4 d_ndc = p_c_ndc - o_c_ndc;

    // 7. Calculate the final ndc position, pos_dnc by shift o_pos_ndc by d_ndc
    // This is the key step were we shift our the orthoghraphic projection of our sprite centre to where
    // it would be located if it were rendered with a perspective projection
    vec4 pos_ndc = o_pos_ndc + d_ndc;

    // 8. Convert back to clip space for output to the rasteriser
    vec4 pos = o_pos_ndc * o_pos.w;

    gl_Position = pos;

}
```

#### Some notes on the Orthographic and Perspective Cameras

The orthographic camera parameters will ultimately determine the size of the sprite on the screen. In order to avoid
artifacts from texture filtering you will need to ensure that that orthographic width and height result in a 1 to 1
pixel mapping of your sprite.

The perspective camera will determine the position of the sprite on the screen. The perspective camera should be defined
normally. 

See the [camera](src/camera.res) struct for an example of how to generate perspective and orthographic matrices.
