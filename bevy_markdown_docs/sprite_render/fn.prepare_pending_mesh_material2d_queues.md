[bevy](../index.html)::[sprite\_render](index.html)

# Function prepare\_pending\_mesh\_material2d\_queues 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#731-734)

```rust
pub fn prepare_pending_mesh_material2d_queues(
    pending_mesh_material2d_queues: ResMut<'_, PendingMeshMaterial2dQueues>,
    views: Query<'_, '_, &ExtractedView>,
)
```

Prepares the [`PendingMeshMaterial2dQueues`](struct.PendingMeshMaterial2dQueues.html "struct bevy::sprite_render::PendingMeshMaterial2dQueues") for a new frame by swapping the current and previous frame queues for each view.