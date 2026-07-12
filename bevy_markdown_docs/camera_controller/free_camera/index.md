[bevy](../../index.html)::[camera\_controller](../index.html)

# Module free\_camera 

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/lib.rs.html#40)

Available on **crate feature `free_camera`** only.

A camera controller that allows the user to move freely around the scene.

Free cameras are helpful for exploring large scenes, level editors and for debugging. They are rarely useful as-is for gameplay, as they allow the user to move freely in all directions, which can be disorienting, and they can clip through objects and terrain.

You may have heard of a “fly camera” — a type of free camera designed for fluid “flying” movement and quickly surveying large areas. By contrast, the default settings of this particular free camera are optimized for precise control.

To use this controller, add [`FreeCameraPlugin`](struct.FreeCameraPlugin.html "struct bevy::camera_controller::free_camera::FreeCameraPlugin") to your app, and attach the [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") component to your camera entity. The required [`FreeCameraState`](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState") component will be added automatically.

To configure the settings of this controller, modify the fields of the [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") component.

## Structs

[FreeCamera](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera")

Stores the settings for the [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") controller.

[FreeCameraPlugin](struct.FreeCameraPlugin.html "struct bevy::camera_controller::free_camera::FreeCameraPlugin")

A freecam-style camera controller plugin.

[FreeCameraState](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState")

Tracks the runtime state of a [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") controller.

## Enums

[VerticalMovementAxis](enum.VerticalMovementAxis.html "enum bevy::camera_controller::free_camera::VerticalMovementAxis")

Whether the vertical inputs translate the camera in world or local space axes.

## Functions

[rotate\_freecam\_to](fn.rotate_freecam_to.html "fn bevy::camera_controller::free_camera::rotate_freecam_to")

Smoothly changes orientation([`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")) of [`FreeCamera`](struct.FreeCamera.html "struct bevy::camera_controller::free_camera::FreeCamera") camera according to target orientation in [`FreeCameraState`](struct.FreeCameraState.html "struct bevy::camera_controller::free_camera::FreeCameraState").

[run\_freecamera\_controller](fn.run_freecamera_controller.html "fn bevy::camera_controller::free_camera::run_freecamera_controller")

Updates the camera’s position and orientation based on user input.