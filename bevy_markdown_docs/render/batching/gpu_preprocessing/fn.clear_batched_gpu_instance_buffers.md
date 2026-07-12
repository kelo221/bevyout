[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function clear\_batched\_gpu\_instance\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1521-1526)

```rust
pub fn clear_batched_gpu_instance_buffers<GFBD>(
    gpu_batched_instance_buffers: Option<ResMut<'_, BatchedInstanceBuffers<<GFBD as GetBatchData>::BufferData, <GFBD as GetFullBatchData>::BufferInputData>>>,
)where
    GFBD: GetFullBatchData,
```

A system that runs early in extraction and clears out all the [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") for the frame.

We have to run this during extraction because, if GPU preprocessing is in use, the extraction phase will write to the mesh input uniform buffers directly, so the buffers need to be cleared before then.