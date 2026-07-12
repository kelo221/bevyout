[bevy](../index.html)::[reflect](index.html)

# Macro impl\_from\_reflect\_opaque 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#763)

```rust
impl_from_reflect_opaque!() { /* proc-macro */ }
```

A macro used to generate a `FromReflect` trait implementation for the given type.

This is functionally the same as [deriving `FromReflect`](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") on a type that [derives `Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") using the `#[reflect(opaque)]` container attribute.

The only reason this macro exists is so that `bevy_reflect` can easily implement `FromReflect` on primitives and other opaque types internally.

Please note that this macro will not work with any type that [derives `Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") normally or makes use of the [`impl_reflect_opaque!`](macro.impl_reflect_opaque.html "macro bevy::reflect::impl_reflect_opaque") macro, as those macros also implement `FromReflect` by default.

## Examples

[ⓘ](# "This example is not tested")

```rust
impl_from_reflect_opaque!(foo<T1, T2: Baz> where T1: Bar);
```