[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[deferred](../index.html)::[copy\_lighting\_id](index.html)

# Function init\_copy\_deferred\_lighting\_id\_pipeline 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/copy_lighting_id.rs.html#100-105)

```rust
pub fn init_copy_deferred_lighting_id_pipeline(
    commands: Commands<'_, '_>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
    pipeline_cache: Res<'_, PipelineCache>,
)
```