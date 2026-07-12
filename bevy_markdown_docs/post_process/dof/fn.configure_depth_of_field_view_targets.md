[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function configure\_depth\_of\_field\_view\_targets 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#434-436)

```rust
pub fn configure_depth_of_field_view_targets(
    view_targets: Query<'_, '_, &mut Camera3d, With<DepthOfField>>,
)
```

Configures depth textures so that the depth of field shader can read from them.

By default, the depth buffers that Bevy creates aren’t able to be bound as textures. The depth of field shader, however, needs to read from them. So we need to set the appropriate flag to tell Bevy to make samplable depth buffers.