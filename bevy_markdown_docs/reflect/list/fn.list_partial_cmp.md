[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#503-506)

```rust
pub fn list_partial_cmp<L>(
    a: &L,
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>where
    L: List + ?Sized,
```

Lexicographically compares two [List](trait.List.html "trait bevy::reflect::list::List") values and returns their ordering.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (e.g., kinds mismatch or an element comparison returns `None`).