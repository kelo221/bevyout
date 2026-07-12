[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RChunksMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1826)

```rust
pub type RChunksMut<'a, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, RChunksMut<'a, T>>;
```

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rchunks_mut`](../struct.UniqueEntityEquivalentSlice.html#method.rchunks_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rchunks_mut").

## Aliased Type

```rust
pub struct RChunksMut<'a, T = Entity> { /* private fields */ }
```