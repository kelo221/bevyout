[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias SplitNMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1882)

```rust
pub type SplitNMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, SplitNMut<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

This struct is created by [`UniqueEntityEquivalentSlice::splitn_mut`](../struct.UniqueEntityEquivalentSlice.html#method.splitn_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::splitn_mut").

## Aliased Type

```rust
pub struct SplitNMut<'a, P, T = Entity> { /* private fields */ }
```