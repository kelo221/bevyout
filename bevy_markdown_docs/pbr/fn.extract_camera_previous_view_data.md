[bevy](../index.html)::[pbr](index.html)

# Function extract\_camera\_previous\_view\_data 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#649-652)

```rust
pub fn extract_camera_previous_view_data(
    commands: Commands<'_, '_>,
    cameras_3d: Extract<'_, '_, Query<'_, '_, (RenderEntity, &Camera, Option<&PreviousViewData>), With<Camera3d>>>,
)
```