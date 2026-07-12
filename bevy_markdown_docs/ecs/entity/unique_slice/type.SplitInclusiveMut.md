[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias SplitInclusiveMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1868)

```rust
pub type SplitInclusiveMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, SplitInclusiveMut<'a, T, P>>;
```

An iterator over the mutable subslices of the vector which are separated by elements that match `pred`. Unlike `SplitMut`, it contains the matched parts in the ends of the subslices.

This struct is created by [`UniqueEntityEquivalentSlice::split_inclusive_mut`](../struct.UniqueEntityEquivalentSlice.html#method.split_inclusive_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::split_inclusive_mut").

## Aliased Type

```rust
pub struct SplitInclusiveMut<'a, P, T = Entity> { /* private fields */ }
```