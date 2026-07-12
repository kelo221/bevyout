[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_partial\_eq 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#479)

```rust
pub fn list_partial_eq<L>(
    a: &L,
    b: &(dyn PartialReflect + 'static),
) -> Option<bool>where
    L: List + ?Sized,
```

Compares a [`List`](trait.List.html "trait bevy::reflect::list::List") with a [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

Returns true if and only if all of the following are true:

*   `b` is a list;
*   `b` is the same length as `a`;
*   [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq") returns `Some(true)` for pairwise elements of `a` and `b`.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t even be performed.