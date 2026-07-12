[bevy](../../../index.html)::[render](../../index.html)::[mesh](../index.html)

# Module morph 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#3)

Available on **crate feature `morph`** only.

## Structs

[MorphTargetImage](struct.MorphTargetImage.html "struct bevy::render::mesh::morph::MorphTargetImage")

An image formatted for use with [`bevy_mesh::morph::MorphWeights`](../../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights") for rendering the morph target, containing the vertex displacements.

## Enums

[MorphTargetsResource](enum.MorphTargetsResource.html "enum bevy::render::mesh::morph::MorphTargetsResource")

A reference to the resource in which morph displacements for a mesh are stored.

[RenderMorphTargetAllocator](enum.RenderMorphTargetAllocator.html "enum bevy::render::mesh::morph::RenderMorphTargetAllocator")

Stores the images for all morph target displacement data, if the current platform doesn’t support storage buffers.