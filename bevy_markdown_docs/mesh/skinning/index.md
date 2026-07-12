[bevy](../../index.html)::[mesh](../index.html)

# Module skinning 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#15)

## Structs

[Influence](struct.Influence.html "struct bevy::mesh::skinning::Influence")

A single vertex influence. Used by [`InfluenceIterator`](struct.InfluenceIterator.html "struct bevy::mesh::skinning::InfluenceIterator").

[InfluenceIterator](struct.InfluenceIterator.html "struct bevy::mesh::skinning::InfluenceIterator")

Iterator over all vertex influences with non-zero weight.

[JointAabb](struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[JointIndex](struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[SkinnedMesh](struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[SkinnedMeshBounds](struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

Data that can be used to calculate the AABB of a skinned mesh.

[SkinnedMeshInverseBindposes](struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes")

[SkinnedMeshTemplate](struct.SkinnedMeshTemplate.html "struct bevy::mesh::skinning::SkinnedMeshTemplate")

## Enums

[EntityAabbFromSkinnedMeshBoundsError](enum.EntityAabbFromSkinnedMeshBoundsError.html "enum bevy::mesh::skinning::EntityAabbFromSkinnedMeshBoundsError")

[MeshAttributeError](enum.MeshAttributeError.html "enum bevy::mesh::skinning::MeshAttributeError")

Generic error for when a mesh was expected to have a certain attribute with a certain format.

[SkinnedMeshBoundsError](enum.SkinnedMeshBoundsError.html "enum bevy::mesh::skinning::SkinnedMeshBoundsError")

## Functions

[entity\_aabb\_from\_skinned\_mesh\_bounds](fn.entity_aabb_from_skinned_mesh_bounds.html "fn bevy::mesh::skinning::entity_aabb_from_skinned_mesh_bounds")

Given the components of a skinned mesh entity, return an `Aabb3d` that encloses the skinned vertices of the mesh.