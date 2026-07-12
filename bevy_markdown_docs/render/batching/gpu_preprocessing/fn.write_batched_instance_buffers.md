[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function write\_batched\_instance\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#2331-2341)

```rust
pub fn write_batched_instance_buffers<GFBD>(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    gpu_array_buffer: ResMut<'_, BatchedInstanceBuffers<<GFBD as GetBatchData>::BufferData, <GFBD as GetFullBatchData>::BufferInputData>>,
    pipeline_cache: Res<'_, PipelineCache>,
    bin_unpacking_buffers: ResMut<'_, BinUnpackingBuffers>,
    sparse_buffer_update_jobs: ResMut<'_, SparseBufferUpdateJobs>,
    sparse_buffer_update_bind_groups: ResMut<'_, SparseBufferUpdateBindGroups>,
    sparse_buffer_update_pipelines: Res<'_, SparseBufferUpdatePipelines>,
)where
    GFBD: GetFullBatchData,
```

A system that writes all instance buffers to the GPU.