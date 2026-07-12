[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Function tuple\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#456-459)

```rust
pub fn tuple_partial_cmp<T>(
    a: &T,
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>where
    T: Tuple + ?Sized,
```

Lexicographically compares two [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") values and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).