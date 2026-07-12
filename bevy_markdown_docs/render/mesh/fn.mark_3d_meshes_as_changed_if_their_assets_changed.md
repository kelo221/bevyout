[bevy](../../index.html)::[render](../index.html)::[mesh](index.html)

# Function mark\_3d\_meshes\_as\_changed\_if\_their\_assets\_changed 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#131-134)

```rust
pub fn mark_3d_meshes_as_changed_if_their_assets_changed(
    meshes_3d: Query<'_, '_, &mut Mesh3d>,
    mesh_asset_events: MessageReader<'_, '_, AssetEvent<Mesh>>,
)
```

A system that marks a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") as changed if the associated [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") asset has changed.

This is needed because the systems that extract meshes, such as `extract_meshes_for_gpu_building`, write some metadata about the mesh (like the location within each slab) into the GPU structures that they build that needs to be kept up to date if the contents of the mesh change.