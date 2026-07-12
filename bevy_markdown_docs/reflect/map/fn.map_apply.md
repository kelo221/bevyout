[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Function map\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#577)

```rust
pub fn map_apply<M>(a: &mut M, b: &(dyn PartialReflect + 'static))where
    M: Map,
```

Applies the elements of reflected map `b` to the corresponding elements of map `a`.

If a key from `b` does not exist in `a`, the value is cloned and inserted. If a key from `a` does not exist in `b`, the value is removed.

## Panics

This function panics if `b` is not a reflected map.