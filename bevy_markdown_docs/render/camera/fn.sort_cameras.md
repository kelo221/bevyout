[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function sort\_cameras 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#726-729)

```rust
pub fn sort_cameras(
    sorted_cameras: ResMut<'_, SortedCameras>,
    cameras: Query<'_, '_, (Entity, &mut ExtractedCamera)>,
)
```