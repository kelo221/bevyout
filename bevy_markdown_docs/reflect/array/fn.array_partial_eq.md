[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#454-457)

```rust
pub fn array_partial_eq<A>(
    array: &A,
    reflect: &(dyn PartialReflect + 'static),
) -> Option<bool>where
    A: Array + ?Sized,
```

Compares two [arrays](trait.Array.html "trait bevy::reflect::array::Array") (one concrete and one reflected) to see if they are equal.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.