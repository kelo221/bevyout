[bevy](../../index.html)::[state](../index.html)::[state\_scoped](index.html)

# Function despawn\_entities\_when\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#86-90)

```rust
pub fn despawn_entities_when_state<S>(
    commands: Commands<'_, '_>,
    transitions: MessageReader<'_, '_, StateTransitionEvent<S>>,
    query: Query<'_, '_, (Entity, &DespawnWhen<S>), Allow<Disabled>>,
)where
    S: States,
```

Despawns entities marked with [`DespawnWhen<S>`](../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen") when the state transition message matches their predicate.

If the entity has already been despawned no warning will be emitted.