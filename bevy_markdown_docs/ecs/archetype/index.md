[bevy](../../index.html)::[ecs](../index.html)

# Module archetype 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#29)

Types for defining [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype")s, collections of entities that have the same set of components.

An archetype uniquely describes a group of entities that share the same components: a world only has one archetype for each unique combination of components, and all entities that have those components and only those components belong to that archetype.

Archetypes are not to be confused with [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table")s. Each archetype stores its table components in one table, and each archetype uniquely points to one table, but multiple archetypes may store their table components in the same table. These archetypes differ only by the [`SparseSet`](../storage/struct.SparseSet.html "struct bevy::ecs::storage::SparseSet") components.

Like tables, archetypes can be created but are never cleaned up. Empty archetypes are not removed, and persist until the world is dropped.

Archetypes can be fetched from [`Archetypes`](struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes"), which is accessible via [`World::archetypes`](../../prelude/struct.World.html#method.archetypes "method bevy::prelude::World::archetypes").

## Structs

[Archetype](struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

Metadata for a single archetype within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[ArchetypeEntity](struct.ArchetypeEntity.html "struct bevy::ecs::archetype::ArchetypeEntity")

Metadata about an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") in a [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype").

[ArchetypeGeneration](struct.ArchetypeGeneration.html "struct bevy::ecs::archetype::ArchetypeGeneration")

The next [`ArchetypeId`](struct.ArchetypeId.html "struct bevy::ecs::archetype::ArchetypeId") in an [`Archetypes`](struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes") collection.

[ArchetypeId](struct.ArchetypeId.html "struct bevy::ecs::archetype::ArchetypeId")

An opaque unique ID for a single [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype") within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[ArchetypeRecord](struct.ArchetypeRecord.html "struct bevy::ecs::archetype::ArchetypeRecord")

Metadata about how a component is stored in an [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype").

[ArchetypeRow](struct.ArchetypeRow.html "struct bevy::ecs::archetype::ArchetypeRow")

An opaque location within a [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype").

[Archetypes](struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

The backing store of all [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype")s within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Edges](struct.Edges.html "struct bevy::ecs::archetype::Edges")

Archetypes and bundles form a graph. Adding or removing a bundle moves an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a new [`Archetype`](struct.Archetype.html "struct bevy::ecs::archetype::Archetype").

## Type Aliases

[ComponentIndex](type.ComponentIndex.html "type bevy::ecs::archetype::ComponentIndex")

Maps a [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") to the list of [`Archetypes`](%5B%60Archetype%60%5D) that contain the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), along with an [`ArchetypeRecord`](struct.ArchetypeRecord.html "struct bevy::ecs::archetype::ArchetypeRecord") which contains some metadata about how the component is stored in the archetype.