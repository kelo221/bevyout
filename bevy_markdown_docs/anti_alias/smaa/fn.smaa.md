[bevy](../../index.html)::[anti\_alias](../index.html)::[smaa](index.html)

# Function smaa 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#805-817)

```rust
pub fn smaa(
    view: ViewQuery<'_, '_, (&ViewTarget, &ViewSmaaPipelines, &SmaaInfoUniformOffset, &SmaaTextures, &SmaaBindGroups)>,
    smaa_pipelines: Res<'_, SmaaPipelines>,
    smaa_info_uniform_buffer: Res<'_, SmaaInfoUniformBuffer>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```