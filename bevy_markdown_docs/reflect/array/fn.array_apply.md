[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#404)

```rust
pub fn array_apply<A>(array: &mut A, reflect: &(dyn PartialReflect + 'static))where
    A: Array + ?Sized,
```

Applies the reflected [array](trait.Array.html "trait bevy::reflect::array::Array") data to the given [array](trait.Array.html "trait bevy::reflect::array::Array").

## Panics

*   Panics if the two arrays have differing lengths.
*   Panics if the reflected value is not a [valid array](../enum.ReflectRef.html#variant.Array "variant bevy::reflect::ReflectRef::Array").