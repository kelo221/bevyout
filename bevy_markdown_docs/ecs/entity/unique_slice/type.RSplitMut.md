[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RSplitMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1875)

```rust
pub type RSplitMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, RSplitMut<'a, T, P>>;
```

An iterator over the subslices of the vector which are separated by elements that match `pred`, starting from the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rsplit_mut`](../struct.UniqueEntityEquivalentSlice.html#method.rsplit_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rsplit_mut").

## Aliased Type

```rust
pub struct RSplitMut<'a, P, T = Entity> { /* private fields */ }
```