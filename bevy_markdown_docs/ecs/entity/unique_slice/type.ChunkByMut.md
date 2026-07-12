[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Type Alias ChunkByMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1853)

```rust
pub type ChunkByMut<'a, P, T = Entity> = UniqueEntityEquivalentSliceIterMut<'a, T, ChunkByMut<'a, T, P>>;
```

An iterator over slice in (non-overlapping) mutable chunks separated by a predicate.

This struct is created by [`UniqueEntityEquivalentSlice::chunk_by_mut`](../struct.UniqueEntityEquivalentSlice.html#method.chunk_by_mut "method bevy::ecs::entity::UniqueEntityEquivalentSlice::chunk_by_mut").

## Aliased Type

```rust
pub struct ChunkByMut<'a, P, T = Entity> { /* private fields */ }
```