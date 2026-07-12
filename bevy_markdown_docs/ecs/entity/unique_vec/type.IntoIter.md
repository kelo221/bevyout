[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_vec](index.html)

# Type Alias IntoIter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#1115)

```rust
pub type IntoIter<T = Entity> = UniqueEntityIter<IntoIter<T>>;
```

An iterator that moves out of a vector.

This `struct` is created by the [`IntoIterator::into_iter`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter "method core::iter::traits::collect::IntoIterator::into_iter") trait method on [`UniqueEntityEquivalentVec`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

## Aliased Type

```rust
pub struct IntoIter<T = Entity> { /* private fields */ }
```