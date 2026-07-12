[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function add\_pointer\_debug 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#150-153)

```rust
pub fn add_pointer_debug(
    commands: Commands<'_, '_>,
    pointers: Query<'_, '_, Entity, (With<PointerId>, Without<PointerDebug>)>,
)
```

Adds [`PointerDebug`](struct.PointerDebug.html "struct bevy::dev_tools::picking_debug::PointerDebug") to pointers automatically.