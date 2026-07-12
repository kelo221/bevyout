[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function write\_indirect\_parameters\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#2647-2651)

```rust
pub fn write_indirect_parameters_buffers(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    indirect_parameters_buffers: ResMut<'_, IndirectParametersBuffers>,
)
```