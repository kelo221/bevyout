[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function prepare\_depth\_of\_field\_view\_bind\_group\_layouts 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#374-377)

```rust
pub fn prepare_depth_of_field_view_bind_group_layouts(
    commands: Commands<'_, '_>,
    view_targets: Query<'_, '_, (Entity, &DepthOfField, &Msaa)>,
)
```

Creates the bind group layouts for the depth of field effect that are specific to each view.