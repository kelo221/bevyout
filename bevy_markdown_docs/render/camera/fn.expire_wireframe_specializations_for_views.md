[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function expire\_wireframe\_specializations\_for\_views 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1069-1072)

```rust
pub fn expire_wireframe_specializations_for_views(
    views: Query<'_, '_, &ExtractedView>,
    dirty_wireframe_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```

A system that removes views that don’t exist any longer from [`DirtyWireframeSpecializations`](struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").