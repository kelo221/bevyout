[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function log\_message\_debug 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#125)

```rust
pub fn log_message_debug<M>(events: MessageReader<'_, '_, PointerInput>)where
    M: Message + Debug,
```

Listen for any message and logs it at the debug level