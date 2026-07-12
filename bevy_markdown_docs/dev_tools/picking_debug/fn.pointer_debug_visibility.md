[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function pointer\_debug\_visibility 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#160-163)

```rust
pub fn pointer_debug_visibility(
    debug: Res<'_, DebugPickingMode>,
    pointers: Query<'_, '_, &mut Visibility, With<PointerId>>,
)
```

Hide text from pointers.