[bevy](../index.html)::[pbr](index.html)

# Constant MAX\_JOINTS 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/skin.rs.html#28)

```rust
pub const MAX_JOINTS: usize = 256; // 256usize
```

Maximum number of joints supported for skinned meshes.

It is used to allocate buffers. The correctness of the value depends on the GPU/platform. The current value is chosen because it is guaranteed to work everywhere. To allow for bigger values, a check must be made for the limits of the GPU at runtime, which would mean not using consts anymore.