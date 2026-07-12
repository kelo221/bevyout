[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function initialize\_generated\_environment\_map\_resources 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#165-172)

```rust
pub fn initialize_generated_environment_map_resources(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_adapter: Res<'_, RenderAdapter>,
    pipeline_cache: Res<'_, PipelineCache>,
    asset_server: Res<'_, AssetServer>,
    downsample_shaders: Res<'_, DownsampleShaders>,
)
```

Initializes all render-world resources used by the environment-map generator once on [`bevy_render::RenderStartup`](../../render/struct.RenderStartup.html "struct bevy::render::RenderStartup").