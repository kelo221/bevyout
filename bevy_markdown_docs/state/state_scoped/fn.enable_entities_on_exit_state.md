[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function enable\_entities\_on\_exit\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#656-660)

```rust
pub fn enable_entities_on_exit_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &EnableOnExit<S>), With<Disabled>>,
)where
    S: States,
```

Enables entities marked with [`EnableOnExit<S>`](../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit") when their state no longer matches the world state.