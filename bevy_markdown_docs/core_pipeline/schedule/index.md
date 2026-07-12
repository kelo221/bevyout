[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module schedule 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#17)

The core rendering pipelines schedules. These schedules define the “default” render graph for 2D and 3D rendering in Bevy.

Rendering in Bevy is “camera driven”, meaning that for each camera in the world, its associated rendering schedule is executed. This allows different cameras to have different rendering pipelines, for example a 3D camera with post-processing effects and a 2D camera with a simple clear and sprite rendering.

The [`camera_driver`](fn.camera_driver.html "fn bevy::core_pipeline::schedule::camera_driver") system is responsible for iterating over all cameras in the world and executing their associated schedules. In this way, the schedule for each camera is a sub-schedule or sub-graph of the root render graph schedule.

## Structs

[Core2d](struct.Core2d.html "struct bevy::core_pipeline::schedule::Core2d")

Schedule label for the Core 2D rendering pipeline.

[Core3d](struct.Core3d.html "struct bevy::core_pipeline::schedule::Core3d")

Schedule label for the Core 3D rendering pipeline.

[RootNonCameraView](struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

A render-world marker component for a view that corresponds to neither a camera nor a camera-associated shadow map.

## Enums

[Core2dSystems](enum.Core2dSystems.html "enum bevy::core_pipeline::schedule::Core2dSystems")

System sets for the Core 2D rendering pipeline, defining the main stages of rendering. These stages include and run in the following order:

[Core3dSystems](enum.Core3dSystems.html "enum bevy::core_pipeline::schedule::Core3dSystems")

System sets for the Core 3D rendering pipeline, defining the main stages of rendering. These stages include and run in the following order:

## Functions

[camera\_driver](fn.camera_driver.html "fn bevy::core_pipeline::schedule::camera_driver")

The default entry point for camera driven rendering added to the root [`bevy_render::renderer::RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule. This system iterates over all cameras in the world, executing their associated rendering schedules defined by the [`bevy_render::camera::CameraRenderGraph`](../../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph") component.