[bevy](../../index.html)::[scene](../index.html)

# Module macro\_utils 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/lib.rs.html#909)

Functionality used by the \[`bsn!`\] macro.

## Structs

[CallCounter](struct.CallCounter.html "struct bevy::scene::macro_utils::CallCounter")

A counter meant to be stored in a `static` to differentiate multiple evaluations of the expression returned by [`bsn!`](crate::bsn)

## Functions

[touch\_type](fn.touch_type.html "fn bevy::scene::macro_utils::touch_type")

This is used by the [`bsn!`](crate::bsn) macro to generate compile-time only references to symbols. Currently this is used to add IDE support for nested type names, as it allows us to pass the input Ident from the input to the output code.

## Type Aliases

[PathResolveHelper](type.PathResolveHelper.html "type bevy::scene::macro_utils::PathResolveHelper")

This is used by the [`bsn!`](crate::bsn) derive to work around [this Rust limitation](https://github.com/rust-lang/rust/issues/86935). A fix is implemented and on track for stabilization. If it is ever implemented, we can remove this.