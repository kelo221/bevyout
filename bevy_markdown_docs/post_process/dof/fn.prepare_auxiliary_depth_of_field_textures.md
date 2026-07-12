[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function prepare\_auxiliary\_depth\_of\_field\_textures 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#469-474)

```rust
pub fn prepare_auxiliary_depth_of_field_textures(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    texture_cache: ResMut<'_, TextureCache>,
    view_targets: Query<'_, '_, (Entity, &ViewTarget, &DepthOfField)>,
)
```

Creates the second render target texture that the first pass of the bokeh effect needs.