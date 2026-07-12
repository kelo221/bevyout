[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module oit 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#15)

Order Independent Transparency (OIT) for 3d rendering. See [`OrderIndependentTransparencyPlugin`](struct.OrderIndependentTransparencyPlugin.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencyPlugin") for more details.

## Modules

[resolve](resolve/index.html "mod bevy::core_pipeline::oit::resolve")

Module that defines the necessary systems to resolve the OIT buffer and render it to the screen.

## Structs

[OitBuffers](struct.OitBuffers.html "struct bevy::core_pipeline::oit::OitBuffers")

Holds the buffers that contain the data of all OIT layers. We use one big buffer for the entire app. Each camera will reuse it so it will always be the size of the biggest OIT enabled camera.

[OitFragmentNode](struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode")

[OrderIndependentTransparencyPlugin](struct.OrderIndependentTransparencyPlugin.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencyPlugin")

A plugin that adds support for Order Independent Transparency (OIT). This can correctly render some scenes that would otherwise have artifacts due to alpha blending, but uses more memory.

[OrderIndependentTransparencySettings](struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

Used to identify which camera will use OIT to render transparent meshes and to configure OIT.

[OrderIndependentTransparencySettingsOffset](struct.OrderIndependentTransparencySettingsOffset.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettingsOffset")

## Functions

[init\_oit\_buffers](fn.init_oit_buffers.html "fn bevy::core_pipeline::oit::init_oit_buffers")

[prepare\_oit\_buffers](fn.prepare_oit_buffers.html "fn bevy::core_pipeline::oit::prepare_oit_buffers")

This creates or resizes the oit buffers for each camera. It will always create one big buffer that’s as big as the biggest buffer needed. Cameras with smaller viewports or less layers will simply use the big buffer and ignore the rest.