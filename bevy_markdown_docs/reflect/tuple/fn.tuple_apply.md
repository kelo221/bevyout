[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Function tuple\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#397)

```rust
pub fn tuple_apply<T>(a: &mut T, b: &(dyn PartialReflect + 'static))where
    T: Tuple,
```

Applies the elements of `b` to the corresponding elements of `a`.

## Panics

This function panics if `b` is not a tuple.