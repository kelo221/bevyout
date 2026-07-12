[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Trait GetField 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#262)

```rust
pub trait GetField {
    // Required methods
    fn get_field<T>(&self, name: &str) -> Option<&T>
       where T: Reflect;
    fn get_field_mut<T>(&mut self, name: &str) -> Option<&mut T>
       where T: Reflect;
}
```

A convenience trait which combines fetching and downcasting of struct fields.

## Example

```rust
use bevy_reflect::{structs::GetField, Reflect};

#[derive(Reflect)]
struct Foo {
    bar: String,
}

let mut foo = Foo { bar: "Hello, world!".to_string() };

foo.get_field_mut::<String>("bar").unwrap().truncate(5);
assert_eq!(foo.get_field::<String>("bar"), Some(&"Hello".to_string()));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#265)

#### fn [get\_field](#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#269)

#### fn [get\_field\_mut](#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#284)

### impl [GetField](../../prelude/trait.GetField.html "trait bevy::prelude::GetField") for dyn [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct"),