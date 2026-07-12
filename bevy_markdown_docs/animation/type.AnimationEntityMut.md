[bevy](../index.html)::[animation](index.html)

# Type Alias AnimationEntityMut 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1069)

```rust
pub type AnimationEntityMut<'w, 's> = EntityMutExcept<'w, 's, (AnimationTargetId, AnimatedBy, AnimationPlayer, AnimationGraphHandle)>;
```

A type alias for [`EntityMutExcept`](../ecs/world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept") as used in animation.

## Aliased Type

```rust
pub struct AnimationEntityMut<'w, 's> { /* private fields */ }
```