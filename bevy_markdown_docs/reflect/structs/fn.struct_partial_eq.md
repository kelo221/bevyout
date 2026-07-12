[bevy](../../index.html)::[reflect](../index.html)::[structs](index.html)

# Function struct\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#597)

```rust
pub fn struct_partial_eq(
    a: &(dyn Struct + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>
```

Compares a [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is a struct;
*   For each field in `a`, `b` contains a field with the same name and [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for the two field values.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.