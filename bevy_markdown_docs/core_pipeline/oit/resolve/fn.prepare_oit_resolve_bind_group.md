[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[oit](../index.html)::[resolve](index.html)

# Function prepare\_oit\_resolve\_bind\_group 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/mod.rs.html#246-253)

```rust
pub fn prepare_oit_resolve_bind_group(
    commands: Commands<'_, '_>,
    resolve_pipeline: Res<'_, OitResolvePipeline>,
    render_device: Res<'_, RenderDevice>,
    view_uniforms: Res<'_, ViewUniforms>,
    pipeline_cache: Res<'_, PipelineCache>,
    buffers: Res<'_, OitBuffers>,
)
```