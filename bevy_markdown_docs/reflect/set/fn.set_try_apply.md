[bevy](../../index.html)::[reflect](../index.html)::[set](index.html)

# Function set\_try\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#479)

```rust
pub fn set_try_apply<S>(
    a: &mut S,
    b: &(dyn PartialReflect + 'static),
) -> Result<(), ApplyError>where
    S: Set,
```

Tries to apply the elements of reflected set `b` to the corresponding elements of set `a` and returns a Result.

If a value from `b` does not exist in `a`, the value is cloned and inserted. If a value from `a` does not exist in `b`, the value is removed.

## Errors

This function returns an [`ApplyError::MismatchedKinds`](../enum.ApplyError.html#variant.MismatchedKinds "variant bevy::reflect::ApplyError::MismatchedKinds") if `b` is not a reflected set or if applying elements to each other fails.