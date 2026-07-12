[bevy](../../index.html)::[reflect](../index.html)::[enums](index.html)

# Function enum\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/helpers.rs.html#174)

```rust
pub fn enum_debug(
    dyn_enum: &(dyn Enum + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") types.

## Example

```rust
use bevy_reflect::Reflect;
#[derive(Reflect)]
enum MyEnum {
  A,
  B (usize),
  C {value: i32}
}

let my_enum: &dyn Reflect = &MyEnum::B(123);
println!("{:#?}", my_enum);

// Output:

// B (
//   123,
// )
```