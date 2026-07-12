[bevy](../index.html)::[light](index.html)

# Function spot\_light\_clip\_from\_view 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#196)

```rust
pub fn spot_light_clip_from_view(angle: f32, near_z: f32) -> Mat4
```

Creates the projection matrix that transforms the light’s view space into the light’s clip space.