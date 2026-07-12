[bevy](../../../index.html)::[platform](../../index.html)::[collections](../index.html)

# Module hash\_map 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/mod.rs.html#10)

Provides [`HashMap`](../struct.HashMap.html "struct bevy::platform::collections::HashMap") based on [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown")’s implementation. Unlike [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap"), [`HashMap`](../struct.HashMap.html "struct bevy::platform::collections::HashMap") defaults to [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") instead of [`RandomState`](../../hash/struct.RandomState.html "struct bevy::platform::hash::RandomState"). This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

## Structs

[DefaultHasher](struct.DefaultHasher.html "struct bevy::platform::collections::hash_map::DefaultHasher")

A [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher") instance implementing foldhash, optimized for speed.

[Drain](struct.Drain.html "struct bevy::platform::collections::hash_map::Drain")

A draining iterator over the entries of a `HashMap` in arbitrary order. The iterator element type is `(K, V)`.

[ExtractIf](struct.ExtractIf.html "struct bevy::platform::collections::hash_map::ExtractIf")

A draining iterator over entries of a `HashMap` which don’t satisfy the predicate `f(&k, &mut v)` in arbitrary order. The iterator element type is `(K, V)`.

[HashMap](struct.HashMap.html "struct bevy::platform::collections::hash_map::HashMap")

New-type for [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider. Can be trivially converted to and from a [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") using [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From").

[IntoIter](struct.IntoIter.html "struct bevy::platform::collections::hash_map::IntoIter")

An owning iterator over the entries of a `HashMap` in arbitrary order. The iterator element type is `(K, V)`.

[IntoKeys](struct.IntoKeys.html "struct bevy::platform::collections::hash_map::IntoKeys")

An owning iterator over the keys of a `HashMap` in arbitrary order. The iterator element type is `K`.

[IntoValues](struct.IntoValues.html "struct bevy::platform::collections::hash_map::IntoValues")

An owning iterator over the values of a `HashMap` in arbitrary order. The iterator element type is `V`.

[Iter](struct.Iter.html "struct bevy::platform::collections::hash_map::Iter")

An iterator over the entries of a `HashMap` in arbitrary order. The iterator element type is `(&'a K, &'a V)`.

[IterMut](struct.IterMut.html "struct bevy::platform::collections::hash_map::IterMut")

A mutable iterator over the entries of a `HashMap` in arbitrary order. The iterator element type is `(&'a K, &'a mut V)`.

[Keys](struct.Keys.html "struct bevy::platform::collections::hash_map::Keys")

An iterator over the keys of a `HashMap` in arbitrary order. The iterator element type is `&'a K`.

[OccupiedEntry](struct.OccupiedEntry.html "struct bevy::platform::collections::hash_map::OccupiedEntry")

A view into an occupied entry in a [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap"). It is part of the [`Entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/enum.Entry.html "enum hashbrown::map::Entry") and [`EntryRef`](enum.EntryRef.html "enum bevy::platform::collections::hash_map::EntryRef") enums.

[OccupiedError](struct.OccupiedError.html "struct bevy::platform::collections::hash_map::OccupiedError")

The error returned by [`try_insert`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.try_insert "method hashbrown::map::HashMap::try_insert") when the key already exists.

[RandomState](struct.RandomState.html "struct bevy::platform::collections::hash_map::RandomState")

A [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") for [`fast::FoldHasher`](../../hash/struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher") that is randomly initialized.

[RawEntryBuilder](struct.RawEntryBuilder.html "struct bevy::platform::collections::hash_map::RawEntryBuilder")

A builder for computing where in a [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") a key-value pair would be stored.

[RawEntryBuilderMut](struct.RawEntryBuilderMut.html "struct bevy::platform::collections::hash_map::RawEntryBuilderMut")

A builder for computing where in a [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") a key-value pair would be stored.

[RawOccupiedEntryMut](struct.RawOccupiedEntryMut.html "struct bevy::platform::collections::hash_map::RawOccupiedEntryMut")

A view into an occupied entry in a `HashMap`. It is part of the [`RawEntryMut`](enum.RawEntryMut.html) enum.

[VacantEntry](struct.VacantEntry.html "struct bevy::platform::collections::hash_map::VacantEntry")

A view into a vacant entry in a `HashMap`. It is part of the [`Entry`](enum.Entry.html) enum.

[Values](struct.Values.html "struct bevy::platform::collections::hash_map::Values")

An iterator over the values of a `HashMap` in arbitrary order. The iterator element type is `&'a V`.

[ValuesMut](struct.ValuesMut.html "struct bevy::platform::collections::hash_map::ValuesMut")

A mutable iterator over the values of a `HashMap` in arbitrary order. The iterator element type is `&'a mut V`.

## Enums

[EntryRef](enum.EntryRef.html "enum bevy::platform::collections::hash_map::EntryRef")

A view into a single entry in a map, which may either be vacant or occupied, with any borrowed form of the map’s key type.

[RawEntryMut](enum.RawEntryMut.html "enum bevy::platform::collections::hash_map::RawEntryMut")

A view into a single entry in a map, which may either be vacant or occupied.

## Type Aliases

[Entry](type.Entry.html "type bevy::platform::collections::hash_map::Entry")

Shortcut for [`Entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/enum.Entry.html "enum hashbrown::map::Entry") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider.