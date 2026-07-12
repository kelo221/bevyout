[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function despawn\_entities\_on\_exit\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#164-168)

```rust
pub fn despawn_entities_on_exit_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DespawnOnExit<S>), Allow<Disabled>>,
)where
    S: States,
```

Despawns entities marked with [`DespawnOnExit<S>`](../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit") when their state no longer matches the world state.

If the entity has already been despawned no warning will be emitted.