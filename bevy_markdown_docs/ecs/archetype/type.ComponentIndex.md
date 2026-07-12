[bevy](../../index.html)::[ecs](../index.html)::[archetype](index.html)

# Type Alias ComponentIndex 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/archetype.rs.html#765)

```rust
pub type ComponentIndex = HashMap<ComponentId, HashMap<ArchetypeId, ArchetypeRecord>>;
```

Maps a [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") to the list of [`Archetypes`](%5B%60Archetype%60%5D) that contain the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), along with an [`ArchetypeRecord`](struct.ArchetypeRecord.html "struct bevy::ecs::archetype::ArchetypeRecord") which contains some metadata about how the component is stored in the archetype.

## Aliased Type

```rust
pub struct ComponentIndex(/* private fields */);
```