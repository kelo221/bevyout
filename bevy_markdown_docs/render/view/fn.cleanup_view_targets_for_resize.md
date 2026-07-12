[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function cleanup\_view\_targets\_for\_resize 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#1177-1181)

```rust
pub fn cleanup_view_targets_for_resize(
    commands: Commands<'_, '_>,
    windows: Res<'_, ExtractedWindows>,
    cameras: Query<'_, '_, (Entity, &ExtractedCamera), With<ViewTarget>>,
)
```