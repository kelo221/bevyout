[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias Iter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1476)

```rust
pub type Iter<'a, T> = UniqueEntityIter<Iter<'a, T>>;
```

Immutable slice iterator.

This struct is created by [`iter`](../struct.UniqueEntityEquivalentSlice.html#method.iter "method bevy::ecs::entity::UniqueEntityEquivalentSlice::iter") method on [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice") and the [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") impls on it and [`UniqueEntityEquivalentVec`](../struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec").

## Aliased Type

```rust
pub struct Iter<'a, T> { /* private fields */ }
```