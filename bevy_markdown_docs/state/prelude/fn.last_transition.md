[bevy](../../index.html)::[state](../index.html)::[prelude](index.html)

# Function last\_transition 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#237-239)

```rust
pub fn last_transition<S>(
    reader: MessageReader<'_, '_, StateTransitionEvent<S>>,
) -> Option<StateTransitionEvent<S>>where
    S: States,
```

Returns the latest state transition event of type `S`, if any are available.