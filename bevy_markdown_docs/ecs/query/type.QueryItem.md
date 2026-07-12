[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Type Alias QueryItem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#476)

```rust
pub type QueryItem<'w, 's, Q> = <Q as QueryData>::Item<'w, 's>;
```

The item type returned when a [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") is iterated over