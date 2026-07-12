[bevy](../../../index.html)::[pbr](../../index.html)::[experimental](../index.html)

# Module meshlet 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#22)

Available on **crate feature `meshlet`** only.

Render high-poly 3d meshes using an efficient GPU-driven method. See [`MeshletPlugin`](struct.MeshletPlugin.html "struct bevy::pbr::experimental::meshlet::MeshletPlugin") and [`MeshletMesh`](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") for details.

## Structs

[MeshletMesh](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

A mesh that has been pre-processed into multiple small clusters of triangles called meshlets.

[MeshletMesh3d](struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

The meshlet mesh equivalent of [`bevy_mesh::Mesh3d`](../../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d").

[MeshletMesh3dTemplate](struct.MeshletMesh3dTemplate.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3dTemplate")

[MeshletMeshLoader](struct.MeshletMeshLoader.html "struct bevy::pbr::experimental::meshlet::MeshletMeshLoader")

An [`AssetLoader`](../../../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") for `.meshlet_mesh` [`MeshletMesh`](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") assets.

[MeshletMeshSaver](struct.MeshletMeshSaver.html "struct bevy::pbr::experimental::meshlet::MeshletMeshSaver")

An [`AssetSaver`](../../../asset/saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") for `.meshlet_mesh` [`MeshletMesh`](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") assets.

[MeshletPlugin](struct.MeshletPlugin.html "struct bevy::pbr::experimental::meshlet::MeshletPlugin")

Provides a plugin for rendering large amounts of high-poly 3d meshes using an efficient GPU-driven method. See also [`MeshletMesh`](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh").

## Enums

[MeshToMeshletMeshConversionError](enum.MeshToMeshletMeshConversionError.html "enum bevy::pbr::experimental::meshlet::MeshToMeshletMeshConversionError")

An error produced by [`MeshletMesh::from_mesh`](struct.MeshletMesh.html#method.from_mesh "associated function bevy::pbr::experimental::meshlet::MeshletMesh::from_mesh").

## Constants

[MESHLET\_DEFAULT\_VERTEX\_POSITION\_QUANTIZATION\_FACTOR](constant.MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR.html "constant bevy::pbr::experimental::meshlet::MESHLET_DEFAULT_VERTEX_POSITION_QUANTIZATION_FACTOR")

Default vertex position quantization factor for use with [`MeshletMesh::from_mesh`](struct.MeshletMesh.html#method.from_mesh "associated function bevy::pbr::experimental::meshlet::MeshletMesh::from_mesh").

[MESHLET\_MESH\_ASSET\_VERSION](constant.MESHLET_MESH_ASSET_VERSION.html "constant bevy::pbr::experimental::meshlet::MESHLET_MESH_ASSET_VERSION")

The current version of the [`MeshletMesh`](struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") asset format.