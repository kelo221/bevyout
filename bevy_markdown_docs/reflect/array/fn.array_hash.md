[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Function array\_hash 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#387)

```rust
pub fn array_hash<A>(array: &A) -> Option<u64>where
    A: Array + ?Sized,
```

Returns the `u64` hash of the given [array](trait.Array.html "trait bevy::reflect::array::Array").