[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function filtering\_system 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#942-947)

```rust
pub fn filtering_system(
    query: Query<'_, '_, (&GeneratorBindGroups, &RenderEnvironmentMap)>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipelines: Option<Res<'_, GeneratorPipelines>>,
    ctx: RenderContext<'_, '_>,
)
```