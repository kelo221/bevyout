[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_hash 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#418)

```rust
pub fn list_hash<L>(list: &L) -> Option<u64>where
    L: List,
```

Returns the `u64` hash of the given [list](trait.List.html "trait bevy::reflect::list::List").