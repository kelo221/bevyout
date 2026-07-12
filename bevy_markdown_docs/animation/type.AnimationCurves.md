[bevy](../index.html)::[animation](index.html)

# Type Alias AnimationCurves 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#161)

```rust
pub type AnimationCurves = HashMap<AnimationTargetId, Vec<VariableCurve>, NoOpHash>;
```

A mapping from [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") (e.g. bone in a skinned mesh) to the animation curves.

## Aliased Type

```rust
pub struct AnimationCurves(/* private fields */);
```