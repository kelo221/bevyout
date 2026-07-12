[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Trait GetTupleStructField 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#197)

```rust
pub trait GetTupleStructField {
    // Required methods
    fn get_field<T>(&self, index: usize) -> Option<&T>
       where T: Reflect;
    fn get_field_mut<T>(&mut self, index: usize) -> Option<&mut T>
       where T: Reflect;
}
```

A convenience trait which combines fetching and downcasting of tuple struct fields.

## Example

```rust
use bevy_reflect::{tuple_struct::GetTupleStructField, Reflect};

#[derive(Reflect)]
struct Foo(String);

let mut foo = Foo("Hello, world!".to_string());

foo.get_field_mut::<String>(0).unwrap().truncate(5);
assert_eq!(foo.get_field::<String>(0), Some(&"Hello".to_string()));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#200)

#### fn [get\_field](#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#204)

#### fn [get\_field\_mut](#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#219)

### impl [GetTupleStructField](../../prelude/trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for dyn [TupleStruct](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#207)

### impl<S> [GetTupleStructField](../../prelude/trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for S

where S: [TupleStruct](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct"),