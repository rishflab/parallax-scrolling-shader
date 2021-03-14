---
layout: post
title: GPU Parallax Scrolling 
---

If you have reading this article you are probably already familiar with parallax scrolling through a tutorial or article
explaining how to implement the effect using translating layers.
There is a much easier and more performant way to do this by combining orthographic and perspective projections in the [vertex shader](https://github.com/rishflab/parallax-scrolling-shader/blob/master/shaders/shader.vert).
 
 #### Adding the perspective Camera
First we add a perspective camera to our vertex shader uniforms. The perspective camera will determine the 
position of the sprite on the screen. The position, look-at direction and min and max distance must be the same as
the orthographic camera. The field-of-view can be modified to adjust the intensity of the parallax affect.

```glsl
layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    // NEW!
    mat4 persp;
};
```


The orthographic camera parameters will ultimately determine the size of the sprite on the screen. In order to avoid
artifacts from texture filtering you will need to  ensure that that orthographic width and height result in a 1 to 1
pixel mapping of your sprite.
See [camera](https://github.com/rishflab/parallax-scrolling-shader/blob/master/src/camera.rs) for an example of how to generate perspective and orthographic matrices.


 #### Shifting the Orthographic Camera
Next we use the perspective and orthographic cameras to calculate where the sprite centre will be
in normalised device coordinates (`ndc`); rendered using perspective and
orthographic cameras. We calculate the distance between these two centres in `ndc` space and shift
the orthographic projection of the sprite vertex by this distance. 

```glsl
void main() {
    ...

    // 1. We assume our sprites centres are always at (0.0, 0.0, 0.0)
    vec4 centre = vec4(vec3(0.0), 1.0);

    // 2. Calculate p_c, the perspective projection of the sprite quad centre
    vec4 p_c = persp * model * centre;

    // 3. Calculate o_c, the orthographic projection of the sprite quad centre
    vec4 o_c = ortho * model * centre;

    // 4. Calculate o_pos, the orthographic projection of the vertex
    vec4 o_pos = ortho * model * vertex;

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

We have produced a pixel-perfect orthographic rendering of the sprite and shifted it, to where it should of been if it
were rendered using a perspective camera. By combining orthographic and perspective cameras we achieve pixel-perfect
parallax scrolling without having to add a layer system to our renderer.


#### Try it yourself

Clone the [repo](https://github.com/rishflab/parallax-scrolling-shader.git)

Build and run the example using `cargo run` to see it in action.
Note: you will need to [install the Rust programming language](https://rustup.rs) to compile the example. 

Use the left and right arrow keys to move camera and observe the parallax scrolling effect.

The scene is defined in [main.rs](https://github.com/rishflab/parallax-scrolling-shader/blob/master/src/main.rs). Feel
free to move the trees around and adjust the field-of-view of the parallax camera. The trees that are further from the
camera appear to move slower as expected.


