[bevy](../../index.html)::[mesh](../index.html)

# Module morph 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#13)

Available on **crate feature `morph`** only.

## Structs

[MorphAttributes](struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

Attributes **differences** used for morph targets.

[MorphWeights](struct.MorphWeights.html "struct bevy::mesh::morph::MorphWeights")

Controls the [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for all child [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities. In most cases, [`MorphWeights`](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights") should be considered the “source of truth” when writing [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for meshes. However you can choose to write child [`MeshMorphWeights`](enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") if your situation requires more granularity. Just note that if you set [`MorphWeights`](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights"), it will overwrite child [`MeshMorphWeights`](enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") values.

## Enums

[MeshMorphWeights](enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

A component that controls the [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) of a mesh. Must be assigned to an entity with a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") component.

[MorphBuildError](enum.MorphBuildError.html "enum bevy::mesh::morph::MorphBuildError")

## Constants

[MAX\_MORPH\_WEIGHTS](constant.MAX_MORPH_WEIGHTS.html "constant bevy::mesh::morph::MAX_MORPH_WEIGHTS")

Max target count available for [morph targets](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights").

[MAX\_TEXTURE\_WIDTH](constant.MAX_TEXTURE_WIDTH.html "constant bevy::mesh::morph::MAX_TEXTURE_WIDTH")

The maximum size of the morph target texture, if morph target textures are in use on the current platform.