[bevy](../../index.html)::[reflect](../index.html)::[structs](index.html)

# Function struct\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#625)

```rust
pub fn struct_partial_cmp(
    a: &(dyn Struct + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>
```

Lexicographically compares two [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") values and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).