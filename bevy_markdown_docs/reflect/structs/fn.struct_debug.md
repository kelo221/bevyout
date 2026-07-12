[bevy](../../index.html)::[reflect](../index.html)::[structs](index.html)

# Function struct\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#745)

```rust
pub fn struct_debug(
    dyn_struct: &(dyn Struct + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") types.

## Example

```rust
use bevy_reflect::Reflect;
#[derive(Reflect)]
struct MyStruct {
  foo: usize
}

let my_struct: &dyn Reflect = &MyStruct { foo: 123 };
println!("{:#?}", my_struct);

// Output:

// MyStruct {
//   foo: 123,
// }
```