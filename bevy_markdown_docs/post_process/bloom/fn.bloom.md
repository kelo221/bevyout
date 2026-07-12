[bevy](../../index.html)::[post\_process](../index.html)::[bloom](index.html)

# Function bloom 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/mod.rs.html#88-103)

```rust
pub fn bloom(
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ViewTarget, &BloomTexture, &BloomBindGroups, &DynamicUniformIndex<BloomUniforms>, &Bloom, &UpsamplingPipelineIds, &BloomDownsamplingPipelineIds)>,
    downsampling_pipeline_res: Res<'_, BloomDownsamplingPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    uniforms: Res<'_, ComponentUniforms<BloomUniforms>>,
    ctx: RenderContext<'_, '_>,
)
```