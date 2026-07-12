[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function enable\_entities\_on\_enter\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#734-738)

```rust
pub fn enable_entities_on_enter_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &EnableOnEnter<S>), With<Disabled>>,
)where
    S: States,
```

Enables entities marked with [`EnableOnEnter<S>`](../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter") when their state matches the world state.