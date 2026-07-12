[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)

# Module unique\_slice 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#111)

A wrapper around entity slices with a uniqueness invariant.

## Structs

[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSlice")

A slice that contains only unique entities.

[UniqueEntityEquivalentSliceIter](struct.UniqueEntityEquivalentSliceIter.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIter")

An iterator that yields `&UniqueEntityEquivalentSlice`. Note that an entity may appear in multiple slices, depending on the wrapped iterator.

[UniqueEntityEquivalentSliceIterMut](struct.UniqueEntityEquivalentSliceIterMut.html "struct bevy::ecs::entity::unique_slice::UniqueEntityEquivalentSliceIterMut")

An iterator that yields `&mut UniqueEntityEquivalentSlice`. Note that an entity may appear in multiple slices, depending on the wrapped iterator.

## Functions

[cast\_slice\_of\_mut\_unique\_entity\_slice\_mut](fn.cast_slice_of_mut_unique_entity_slice_mut.html "fn bevy::ecs::entity::unique_slice::cast_slice_of_mut_unique_entity_slice_mut")⚠

Casts a mutable slice of mutable entity slices to a slice of mutable [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")s.

[cast\_slice\_of\_unique\_entity\_slice](fn.cast_slice_of_unique_entity_slice.html "fn bevy::ecs::entity::unique_slice::cast_slice_of_unique_entity_slice")⚠

Casts a slice of entity slices to a slice of [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")s.

[cast\_slice\_of\_unique\_entity\_slice\_mut](fn.cast_slice_of_unique_entity_slice_mut.html "fn bevy::ecs::entity::unique_slice::cast_slice_of_unique_entity_slice_mut")⚠

Casts a mutable slice of entity slices to a slice of [`UniqueEntityEquivalentSlice`](../struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")s.

[from\_mut](fn.from_mut.html "fn bevy::ecs::entity::unique_slice::from_mut")

Converts a reference to T into a slice of length 1 (without copying).

[from\_raw\_parts](fn.from_raw_parts.html "fn bevy::ecs::entity::unique_slice::from_raw_parts")⚠

Forms a slice from a pointer and a length.

[from\_raw\_parts\_mut](fn.from_raw_parts_mut.html "fn bevy::ecs::entity::unique_slice::from_raw_parts_mut")⚠

Performs the same functionality as [`from_raw_parts`](fn.from_raw_parts.html "fn bevy::ecs::entity::unique_slice::from_raw_parts"), except that a mutable slice is returned.

[from\_ref](fn.from_ref.html "fn bevy::ecs::entity::unique_slice::from_ref")

Converts a reference to T into a slice of length 1 (without copying).

## Type Aliases

[ChunkBy](type.ChunkBy.html "type bevy::ecs::entity::unique_slice::ChunkBy")

An iterator over slice in (non-overlapping) chunks separated by a predicate.

[ChunkByMut](type.ChunkByMut.html "type bevy::ecs::entity::unique_slice::ChunkByMut")

An iterator over slice in (non-overlapping) mutable chunks separated by a predicate.

[Chunks](type.Chunks.html "type bevy::ecs::entity::unique_slice::Chunks")

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

[ChunksExact](type.ChunksExact.html "type bevy::ecs::entity::unique_slice::ChunksExact")

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

[ChunksExactMut](type.ChunksExactMut.html "type bevy::ecs::entity::unique_slice::ChunksExactMut")

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

[ChunksMut](type.ChunksMut.html "type bevy::ecs::entity::unique_slice::ChunksMut")

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the beginning of the slice.

[Iter](type.Iter.html "type bevy::ecs::entity::unique_slice::Iter")

Immutable slice iterator.

[IterMut](type.IterMut.html "type bevy::ecs::entity::unique_slice::IterMut")

Mutable slice iterator.

[RChunks](type.RChunks.html "type bevy::ecs::entity::unique_slice::RChunks")

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the end of the slice.

[RChunksExact](type.RChunksExact.html "type bevy::ecs::entity::unique_slice::RChunksExact")

An iterator over a slice in (non-overlapping) chunks (`chunk_size` elements at a time), starting at the end of the slice.

[RChunksExactMut](type.RChunksExactMut.html "type bevy::ecs::entity::unique_slice::RChunksExactMut")

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the end of the slice.

[RChunksMut](type.RChunksMut.html "type bevy::ecs::entity::unique_slice::RChunksMut")

An iterator over a slice in (non-overlapping) mutable chunks (`chunk_size` elements at a time), starting at the end of the slice.

[RSplit](type.RSplit.html "type bevy::ecs::entity::unique_slice::RSplit")

An iterator over subslices separated by elements that match a predicate function, starting from the end of the slice.

[RSplitMut](type.RSplitMut.html "type bevy::ecs::entity::unique_slice::RSplitMut")

An iterator over the subslices of the vector which are separated by elements that match `pred`, starting from the end of the slice.

[RSplitN](type.RSplitN.html "type bevy::ecs::entity::unique_slice::RSplitN")

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

[RSplitNMut](type.RSplitNMut.html "type bevy::ecs::entity::unique_slice::RSplitNMut")

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits, starting from the end of the slice.

[Split](type.Split.html "type bevy::ecs::entity::unique_slice::Split")

An iterator over subslices separated by elements that match a predicate function.

[SplitInclusive](type.SplitInclusive.html "type bevy::ecs::entity::unique_slice::SplitInclusive")

An iterator over subslices separated by elements that match a predicate function.

[SplitInclusiveMut](type.SplitInclusiveMut.html "type bevy::ecs::entity::unique_slice::SplitInclusiveMut")

An iterator over the mutable subslices of the vector which are separated by elements that match `pred`. Unlike `SplitMut`, it contains the matched parts in the ends of the subslices.

[SplitMut](type.SplitMut.html "type bevy::ecs::entity::unique_slice::SplitMut")

An iterator over the mutable subslices of the vector which are separated by elements that match `pred`.

[SplitN](type.SplitN.html "type bevy::ecs::entity::unique_slice::SplitN")

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

[SplitNMut](type.SplitNMut.html "type bevy::ecs::entity::unique_slice::SplitNMut")

An iterator over subslices separated by elements that match a predicate function, limited to a given number of splits.

[UniqueEntitySlice](type.UniqueEntitySlice.html "type bevy::ecs::entity::unique_slice::UniqueEntitySlice")

A slice that contains only unique [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[Windows](type.Windows.html "type bevy::ecs::entity::unique_slice::Windows")

An iterator over overlapping subslices of length `size`.