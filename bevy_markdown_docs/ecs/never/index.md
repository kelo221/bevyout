[bevy](../../index.html)::[ecs](../index.html)

# Module never 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#44)

A workaround for the `!` type in stable Rust.

This approach is taken from the [`never_say_never`](https://crates.io/crates/never_say_never) crate, reimplemented here to avoid adding a new dependency.

This module exists due to a change in [never type fallback inference](https://doc.rust-lang.org/edition-guide/rust-2024/never-type-fallback.html) in the Rust 2024 edition. This caused failures in `bevy_ecs`’s traits which are implemented for functions (like [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")) when working with panicking closures.

Note that using this hack is not recommended in general; by doing so you are knowingly opting out of rustc’s stability guarantees. Code that compiles due to this hack may break in future versions of Rust.

Please read [issue #18778](https://github.com/bevyengine/bevy/issues/18778) for an explanation of why Bevy has chosen to use this workaround.

## Type Aliases

[Never](type.Never.html "type bevy::ecs::never::Never")

A hacky type alias for the `!` (never) type.