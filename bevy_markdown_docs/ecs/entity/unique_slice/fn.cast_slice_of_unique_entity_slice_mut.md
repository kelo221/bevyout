[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Function cast\_slice\_of\_unique\_entity\_slice\_mut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#916-918)

```rust
pub unsafe fn cast_slice_of_unique_entity_slice_mut<'a, 'b, T>(
    slice: &'b mut [&'a [T]],
) -> &'b mut [&'a UniqueEntityEquivalentSlice<T>]where
    T: EntityEquivalent + 'a,
```

Casts a mutable slice of entity slices to a slice of [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")s.

## Safety

All elements in each of the cast slices must be unique.