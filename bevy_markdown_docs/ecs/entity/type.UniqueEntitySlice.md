[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Type Alias UniqueEntitySlice 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#45)

```rust
pub type UniqueEntitySlice = UniqueEntityEquivalentSlice<Entity>;
```

A slice that contains only unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This is the default case of a [`UniqueEntityEquivalentSlice`](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice").

## Aliased Type

```rust
pub struct UniqueEntitySlice(/* private fields */);
```