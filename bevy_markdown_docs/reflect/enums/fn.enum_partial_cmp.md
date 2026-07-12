[bevy](../../index.html)::[reflect](../index.html)::[enums](index.html)

# Function enum\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/helpers.rs.html#98)

```rust
pub fn enum_partial_cmp(
    a: &(dyn Enum + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>
```

Compares two [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") values (by variant) and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).

The ordering is same with `derive` macro. First order by variant index, then by fields.