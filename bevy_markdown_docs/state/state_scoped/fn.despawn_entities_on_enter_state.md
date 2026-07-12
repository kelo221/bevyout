[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function despawn\_entities\_on\_enter\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#242-246)

```rust
pub fn despawn_entities_on_enter_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DespawnOnEnter<S>), Allow<Disabled>>,
)where
    S: States,
```

Despawns entities marked with [`DespawnOnEnter<S>`](../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter") when their state matches the world state.

If the entity has already been despawned no warning will be emitted.