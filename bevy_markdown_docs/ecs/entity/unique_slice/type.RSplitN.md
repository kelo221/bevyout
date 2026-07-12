[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RSplitN 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1692)

```rust
pub type RSplitN<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, RSplitN<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rsplitn`](../struct.UniqueEntityEquivalentSlice.html#method.rsplitn "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rsplitn").

## Aliased Type

```rust
pub struct RSplitN<'a, P, T = Entity> { /* private fields */ }
```