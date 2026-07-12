[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Function map\_partial\_cmp 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#509-512)

```rust
pub fn map_partial_cmp<M>(
    a: &M,
    b: &(dyn PartialReflect + 'static),
) -> Option<Ordering>where
    M: Map + ?Sized,
```

Lexicographically compares two [`Map`](trait.Map.html "trait bevy::reflect::map::Map") values according to their iteration order (suitable for ordered maps like `BTreeMap`).

For each entry pair `(a_k, a_v)` and `(b_k, b_v)` in the iteration order, compare `a_k` to `b_k` using `reflect_partial_cmp`, returning the first non-equal ordering. If keys are equal, compare values `a_v` and `b_v` similarly. If all compared entries are equal, the shorter map is `Less` and longer is `Greater`.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the comparison couldn’t be performed (kinds mismatch or an element comparison returns `None`).