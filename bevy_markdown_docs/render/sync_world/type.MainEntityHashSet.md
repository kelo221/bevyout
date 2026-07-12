[bevy](../../index.html)::[render](../index.html)::[sync\_world](index.html)

# Type Alias MainEntityHashSet 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#187)

```rust
pub type MainEntityHashSet = HashSet<MainEntity, EntityHash>;
```

A [`HashSet`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing with a [`MainEntity`](struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")..

## Aliased Type

```rust
pub struct MainEntityHashSet(/* private fields */);
```