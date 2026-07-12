[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias SparseBufferHandle 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#83)

```rust
pub type SparseBufferHandle = Arc<SparseBufferId>;
```

An object that allows the sparse buffer ID to be query and holds the bind group for that sparse buffer alive.

Each sparse buffer holds a strong reference to this handle, and the [`SparseBufferUpdateBindGroups`](struct.SparseBufferUpdateBindGroups.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroups") resource contains a weak map from this handle to the bind group. This setup ensures that, when the sparse buffer is freed, the bind groups for that sparse buffer are freed as well.

## Aliased Type

```rust
pub struct SparseBufferHandle { /* private fields */ }
```