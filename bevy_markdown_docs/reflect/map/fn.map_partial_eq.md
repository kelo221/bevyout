[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Function map\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#475)

```rust
pub fn map_partial_eq<M>(
    a: &M,
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>where
    M: Map + ?Sized,
```

Compares a [`Map`](trait.Map.html "trait bevy::reflect::map::Map") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is a map;
*   `b` is the same length as `a`;
*   For each key-value pair in `a`, `b` contains a value for the given key, and [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for the two values.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.