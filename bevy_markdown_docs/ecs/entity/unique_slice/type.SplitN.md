[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias SplitN 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1684)

```rust
pub type SplitN<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, SplitN<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

This struct is created by [`UniqueEntityEquivalentSlice::splitn`](../struct.UniqueEntityEquivalentSlice.html#method.splitn "method bevy::ecs::entity::UniqueEntityEquivalentSlice::splitn").

## Aliased Type

```rust
pub struct SplitN<'a, P, T = Entity> { /* private fields */ }
```