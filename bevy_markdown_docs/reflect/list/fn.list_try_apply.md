[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_try\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#454)

```rust
pub fn list_try_apply<L>(
    a: &mut L,
    b: &(dyn PartialReflect + 'static),
) -> Result<(), ApplyError>where
    L: List,
```

Tries to apply the elements of `b` to the corresponding elements of `a` and returns a Result.

If the length of `b` is greater than that of `a`, the excess elements of `b` are cloned and appended to `a`.

## Errors

This function returns an [`ApplyError::MismatchedKinds`](../enum.ApplyError.html#variant.MismatchedKinds "variant bevy::reflect::ApplyError::MismatchedKinds") if `b` is not a list or if applying elements to each other fails.