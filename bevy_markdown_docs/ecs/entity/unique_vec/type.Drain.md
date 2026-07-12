[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_vec](index.html)

# Type Alias Drain 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#1143)

```rust
pub type Drain<'a, T = Entity> = UniqueEntityIter<Drain<'a, T>>;
```

A draining iterator for [`UniqueEntityEquivalentVec<T>`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

This struct is created by [`UniqueEntityEquivalentVec::drain`](../struct.UniqueEntityEquivalentVec.html#method.drain "method bevy::ecs::entity::UniqueEntityEquivalentVec::drain"). See its documentation for more.

## Aliased Type

```rust
pub struct Drain<'a, T = Entity> { /* private fields */ }
```