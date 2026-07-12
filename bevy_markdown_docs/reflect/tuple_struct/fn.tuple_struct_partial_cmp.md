[bevy](../../index.html)::[reflect](../index.html)::[tuple\_struct](index.html)

# Function tuple\_struct\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#459-462)

```rust
pub fn tuple_struct_partial_cmp(
    a: &(dyn TupleStruct + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>
```

Lexicographically compares two [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") values and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).