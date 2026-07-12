[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[no\_gpu\_preprocessing](index.html)

# Function clear\_batched\_cpu\_instance\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#64-67)

```rust
pub fn clear_batched_cpu_instance_buffers<GBD>(
    cpu_batched_instance_buffer: Option<ResMut<'_, BatchedInstanceBuffer<<GBD as GetBatchData>::BufferData>>>,
)where
    GBD: GetBatchData,
```

A system that clears out the [`BatchedInstanceBuffer`](struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer") for the frame.

This needs to run before the CPU batched instance buffers are used.