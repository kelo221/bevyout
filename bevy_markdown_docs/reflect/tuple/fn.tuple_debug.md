[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Function tuple\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#496)

```rust
pub fn tuple_debug(
    dyn_tuple: &(dyn Tuple + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") types.

## Example

```rust
use bevy_reflect::Reflect;

let my_tuple: &dyn Reflect = &(1, 2, 3);
println!("{:#?}", my_tuple);

// Output:

// (
//   1,
//   2,
//   3,
// )
```