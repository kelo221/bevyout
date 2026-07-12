[bevy](../index.html)::[pbr](index.html)

# Function early\_gpu\_preprocess 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#602-619)

```rust
pub fn early_gpu_preprocess(
    current_view: ViewQuery<'_, '_, Option<&ViewLightEntities>, Without<SkipGpuPreprocess>>,
    view_query: Query<'_, '_, (&ExtractedView, Option<&PreprocessBindGroups>, Option<&ViewUniformOffset>, Has<NoIndirectDrawing>, Has<OcclusionCulling>), Without<SkipGpuPreprocess>>,
    light_query: Query<'_, '_, &LightEntity>,
    batched_instance_buffers: Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    pipeline_cache: Res<'_, PipelineCache>,
    preprocess_pipelines: Res<'_, PreprocessPipelines>,
    ctx: RenderContext<'_, '_>,
)
```