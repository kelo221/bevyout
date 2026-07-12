[bevy](../../index.html)::[picking](../index.html)::[mesh\_picking](index.html)

# Function update\_hits 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#77-86)

```rust
pub fn update_hits(
    backend_settings: Res<'_, MeshPickingSettings>,
    ray_map: Res<'_, RayMap>,
    picking_cameras: Query<'_, '_, (&Camera, Has<MeshPickingCamera>, Option<&RenderLayers>)>,
    pickables: Query<'_, '_, &Pickable>,
    marked_targets: Query<'_, '_, &Pickable>,
    layers: Query<'_, '_, &RenderLayers>,
    ray_cast: MeshRayCast<'_, '_>,
    pointer_hits_writer: MessageWriter<'_, PointerHits>,
)
```

Available on **crate feature `mesh_picking`** only.

Casts rays into the scene using [`MeshPickingSettings`](../../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings") and sends [`PointerHits`](../backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits") events.