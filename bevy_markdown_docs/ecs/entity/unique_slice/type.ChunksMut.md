[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias ChunksMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1799)

```rust
pub type ChunksMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, ChunksMut<'a, T>>;
```

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::chunks_mut`](../struct.UniqueEntityEquivalentSlice.html#method.chunks_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::chunks_mut").

## Aliased Type

```rust
pub struct ChunksMut<'a, T = Entity> { /* private fields */ }
```