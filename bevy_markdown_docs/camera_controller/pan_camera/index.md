[bevy](../../index.html)::[camera\_controller](../index.html)

# Module pan\_camera 

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/lib.rs.html#43)

Available on **crate feature `pan_camera`** only.

A controller for 2D cameras that supports panning, zooming, and rotation.

To use this controller, add [`PanCameraPlugin`](struct.PanCameraPlugin.html "struct bevy::camera_controller::pan_camera::PanCameraPlugin") to your app, and insert a [`PanCamera`](struct.PanCamera.html "struct bevy::camera_controller::pan_camera::PanCamera") component into your camera entity.

To configure the settings of this controller, modify the fields of the [`PanCamera`](struct.PanCamera.html "struct bevy::camera_controller::pan_camera::PanCamera") component.

## Structs

[MousePanSettings](struct.MousePanSettings.html "struct bevy::camera_controller::pan_camera::MousePanSettings")

Settings for mouse panning for the [`PanCamera`](struct.PanCamera.html "struct bevy::camera_controller::pan_camera::PanCamera") controller.

[PanCamera](struct.PanCamera.html "struct bevy::camera_controller::pan_camera::PanCamera")

Configuration and state for a 2D panning camera controller.

[PanCameraPlugin](struct.PanCameraPlugin.html "struct bevy::camera_controller::pan_camera::PanCameraPlugin")

A plugin that enables 2D camera panning and zooming controls.