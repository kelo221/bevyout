[bevy](../index.html)::[pbr](index.html)

# Function init\_prepass\_view\_bind\_group 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#718-723)

```rust
pub fn init_prepass_view_bind_group(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipeline: Res<'_, PrepassPipeline>,
)
```