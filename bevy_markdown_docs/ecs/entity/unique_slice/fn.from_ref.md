[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Function from\_ref 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#854)

```rust
pub const fn from_ref<T>(s: &T) -> &UniqueEntityEquivalentSlice<T>where
    T: EntityEquivalent,
```

Converts a reference to T into a slice of length 1 (without copying).