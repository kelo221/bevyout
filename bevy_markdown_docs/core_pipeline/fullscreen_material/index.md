[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module fullscreen\_material 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#13)

This is mostly a pluginified version of the `custom_post_processing` example

The plugin will create a new system that runs a fullscreen triangle.

Users need to use the [`FullscreenMaterial`](trait.FullscreenMaterial.html "trait bevy::core_pipeline::fullscreen_material::FullscreenMaterial") trait to define the parameters like ordering.

## Structs

[FullscreenMaterialBindGroup](struct.FullscreenMaterialBindGroup.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialBindGroup")

Holds the bind groups for both main textures

[FullscreenMaterialPipeline](struct.FullscreenMaterialPipeline.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPipeline")

[FullscreenMaterialPipelineId](struct.FullscreenMaterialPipelineId.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPipelineId")

[FullscreenMaterialPlugin](struct.FullscreenMaterialPlugin.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin")

## Traits

[FullscreenMaterial](trait.FullscreenMaterial.html "trait bevy::core_pipeline::fullscreen_material::FullscreenMaterial")

A trait to define a material that will render to the entire screen using a fullscreen triangle.

## Functions

[fullscreen\_material\_system](fn.fullscreen_material_system.html "fn bevy::core_pipeline::fullscreen_material::fullscreen_material_system")