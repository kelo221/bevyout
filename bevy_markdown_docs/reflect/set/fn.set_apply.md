[bevy](../../index.html)::[reflect](../index.html)::[set](index.html)

# Function set\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#462)

```rust
pub fn set_apply<M>(a: &mut M, b: &(dyn PartialReflect + 'static))where
    M: Set,
```

Applies the elements of reflected set `b` to the corresponding elements of set `a`.

If a value from `b` does not exist in `a`, the value is cloned and inserted. If a value from `a` does not exist in `b`, the value is removed.

## Panics

This function panics if `b` is not a reflected set.