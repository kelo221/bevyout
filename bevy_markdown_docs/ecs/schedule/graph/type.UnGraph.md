[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[graph](index.html)

# Type Alias UnGraph 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#46)

```rust
pub type UnGraph<N, S = FixedHasher> = Graph<false, N, S>;
```

A `Graph` with undirected edges of some [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") `N`.

For example, an edge between _1_ and _2_ is equivalent to an edge between _2_ and _1_.

## Aliased Type

```rust
pub struct UnGraph<N, S = FixedHasher> { /* private fields */ }
```