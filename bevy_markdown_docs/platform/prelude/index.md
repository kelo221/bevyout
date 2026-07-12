[bevy](../../index.html)::[platform](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#41)

Frequently used items which would typically be included in most contexts.

When adding `no_std` support to a crate for the first time, often there’s a substantial refactor required due to the change in implicit prelude from `std::prelude` to `core::prelude`. This unfortunately leaves out many items from `alloc`, even if the crate unconditionally includes that crate.

This prelude aims to ease the transition by re-exporting items from `alloc` which would otherwise be included in the `std` implicit prelude.

## Macros

[format](macro.format.html "macro bevy::platform::prelude::format")

Creates a `String` using interpolation of runtime expressions.

[vec](macro.vec.html "macro bevy::platform::prelude::vec")Non-`no_global_oom_handling`

Creates a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") containing the arguments.

## Structs

[Box](struct.Box.html "struct bevy::platform::prelude::Box")

A pointer type that uniquely owns a heap allocation of type `T`.

[String](struct.String.html "struct bevy::platform::prelude::String")

A UTF-8–encoded, growable string.

[Vec](struct.Vec.html "struct bevy::platform::prelude::Vec")

A contiguous growable array type, written as `Vec<T>`, short for ‘vector’.

## Traits

[ToOwned](trait.ToOwned.html "trait bevy::platform::prelude::ToOwned")

A generalization of `Clone` to borrowed data.

[ToString](trait.ToString.html "trait bevy::platform::prelude::ToString")

A trait for converting a value to a `String`.