[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function disable\_entities\_on\_enter\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#487-491)

```rust
pub fn disable_entities_on_enter_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DisableOnEnter<S>), Allow<Disabled>>,
)where
    S: States,
```

Disables entities marked with [`DisableOnEnter<S>`](../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter") when their state matches the world state.