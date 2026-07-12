[bevy](../../index.html)::[ecs](../index.html)

# Module storage 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#53)

Storage layouts for ECS data.

This module implements the low-level collections that store data in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). These all offer minimal and often unsafe APIs, and have been made `pub` primarily for debugging and monitoring purposes.

## Fetching Storages

Each of the below data stores can be fetched via [`Storages`](struct.Storages.html "struct bevy::ecs::storage::Storages"), which can be fetched from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") via [`World::storages`](../../prelude/struct.World.html#method.storages "method bevy::prelude::World::storages"). It exposes a top level container for each class of storage within ECS:

*   [`Tables`](struct.Tables.html "struct bevy::ecs::storage::Tables") - columnar contiguous blocks of memory, optimized for fast iteration.
*   [`SparseSets`](struct.SparseSets.html "struct bevy::ecs::storage::SparseSets") - sparse `HashMap`\-like mappings from entities to components, optimized for random lookup and regular insertion/removal of components.
*   [`NonSends`](struct.NonSends.html "struct bevy::ecs::storage::NonSends") - singleton storage for non send data in the world.

## Safety

To avoid trivially unsound use of the APIs in this module, it is explicitly impossible to get a mutable reference to [`Storages`](struct.Storages.html "struct bevy::ecs::storage::Storages") from [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), and none of the types publicly expose a mutable interface.

## Structs

[Column](struct.Column.html "struct bevy::ecs::storage::Column")

A type-erased contiguous container for data of a homogeneous type.

[ComponentSparseSet](struct.ComponentSparseSet.html "struct bevy::ecs::storage::ComponentSparseSet")

A sparse data structure of [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s.

[NonSendData](struct.NonSendData.html "struct bevy::ecs::storage::NonSendData")

The type-erased backing storage and metadata for a single resource within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[NonSends](struct.NonSends.html "struct bevy::ecs::storage::NonSends")

The backing store for all [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")s stored in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[SparseSet](struct.SparseSet.html "struct bevy::ecs::storage::SparseSet")

A map from `I` to `V` that combines dense and sparse storage.

[SparseSets](struct.SparseSets.html "struct bevy::ecs::storage::SparseSets")

A collection of [`ComponentSparseSet`](struct.ComponentSparseSet.html "struct bevy::ecs::storage::ComponentSparseSet") storages, indexed by [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Storages](struct.Storages.html "struct bevy::ecs::storage::Storages")

The raw data stores of a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World")

[Table](struct.Table.html "struct bevy::ecs::storage::Table")

A column-oriented [structure-of-arrays](https://en.wikipedia.org/wiki/AoS_and_SoA#Structure_of_arrays) based storage for [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s of entities in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[TableId](struct.TableId.html "struct bevy::ecs::storage::TableId")

An opaque unique ID for a [`Table`](struct.Table.html "struct bevy::ecs::storage::Table") within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[TableRow](struct.TableRow.html "struct bevy::ecs::storage::TableRow")

An opaque newtype for rows in [`Table`](struct.Table.html "struct bevy::ecs::storage::Table")s. Specifies a single row in a specific table.

[Tables](struct.Tables.html "struct bevy::ecs::storage::Tables")

A collection of [`Table`](struct.Table.html "struct bevy::ecs::storage::Table") storages, indexed by [`TableId`](struct.TableId.html "struct bevy::ecs::storage::TableId")

## Traits

[SparseSetIndex](trait.SparseSetIndex.html "trait bevy::ecs::storage::SparseSetIndex")

Represents something that can be stored in a [`SparseSet`](struct.SparseSet.html "struct bevy::ecs::storage::SparseSet") as an integer.