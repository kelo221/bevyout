[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Type Alias ROQueryItem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#478)

```rust
pub type ROQueryItem<'w, 's, D> = <<D as QueryData>::ReadOnly as QueryData>::Item<'w, 's>;
```

The read-only variant of the item type returned when a [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") is iterated over immutably