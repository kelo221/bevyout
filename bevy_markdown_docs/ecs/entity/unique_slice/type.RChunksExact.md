[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RChunksExact 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1640)

```rust
pub type RChunksExact<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, RChunksExact<'a, T>>;
```

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rchunks_exact`](../struct.UniqueEntityEquivalentSlice.html#method.rchunks_exact "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rchunks_exact").

## Aliased Type

```rust
pub struct RChunksExact<'a, T = Entity> { /* private fields */ }
```