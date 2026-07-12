[bevy](../index.html)::[reflect](index.html)

# Macro impl\_reflect\_opaque 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#673)

```rust
impl_reflect_opaque!() { /* proc-macro */ }
```

A macro used to generate reflection trait implementations for the given type.

This is functionally the same as [deriving `Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") using the `#[reflect(opaque)]` container attribute.

The only reason for this macro’s existence is so that `bevy_reflect` can easily implement the reflection traits on primitives and other opaque types internally.

Since this macro also implements `TypePath`, the type path must be explicit. See [`impl_type_path!`](macro.impl_type_path.html "macro bevy::reflect::impl_type_path") for the exact syntax.

## Examples

Types can be passed with or without registering type data:

[ⓘ](# "This example is not tested")

```rust
impl_reflect_opaque!(my_crate::Foo);
impl_reflect_opaque!(my_crate::Bar(Debug, Default, Serialize, Deserialize));
```

Generic types can also specify their parameters and bounds:

[ⓘ](# "This example is not tested")

```rust
impl_reflect_opaque!(my_crate::Foo<T1, T2: Baz> where T1: Bar (Default, Serialize, Deserialize));
```

Custom type paths can be specified:

[ⓘ](# "This example is not tested")

```rust
impl_reflect_opaque!((in not_my_crate as NotFoo) Foo(Debug, Default));
```