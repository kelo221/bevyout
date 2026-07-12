[bevy](../index.html)::[pbr](index.html)

# Function unpack\_bins 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#526-535)

```rust
pub fn unpack_bins(
    current_view: ViewQuery<'_, '_, Option<&ViewLightEntities>, Without<SkipGpuPreprocess>>,
    view_query: Query<'_, '_, &ExtractedView, Without<SkipGpuPreprocess>>,
    light_query: Query<'_, '_, &LightEntity>,
    batched_instance_buffers: Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    pipeline_cache: Res<'_, PipelineCache>,
    preprocess_pipelines: Res<'_, PreprocessPipelines>,
    bin_unpacking_bind_groups: Res<'_, BinUnpackingBindGroups>,
    render_context: RenderContext<'_, '_>,
)
```

A rendering system that invokes a compute shader for each batch set in order to generate preprocessing jobs for the subsequent mesh preprocessing shader.

This shader exists because performing the unpack operation on the CPU is slow when there are many entities. By caching the bins on the GPU from frame to frame, we avoid having to perform a CPU-side traversal of every mesh instance every frame.