# Ray Tracing

## 1. What is Ray Tracing?

Ray tracing is a rendering technique in computer graphics that simulates realistic light behavior and its interaction with objects in a scene. It traces rays of light from the light source, calculating intersections with objects in the scene and depending on the position of our camera (point of view) produce realistic images.

## 2. JSON Structure for the Scene

3D point will be used frequently all along this project. It's define as an object with the coodinates:

```
{ "e": [x, y, z] }
```

The JSON file, [`scene.json`](./scene.json), defines the essential components of a ray-tracing scene:

### a) Camera

Specifies the position and orientation of the camera (POV).

These are the parameters of the camera :
- **origin**: A 3D point defining the camera's position in space.
- **look_at**: A 3D point representing the target point the camera is aimed at.

``` json
    "camera": {
        "origin": { "e": [3.0, 2.2, -4.0] },
        "look_at": { "e": [2.0, 0.0, 0.0] }
    }
```

### b) Light

Defines the properties of the light source.

- **position**: A 3D point for the light's position.
- **color**: A Vec3 for the light's color (e.g., white = [1.0, 1.0, 1.0]).
- **intensity**: The intensity of the light source.

``` json
    "light": {
        "position": { "e": [2.0, 6.0, -4.0] },
        "color": { "e": [1.0, 1.0, 1.0] },
        "intensity": 1.0
    }
```

The `color` field represents normalized **RGB** values, where each component (`R`, `G`, `B`) is divided by 255. For example, pure red (`255, 0, 0`) is represented as `[1.0, 0.0, 0.0]`.

### c) Spheres

Defines spheres in the scene. Each sphere is represented by:

- **center**: A 3D point specifying the sphere's center.
- **radius**: The radius of the sphere.
- **color**: The sphere's color as a Vec3 (normalized RGB).

```json
    "spheres": [
        {
            "center": { "e": [4.0, 1.0, 0.0] },
            "radius": 0.5,
            "color": { "e": [1.0, 1.0, 1.0] }
        },
        {
            "center": { "e": [-1.0, 0.0, -1.0] },
            "radius": 0.1,
            "color": { "e": [1.0, 0.0, 0.0] }
        },
        {
            "center": { "e": [0.0, 1.0, 0.0] },
            "radius": 0.1,
            "color": { "e": [0.0, 0.0, 1.0] }
        }
    ]
```

### d) Planes

Defines planes in the scene. Each plane includes:

- **origine**: A 3D point representing the origin of the plane.
- **width**: The width of the plane.
- **height**: The height of the plane.
- **color**: The plane's color as a Vec3 (normalized RGB).

``` json
    "planes": [
        {
            "origine": { "e": [-3.0, -1.0, -2.0] },
            "width": 10,
            "height": 10,
            "color": { "e": [0.5, 0.35, 0.34] }
        }
    ]
```

### e) Cubes

Defines cubes in the scene. Each cube contains:

- **min**: A 3D point for the cube's minimum corner.
- **max**: A 3D point for the cube's maximum corner.
- **color**: The cube's color as a Vec3 (normalized RGB).

``` json
    "cubes": [
        {
            "min": { "e": [-1.0, 0.0, -1.0] },
            "max": { "e": [0.0, 1.0, 0.0] },
            "color": { "e": [1.0, 1.0, 1.0] }
        }
    ]
```

### f) Cylinders

Defines cylinders in the scene. Each cylinder includes:

- **base**: A 3D point specifying the base position of the cylinder.
- **height**: The height of the cylinder.
- **radius**: The radius of the cylinder.
- **color**: The cylinder's color as a Vec3 (normalized RGB).

``` json
    "cylinders": [
        {
            "base": { "e": [2.0, 1.0, -1.0] },
            "height": 0.7,
            "radius": 0.5,
            "color": { "e": [1.0, 1.0, 1.0] }
        }
    ]
```