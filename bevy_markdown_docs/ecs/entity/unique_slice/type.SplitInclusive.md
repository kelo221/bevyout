[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias SplitInclusive 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1670)

```rust
pub type SplitInclusive<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, SplitInclusive<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function.

This struct is created by [`UniqueEntityEquivalentSlice::split_inclusive`](../struct.UniqueEntityEquivalentSlice.html#method.split_inclusive "method bevy::ecs::entity::UniqueEntityEquivalentSlice::split_inclusive").

## Aliased Type

```rust
pub struct SplitInclusive<'a, P, T = Entity> { /* private fields */ }
```