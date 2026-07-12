[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function disable\_entities\_on\_exit\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#409-413)

```rust
pub fn disable_entities_on_exit_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DisableOnExit<S>), Allow<Disabled>>,
)where
    S: States,
```

Disables entities marked with [`DisableOnExit<S>`](../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit") when their state no longer matches the world state.