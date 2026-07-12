[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Trait GetTupleField 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#123)

```rust
pub trait GetTupleField {
    // Required methods
    fn get_field<T>(&self, index: usize) -> Option<&T>
       where T: Reflect;
    fn get_field_mut<T>(&mut self, index: usize) -> Option<&mut T>
       where T: Reflect;
}
```

A convenience trait which combines fetching and downcasting of tuple fields.

## Example

```rust
use bevy_reflect::tuple::GetTupleField;

let foo = ("blue".to_string(), 42_i32);

assert_eq!(foo.get_field::<String>(0), Some(&"blue".to_string()));
assert_eq!(foo.get_field::<i32>(1), Some(&42));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#126)

#### fn [get\_field](#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#130)

#### fn [get\_field\_mut](#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#145)

### impl [GetTupleField](trait.GetTupleField.html "trait bevy::reflect::tuple::GetTupleField") for dyn [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#133)

### impl<S> [GetTupleField](trait.GetTupleField.html "trait bevy::reflect::tuple::GetTupleField") for S

where S: [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple"),