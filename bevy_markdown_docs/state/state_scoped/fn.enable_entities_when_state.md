[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function enable\_entities\_when\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#578-582)

```rust
pub fn enable_entities_when_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &EnableWhen<S>), With<Disabled>>,
)where
    S: States,
```

Enable entities marked with [`EnableWhen<S>`](../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen") when the state transition message matches their predicate.