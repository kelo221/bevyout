[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Function tuple\_try\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#411)

```rust
pub fn tuple_try_apply<T>(
    a: &mut T,
    b: &(dyn PartialReflect + 'static),
) -> Result<(), ApplyError>where
    T: Tuple,
```

Tries to apply the elements of `b` to the corresponding elements of `a` and returns a Result.

## Errors

This function returns an [`ApplyError::MismatchedKinds`](../enum.ApplyError.html#variant.MismatchedKinds "variant bevy::reflect::ApplyError::MismatchedKinds") if `b` is not a tuple or if applying elements to each other fails.