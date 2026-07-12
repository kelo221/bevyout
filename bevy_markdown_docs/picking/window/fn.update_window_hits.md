[bevy](../../index.html)::[picking](../index.html)::[window](index.html)

# Function update\_window\_hits 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/window.rs.html#30-33)

```rust
pub fn update_window_hits(
    pointers: Query<'_, '_, (&PointerId, &PointerLocation)>,
    pointer_hits_writer: MessageWriter<'_, PointerHits>,
)
```

Generates pointer hit events for window entities.

A pointer is treated as hitting a window when it is located on that window. The order of the hit event is negative infinity, meaning it should appear behind all other entities.

The depth of the hit will be listed as zero.