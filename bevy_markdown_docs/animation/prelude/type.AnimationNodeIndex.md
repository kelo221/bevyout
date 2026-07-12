[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Type Alias AnimationNodeIndex 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#161)

```rust
pub type AnimationNodeIndex = NodeIndex;
```

The index of either an animation or blend node in the animation graph.

These indices are the way that [animation players](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") identify each animation.

## Aliased Type

```rust
pub struct AnimationNodeIndex(/* private fields */);
```