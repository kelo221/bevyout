[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias Windows 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1604)

```rust
pub type Windows<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, Windows<'a, T>>;
```

An iterator over overlapping subslices of length `size`.

This struct is created by [`UniqueEntityEquivalentSlice::windows`](../struct.UniqueEntityEquivalentSlice.html#method.windows "method bevy::ecs::entity::UniqueEntityEquivalentSlice::windows").

## Aliased Type

```rust
pub struct Windows<'a, T = Entity> { /* private fields */ }
```