[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_debug 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#542)

```rust
pub fn list_debug(
    dyn_list: &(dyn List + 'static),
    f: &mut Formatter<'_>,
) -> Result<(), Error>
```

The default debug formatter for [`List`](trait.List.html "trait bevy::reflect::list::List") types.

## Example

```rust
use bevy_reflect::Reflect;

let my_list: &dyn Reflect = &vec![1, 2, 3];
println!("{:#?}", my_list);

// Output:

// [
//   1,
//   2,
//   3,
// ]
```