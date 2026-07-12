[bevy](../index.html)::[math](index.html)

# Function reflection\_matrix 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/mat3.rs.html#21)

```rust
pub fn reflection_matrix(plane_normal: Vec3) -> Mat3A
```

Creates a 3×3 matrix that reflects points across the plane at the origin with the given normal.

This is also known as a [Householder matrix](https://en.wikipedia.org/wiki/Householder_transformation). It has the general form I - 2NNᵀ, where N is the normal of the plane and I is the identity matrix.

If the plane across which points are to be reflected isn’t at the origin, you can create a translation matrix that translates the points to the origin, then apply the matrix that this function returns on top of that, and finally translate back to the original position.

See the `mirror` example for a demonstration of how you might use this function.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 355](../../src/mirror/mirror.rs.html#355))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```