[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function clear\_bin\_unpacking\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#2624)

```rust
pub fn clear_bin_unpacking_buffers(
    bin_unpacking_buffers: ResMut<'_, BinUnpackingBuffers>,
)
```

Clears out the [`BinUnpackingBuffers`](struct.BinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers") in preparation for a new frame.