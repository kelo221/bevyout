[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RSplitNMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1890)

```rust
pub type RSplitNMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, RSplitNMut<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rsplitn_mut`](../struct.UniqueEntityEquivalentSlice.html#method.rsplitn_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rsplitn_mut").

## Aliased Type

```rust
pub struct RSplitNMut<'a, P, T = Entity> { /* private fields */ }
```