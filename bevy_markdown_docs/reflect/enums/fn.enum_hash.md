[bevy](../../index.html)::[reflect](../index.html)::[enums](index.html)

# Function enum\_hash 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/helpers.rs.html#13)

```rust
pub fn enum_hash(value: &(dyn Enum + 'static)) -> Option<u64>
```

Returns the `u64` hash of the given [enum](trait.Enum.html "trait bevy::reflect::enums::Enum").