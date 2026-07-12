[bevy](../index.html)::[prelude](index.html)

# Function expire\_completed\_transitions 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#147-149)

```rust
pub fn expire_completed_transitions(
    query: Query<'_, '_, (&mut AnimationTransitions, &mut AnimationPlayer)>,
)
```

A system that removed transitions that have completed from the [`AnimationTransitions`](struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions") object.