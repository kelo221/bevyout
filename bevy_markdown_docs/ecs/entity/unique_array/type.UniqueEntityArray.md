[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_array](index.html)

# Type Alias UniqueEntityArray 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_array.rs.html#41)

```rust
pub type UniqueEntityArray<const N: usize> = UniqueEntityEquivalentArray<Entity, N>;
```

An array that contains only unique [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This is the default case of a [`UniqueEntityEquivalentArray`](../struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray").

## Aliased Type

```rust
pub struct UniqueEntityArray<const N: usize>(/* private fields */);
```