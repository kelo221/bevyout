[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Function has\_conflicts 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/access_iter.rs.html#18)

```rust
pub fn has_conflicts<Q>(components: &Components) -> Result<(), QueryAccessError>where
    Q: QueryData,
```

Check if `Q` has any internal conflicts.