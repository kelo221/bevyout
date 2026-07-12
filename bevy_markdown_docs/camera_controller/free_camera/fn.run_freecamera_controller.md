[bevy](../../index.html)::[camera\_controller](../index.html)::[free\_camera](index.html)

# Function run\_freecamera\_controller 

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/free_camera.rs.html#265-276)

```rust
pub fn run_freecamera_controller(
    time: Res<'_, Time<Real>>,
    windows: Query<'_, '_, (&Window, &mut CursorOptions)>,
    accumulated_mouse_motion: Res<'_, AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<'_, AccumulatedMouseScroll>,
    touch_input: Res<'_, Touches>,
    mouse_button_input: Res<'_, ButtonInput<MouseButton>>,
    key_input: Res<'_, ButtonInput<KeyCode>>,
    toggle_cursor_grab: Local<'_, bool>,
    mouse_cursor_grab: Local<'_, bool>,
    query: Query<'_, '_, (&mut Transform, &mut FreeCameraState, &FreeCamera), With<Camera>>,
)
```

Available on **crate feature `free_camera`** only.

Updates the camera’s position and orientation based on user input.

*   [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") contains static configuration such as key bindings, movement speed, and sensitivity.
*   [`FreeCameraState`](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState") stores the dynamic runtime state, including pitch, yaw, velocity, and enable flags.

This system is typically added via the [`FreeCameraPlugin`](struct.FreeCameraPlugin.html "struct bevy::camera_controller::free_camera::FreeCameraPlugin").

Axis snapping takes priority over mouse movement.