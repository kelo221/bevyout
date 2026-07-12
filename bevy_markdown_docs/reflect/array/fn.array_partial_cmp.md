[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#478-481)

```rust
pub fn array_partial_cmp<A>(
    array: &A,
    reflect: &(dyn PartialReflect + 'static),
) -> Option<Ordering>where
    A: Array + ?Sized,
```

Lexicographically compares two [arrays](trait.Array.html "trait bevy::reflect::array::Array") and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).