[bevy](../index.html)::[pbr](index.html)

# Function prepare\_prepass\_view\_bind\_group 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#736-745)

```rust
pub fn prepare_prepass_view_bind_group(
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    prepass_pipeline: Res<'_, PrepassPipeline>,
    view_uniforms: Res<'_, ViewUniforms>,
    globals_buffer: Res<'_, GlobalsBuffer>,
    previous_view_uniforms: Res<'_, PreviousViewUniforms>,
    visibility_ranges: Res<'_, RenderVisibilityRanges>,
    prepass_view_bind_group: ResMut<'_, PrepassViewBindGroup>,
)
```