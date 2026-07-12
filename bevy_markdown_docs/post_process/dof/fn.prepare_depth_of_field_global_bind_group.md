[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function prepare\_depth\_of\_field\_global\_bind\_group 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#446-452)

```rust
pub fn prepare_depth_of_field_global_bind_group(
    global_bind_group_layout: Res<'_, DepthOfFieldGlobalBindGroupLayout>,
    dof_bind_group: ResMut<'_, DepthOfFieldGlobalBindGroup>,
    depth_of_field_uniforms: Res<'_, ComponentUniforms<DepthOfFieldUniform>>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
)
```

Creates depth of field bind group 1, which is shared among all instances of the depth of field shader.