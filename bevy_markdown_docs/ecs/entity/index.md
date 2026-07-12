[bevy](../../index.html)::[ecs](../index.html)

# Module entity 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#34)

This module contains all entity types and utilities for interacting with their ids.

## What is an Entity?

The ecs [docs](../index.html "mod bevy::ecs") give an overview of what entities are and generally how to use them. These docs provide more detail into how they actually work. In these docs [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and “entity id” are synonymous and refer to the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") type, which identifies an entity. The term “entity” used on its own refers to the “thing”/“game object” that id references.

## In this Module

This module contains four main things:

*   Core ECS types like [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`Entities`](struct.Entities.html "struct bevy::ecs::entity::Entities"), and [`EntityAllocator`](struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator").
*   Utilities for [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ids like [`MapEntities`](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"), [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash"), and [`UniqueEntityVec`](type.UniqueEntityVec.html "type bevy::ecs::entity::UniqueEntityVec").
*   Helpers for entity tasks like [`EntityCloner`](struct.EntityCloner.html "struct bevy::ecs::entity::EntityCloner").
*   Entity-related error types like [`EntityNotSpawnedError`](enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError").

## Entity Life Cycle

Entities have life cycles. They are created, used for a while, and eventually destroyed. Let’s start from the top:

**Spawn:** An entity is created. In bevy, this is called spawning. Most commonly, this is done through [`World::spawn`](../../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn") or [`Commands::spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn"). This creates a fresh entity in the world and returns its [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id, which can be used to interact with the entity it identifies. These methods initialize the entity with a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), a group of [components](../../prelude/trait.Component.html "trait bevy::prelude::Component") that it starts with. It is also possible to use [`World::spawn_empty`](../../prelude/struct.World.html#method.spawn_empty "method bevy::prelude::World::spawn_empty") or [`Commands::spawn_empty`](../../prelude/struct.Commands.html#method.spawn_empty "method bevy::prelude::Commands::spawn_empty"), which are similar but do not add any components to the entity. In either case, the returned [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id is used to further interact with the entity.

**Update:** Once an entity is created, you will need its [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id to perform further actions on it. This can be done through [`World::entity_mut`](../../prelude/struct.World.html#method.entity_mut "method bevy::prelude::World::entity_mut") and [`Commands::entity`](../../prelude/struct.Commands.html#method.entity "method bevy::prelude::Commands::entity"). Even if you don’t store the id, you can still find the entity you spawned by searching for it in a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query"). Queries are also the primary way of interacting with an entity’s components. You can use [`EntityWorldMut::remove`](../../prelude/struct.EntityWorldMut.html#method.remove "method bevy::prelude::EntityWorldMut::remove") and [`EntityCommands::remove`](../../prelude/struct.EntityCommands.html#method.remove "method bevy::prelude::EntityCommands::remove") to remove components, and you can use [`EntityWorldMut::insert`](../../prelude/struct.EntityWorldMut.html#method.insert "method bevy::prelude::EntityWorldMut::insert") and [`EntityCommands::insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert") to insert more components. Be aware that each entity can only have 0 or 1 values for each kind of component, so inserting a bundle may overwrite existing component values. This can also be further configured based on the insert method.

**Despawn:** Despawn an entity when it is no longer needed. This destroys it and all its components. The entity is no longer reachable through the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands"), or [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query")s. Note that this means an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id may refer to an entity that has since been despawned! Not all [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ids refer to active entities. If an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id is used when its entity has been despawned, an [`EntityNotSpawnedError`](enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError") is emitted. Any [`System`](../system/index.html "mod bevy::ecs::system") could despawn any entity; even if you never share its id, it could still be despawned unexpectedly. Your code should do its best to handle these errors gracefully.

In short:

*   Entities are spawned through methods like [`World::spawn`](../../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn"), which return an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id for the new entity.
*   Once spawned, they can be accessed and modified through [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query")s and other apis.
*   You can get the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id of an entity through [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query")s, so losing an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id is not a problem.
*   Entities can have components inserted and removed via [`World::entity_mut`](../../prelude/struct.World.html#method.entity_mut "method bevy::prelude::World::entity_mut") and [`Commands::entity`](../../prelude/struct.Commands.html#method.entity "method bevy::prelude::Commands::entity").
*   Entities are eventually despawned, destroying the entity and causing its [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") id to no longer refer to an entity.
*   Not all [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ids point to actual entities, which makes many entity methods fallible.

## [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") Allocation

Entity spawning is actually done in two stages:

1.  Allocate: We generate a new valid / unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").
2.  Spawn: We make the entity “exist” in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). It will show up in queries, it can have components, etc.

The reason for this split is that we need to be able to _allocate_ entity ids concurrently, whereas spawning requires unique (non-concurrent) access to the world.

An [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") therefore goes through the following lifecycle:

1.  Unallocated (and “valid”): Only the allocator has any knowledge of this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), but it _could_ be spawned, theoretically.
2.  Allocated (and “valid”): The allocator has handed out the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), but it is not yet spawned.
3.  Spawned: The entity now “exists” in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). It will show up in queries, it can have components, etc.
4.  Despawned: The entity no longer “exist” in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").
5.  Freed (and “invalid”): The [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") is returned to the allocator. The [`Entity::generation`](../../prelude/struct.Entity.html#method.generation "method bevy::prelude::Entity::generation") is bumped, which makes all existing [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") references with the previous generation “invalid”.

Note that by default, most spawn and despawn APIs handle the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") allocation and freeing process for developers.

## Modules

[hash\_map](hash_map/index.html "mod bevy::ecs::entity::hash_map")

Contains the [`EntityHashMap`](struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") type, a [`HashMap`](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[hash\_set](hash_set/index.html "mod bevy::ecs::entity::hash_set")

Contains the [`EntityHashSet`](struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") type, a [`HashSet`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[index\_map](index_map/index.html "mod bevy::ecs::entity::index_map")

Contains the [`EntityIndexMap`](struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap") type, an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[index\_set](index_set/index.html "mod bevy::ecs::entity::index_set")

Contains the [`EntityIndexSet`](struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet") type, a [`IndexSet`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[unique\_array](unique_array/index.html "mod bevy::ecs::entity::unique_array")

A wrapper around entity arrays with a uniqueness invariant.

[unique\_slice](unique_slice/index.html "mod bevy::ecs::entity::unique_slice")

A wrapper around entity slices with a uniqueness invariant.

[unique\_vec](unique_vec/index.html "mod bevy::ecs::entity::unique_vec")

A wrapper around entity [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")s with a uniqueness invariant.

## Structs

[AllocEntitiesIterator](struct.AllocEntitiesIterator.html "struct bevy::ecs::entity::AllocEntitiesIterator")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") returning a sequence of unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") values from [`Entities`](struct.Entities.html "struct bevy::ecs::entity::Entities"). Dropping this will still retain the entities as allocated; this is effectively a leak. To prevent this, ensure the iterator is exhausted before dropping it.

[ComponentCloneCtx](struct.ComponentCloneCtx.html "struct bevy::ecs::entity::ComponentCloneCtx")

Context for component clone handlers.

[Entities](struct.Entities.html "struct bevy::ecs::entity::Entities")

[`Entities`](struct.Entities.html "struct bevy::ecs::entity::Entities") tracks all known [`EntityIndex`](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")s and their metadata. This is like a base table of information all entities have.

[Entity](struct.Entity.html "struct bevy::ecs::entity::Entity")

Unique identifier for an entity in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). Note that this is just an id, not the entity itself. Further, the entity this id refers to may no longer exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). For more information about entities, their ids, and how to use them, see the module [docs](index.html "mod bevy::ecs::entity").

[EntityAllocator](struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

Allocates [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ids uniquely. This is used in [`World::spawn_at`](../../prelude/struct.World.html#method.spawn_at "method bevy::prelude::World::spawn_at") and [`World::despawn_no_free`](../../prelude/struct.World.html#method.despawn_no_free "method bevy::prelude::World::despawn_no_free") to track entity ids no longer in use. Allocating is fully concurrent and can be done from multiple threads.

[EntityCloner](struct.EntityCloner.html "struct bevy::ecs::entity::EntityCloner")

A configuration determining how to clone entities. This can be built using [`EntityCloner::build_opt_out`](struct.EntityCloner.html#method.build_opt_out "associated function bevy::ecs::entity::EntityCloner::build_opt_out")/ [`opt_in`](struct.EntityCloner.html#method.build_opt_in "associated function bevy::ecs::entity::EntityCloner::build_opt_in"), which returns an [`EntityClonerBuilder`](struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

[EntityClonerBuilder](struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")

A builder for configuring [`EntityCloner`](struct.EntityCloner.html "struct bevy::ecs::entity::EntityCloner"). See [`EntityCloner`](struct.EntityCloner.html "struct bevy::ecs::entity::EntityCloner") for more information.

[EntityGeneration](struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration")

This tracks different versions or generations of an [`EntityIndex`](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex"). Importantly, this can wrap, meaning each generation is not necessarily unique per [`EntityIndex`](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex").

[EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

A [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") that results in a [`EntityHasher`](struct.EntityHasher.html "struct bevy::ecs::entity::EntityHasher").

[EntityHashMap](struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")

A [`HashMap`](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[EntityHashSet](struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

A [`HashSet`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[EntityHasher](struct.EntityHasher.html "struct bevy::ecs::entity::EntityHasher")

A very fast hash that is only designed to work on generational indices like [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"). It will panic if attempting to hash a type containing non-u64 fields.

[EntityIndex](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

This represents the index of an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") within the [`Entities`](struct.Entities.html "struct bevy::ecs::entity::Entities") array. This is a lighter weight version of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityIndexMap](struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")

A [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[EntityIndexSet](struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

An [`IndexSet`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet") pre-configured to use [`EntityHash`](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

[EntityLocation](struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

A location of an entity in an archetype.

[EntityValidButNotSpawnedError](struct.EntityValidButNotSpawnedError.html "struct bevy::ecs::entity::EntityValidButNotSpawnedError")

An error that occurs when a specified [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") is certain to be valid and is expected to be spawned but is not spawned yet. This includes when an [`EntityIndex`](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex") is requested but is not spawned, since each index always corresponds to exactly one valid entity.

[InvalidEntityError](struct.InvalidEntityError.html "struct bevy::ecs::entity::InvalidEntityError")

An error that occurs when a specified [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") does not exist in the entity id space. See [module](index.html "mod bevy::ecs::entity") docs for more about entity validity.

[OptIn](struct.OptIn.html "struct bevy::ecs::entity::OptIn")

Generic for [`EntityClonerBuilder`](struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") that makes the cloner try to clone every component that was explicitly allowed from the source entity, for example by using the [`allow`](struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") method.

[OptOut](struct.OptOut.html "struct bevy::ecs::entity::OptOut")

Generic for [`EntityClonerBuilder`](struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") that makes the cloner try to clone every component from the source entity except for components that were explicitly denied, for example by using the [`deny`](struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") method.

[RemoteAllocator](struct.RemoteAllocator.html "struct bevy::ecs::entity::RemoteAllocator")

This is a stripped down entity allocator that operates on fewer assumptions than [`EntityAllocator`](struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator"). As a result, using this will be slower than the main allocator but this offers additional freedoms. In particular, this type is fully owned, allowing you to allocate entities for a world without locking or holding reference to the world. This is especially useful in async contexts.

[SceneEntityMapper](struct.SceneEntityMapper.html "struct bevy::ecs::entity::SceneEntityMapper")

A wrapper for [`EntityHashMap<Entity>`](struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap"), augmenting it with the ability to allocate new [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") references in a destination world. These newly allocated references are guaranteed to never point to any living entity in that world.

[SourceComponent](struct.SourceComponent.html "struct bevy::ecs::entity::SourceComponent")

Provides read access to the source component (the component being cloned) in a [`ComponentCloneFn`](../component/type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn").

[UniqueEntityEquivalentArray](struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")

An array that contains only unique entities.

[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")

A slice that contains only unique entities.

[UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")

A `Vec` that contains only unique entities.

[UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")

An iterator that yields unique entities.

## Enums

[EntityNotSpawnedError](enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")

An error that occurs when a specified [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") is expected to be valid and spawned but is not. Represents an error of either [`InvalidEntityError`](struct.InvalidEntityError.html "struct bevy::ecs::entity::InvalidEntityError") (when the entity is invalid) or [`EntityValidButNotSpawnedError`](struct.EntityValidButNotSpawnedError.html "struct bevy::ecs::entity::EntityValidButNotSpawnedError") (when the [`EntityGeneration`](struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration") is correct but the [`EntityIndex`](struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex") is not spawned).

[SpawnError](enum.SpawnError.html "enum bevy::ecs::entity::SpawnError")

An error that occurs when a specified [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") can not be spawned.

## Traits

[ContainsEntity](trait.ContainsEntity.html "trait bevy::ecs::entity::ContainsEntity")

A trait for types that contain an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent")

A trait for types that represent an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityMapper](trait.EntityMapper.html "trait bevy::ecs::entity::EntityMapper")

An implementor of this trait knows how to map an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") into another [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntitySet](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet")

A set of unique entities.

[EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator")

An iterator over a set of unique entities.

[FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")

Conversion from an `EntitySetIterator`.

[MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities")

Operation to map all contained [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") fields in a type to new values.

## Type Aliases

[UniqueEntityArray](type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray")

An array that contains only unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[UniqueEntitySlice](type.UniqueEntitySlice.html "type bevy::ecs::entity::UniqueEntitySlice")

A slice that contains only unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[UniqueEntityVec](type.UniqueEntityVec.html "type bevy::ecs::entity::UniqueEntityVec")

A `Vec` that contains only unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

## Derive Macros

[MapEntities](derive.MapEntities.html "derive bevy::ecs::entity::MapEntities")

Implement the `MapEntities` trait.