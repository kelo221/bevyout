[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Constant VISIBILITY\_RANGES\_STORAGE\_BUFFER\_COUNT 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/range.rs.html#34)

```rust
pub const VISIBILITY_RANGES_STORAGE_BUFFER_COUNT: u32 = 4; // 4u32
```

We need at least 4 storage buffer bindings available to enable the visibility range buffer.

Even though we only use one storage buffer, the first 3 available storage buffers will go to various light-related buffers. We will grab the fourth buffer slot.