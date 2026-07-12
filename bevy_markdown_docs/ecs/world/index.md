[bevy](../../index.html)::[ecs](../index.html)

# Module world 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#57)

Defines the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and APIs for accessing it directly.

## Modules

[error](error/index.html "mod bevy::ecs::world::error")

Contains error types returned by bevy’s schedule.

[reflect](reflect/index.html "mod bevy::ecs::world::reflect")`bevy_reflect`

Provides additional functionality for [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") when the `bevy_reflect` feature is enabled.

[unsafe\_world\_cell](unsafe_world_cell/index.html "mod bevy::ecs::world::unsafe_world_cell")

Contains types that allow disjoint mutable access to a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Structs

[CommandQueue](struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")

Densely and efficiently stores a queue of heterogenous types implementing [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").

[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")

A [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") reference that disallows structural ECS changes. This includes initializing resources, registering components or spawning entities.

[EntityFetcher](struct.EntityFetcher.html "struct bevy::ecs::world::EntityFetcher")

Provides a safe interface for non-structural access to the entities in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[EntityMut](struct.EntityMut.html "struct bevy::ecs::world::EntityMut")

Provides mutable access to a single entity and all of its components.

[EntityMutExcept](struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")

Provides mutable access to all components of an entity, with the exception of an explicit set.

[EntityRef](struct.EntityRef.html "struct bevy::ecs::world::EntityRef")

A read-only reference to a particular [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and all of its components.

[EntityRefExcept](struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")

Provides read-only access to a single entity and all its components, save for an explicitly-enumerated set.

[EntityWorldMut](struct.EntityWorldMut.html "struct bevy::ecs::world::EntityWorldMut")

A mutable reference to a particular [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), and the entire world.

[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")

Provides mutable access to a single entity and some of its components defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")

Provides read-only access to a single entity and some of its components defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredResources](struct.FilteredResources.html "struct bevy::ecs::world::FilteredResources")

Provides read-only access to a set of [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredResourcesBuilder](struct.FilteredResourcesBuilder.html "struct bevy::ecs::world::FilteredResourcesBuilder")

Builder struct to define the access for a [`FilteredResources`](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources").

[FilteredResourcesMut](struct.FilteredResourcesMut.html "struct bevy::ecs::world::FilteredResourcesMut")

Provides mutable access to a set of [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredResourcesMutBuilder](struct.FilteredResourcesMutBuilder.html "struct bevy::ecs::world::FilteredResourcesMutBuilder")

Builder struct to define the access for a [`FilteredResourcesMut`](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut").

[Mut](struct.Mut.html "struct bevy::ecs::world::Mut")

Unique mutable borrow of an entity’s component or of a resource.

[OccupiedComponentEntry](struct.OccupiedComponentEntry.html "struct bevy::ecs::world::OccupiedComponentEntry")

A view into an occupied entry in a [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut"). It is part of the [`OccupiedComponentEntry`](struct.OccupiedComponentEntry.html "struct bevy::ecs::world::OccupiedComponentEntry") enum.

[Ref](struct.Ref.html "struct bevy::ecs::world::Ref")

Shared borrow of an entity’s component with access to change detection. Similar to [`Mut`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut") but is immutable and so doesn’t require unique access.

[SpawnBatchIter](struct.SpawnBatchIter.html "struct bevy::ecs::world::SpawnBatchIter")

An iterator that spawns a series of entities and returns the [ID](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of each spawned entity.

[UnsafeFilteredEntityMut](struct.UnsafeFilteredEntityMut.html "struct bevy::ecs::world::UnsafeFilteredEntityMut")

Variant of [`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") that can be used to create copies of a [`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut"), as long as the user ensures that these won’t cause aliasing violations.

[VacantComponentEntry](struct.VacantComponentEntry.html "struct bevy::ecs::world::VacantComponentEntry")

A view into a vacant entry in a [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut"). It is part of the [`ComponentEntry`](enum.ComponentEntry.html "enum bevy::ecs::world::ComponentEntry") enum.

[World](struct.World.html "struct bevy::ecs::world::World")

Stores and exposes operations on [entities](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [components](../../prelude/trait.Component.html "trait bevy::prelude::Component"), resources, and their associated metadata.

[WorldId](struct.WorldId.html "struct bevy::ecs::world::WorldId")

A unique identifier for a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Enums

[ComponentEntry](enum.ComponentEntry.html "enum bevy::ecs::world::ComponentEntry")

A view into a single entity and component in a world, which may either be vacant or occupied.

[TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")

Error type returned by [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom") conversions from filtered entity types ([`FilteredEntityRef`](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")/[`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")) to full-access entity types ([`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")/[`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")).

## Constants

[CHECK\_TICK\_THRESHOLD](constant.CHECK_TICK_THRESHOLD.html "constant bevy::ecs::world::CHECK_TICK_THRESHOLD")

The (arbitrarily chosen) minimum number of world tick increments between `check_tick` scans.

## Traits

[DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")

Types that can be used to fetch components from an entity dynamically by [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

[FromWorld](trait.FromWorld.html "trait bevy::ecs::world::FromWorld")

Creates an instance of the type this trait is implemented for using data from the supplied [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")

Types that can be used to fetch [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") references from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Derive Macros

[FromWorld](derive.FromWorld.html "derive bevy::ecs::world::FromWorld")

Implement the `FromWorld` trait.