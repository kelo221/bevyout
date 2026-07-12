[bevy](../index.html)::[camera](index.html)

# Trait CameraProjection 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#43)

```rust
pub trait CameraProjection {
    // Required methods
    fn get_clip_from_view(&self) -> Mat4;
    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4;
    fn update(&mut self, width: f32, height: f32);
    fn far(&self) -> f32;
    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8];

    // Provided method
    fn compute_frustum(&self, camera_transform: &GlobalTransform) -> Frustum { ... }
}
```

Describes a type that can generate a projection matrix, allowing it to be added to a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera")’s [`Projection`](../prelude/enum.Projection.html "enum bevy::prelude::Projection") component.

Once implemented, the projection can be added to a camera using [`Projection::custom`](../prelude/enum.Projection.html#method.custom "associated function bevy::prelude::Projection::custom").

The projection will be automatically updated as the render area is resized. This is useful when, for example, a projection type has a field like `fov` that should change when the window width is changed but not when the height changes.

This trait is implemented by bevy’s built-in projections [`PerspectiveProjection`](../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection") and [`OrthographicProjection`](../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection").

## Required Methods

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#45)

#### fn [get\_clip\_from\_view](#tymethod.get_clip_from_view)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Generate the projection matrix.

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#48)

#### fn [get\_clip\_from\_view\_for\_sub](#tymethod.get_clip_from_view_for_sub)(&self, sub\_view: &[SubCameraView](struct.SubCameraView.html "struct bevy::camera::SubCameraView")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Generate the projection matrix for a [`SubCameraView`](struct.SubCameraView.html "struct bevy::camera::SubCameraView").

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#53)

#### fn [update](#tymethod.update)(&mut self, width: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), height: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

When the area this camera renders to changes dimensions, this method will be automatically called. Use this to update any projection properties that depend on the aspect ratio or dimensions of the render area.

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#56)

#### fn [far](#tymethod.far)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The far plane distance of the projection.

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#64)

#### fn [get\_frustum\_corners](#tymethod.get_frustum_corners)(&self, z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> \[[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A"); [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The eight corners of the camera frustum, as defined by this projection.

The corners should be provided in the following order: first the bottom right, top right, top left, bottom left for the near plane, then similar for the far plane.

## Provided Methods

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#70)

#### fn [compute\_frustum](#method.compute_frustum)(&self, camera\_transform: &[GlobalTransform](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")) -> [Frustum](primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

Compute camera frustum for camera with given projection and transform.

This code is called by [`update_frusta`](visibility/fn.update_frusta.html "fn bevy::camera::visibility::update_frusta") system for each camera to update its frustum.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#636)

### impl [CameraProjection](trait.CameraProjection.html "trait bevy::camera::CameraProjection") for [OrthographicProjection](../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#336)

### impl [CameraProjection](trait.CameraProjection.html "trait bevy::camera::CameraProjection") for [PerspectiveProjection](../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")