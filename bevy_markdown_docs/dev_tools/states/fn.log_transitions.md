[bevy](../../index.html)::[dev\_tools](../index.html)::[states](index.html)

# Function log\_transitions 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/states.rs.html#10)

```rust
pub fn log_transitions<S>(
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
)where
    S: States,
```

Logs state transitions into console.

This system is provided to make debugging easier by tracking state changes.