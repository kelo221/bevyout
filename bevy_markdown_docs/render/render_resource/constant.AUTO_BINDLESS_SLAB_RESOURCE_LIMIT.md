[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Constant AUTO\_BINDLESS\_SLAB\_RESOURCE\_LIMIT 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bindless.rs.html#32)

```rust
pub const AUTO_BINDLESS_SLAB_RESOURCE_LIMIT: u32 = 2048; // 2_048u32
```

Available on **neither iOS nor macOS**.

The default value for the number of resources that can be stored in a slab on this platform.

See the documentation for [`BindlessSlabResourceLimit`](enum.BindlessSlabResourceLimit.html "enum bevy::render::render_resource::BindlessSlabResourceLimit") for more information.