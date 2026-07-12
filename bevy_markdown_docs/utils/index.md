[bevy](../index.html)

# Crate utils 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/lib.rs.html#1-132)

General utilities for first-party [Bevy](https://bevy.org/) engine crates.

## Modules

[cfg](cfg/index.html "mod bevy::utils::cfg")

Configuration information for this crate.

[prelude](prelude/index.html "mod bevy::utils::prelude")

The utilities prelude.

## Macros

[define\_atomic\_id](macro.define_atomic_id.html "macro bevy::utils::define_atomic_id")

Defines a 32-bit id type which guarantees global uniqueness via atomics on a static global.

[once](macro.once.html "macro bevy::utils::once")

Call some expression only once per call site.

## Structs

[BloomFilter](struct.BloomFilter.html "struct bevy::utils::BloomFilter")

A Bloom filter, parameterized by number of u64 segments `N` and number of hash functions `K`.

[BufferedChannel](struct.BufferedChannel.html "struct bevy::utils::BufferedChannel")

An asynchronous MPSC channel that buffers messages and reuses allocations with thread locals.

[BufferedReceiver](struct.BufferedReceiver.html "struct bevy::utils::BufferedReceiver")

A wrapper around a [`Receiver`](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Receiver.html "struct async_channel::Receiver") that returns [`RecycledVec`](struct.RecycledVec.html "struct bevy::utils::RecycledVec")s to automatically return buffers to the [`BufferedChannel`](struct.BufferedChannel.html "struct bevy::utils::BufferedChannel") pool.

[BufferedSender](struct.BufferedSender.html "struct bevy::utils::BufferedSender")

A [`BufferedChannel`](struct.BufferedChannel.html "struct bevy::utils::BufferedChannel") sender that buffers messages locally, flushing it when the sender is dropped or [`BufferedChannel::chunk_size`](struct.BufferedChannel.html#structfield.chunk_size "field bevy::utils::BufferedChannel::chunk_size") is reached.

[DebugName](struct.DebugName.html "struct bevy::utils::DebugName")

Wrapper to help debugging ECS issues. This is used to display the names of systems, components, …

[OnDrop](struct.OnDrop.html "struct bevy::utils::OnDrop")

A type which calls a function when dropped. This can be used to ensure that cleanup code is run even in case of a panic.

[Parallel](struct.Parallel.html "struct bevy::utils::Parallel")

A cohesive set of thread-local values of a given type.

[RecycledVec](struct.RecycledVec.html "struct bevy::utils::RecycledVec")

A wrapper around a `Vec<T>` that automatically returns it to the [`BufferedChannel`](struct.BufferedChannel.html "struct bevy::utils::BufferedChannel")’s pool when dropped.

## Enums

[TypeIdMapEntry](enum.TypeIdMapEntry.html "enum bevy::utils::TypeIdMapEntry")

Entry for an existing key-value pair in an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") or a vacant location to insert one.

## Traits

[PreHashMapExt](trait.PreHashMapExt.html "trait bevy::utils::PreHashMapExt")

Extension methods intended to add functionality to [`PreHashMap`](type.PreHashMap.html "type bevy::utils::PreHashMap").

[TypeIdMapExt](trait.TypeIdMapExt.html "trait bevy::utils::TypeIdMapExt")

Extension trait to make use of [`TypeIdMap`](type.TypeIdMap.html "type bevy::utils::TypeIdMap") more ergonomic.

## Functions

[default](fn.default.html "fn bevy::utils::default")

An ergonomic abbreviation for [`Default::default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") to make initializing structs easier.

## Type Aliases

[PreHashMap](type.PreHashMap.html "type bevy::utils::PreHashMap")

A [`HashMap`](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`Hashed`](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed") keys and [`PassHash`](../platform/hash/struct.PassHash.html "struct bevy::platform::hash::PassHash") passthrough hashing. Iteration order only depends on the order of insertions and deletions.

[TypeIdMap](type.TypeIdMap.html "type bevy::utils::TypeIdMap")

A specialized map type with Key of [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") Iteration order only depends on the order of insertions and deletions.