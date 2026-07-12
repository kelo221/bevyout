[bevy](../../index.html)::[platform](../index.html)

# Module collections 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#21)

Provides [`HashMap`](struct.HashMap.html "struct bevy::platform::collections::HashMap") and [`HashSet`](struct.HashSet.html "struct bevy::platform::collections::HashSet") from [`hashbrown`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") with some customized defaults.

Also provides the [`HashTable`](struct.HashTable.html "struct bevy::platform::collections::HashTable") type, which is specific to [`hashbrown`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown").

## Modules

[hash\_map](hash_map/index.html "mod bevy::platform::collections::hash_map")

Provides [`HashMap`](struct.HashMap.html "struct bevy::platform::collections::HashMap") based on [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown")’s implementation. Unlike [`hashbrown::HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap"), [`HashMap`](struct.HashMap.html "struct bevy::platform::collections::HashMap") defaults to [`FixedHasher`](../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") instead of [`RandomState`](../hash/struct.RandomState.html "struct bevy::platform::hash::RandomState"). This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

[hash\_set](hash_set/index.html "mod bevy::platform::collections::hash_set")

Provides [`HashSet`](struct.HashSet.html "struct bevy::platform::collections::HashSet") based on [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown")’s implementation. Unlike [`hashbrown::HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet"), [`HashSet`](struct.HashSet.html "struct bevy::platform::collections::HashSet") defaults to [`FixedHasher`](../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") instead of [`RandomState`](../hash/struct.RandomState.html "struct bevy::platform::hash::RandomState"). This provides determinism by default with an acceptable compromise to denial of service resistance in the context of a game engine.

[hash\_table](hash_table/index.html "mod bevy::platform::collections::hash_table")

Provides [`HashTable`](struct.HashTable.html "struct bevy::platform::collections::HashTable")

## Structs

[HashMap](struct.HashMap.html "struct bevy::platform::collections::HashMap")

New-type for [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") with [`FixedHasher`](../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider. Can be trivially converted to and from a [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") [`HashMap`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap") using [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From").

[HashSet](struct.HashSet.html "struct bevy::platform::collections::HashSet")

New-type for [`HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") with [`FixedHasher`](../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider. Can be trivially converted to and from a [hashbrown](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") [`HashSet`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") using [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From").

[HashTable](struct.HashTable.html "struct bevy::platform::collections::HashTable")

Low-level hash table with explicit hashing.

## Traits

[Equivalent](trait.Equivalent.html "trait bevy::platform::collections::Equivalent")

Key equivalence trait.