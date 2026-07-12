[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[oit](../../index.html)::[resolve](../index.html)::[node](index.html)

# Function oit\_resolve 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/node.rs.html#15-29)

```rust
pub fn oit_resolve(
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ViewTarget, &ViewUniformOffset, &OitResolvePipelineId, &ViewDepthTexture, Option<&MainPassResolutionOverride>, Has<DepthPrepass>)>,
    resolve_pipeline: Option<Res<'_, OitResolvePipeline>>,
    bind_group: Option<Res<'_, OitResolveBindGroup>>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```