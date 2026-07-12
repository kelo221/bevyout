[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Function map\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#560)

```rust
pub fn map_debug(
    dyn_map: &(dyn Map + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Map`](trait.Map.html "trait bevy::reflect::map::Map") types.

## Example

```rust
use bevy_reflect::Reflect;

let mut my_map = HashMap::new();
my_map.insert(123, String::from("Hello"));
println!("{:#?}", &my_map as &dyn Reflect);

// Output:

// {
//   123: "Hello",
// }
```