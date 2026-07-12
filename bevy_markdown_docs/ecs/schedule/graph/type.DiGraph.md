[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[graph](index.html)

# Type Alias DiGraph 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#52)

```rust
pub type DiGraph<N, S = FixedHasher> = Graph<true, N, S>;
```

A `Graph` with directed edges of some [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") `N`.

For example, an edge from _1_ to _2_ is distinct from an edge from _2_ to _1_.

## Aliased Type

```rust
pub struct DiGraph<N, S = FixedHasher> { /* private fields */ }
```