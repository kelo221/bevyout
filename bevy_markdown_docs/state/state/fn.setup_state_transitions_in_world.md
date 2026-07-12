[bevy](../../index.html)::[state](../index.html)::[state](index.html)

# Function setup\_state\_transitions\_in\_world 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#217)

```rust
pub fn setup_state_transitions_in_world(world: &mut World)
```

Sets up the schedules and systems for handling state transitions within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

Runs automatically when using `App` to insert states, but needs to be added manually in other situations.