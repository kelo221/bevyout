[bevy](../../index.html)::[ecs](../index.html)

# Module query 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#46)

Contains APIs for retrieving component data from the world.

## Structs

[Access](struct.Access.html "struct bevy::ecs::query::Access")

Tracks read and write access to specific elements in a collection.

[AccessConflictError](struct.AccessConflictError.html "struct bevy::ecs::query::AccessConflictError")

Error returned from [`EcsAccessType::is_compatible`](enum.EcsAccessType.html#method.is_compatible "method bevy::ecs::query::EcsAccessType::is_compatible")

[AccessFilters](struct.AccessFilters.html "struct bevy::ecs::query::AccessFilters")

A clause in disjunctive normal form that filters entities by their components. An [`AccessFilters`](struct.AccessFilters.html "struct bevy::ecs::query::AccessFilters") matches entities that have _all_ the components in the `with` filters and _none_ of the components in the `without` filters.

[Added](struct.Added.html "struct bevy::ecs::query::Added")

A filter on a component that only retains results the first time after they have been added.

[Allow](struct.Allow.html "struct bevy::ecs::query::Allow")

Allows a query to contain entities with the component `T`, bypassing [`DefaultQueryFilters`](../entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters").

[AnyOf](struct.AnyOf.html "struct bevy::ecs::query::AnyOf")

The `AnyOf` query parameter fetches entities with any of the component types included in T.

[Changed](struct.Changed.html "struct bevy::ecs::query::Changed")

A filter on a component that only retains results the first time after they have been added or mutably dereferenced.

[ComponentIdIter](struct.ComponentIdIter.html "struct bevy::ecs::query::ComponentIdIter")

An iterator of [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

[ComponentIdSet](struct.ComponentIdSet.html "struct bevy::ecs::query::ComponentIdSet")

A set of [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

[FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess")

An [`Access`](struct.Access.html "struct bevy::ecs::query::Access") that has been filtered to include and exclude certain combinations of elements.

[FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet")

A collection of [`FilteredAccess`](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess") instances.

[Has](struct.Has.html "struct bevy::ecs::query::Has")

Returns a bool that describes if an entity has the component `T`.

[NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")

A helper type for accessing a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") within a [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData").

[Or](struct.Or.html "struct bevy::ecs::query::Or")

A filter that tests if any of the given filters apply.

[QueryBuilder](struct.QueryBuilder.html "struct bevy::ecs::query::QueryBuilder")

Builder struct to create [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") instances at runtime.

[QueryCombinationIter](struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")

An iterator over `K`\-sized combinations of query items without repetition.

[QueryContiguousIter](struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")

Iterator for contiguous chunks of memory

[QueryIter](struct.QueryIter.html "struct bevy::ecs::query::QueryIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over query results of a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

[QueryManyIter](struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items generated from an iterator of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s.

[QueryManyUniqueIter](struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items generated from an iterator of unique [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s.

[QueryNotDenseError](struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")

An error that occurs when creating a contiguous iterator from a non-dense [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") or [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") via [`contiguous_iter`](../../prelude/struct.Query.html#method.contiguous_iter "method bevy::prelude::Query::contiguous_iter") or [`contiguous_iter_mut`](../../prelude/struct.Query.html#method.contiguous_iter_mut "method bevy::prelude::Query::contiguous_iter_mut").

[QueryParIter](struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")

A parallel iterator over query results of a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

[QueryParManyIter](struct.QueryParManyIter.html "struct bevy::ecs::query::QueryParManyIter")

A parallel iterator over the unique query items generated from an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") list.

[QueryParManyUniqueIter](struct.QueryParManyUniqueIter.html "struct bevy::ecs::query::QueryParManyUniqueIter")

A parallel iterator over the unique query items generated from an [`EntitySet`](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

[QuerySortedIter](struct.QuerySortedIter.html "struct bevy::ecs::query::QuerySortedIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over sorted query results of a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

[QuerySortedManyIter](struct.QuerySortedManyIter.html "struct bevy::ecs::query::QuerySortedManyIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over sorted query results of a [`QueryManyIter`](struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter").

[QueryState](struct.QueryState.html "struct bevy::ecs::query::QueryState")

Provides scoped access to a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") state according to a given [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") and [`QueryFilter`](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[ReadFetch](struct.ReadFetch.html "struct bevy::ecs::query::ReadFetch")

The [`WorldQuery::Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch") type for `& T`.

[SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

The `SpawnDetails` query parameter fetches the [`Tick`](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick") the entity was spawned at.

[Spawned](struct.Spawned.html "struct bevy::ecs::query::Spawned")

A filter that only retains results the first time after the entity has been spawned.

[UnboundedAccessError](struct.UnboundedAccessError.html "struct bevy::ecs::query::UnboundedAccessError")

Error returned when attempting to iterate over items included in an [`Access`](struct.Access.html "struct bevy::ecs::query::Access") if the access excludes items rather than including them.

[With](struct.With.html "struct bevy::ecs::query::With")

Filter that selects entities with a component `T`.

[Without](struct.Without.html "struct bevy::ecs::query::Without")

Filter that selects entities without a component `T`.

[WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")

The [`WorldQuery::Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch") type for `&mut T`.

## Enums

[AccessConflicts](enum.AccessConflicts.html "enum bevy::ecs::query::AccessConflicts")

Records how two accesses conflict with each other

[ComponentAccessKind](enum.ComponentAccessKind.html "enum bevy::ecs::query::ComponentAccessKind")

Describes the level of access for a particular component as defined in an [`Access`](struct.Access.html "struct bevy::ecs::query::Access").

[EcsAccessLevel](enum.EcsAccessLevel.html "enum bevy::ecs::query::EcsAccessLevel")

The way the data will be accessed and whether we take access on all the components on an entity or just one component.

[EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")

The data storage type that is being accessed.

[QueryAccessError](enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")

Error returned from [`has_conflicts`](fn.has_conflicts.html "fn bevy::ecs::query::has_conflicts").

[QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")

An error that occurs when retrieving a specific [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")’s query result from [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") or [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

[QuerySingleError](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")

An error that occurs when evaluating a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") or [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") as a single expected result via [`single`](../../prelude/struct.Query.html#method.single "method bevy::prelude::Query::single") or [`single_mut`](../../prelude/struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut").

## Traits

[ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter")

A marker trait to indicate that the filter works at an archetype level.

[ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData")

A marker trait to indicate that the query data filters at an archetype level.

[ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") which allows getting a direct access to contiguous chunks of components’ values, which may be used to apply simd-operations.

[IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData")

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") for which instances may be alive for different entities concurrently.

[QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")

Types that can be fetched from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") using a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

[QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter")

Types that filter the results of a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

[ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that is read only.

[ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData")

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that does not borrow from its [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

[SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData")

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that only accesses data from the current entity, the one passed to [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch").

[WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")

Types that can be used as parameters in a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query"). Types that implement this should also implement either [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") or [`QueryFilter`](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter")

## Functions

[has\_conflicts](fn.has_conflicts.html "fn bevy::ecs::query::has_conflicts")

Check if `Q` has any internal conflicts.

## Type Aliases

[QueryItem](type.QueryItem.html "type bevy::ecs::query::QueryItem")

The item type returned when a [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") is iterated over

[ROQueryItem](type.ROQueryItem.html "type bevy::ecs::query::ROQueryItem")

The read-only variant of the item type returned when a [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") is iterated over immutably

## Derive Macros

[QueryData](derive.QueryData.html "derive bevy::ecs::query::QueryData")

Implement `QueryData` to use a struct as a data parameter in a query

[QueryFilter](derive.QueryFilter.html "derive bevy::ecs::query::QueryFilter")

Implement `QueryFilter` to use a struct as a filter parameter in a query