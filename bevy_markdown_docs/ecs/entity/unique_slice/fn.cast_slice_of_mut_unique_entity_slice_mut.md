[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Function cast\_slice\_of\_mut\_unique\_entity\_slice\_mut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#928-930)

```rust
pub unsafe fn cast_slice_of_mut_unique_entity_slice_mut<'a, 'b, T>(
    slice: &'b mut [&'a mut [T]],
) -> &'b mut [&'a mut UniqueEntityEquivalentSlice<T>]where
    T: EntityEquivalent + 'a,
```

Casts a mutable slice of mutable entity slices to a slice of mutable [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")s.

## Safety

All elements in each of the cast slices must be unique.