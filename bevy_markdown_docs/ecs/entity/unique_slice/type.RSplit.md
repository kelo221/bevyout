[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RSplit 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1677)

```rust
pub type RSplit<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, RSplit<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function, starting from the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rsplit`](../struct.UniqueEntityEquivalentSlice.html#method.rsplit "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rsplit").

## Aliased Type

```rust
pub struct RSplit<'a, P, T = Entity> { /* private fields */ }
```