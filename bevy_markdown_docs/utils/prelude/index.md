[bevy](../../index.html)::[utils](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/lib.rs.html#54)

The utilities prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[DebugName](struct.DebugName.html "struct bevy::utils::prelude::DebugName")

Wrapper to help debugging ECS issues. This is used to display the names of systems, components, …

[ShortName](struct.ShortName.html "struct bevy::utils::prelude::ShortName")

Lazily shortens a type name to remove all module paths.

## Functions

[default](fn.default.html "fn bevy::utils::prelude::default")

An ergonomic abbreviation for [`Default::default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") to make initializing structs easier.