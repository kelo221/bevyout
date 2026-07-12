[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#518)

```rust
pub fn array_debug(
    dyn_array: &(dyn Array + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Array`](trait.Array.html "trait bevy::reflect::array::Array") types.

## Example

```rust
use bevy_reflect::Reflect;

let my_array: &dyn Reflect = &[1, 2, 3];
println!("{:#?}", my_array);

// Output:

// [
//   1,
//   2,
//   3,
// ]
```