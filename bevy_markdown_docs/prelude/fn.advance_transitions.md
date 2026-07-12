[bevy](../index.html)::[prelude](index.html)

# Function advance\_transitions 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#111-114)

```rust
pub fn advance_transitions(
    query: Query<'_, '_, (&mut AnimationTransitions, &mut AnimationPlayer)>,
    time: Res<'_, Time>,
)
```

A system that alters the weight of currently-playing transitions based on the current time and decline amount.