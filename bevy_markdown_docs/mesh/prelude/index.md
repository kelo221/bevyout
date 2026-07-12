[bevy](../../index.html)::[mesh](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#33)

The mesh prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Mesh](struct.Mesh.html "struct bevy::mesh::prelude::Mesh")

A 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.

[Mesh2d](struct.Mesh2d.html "struct bevy::mesh::prelude::Mesh2d")

A component for 2D meshes. Requires a [`MeshMaterial2d`](https://docs.rs/bevy/latest/bevy/prelude/struct.MeshMaterial2d.html) to be rendered, commonly using a [`ColorMaterial`](https://docs.rs/bevy/latest/bevy/prelude/struct.ColorMaterial.html).

[Mesh3d](struct.Mesh3d.html "struct bevy::mesh::prelude::Mesh3d")

A component for 3D meshes. Requires a [`MeshMaterial3d`](https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html) to be rendered, commonly using a [`StandardMaterial`](https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html).

[MorphWeights](struct.MorphWeights.html "struct bevy::mesh::prelude::MorphWeights")

Controls the [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for all child [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities. In most cases, [`MorphWeights`](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights") should be considered the “source of truth” when writing [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for meshes. However you can choose to write child [`MeshMorphWeights`](../morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") if your situation requires more granularity. Just note that if you set [`MorphWeights`](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights"), it will overwrite child [`MeshMorphWeights`](../morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") values.

## Traits

[MeshBuilder](trait.MeshBuilder.html "trait bevy::mesh::prelude::MeshBuilder")

A trait used to build [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

[Meshable](trait.Meshable.html "trait bevy::mesh::prelude::Meshable")

A trait for shapes that can be turned into a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").