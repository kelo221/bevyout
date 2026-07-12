[bevy](../../index.html)::[reflect](../index.html)::[tuple\_struct](index.html)

# Function tuple\_struct\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#432)

```rust
pub fn tuple_struct_partial_eq(
    a: &(dyn TupleStruct + 'static),
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>
```

Compares a [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

Returns true if and only if all of the following are true:

*   `b` is a tuple struct;
*   `b` has the same number of fields as `a`;
*   [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for pairwise fields of `a` and `b`.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.