[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Function tuple\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#432)

```rust
pub fn tuple_partial_eq<T>(
    a: &T,
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>where
    T: Tuple + ?Sized,
```

Compares a [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is a tuple;
*   `b` has the same number of elements as `a`;
*   [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for pairwise elements of `a` and `b`.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.