[bevy](../../index.html)::[reflect](../index.html)::[enums](index.html)

# Function enum\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/helpers.rs.html#33)

```rust
pub fn enum_partial_eq(
    a: &(dyn Enum + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>
```

Compares an [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is an enum;
*   `b` is the same variant as `a`;
*   For each field in `a`, `b` contains a field with the same name and [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for the two field values.