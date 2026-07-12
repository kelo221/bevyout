[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias ChunkBy 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1657)

```rust
pub type ChunkBy<'a, P, T = Entity> = UniqueEntityEquivalentSliceIter<'a, T, ChunkBy<'a, T, P>>;
```

An iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by [`UniqueEntityEquivalentSlice::chunk_by`](../struct.UniqueEntityEquivalentSlice.html#method.chunk_by "method bevy::ecs::entity::UniqueEntityEquivalentSlice::chunk_by").

## Aliased Type

```rust
pub struct ChunkBy<'a, P, T = Entity> { /* private fields */ }
```