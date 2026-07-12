[bevy](../../index.html)::[reflect](../index.html)::[set](index.html)

# Function set\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#404)

```rust
pub fn set_partial_eq<M>(
    a: &M,
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>where
    M: Set,
```

Compares a [`Set`](trait.Set.html "trait bevy::reflect::set::Set") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is a set;
*   `b` is the same length as `a`;
*   For each value pair in `a`, `b` contains the value too, and [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for the two values.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.