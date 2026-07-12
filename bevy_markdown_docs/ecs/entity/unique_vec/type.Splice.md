[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_vec](index.html)

# Type Alias Splice 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#1159)

```rust
pub type Splice<'a, I> = UniqueEntityIter<Splice<'a, I>>;
```

A splicing iterator for [`UniqueEntityEquivalentVec`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

This struct is created by [`UniqueEntityEquivalentVec::splice`](../struct.UniqueEntityEquivalentVec.html#method.splice "method bevy::ecs::entity::UniqueEntityEquivalentVec::splice"). See its documentation for more.

## Aliased Type

```rust
pub struct Splice<'a, I> { /* private fields */ }
```