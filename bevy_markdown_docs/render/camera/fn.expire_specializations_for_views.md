[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function expire\_specializations\_for\_views 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1056-1059)

```rust
pub fn expire_specializations_for_views(
    views: Query<'_, '_, &ExtractedView>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)
```

A system that removes views that don’t exist any longer from [`DirtySpecializations`](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").