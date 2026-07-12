[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function clear\_dirty\_wireframe\_specializations 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1046-1048)

```rust
pub fn clear_dirty_wireframe_specializations(
    dirty_wireframe_specializations: ResMut<'_, DirtyWireframeSpecializations>,
)
```

Clears out the [`DirtyWireframeSpecializations`](struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations") resource in preparation for a new frame.