[bevy](../index.html)

# Crate core\_pipeline 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#1-73)

## Modules

[blit](blit/index.html "mod bevy::core_pipeline::blit")

[core\_2d](core_2d/index.html "mod bevy::core_pipeline::core_2d")

[core\_3d](core_3d/index.html "mod bevy::core_pipeline::core_3d")

[deferred](deferred/index.html "mod bevy::core_pipeline::deferred")

[fullscreen\_material](fullscreen_material/index.html "mod bevy::core_pipeline::fullscreen_material")

This is mostly a pluginified version of the `custom_post_processing` example

[mip\_generation](mip_generation/index.html "mod bevy::core_pipeline::mip_generation")

Downsampling of textures to produce mipmap levels.

[oit](oit/index.html "mod bevy::core_pipeline::oit")

Order Independent Transparency (OIT) for 3d rendering. See [`OrderIndependentTransparencyPlugin`](oit/struct.OrderIndependentTransparencyPlugin.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencyPlugin") for more details.

[prepass](prepass/index.html "mod bevy::core_pipeline::prepass")

Run a prepass before the main pass to generate depth, normals, and/or motion vectors textures, sometimes called a thin g-buffer. These textures are useful for various screen-space effects and reducing overdraw in the main pass.

[schedule](schedule/index.html "mod bevy::core_pipeline::schedule")

The core rendering pipelines schedules. These schedules define the “default” render graph for 2D and 3D rendering in Bevy.

[skybox](skybox/index.html "mod bevy::core_pipeline::skybox")

[tonemapping](tonemapping/index.html "mod bevy::core_pipeline::tonemapping")

[upscaling](upscaling/index.html "mod bevy::core_pipeline::upscaling")

## Structs

[Core2d](struct.Core2d.html "struct bevy::core_pipeline::Core2d")

Schedule label for the Core 2D rendering pipeline.

[Core3d](struct.Core3d.html "struct bevy::core_pipeline::Core3d")

Schedule label for the Core 3D rendering pipeline.

[CorePipelinePlugin](struct.CorePipelinePlugin.html "struct bevy::core_pipeline::CorePipelinePlugin")

[FullscreenShader](struct.FullscreenShader.html "struct bevy::core_pipeline::FullscreenShader")

A shader that renders to the whole screen. Useful for post-processing.

[Skybox](struct.Skybox.html "struct bevy::core_pipeline::Skybox")

Adds a skybox to a 3D camera, based on a cubemap texture.

## Enums

[Core2dSystems](enum.Core2dSystems.html "enum bevy::core_pipeline::Core2dSystems")

System sets for the Core 2D rendering pipeline, defining the main stages of rendering. These stages include and run in the following order:

[Core3dSystems](enum.Core3dSystems.html "enum bevy::core_pipeline::Core3dSystems")

System sets for the Core 3D rendering pipeline, defining the main stages of rendering. These stages include and run in the following order: