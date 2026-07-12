[bevy](../../../index.html)::[platform](../../index.html)::[collections](../index.html)

# Module hash\_table 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/mod.rs.html#12)

Provides [`HashTable`](../struct.HashTable.html "struct bevy::platform::collections::HashTable")

## Structs

[AbsentEntry](struct.AbsentEntry.html "struct bevy::platform::collections::hash_table::AbsentEntry")

Type representing the absence of an entry, as returned by [`HashTable::find_entry`](../struct.HashTable.html#method.find_entry "method bevy::platform::collections::HashTable::find_entry") and [`HashTable::get_bucket_entry`](../struct.HashTable.html#method.get_bucket_entry "method bevy::platform::collections::HashTable::get_bucket_entry").

[Drain](struct.Drain.html "struct bevy::platform::collections::hash_table::Drain")

A draining iterator over the items of a `HashTable`.

[ExtractIf](struct.ExtractIf.html "struct bevy::platform::collections::hash_table::ExtractIf")

A draining iterator over entries of a `HashTable` which don’t satisfy the predicate `f`.

[HashTable](struct.HashTable.html "struct bevy::platform::collections::hash_table::HashTable")

Low-level hash table with explicit hashing.

[IntoIter](struct.IntoIter.html "struct bevy::platform::collections::hash_table::IntoIter")

An owning iterator over the entries of a `HashTable` in arbitrary order. The iterator element type is `T`.

[Iter](struct.Iter.html "struct bevy::platform::collections::hash_table::Iter")

An iterator over the entries of a `HashTable` in arbitrary order. The iterator element type is `&'a T`.

[IterHash](struct.IterHash.html "struct bevy::platform::collections::hash_table::IterHash")

An iterator over the entries of a `HashTable` that could match a given hash. The iterator element type is `&'a T`.

[IterHashMut](struct.IterHashMut.html "struct bevy::platform::collections::hash_table::IterHashMut")

A mutable iterator over the entries of a `HashTable` that could match a given hash. The iterator element type is `&'a mut T`.

[IterMut](struct.IterMut.html "struct bevy::platform::collections::hash_table::IterMut")

A mutable iterator over the entries of a `HashTable` in arbitrary order. The iterator element type is `&'a mut T`.

[OccupiedEntry](struct.OccupiedEntry.html "struct bevy::platform::collections::hash_table::OccupiedEntry")

A view into an occupied entry in a `HashTable`. It is part of the [`Entry`](enum.Entry.html) enum.

[VacantEntry](struct.VacantEntry.html "struct bevy::platform::collections::hash_table::VacantEntry")

A view into a vacant entry in a `HashTable`. It is part of the [`Entry`](enum.Entry.html) enum.

## Enums

[Entry](enum.Entry.html "enum bevy::platform::collections::hash_table::Entry")

A view into a single entry in a table, which may either be vacant or occupied.