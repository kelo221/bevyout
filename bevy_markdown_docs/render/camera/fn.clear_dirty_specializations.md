[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function clear\_dirty\_specializations 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1038)

```rust
pub fn clear_dirty_specializations(
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)
```

Clears out the [`DirtySpecializations`](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations") resource in preparation for a new frame.