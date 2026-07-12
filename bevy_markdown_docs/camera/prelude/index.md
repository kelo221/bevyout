[bevy](../../index.html)::[camera](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#33)

The camera prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Camera](struct.Camera.html "struct bevy::camera::prelude::Camera")

The defining [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") for camera entities, storing information about how and what to render through this camera.

[Camera2d](struct.Camera2d.html "struct bevy::camera::prelude::Camera2d")

A 2D camera component. Enables the 2D render graph for a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera").

[Camera3d](struct.Camera3d.html "struct bevy::camera::prelude::Camera3d")

A 3D camera component. Enables the main 3D render graph for a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera").

[ClearColor](struct.ClearColor.html "struct bevy::camera::prelude::ClearColor")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") that stores the default color that cameras use to clear the screen between frames.

[InheritedVisibility](struct.InheritedVisibility.html "struct bevy::camera::prelude::InheritedVisibility")

Whether or not an entity is visible in the hierarchy.

[OrthographicProjection](struct.OrthographicProjection.html "struct bevy::camera::prelude::OrthographicProjection")

Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`](../../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection"), the size of objects remains the same regardless of their distance to the camera.

[PerspectiveProjection](struct.PerspectiveProjection.html "struct bevy::camera::prelude::PerspectiveProjection")

A 3D camera projection in which distant objects appear smaller than close objects.

[ViewVisibility](struct.ViewVisibility.html "struct bevy::camera::prelude::ViewVisibility")

Algorithmically computed indication of whether an entity is visible and should be extracted for rendering.

## Enums

[ClearColorConfig](enum.ClearColorConfig.html "enum bevy::camera::prelude::ClearColorConfig")

For a camera, specifies the color used to clear the viewport [before rendering](../../prelude/struct.Camera.html#structfield.clear_color "field bevy::prelude::Camera::clear_color") or when [writing to the final render target texture](../../prelude/struct.Camera.html#structfield.output_mode "field bevy::prelude::Camera::output_mode").

[CompositingSpace](enum.CompositingSpace.html "enum bevy::camera::prelude::CompositingSpace")

Color space for alpha compositing. Affects how overlapping semi-transparent layers blend.

[MsaaWriteback](enum.MsaaWriteback.html "enum bevy::camera::prelude::MsaaWriteback")

Controls when MSAA writeback occurs for a camera.

[Projection](enum.Projection.html "enum bevy::camera::prelude::Projection")

Component that defines how to compute a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")’s projection matrix.

[Visibility](enum.Visibility.html "enum bevy::camera::prelude::Visibility")

User indication of whether an entity is visible. Propagates down the entity hierarchy.