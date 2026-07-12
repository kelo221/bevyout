[bevy](../../../index.html)::[pbr](../../index.html)::[decal](../index.html)

# Module clustered 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/mod.rs.html#8)

Clustered decals, bounding regions that project textures onto surfaces.

A _clustered decal_ is a bounding box that projects a texture onto any surface within its bounds along the positive Z axis. In Bevy, clustered decals use the _clustered forward_ rendering technique.

Clustered decals are the highest-quality types of decals that Bevy supports, but they require bindless textures. This means that they presently can’t be used on WebGL 2 or WebGPU. Bevy’s clustered decals can be used with forward or deferred rendering and don’t require a prepass.

Each clustered decal may contain up to 4 textures. By default, the 4 textures correspond to the base color, a normal map, a metallic-roughness map, and an emissive map respectively. However, with a custom shader, you can use these 4 textures for whatever you wish. Additionally, you can use the built-in _tag_ field to store additional application-specific data; by reading the tag in the shader, you can modify the appearance of a clustered decal arbitrarily. See the documentation in `clustered.wgsl` for more information and the `clustered_decals` example for an example of use.

## Structs

[ClusteredDecalPlugin](struct.ClusteredDecalPlugin.html "struct bevy::pbr::decal::clustered::ClusteredDecalPlugin")

A plugin that adds support for clustered decals.

[DecalsBuffer](struct.DecalsBuffer.html "struct bevy::pbr::decal::clustered::DecalsBuffer")

A render-world resource that holds the buffer of [`ClusteredDecal`](../../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")s ready to upload to the GPU.

[RenderClusteredDecal](struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal")

The GPU data structure that stores information about each decal.

[RenderClusteredDecals](struct.RenderClusteredDecals.html "struct bevy::pbr::decal::clustered::RenderClusteredDecals")

Stores information about all the clustered decals in the scene.

## Functions

[clustered\_decals\_are\_usable](fn.clustered_decals_are_usable.html "fn bevy::pbr::decal::clustered::clustered_decals_are_usable")

Returns true if clustered decals are usable on the current platform or false otherwise.

[extract\_decals](fn.extract_decals.html "fn bevy::pbr::decal::clustered::extract_decals")

Extracts decals from the main world into the render world.