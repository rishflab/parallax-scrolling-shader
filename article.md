# The Correct Way to Implement Parallax Scrolling in 2D games

**Prerequisite Knowledge: Basic understanding of computer graphics or cameras and linear algebra**

Parallax scrolling is a technique in computer graphics where background images move past the camera more slowly than 
foreground images, creating an illusion of depth in a 2D scene of distance. Parallax scrolling was first used in 2D 
animated cartoons in 1920's. 

Walt Disney Studio's describes the technique they used to achieve the parallax scrolling effect as multiplane camera
process. Various parts of the artwork layers are left transparent to allow other layers to be seen behind them.
The movements are calculated and photographed frame by frame, with the result being an illusion of depth by having 
several layers of artwork moving at different speeds: the further away from the camera, the slower the speed. 

This effect is used heavily in 2D video games. The implementation of the parallax effect in 2D games draws inspiration
from the multiplane camera technique. Sprites are assigned to a layers based on their depth and the layers are
translated at different speeds to create the parallax scrolling effect. To increase the depth 'resolution' the artist
must create new layers and fine tune the translation speed of these layers to achieve the desired depth effect.

This article describes a layer-less approach to the parallax scrolling technique where the parallax effect is computed
from the depth of the sprite. This method allows the artist or game developer to specify the 3D coordinates of the
sprite and the field-of-view of the camera as they would in a 3D scene. This enables the artist or game-developer to
think of their game world in 3D dimensions instead of in layers with different and somewhat arbitrary translation 
speeds.

#### Why not use a perspective camera?

Perspective cameras intrinsically render objects with depth effects. Far away objects appear smaller and appear to move
slower than close objects. The problem with using a 3D camera to render 2D sprites is that sprites appear skewed
due to perspective distortion. 

## Implementation using a Vertex Shader 

This approach uses both perspective and orthographic cameras to create the parallax scrolling effect without the use
of layers. We take advantage of the intrinsic 3D nature of the  perspective camera to compute the location and 
size of the sprite in the final scene however we render the sprite using an orthographic camera to prevent perspective
distortion. 

#### Calculating a Compatible Orthographic Projection Matrix

In this technique both perspective and orthographic cameras are used. In order for this technique to work the
the orthographic and perspective cameras must be compatible. The maximum height and width of the
perspective camera frustrum is should be equal to objective camera frustrum.

```
pub fn camera_matrices(fov_y: f32, aspect_ratio, far: f32, near: f32) -> (Mat4, Mat4) {
    let h = (0.5 * fov_y).tan() * far;
    let w = h * aspect_ratio;

    let orthographic = Mat4::orthographic_lh(-w, w, -h, h, self.far, self.near);
    let perspective = Mat4::perspective_lh(self.fov_y, self.aspect_ratio, self.near, self.far);

    (orthographic, perspective)
}
```


#### Calculating a Compatible Orthographic Projection Matrix

We calculate the location of the sprite centre in the perspective clip space. The operations to do this are
the same as if you were calculating the position of a vertex in perspective clip space in the standard rendering process.
The only difference is we are applying the model and camera transforms to the centre of the quad the vertex
belongs to rather the vertex itself. The `perspectiveCentre` gives us the location of the sprite in final image.

```
vec4 perspectiveCentre = perspectiveCameraMatrix * modelMatrix * centre;
```
 
Now we use our orthographic projection matrix to calculate how far the vertex is offset from its centre if
it were rendering using an orthographic camera. We need to multiply the offset by the far bounds of the
camera frustrum to 

```
    vec4 orthographicCentre = ortho * perspectiveOrthographicMatrix * centre;
    vec4 orthographicVertex = ortho * perspectiveOrthographicMatrix * vertex;
    vec4 vertexOffset = frustrumFar * (orthographicVertex - orthopraphicCentre)
```

#### Parallax Effect

We apply this offset to the `vertexOffset` to the `perspectiveCentre`. By applying our `perspectiveCameraMatrix` to 
a single point, the `spriteCentre`, we avoid perspective distortion while achieving the parallax effect.
The orthographic projections of the vertices are reconstructed around the `perspectiveCentre` with pixel perfect
precision.

```
gl_Position =  perspectiveCentre + vertexOffset;
```

#### Shrinking far away objects

Another benefit of this technique is that sprites are automatically shrunk based on their distance without any further
input required from the artist or game designer. `perspectiveCentre.Z` remains intact as
`vertexOffet.z` is always equal to 0 as long as the sprite quads are parallel to the camera plane, a condition which
can be assumed for 2D rendering.


#### Final Shader Code

```
#version 450

layout(location=0) in vec4 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 centre;

layout(location=0) out vec2 v_tex_coords;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 ortho;
    mat4 persp;
    float frustrum_far;
};

layout(set=0, binding=1)
buffer Instances {
    mat4 s_models[];
};

void main() {
    v_tex_coords = a_tex_coords;

    vec4 persp_centre = persp * s_models[gl_InstanceIndex] * centre;

    vec4 ortho_centre = ortho * s_models[gl_InstanceIndex] * centre;
    vec4 ortho_pos = ortho * s_models[gl_InstanceIndex] * a_position;

    gl_Position =  persp_centre +  frustrum_far * (ortho_pos - ortho_centre);
```
