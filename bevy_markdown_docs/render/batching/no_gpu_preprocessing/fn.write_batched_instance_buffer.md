[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[no\_gpu\_preprocessing](index.html)

# Function write\_batched\_instance\_buffer 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#185-190)

```rust
pub fn write_batched_instance_buffer<GBD>(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    cpu_batched_instance_buffer: ResMut<'_, BatchedInstanceBuffer<<GBD as GetBatchData>::BufferData>>,
)where
    GBD: GetBatchData,
```

Writes the instance buffer data to the GPU.