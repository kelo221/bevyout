[bevy](../index.html)::[pbr](index.html)

# Function late\_sweep\_material\_instances 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#739-742)

```rust
pub fn late_sweep_material_instances(
    material_instances: ResMut<'_, RenderMaterialInstances>,
    removed_meshes_query: Extract<'_, '_, RemovedComponents<'_, '_, Mesh3d>>,
)
```

Removes mesh materials from [`RenderMaterialInstances`](struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances") when their [`ViewVisibility`](../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility") components are removed.

This runs after all invocations of `early_sweep_material_instances` and is responsible for bumping [`RenderMaterialInstances::current_change_tick`](struct.RenderMaterialInstances.html#structfield.current_change_tick "field bevy::pbr::RenderMaterialInstances::current_change_tick") in preparation for a new frame.