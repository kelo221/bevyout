[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Function map\_try\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#594)

```rust
pub fn map_try_apply<M>(
    a: &mut M,
    b: &(dyn PartialReflect + 'static),
) -> Result<(), ApplyError>where
    M: Map,
```

Tries to apply the elements of reflected map `b` to the corresponding elements of map `a` and returns a Result.

If a key from `b` does not exist in `a`, the value is cloned and inserted. If a key from `a` does not exist in `b`, the value is removed.

## Errors

This function returns an [`ApplyError::MismatchedKinds`](../enum.ApplyError.html#variant.MismatchedKinds "variant bevy::reflect::ApplyError::MismatchedKinds") if `b` is not a reflected map or if applying elements to each other fails.