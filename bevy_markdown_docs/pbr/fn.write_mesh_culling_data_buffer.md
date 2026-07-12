[bevy](../index.html)::[pbr](index.html)

# Function write\_mesh\_culling\_data\_buffer 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#2949-2957)

```rust
pub fn write_mesh_culling_data_buffer(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    mesh_culling_data_buffer: ResMut<'_, MeshCullingDataBuffer>,
    pipeline_cache: Res<'_, PipelineCache>,
    sparse_buffer_update_jobs: ResMut<'_, SparseBufferUpdateJobs>,
    sparse_buffer_update_bind_groups: ResMut<'_, SparseBufferUpdateBindGroups>,
    sparse_buffer_update_pipelines: Res<'_, SparseBufferUpdatePipelines>,
)
```

Writes the information needed to do GPU mesh culling to the GPU.