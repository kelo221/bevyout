[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias SplitMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1860)

```rust
pub type SplitMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, SplitMut<'a, T, P>>;
```

An iterator over the mutable subslices of the vector which are separated by elements that match `pred`.

This struct is created by [`UniqueEntityEquivalentSlice::split_mut`](../struct.UniqueEntityEquivalentSlice.html#method.split_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::split_mut").

## Aliased Type

```rust
pub struct SplitMut<'a, P, T = Entity> { /* private fields */ }
```