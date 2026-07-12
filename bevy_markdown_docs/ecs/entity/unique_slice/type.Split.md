[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias Split 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1664)

```rust
pub type Split<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, Split<'a, T, P>>;
```

An iterator over subslices separated by elements that match a predicate function.

This struct is created by [`UniqueEntityEquivalentSlice::split`](../struct.UniqueEntityEquivalentSlice.html#method.split "method bevy::ecs::entity::UniqueEntityEquivalentSlice::split").

## Aliased Type

```rust
pub struct Split<'a, P, T = Entity> { /* private fields */ }
```