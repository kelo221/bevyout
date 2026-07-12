[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias ChunksExact 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1616)

```rust
pub type ChunksExact<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, ChunksExact<'a, T>>;
```

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::chunks_exact`](../struct.UniqueEntityEquivalentSlice.html#method.chunks_exact "method bevy::ecs::entity::UniqueEntityEquivalentSlice::chunks_exact").

## Aliased Type

```rust
pub struct ChunksExact<'a, T = Entity> { /* private fields */ }
```