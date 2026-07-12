[bevy](../../index.html)::[platform](../index.html)

# Module hash 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#27)

Provides replacements for `std::hash` items using [`foldhash`](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/index.html "mod foldhash").

Also provides some additional items beyond the standard library.

## Structs

[DefaultHasher](struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher")

A [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher") instance implementing foldhash, optimized for speed.

[FixedHasher](struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher")

Deterministic hasher based upon a random but fixed state.

[FixedState](struct.FixedState.html "struct bevy::platform::hash::FixedState")

A [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") for [`fast::FoldHasher`](struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher") that always has the same fixed seed.

[Hashed](struct.Hashed.html "struct bevy::platform::hash::Hashed")

A pre-hashed value of a specific type. Pre-hashing enables memoization of hashes that are expensive to compute.

[NoOpHash](struct.NoOpHash.html "struct bevy::platform::hash::NoOpHash")

[`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") for types that already contain a high-quality hash.

[PassHash](struct.PassHash.html "struct bevy::platform::hash::PassHash")

A [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") that results in a [`PassHasher`](struct.PassHasher.html "struct bevy::platform::hash::PassHasher").

[PassHasher](struct.PassHasher.html "struct bevy::platform::hash::PassHasher")

A no-op hash that only works on `u64`s. Will panic if attempting to hash a type containing non-u64 fields.

[RandomState](struct.RandomState.html "struct bevy::platform::hash::RandomState")

A [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") for [`fast::FoldHasher`](struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher") that is randomly initialized.

## Functions

[fixed\_hash\_one](fn.fixed_hash_one.html "fn bevy::platform::hash::fixed_hash_one")

Hashes one value with the deterministic [`FixedHasher`](struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher").