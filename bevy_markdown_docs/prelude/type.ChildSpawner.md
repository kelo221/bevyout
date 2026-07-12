[bevy](../index.html)::[prelude](index.html)

# Type Alias ChildSpawner 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#264)

```rust
pub type ChildSpawner<'w> = RelatedSpawner<'w, ChildOf>;
```

A type alias over [`RelatedSpawner`](../ecs/relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") used to spawn child entities containing a [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

## Aliased Type

```rust
pub struct ChildSpawner<'w> { /* private fields */ }
```