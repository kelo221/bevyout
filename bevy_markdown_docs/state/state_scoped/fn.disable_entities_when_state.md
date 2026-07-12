[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function disable\_entities\_when\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#331-335)

```rust
pub fn disable_entities_when_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DisableWhen<S>), Allow<Disabled>>,
)where
    S: States,
```

Disable entities marked with [`DisableWhen<S>`](../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen") when the state transition message matches their predicate.