[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function prepare\_generated\_environment\_map\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#559-570)

```rust
pub fn prepare_generated_environment_map_bind_groups(
    light_probes: Query<'_, '_, (Entity, &IntermediateTextures, &RenderEnvironmentMap)>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    queue: Res<'_, RenderQueue>,
    layouts: Res<'_, GeneratorBindGroupLayouts>,
    samplers: Res<'_, GeneratorSamplers>,
    render_images: Res<'_, RenderAssets<GpuImage>>,
    bluenoise: Res<'_, Bluenoise>,
    config: Res<'_, DownsamplingConfig>,
    commands: Commands<'_, '_>,
)
```

Prepares bind groups for environment map generation pipelines