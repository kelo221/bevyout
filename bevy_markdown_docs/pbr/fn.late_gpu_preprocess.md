[bevy](../index.html)::[pbr](index.html)

# Function late\_gpu\_preprocess 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#818-832)

```rust
pub fn late_gpu_preprocess(
    current_view: ViewQuery<'_, '_, (&ExtractedView, &PreprocessBindGroups, &ViewUniformOffset), (Without<SkipGpuPreprocess>, Without<NoIndirectDrawing>, With<OcclusionCulling>, With<DepthPrepass>)>,
    batched_instance_buffers: Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    pipeline_cache: Res<'_, PipelineCache>,
    preprocess_pipelines: Res<'_, PreprocessPipelines>,
    ctx: RenderContext<'_, '_>,
)
```