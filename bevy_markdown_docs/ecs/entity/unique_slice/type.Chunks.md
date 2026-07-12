[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias Chunks 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1610)

```rust
pub type Chunks<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, Chunks<'a, T>>;
```

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::chunks`](../struct.UniqueEntityEquivalentSlice.html#method.chunks "method bevy::ecs::entity::UniqueEntityEquivalentSlice::chunks").

## Aliased Type

```rust
pub struct Chunks<'a, T = Entity> { /* private fields */ }
```