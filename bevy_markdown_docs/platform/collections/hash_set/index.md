[bevy](../../../index.html)::[platform](../../index.html)::[collections](../index.html)

# Module hash\_set 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/mod.rs.html#11)

Provides [`HashSet`](../struct.HashSet.html "struct bevy::platform::collections::HashSet") based on [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown")’s implementation. Unlike [`hashbrown::HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet"), [`HashSet`](../struct.HashSet.html "struct bevy::platform::collections::HashSet") defaults to [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") instead of [`RandomState`](../../hash/struct.RandomState.html "struct bevy::platform::hash::RandomState"). This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

## Structs

[Difference](struct.Difference.html "struct bevy::platform::collections::hash_set::Difference")

A lazy iterator producing elements in the difference of `HashSet`s.

[Drain](struct.Drain.html "struct bevy::platform::collections::hash_set::Drain")

A draining iterator over the items of a `HashSet`.

[ExtractIf](struct.ExtractIf.html "struct bevy::platform::collections::hash_set::ExtractIf")

A draining iterator over entries of a `HashSet` which don’t satisfy the predicate `f`.

[HashSet](struct.HashSet.html "struct bevy::platform::collections::hash_set::HashSet")

New-type for [`HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider. Can be trivially converted to and from a [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") [`HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") using [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From").

[Intersection](struct.Intersection.html "struct bevy::platform::collections::hash_set::Intersection")

A lazy iterator producing elements in the intersection of `HashSet`s.

[IntoIter](struct.IntoIter.html "struct bevy::platform::collections::hash_set::IntoIter")

An owning iterator over the items of a `HashSet`.

[Iter](struct.Iter.html "struct bevy::platform::collections::hash_set::Iter")

An iterator over the items of a `HashSet`.

[OccupiedEntry](struct.OccupiedEntry.html "struct bevy::platform::collections::hash_set::OccupiedEntry")

A view into an occupied entry in a `HashSet`. It is part of the [`Entry`](enum.Entry.html) enum.

[SymmetricDifference](struct.SymmetricDifference.html "struct bevy::platform::collections::hash_set::SymmetricDifference")

A lazy iterator producing elements in the symmetric difference of `HashSet`s.

[Union](struct.Union.html "struct bevy::platform::collections::hash_set::Union")

A lazy iterator producing elements in the union of `HashSet`s.

[VacantEntry](struct.VacantEntry.html "struct bevy::platform::collections::hash_set::VacantEntry")

A view into a vacant entry in a `HashSet`. It is part of the [`Entry`](enum.Entry.html) enum.

## Type Aliases

[Entry](type.Entry.html "type bevy::platform::collections::hash_set::Entry")

Shortcut for [`Entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/enum.Entry.html "enum hashbrown::set::Entry") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider.