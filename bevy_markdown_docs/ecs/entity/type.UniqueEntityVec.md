[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Type Alias UniqueEntityVec 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#48)

```rust
pub type UniqueEntityVec = UniqueEntityEquivalentVec<Entity>;
```

A `Vec` that contains only unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This is the default case of a [`UniqueEntityEquivalentVec`](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

## Aliased Type

```rust
pub struct UniqueEntityVec(/* private fields */);
```