[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias RChunks 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1634)

```rust
pub type RChunks<'a, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, RChunks<'a, T>>;
```

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the end of the slice.

This struct is created by [`UniqueEntityEquivalentSlice::rchunks`](../struct.UniqueEntityEquivalentSlice.html#method.rchunks "method bevy::ecs::entity::UniqueEntityEquivalentSlice::rchunks").

## Aliased Type

```rust
pub struct RChunks<'a, T = Entity> { /* private fields */ }
```