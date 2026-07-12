[bevy](../index.html)::[reflect](index.html)

# Macro impl\_type\_path 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#828)

```rust
impl_type_path!() { /* proc-macro */ }
```

A replacement for [deriving `TypePath`](../prelude/derive.TypePath.html "derive bevy::prelude::TypePath") for use on foreign types.

Since (unlike the derive) this macro may be invoked in a different module to where the type is defined, it requires an ‘absolute’ path definition.

Specifically, a leading `::` denoting a global path must be specified or a preceding `(in my_crate::foo)` to specify the custom path must be used.

## Examples

Implementing `TypePath` on a foreign type:

[ⓘ](# "This example is not tested")

```rust
impl_type_path!(::foreign_crate::foo::bar::Baz);
```

On a generic type (this can also accept trait bounds):

[ⓘ](# "This example is not tested")

```rust
impl_type_path!(::foreign_crate::Foo<T>);
impl_type_path!(::foreign_crate::Goo<T: ?Sized>);
```

On a primitive (note this will not compile for a non-primitive type):

[ⓘ](# "This example is not tested")

```rust
impl_type_path!(bool);
```

With a custom type path:

[ⓘ](# "This example is not tested")

```rust
impl_type_path!((in other_crate::foo::bar) Baz);
```

With a custom type path and a custom type name:

[ⓘ](# "This example is not tested")

```rust
impl_type_path!((in other_crate::foo as Baz) Bar);
```