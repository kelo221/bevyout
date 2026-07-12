[bevy](../../index.html)::[anti\_alias](../index.html)::[taa](index.html)

# Function temporal\_anti\_alias 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#136-148)

```rust
pub fn temporal_anti_alias(
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ViewTarget, &TemporalAntiAliasHistoryTextures, &ViewPrepassTextures, &TemporalAntiAliasPipelineId, &Msaa)>,
    pipelines: Option<Res<'_, TaaPipeline>>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```