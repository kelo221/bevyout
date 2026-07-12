[bevy](../../index.html)::[reflect](../index.html)::[set](index.html)

# Function set\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#445)

```rust
pub fn set_debug(
    dyn_set: &(dyn Set + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`Set`](trait.Set.html "trait bevy::reflect::set::Set") types.

## Example

```rust
use bevy_reflect::Reflect;

let mut my_set = HashSet::new();
my_set.insert(String::from("Hello"));
println!("{:#?}", &my_set as &dyn Reflect);

// Output:

// {
//   "Hello",
// }
```