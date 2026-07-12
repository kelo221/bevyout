[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Type Alias ChildSpawnerCommands 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#267)

```rust
pub type ChildSpawnerCommands<'w> = RelatedSpawnerCommands<'w, ChildOf>;
```

A type alias over [`RelatedSpawnerCommands`](../relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands") used to spawn child entities containing a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

## Aliased Type

```rust
pub struct ChildSpawnerCommands<'w> { /* private fields */ }
```