[bevy](../index.html)::[sprite\_render](index.html)

# Function prepare\_mesh2d\_bind\_group 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#751-757)

```rust
pub fn prepare_mesh2d_bind_group(
    commands: Commands<'_, '_>,
    mesh2d_pipeline: Res<'_, Mesh2dPipeline>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    mesh2d_uniforms: Res<'_, BatchedInstanceBuffer<Mesh2dUniform>>,
)
```