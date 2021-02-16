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

## Layer-less Parallax Scrolling 

This approach uses both perspective and orthographic cameras to create the parallax scrolling effect without the use
of layers. We take advantage of the intrinsic 3D nature of the  perspective camera is used to compute the location of
the sprite in the final scene however we render the sprite using an orthographic camera to prevent perspective
distortion. 

First we determine the location of the sprite centre if it were rendered using an orthographic camera.

1. Calculate the `spriteCenter` in world coordinates the `modelToWorldMatrix`.
```
vec4 spriteCenter = modelToWorldMatrix * vec4(0.0);
```
2. Calculate the position of the sprite center on the orthographic camera projection plane. 

```
vec4 spriteCenterOrthographic = orthographicProjectionMatrix * vec4(0.0);
```

3. Find the perpendicular projection of `spriteCenterOrthographic` on the orthographic projection plane.
```
vec4 orthographicPlane = todo!(orthographicProjectionMatrix)
vec4 spriteCenterOrthographicPlane = perpProject(orthographicPlane, spriteCenterOrthographic);
```
   
Next we repeat these same steps to calculate sprite centre on the perspective projection plane if it were rendering
using a perspective camera. 

```
vec4 spriteCenter = modelToWorldMatrix * vec4(0.0)
vec4 spriteCenterPerspective = perspectiveProjectionMatrix * vec4(0.0)
vec4 perspectivePlane = todo!(perspectiveProjectionMatrix)
vec4 spriteCenterPerspectivePlane = perpProject(perspectivePlane, spriteCenterPerspective);
```

4. Calculate the translation vector between the orthographic projection and perspective perspective projection

```
vec4 translation = spriteCenterPerspectivePlane - spriteCenterOrthographicPlane;
```

4. Translate the Orthgraphic camera by orthographic-to-perspective translation vector

```
vec4 layerFreeParallaxScrollingProjectionMatrix = orthographicProjectionMatrix + translation;
```

5. The sprite rendered with the `translatedOrthographicCamera` 

#### Orthographic and Perspective Camera "Equivalency"

In this technique both perspective and orthographic cameras are used. In order for this technique to work the
the orthographic and perspective cameras must equivalent in the sense that the maximum height and width of the
perspective camera frustrum is equivalent to the objective camera frustrum.


#### Optional: Shrinking distant objects 

In the previous section we achieved the parallax scrolling effect by calculating the location of a the sprite if
it were to be rendered by a perspective camera and shifting it to that position to achieve a the paralax scrolling effect.

We avoided perspective distortion by rendering the sprite using an orthographic camera. 
Certain aspect of perspective may be desirable for example the shrinking of distant. 

5. Calculate the distance D of the sprite center in perspective camera space to the camera eye.