[bevy](../../index.html)::[camera\_controller](../index.html)::[free\_camera](index.html)

# Function rotate\_freecam\_to 

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/free_camera.rs.html#464-467)

```rust
pub fn rotate_freecam_to(
    query: Query<'_, '_, (&mut Transform, &mut FreeCameraState), With<Camera>>,
    time: Res<'_, Time<Real>>,
)
```

Available on **crate feature `free_camera`** only.

Smoothly changes orientation([`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")) of [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") camera according to target orientation in [`FreeCameraState`](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState").

*   [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") contains static configuration such as key bindings and rotation speed.
*   [`FreeCameraState`](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState") stores the dynamic runtime state, including direction for camera rotation and enable flags.

This system is typically added via the [`FreeCameraPlugin`](struct.FreeCameraPlugin.html "struct bevy::camera_controller::free_camera::FreeCameraPlugin").