[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_try\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#428-431)

```rust
pub fn array_try_apply<A>(
    array: &mut A,
    reflect: &(dyn PartialReflect + 'static),
) -> Result<(), ApplyError>where
    A: Array,
```

Tries to apply the reflected [array](trait.Array.html "trait bevy::reflect::array::Array") data to the given [array](trait.Array.html "trait bevy::reflect::array::Array") and returns a Result.

## Errors

*   Returns an [`ApplyError::DifferentSize`](../enum.ApplyError.html#variant.DifferentSize "variant bevy::reflect::ApplyError::DifferentSize") if the two arrays have differing lengths.
*   Returns an [`ApplyError::MismatchedKinds`](../enum.ApplyError.html#variant.MismatchedKinds "variant bevy::reflect::ApplyError::MismatchedKinds") if the reflected value is not a [valid array](../enum.ReflectRef.html#variant.Array "variant bevy::reflect::ReflectRef::Array").
*   Returns any error that is generated while applying elements to each other.