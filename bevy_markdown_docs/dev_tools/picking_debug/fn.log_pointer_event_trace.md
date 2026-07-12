[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function log\_pointer\_event\_trace 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#141-143)

```rust
pub fn log_pointer_event_trace<E>(
    pointer_reader: MessageReader<'_, '_, Pointer<E>>,
)where
    E: Debug + Clone + Reflect,
```

Listens for pointer events of type `E` and logs them at “trace” level