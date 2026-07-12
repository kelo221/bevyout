[bevy](../../index.html)::[reflect](../index.html)::[tuple\_struct](index.html)

# Function tuple\_struct\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#503-506)

```rust
pub fn tuple_struct_debug(
    dyn_tuple_struct: &(dyn TupleStruct + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") types.

## Example

```rust
use bevy_reflect::Reflect;
#[derive(Reflect)]
struct MyTupleStruct(usize);

let my_tuple_struct: &dyn Reflect = &MyTupleStruct(123);
println!("{:#?}", my_tuple_struct);

// Output:

// MyTupleStruct (
//   123,
// )
```