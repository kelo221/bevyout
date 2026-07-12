[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Struct QueryState 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#79)

```rust
pub struct QueryState<D, F = ()>where
    D: QueryData,
    F: QueryFilter,{ /* private fields */ }
```

Provides scoped access to a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") state according to a given [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") and [`QueryFilter`](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

This data is cached between system runs, and is used to:

*   store metadata about which [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table") or [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype") are matched by the query. “Matched” means that the query will iterate over the data in the matched table/archetype.
*   cache the [`State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") needed to compute the [`Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch") struct used to retrieve data from a specific [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table") or [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")
*   build iterators that can iterate over the query results

## Safety

If the query is not read-only, then before calling any other methods on a new `QueryState` other than [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes"), [`QueryState::update_archetypes_unsafe_world_cell`](../../prelude/struct.QueryState.html#method.update_archetypes_unsafe_world_cell "method bevy::prelude::QueryState::update_archetypes_unsafe_world_cell"), [`Self::init_access`](../../prelude/struct.QueryState.html#method.init_access "method bevy::prelude::QueryState::init_access") must be called.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#122)

### impl<D, F> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#124)

#### pub fn [as\_readonly](#method.as_readonly)(&self) -> &[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Converts this `QueryState` reference to a `QueryState` that does not access anything mutably.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#160)

#### pub fn [component\_access](#method.component_access)(&self) -> &[FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess")

Returns the components accessed by this query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#165)

#### pub fn [matched\_tables](#method.matched_tables)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [TableId](../storage/struct.TableId.html "struct bevy::ecs::storage::TableId")\>

Returns the tables matched by this query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#170)

#### pub fn [matched\_archetypes](#method.matched_archetypes)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ArchetypeId](../archetype/struct.ArchetypeId.html "struct bevy::ecs::archetype::ArchetypeId")\>

Returns the archetypes matched by this query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#185)

#### pub unsafe fn [new\_unchecked](#method.new_unchecked)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Creates a new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") from a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and inherits the result of `world.id()`.

Unlike [`QueryState::new`](../../prelude/struct.QueryState.html#method.new "associated function bevy::prelude::QueryState::new"), this does not check access of nested queries, so [`Self::init_access`](../../prelude/struct.QueryState.html#method.init_access "method bevy::prelude::QueryState::init_access") must be called before querying using this state or returning it to safe code.

##### Safety

If the query is not read-only, then before calling any other methods on the returned `QueryState` other than [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes"), [`QueryState::update_archetypes_unsafe_world_cell`](../../prelude/struct.QueryState.html#method.update_archetypes_unsafe_world_cell "method bevy::prelude::QueryState::update_archetypes_unsafe_world_cell"), [`Self::init_access`](../../prelude/struct.QueryState.html#method.init_access "method bevy::prelude::QueryState::init_access") must be called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#198-203)

#### pub fn [init\_access](#method.init_access)( &self, system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, component\_access\_set: &mut [FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

Adds all access from this query and any nested queries to the `component_access_set`. Panics if the access from this query and any nested queries conflict with each other or with any previous access.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#225)

#### pub fn [new](#method.new)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Creates a new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") from a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and inherits the result of `world.id()`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#236)

#### pub fn [try\_new](#method.try_new)(world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>>

Creates a new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") from an immutable [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") reference and inherits the result of `world.id()`.

This function may fail if, for example, the components that make up this query have not been registered into the world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#306)

#### pub fn [from\_builder](#method.from_builder)(builder: &mut [QueryBuilder](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")<'\_, D, F>) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Creates a new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") from a given [`QueryBuilder`](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder") and inherits its [`FilteredAccess`](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#354)

#### pub fn [query](#method.query)<'w, 's>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This will create read-only queries, see [`Self::query_mut`](../../prelude/struct.QueryState.html#method.query_mut "method bevy::prelude::QueryState::query_mut") for mutable queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#370)

#### pub fn [query\_manual](#method.query_manual)<'w, 's>( &'s self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This method is slightly more efficient than [`QueryState::query`](../../prelude/struct.QueryState.html#method.query "method bevy::prelude::QueryState::query") in some situations, since it does not update this instance’s internal cache. The resulting query may skip an entity that belongs to an archetype that has not been cached.

To ensure that the cache is up to date, call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") before this method. The cache is also updated in [`QueryState::new`](../../prelude/struct.QueryState.html#method.new "associated function bevy::prelude::QueryState::new"), [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get"), or any method with mutable access to `self`.

This will create read-only queries, see [`Self::query_mut`](../../prelude/struct.QueryState.html#method.query_mut "method bevy::prelude::QueryState::query_mut") for mutable queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#382)

#### pub fn [query\_mut](#method.query_mut)<'w, 's>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#395-398)

#### pub unsafe fn [query\_unchecked](#method.query_unchecked)<'w, 's>( &'s mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#420-423)

#### pub unsafe fn [query\_unchecked\_manual](#method.query_unchecked_manual)<'w, 's>( &'s self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This method is slightly more efficient than [`QueryState::query_unchecked`](../../prelude/struct.QueryState.html#method.query_unchecked "method bevy::prelude::QueryState::query_unchecked") in some situations, since it does not update this instance’s internal cache. The resulting query may skip an entity that belongs to an archetype that has not been cached.

To ensure that the cache is up to date, call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") before this method. The cache is also updated in [`QueryState::new`](../../prelude/struct.QueryState.html#method.new "associated function bevy::prelude::QueryState::new"), [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get"), or any method with mutable access to `self`.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query. This does not validate that `world.id()` matches `self.world_id`. Calling this on a `world` with a mismatched [`WorldId`](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId") is unsound.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#438-443)

#### pub unsafe fn [query\_unchecked\_with\_ticks](#method.query_unchecked_with_ticks)<'w, 's>( &'s mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#467-472)

#### pub unsafe fn [query\_unchecked\_manual\_with\_ticks](#method.query_unchecked_manual_with_ticks)<'w, 's>( &'s self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

Creates a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") from the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") and [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This method is slightly more efficient than [`QueryState::query_unchecked_with_ticks`](../../prelude/struct.QueryState.html#method.query_unchecked_with_ticks "method bevy::prelude::QueryState::query_unchecked_with_ticks") in some situations, since it does not update this instance’s internal cache. The resulting query may skip an entity that belongs to an archetype that has not been cached.

To ensure that the cache is up to date, call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") before this method. The cache is also updated in [`QueryState::new`](../../prelude/struct.QueryState.html#method.new "associated function bevy::prelude::QueryState::new"), [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get"), or any method with mutable access to `self`.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query. This does not validate that `world.id()` matches `self.world_id`. Calling this on a `world` with a mismatched [`WorldId`](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId") is unsound.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#494)

#### pub fn [is\_empty](#method.is_empty)(&self, world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if the query is empty for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), where the last change and current tick are given.

This is equivalent to `self.iter().next().is_none()`, and thus the worst case runtime will be `O(n)` where `n` is the number of _potential_ matches. This can be notably expensive for queries that rely on non-archetypal filters such as [`Added`](../../prelude/struct.Added.html "struct bevy::prelude::Added"), [`Changed`](../../prelude/struct.Changed.html "struct bevy::prelude::Changed") or [`Spawned`](struct.Spawned.html "struct bevy::ecs::query::Spawned") which must individually check each query result for a match.

##### Panics

If `world` does not match the one used to call `QueryState::new` for this instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#513)

#### pub fn [contains](#method.contains)( &self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") matches the query.

This is always guaranteed to run in `O(1)` time.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#541)

#### pub fn [update\_archetypes](#method.update_archetypes)(&mut self, world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Updates the state’s internal view of the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World")’s archetypes. If this is not called before querying data, the results may not accurately reflect what is in the `world`.

This is only required if a `manual` method (such as [`Self::get_manual`](../../prelude/struct.QueryState.html#method.get_manual "method bevy::prelude::QueryState::get_manual")) is being called, and it only needs to be called if the `world` has been structurally mutated (i.e. added/removed a component or resource). Users using non-`manual` methods such as [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get") do not need to call this as it will be automatically called for them.

If you have an [`UnsafeWorldCell`](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") instead of `&World`, consider using [`QueryState::update_archetypes_unsafe_world_cell`](../../prelude/struct.QueryState.html#method.update_archetypes_unsafe_world_cell "method bevy::prelude::QueryState::update_archetypes_unsafe_world_cell").

##### Panics

If `world` does not match the one used to call `QueryState::new` for this instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#559)

#### pub fn [update\_archetypes\_unsafe\_world\_cell](#method.update_archetypes_unsafe_world_cell)( &mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

Updates the state’s internal view of the `world`’s archetypes. If this is not called before querying data, the results may not accurately reflect what is in the `world`.

This is only required if a `manual` method (such as [`Self::get_manual`](../../prelude/struct.QueryState.html#method.get_manual "method bevy::prelude::QueryState::get_manual")) is being called, and it only needs to be called if the `world` has been structurally mutated (i.e. added/removed a component or resource). Users using non-`manual` methods such as [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get") do not need to call this as it will be automatically called for them.

##### Note

This method only accesses world metadata.

##### Panics

If `world` does not match the one used to call `QueryState::new` for this instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#622)

#### pub fn [validate\_world](#method.validate_world)(&self, world\_id: [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId"))

##### Panics

If `world_id` does not match the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") used to call `QueryState::new` for this instance.

Many unsafe query methods require the world to match for soundness. This function is the easiest way of ensuring that it matches.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#640)

#### pub unsafe fn [new\_archetype](#method.new_archetype)(&mut self, archetype: &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"))

Update the current [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") with information from the provided [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype") (if applicable, i.e. if the archetype has any intersecting [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") with the current [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")).

##### Safety

`archetype` must be from the `World` this state was initialized from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#667)

#### pub fn [matches\_component\_set](#method.matches_component_set)( &self, set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this query matches a set of components. Otherwise, returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#682-685)

#### pub fn [transmute](#method.transmute)<'a, NewD>( &self, world: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'a>>, ) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<NewD>

where NewD: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Use this to transform a [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") into a more generic [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState"). This can be useful for passing to another function that might take the more general form. See [`Query::transmute_lens`](../../prelude/struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for more details.

You should not call [`update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") on the returned [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") as the result will be unpredictable. You might end up with a mix of archetypes that only matched the original query + archetypes that only match the new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState"). Most of the safe methods on [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") internally, so this best used through a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#693-696)

#### pub fn [transmute\_filtered](#method.transmute_filtered)<'a, NewD, NewF>( &self, world: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'a>>, ) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<NewD, NewF>

where NewD: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Creates a new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") with the same underlying [`FilteredAccess`](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), matched tables and archetypes as self but with a new type signature.

Panics if `NewD` or `NewF` require accesses that this query does not have.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#787-791)

#### pub fn [join](#method.join)<'a, OtherD, NewD>( &self, world: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'a>>, other: &[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<OtherD>, ) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<NewD>

where OtherD: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), NewD: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Use this to combine two queries. The data accessed will be the intersection of archetypes included in both queries. This can be useful for accessing a subset of the entities between two queries.

You should not call `update_archetypes` on the returned `QueryState` as the result could be unpredictable. You might end up with a mix of archetypes that only matched the original query + archetypes that only match the new `QueryState`. Most of the safe methods on `QueryState` call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") internally, so this is best used through a `Query`.

###### Performance

This will have similar performance as constructing a new `QueryState` since much of internal state needs to be reconstructed. But it will be a little faster as it only needs to compare the intersection of matching archetypes rather than iterating over all archetypes.

###### Panics

Will panic if `NewD` contains accesses not in `Q` or `OtherQ`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#801-811)

#### pub fn [join\_filtered](#method.join_filtered)<'a, OtherD, OtherF, NewD, NewF>( &self, world: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'a>>, other: &[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<OtherD, OtherF>, ) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<NewD, NewF>

where OtherD: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), OtherF: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), NewD: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Use this to combine two queries. The data accessed will be the intersection of archetypes included in both queries.

###### Panics

Will panic if `NewD` or `NewF` requires accesses not in `Q` or `OtherQ`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#926-930)

#### pub fn [get](#method.get)<'w>( &mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Gets the query result for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This can only be called for read-only queries, see [`Self::get_mut`](../../prelude/struct.QueryState.html#method.get_mut "method bevy::prelude::QueryState::get_mut") for write-queries.

If you need to get multiple items at once but get borrowing errors, consider using [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") followed by multiple [`Self::get_manual`](../../prelude/struct.QueryState.html#method.get_manual "method bevy::prelude::QueryState::get_manual") calls, or making a single call with [`Self::get_many`](../../prelude/struct.QueryState.html#method.get_many "method bevy::prelude::QueryState::get_many") or [`Self::iter_many`](../../prelude/struct.QueryState.html#method.iter_many "method bevy::prelude::QueryState::iter_many").

This is always guaranteed to run in `O(1)` time.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#967-971)

#### pub fn [get\_many](#method.get_many)<'w, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the read-only query results for the given array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

Note that the unlike [`QueryState::get_many_mut`](../../prelude/struct.QueryState.html#method.get_many_mut "method bevy::prelude::QueryState::get_many_mut"), the entities passed in do not need to be unique.

##### Examples

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();
let entity_vec: Vec<Entity> = (0..3).map(|i|world.spawn(A(i)).id()).collect();
let entities: [Entity; 3] = entity_vec.try_into().unwrap();

world.spawn(A(73));

let mut query_state = world.query::<&A>();

let component_values = query_state.get_many(&world, entities).unwrap();

assert_eq!(component_values, [&A(0), &A(1), &A(2)]);

let wrong_entity = Entity::from_raw_u32(365).unwrap();

assert_eq!(match query_state.get_many(&mut world, [wrong_entity]).unwrap_err() {QueryEntityError::NotSpawned(error) => error.entity(), _ => panic!()}, wrong_entity);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1005-1009)

#### pub fn [get\_many\_unique](#method.get_many_unique)<'w, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: [UniqueEntityEquivalentArray](../entity/struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), N>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the read-only query results for the given [`UniqueEntityArray`](../entity/type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### Examples

```rust
use bevy_ecs::{prelude::*, query::QueryEntityError, entity::{EntitySetIterator, UniqueEntityArray, UniqueEntityVec}};

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();
let entity_set: UniqueEntityVec = world.spawn_batch((0..3).map(A)).collect_set();
let entity_set: UniqueEntityArray<3> = entity_set.try_into().unwrap();

world.spawn(A(73));

let mut query_state = world.query::<&A>();

let component_values = query_state.get_many_unique(&world, entity_set).unwrap();

assert_eq!(component_values, [&A(0), &A(1), &A(2)]);

let wrong_entity = Entity::from_raw_u32(365).unwrap();

assert_eq!(match query_state.get_many_unique(&mut world, UniqueEntityArray::from([wrong_entity])).unwrap_err() {QueryEntityError::NotSpawned(error) => error.entity(), _ => panic!()}, wrong_entity);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1017-1021)

#### pub fn [get\_mut](#method.get_mut)<'w>( &mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Gets the query result for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This is always guaranteed to run in `O(1)` time.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1064-1070)

#### pub fn [get\_many\_mut](#method.get_many_mut)<'w, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query results for the given array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();

let entities: Vec<Entity> = (0..3).map(|i|world.spawn(A(i)).id()).collect();
let entities: [Entity; 3] = entities.try_into().unwrap();

world.spawn(A(73));

let mut query_state = world.query::<&mut A>();

let mut mutable_component_values = query_state.get_many_mut(&mut world, entities).unwrap();

for mut a in &mut mutable_component_values {
    a.0 += 5;
}

let component_values = query_state.get_many(&world, entities).unwrap();

assert_eq!(component_values, [&A(5), &A(6), &A(7)]);

let wrong_entity = Entity::from_raw_u32(57).unwrap();
let invalid_entity = world.spawn_empty().id();

assert_eq!(match query_state.get_many(&mut world, [wrong_entity]).unwrap_err() {QueryEntityError::NotSpawned(error) => error.entity(), _ => panic!()}, wrong_entity);
assert_eq!(match query_state.get_many_mut(&mut world, [invalid_entity]).unwrap_err() {QueryEntityError::QueryDoesNotMatch(entity, _) => entity, _ => panic!()}, invalid_entity);
assert_eq!(query_state.get_many_mut(&mut world, [entities[0], entities[0]]).unwrap_err(), QueryEntityError::AliasedMutability(entities[0]));
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1112-1118)

#### pub fn [get\_many\_unique\_mut](#method.get_many_unique_mut)<'w, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: [UniqueEntityEquivalentArray](../entity/struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), N>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query results for the given [`UniqueEntityArray`](../entity/type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

```rust
use bevy_ecs::{prelude::*, query::QueryEntityError, entity::{EntitySetIterator, UniqueEntityArray, UniqueEntityVec}};

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();

let entity_set: UniqueEntityVec = world.spawn_batch((0..3).map(A)).collect_set();
let entity_set: UniqueEntityArray<3> = entity_set.try_into().unwrap();

world.spawn(A(73));

let mut query_state = world.query::<&mut A>();

let mut mutable_component_values = query_state.get_many_unique_mut(&mut world, entity_set).unwrap();

for mut a in &mut mutable_component_values {
    a.0 += 5;
}

let component_values = query_state.get_many_unique(&world, entity_set).unwrap();

assert_eq!(component_values, [&A(5), &A(6), &A(7)]);

let wrong_entity = Entity::from_raw_u32(57).unwrap();
let invalid_entity = world.spawn_empty().id();

assert_eq!(match query_state.get_many_unique(&mut world, UniqueEntityArray::from([wrong_entity])).unwrap_err() {QueryEntityError::NotSpawned(error) => error.entity(), _ => panic!()}, wrong_entity);
assert_eq!(match query_state.get_many_unique_mut(&mut world, UniqueEntityArray::from([invalid_entity])).unwrap_err() {QueryEntityError::QueryDoesNotMatch(entity, _) => entity, _ => panic!()}, invalid_entity);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1137-1141)

#### pub fn [get\_manual](#method.get_manual)<'w>( &self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Gets the query result for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This method is slightly more efficient than [`QueryState::get`](../../prelude/struct.QueryState.html#method.get "method bevy::prelude::QueryState::get") in some situations, since it does not update this instance’s internal cache. This method will return an error if `entity` belongs to an archetype that has not been cached.

To ensure that the cache is up to date, call [`QueryState::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") before this method. The cache is also updated in [`QueryState::new`](../../prelude/struct.QueryState.html#method.new "associated function bevy::prelude::QueryState::new"), `QueryState::get`, or any method with mutable access to `self`.

This can only be called for read-only queries, see [`Self::get_mut`](../../prelude/struct.QueryState.html#method.get_mut "method bevy::prelude::QueryState::get_mut") for mutable queries.

This is always guaranteed to run in `O(1)` time.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1154-1158)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<'w>( &mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QueryEntityError](enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Gets the query result for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This is always guaranteed to run in `O(1)` time.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1170)

#### pub fn [iter](#method.iter)<'w, 's>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryIter](struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This can only be called for read-only queries, see [`Self::iter_mut`](../../prelude/struct.QueryState.html#method.iter_mut "method bevy::prelude::QueryState::iter_mut") for write-queries.

If you need to iterate multiple times at once but get borrowing errors, consider using [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") followed by multiple [`Self::iter_manual`](../../prelude/struct.QueryState.html#method.iter_manual "method bevy::prelude::QueryState::iter_manual") calls.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1179)

#### pub fn [iter\_mut](#method.iter_mut)<'w, 's>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryIter](struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, D, F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([line 179](../../../src/dynamic/dynamic.rs.html#179))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1191)

#### pub fn [iter\_manual](#method.iter_manual)<'w, 's>( &'s self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryIter](struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") without updating the query’s archetypes. Archetypes must be manually updated before by using [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes").

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

This can only be called for read-only queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1220-1223)

#### pub fn [iter\_combinations](#method.iter_combinations)<'w, 's, const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryCombinationIter](struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, K> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over all possible combinations of `K` query results without repetition. This can only be called for read-only queries.

A combination is an arrangement of a collection of items where order does not matter.

`K` is the number of items that make up each subset, and the number of items returned by the iterator. `N` is the number of total entities output by query.

For example, given the list \[1, 2, 3, 4\], where `K` is 2, the combinations without repeats are \[1, 2\], \[1, 3\], \[1, 4\], \[2, 3\], \[2, 4\], \[3, 4\]. And in this case, `N` would be defined as 4 since the size of the input list is 4.

For combinations of size `K` of query taking `N` inputs, you will get:

*   if `K == N`: one combination of all query results
*   if `K < N`: all possible `K`\-sized combinations of query results, without repetition
*   if `K > N`: empty set (no `K`\-sized combinations exist)

The `iter_combinations` method does not guarantee order of iteration.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

This can only be called for read-only queries, see [`Self::iter_combinations_mut`](../../prelude/struct.QueryState.html#method.iter_combinations_mut "method bevy::prelude::QueryState::iter_combinations_mut") for write-queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1245-1250)

#### pub fn [iter\_combinations\_mut](#method.iter_combinations_mut)<'w, 's, const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryCombinationIter](struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'w, 's, D, F, K> [ⓘ](#)

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over all possible combinations of `K` query results without repetition.

A combination is an arrangement of a collection of items where order does not matter.

`K` is the number of items that make up each subset, and the number of items returned by the iterator. `N` is the number of total entities output by query.

For example, given the list \[1, 2, 3, 4\], where `K` is 2, the combinations without repeats are \[1, 2\], \[1, 3\], \[1, 4\], \[2, 3\], \[2, 4\], \[3, 4\]. And in this case, `N` would be defined as 4 since the size of the input list is 4.

For combinations of size `K` of query taking `N` inputs, you will get:

*   if `K == N`: one combination of all query results
*   if `K < N`: all possible `K`\-sized combinations of query results, without repetition
*   if `K > N`: empty set (no `K`\-sized combinations exist)

The `iter_combinations_mut` method does not guarantee order of iteration.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1267-1271)

#### pub fn [iter\_many](#method.iter_many)<'w, 's, EntityList>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyIter](struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the read-only query items generated from an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

If you need to iterate multiple times at once but get borrowing errors, consider using [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") followed by multiple [`Self::iter_many_manual`](../../prelude/struct.QueryState.html#method.iter_many_manual "method bevy::prelude::QueryState::iter_many_manual") calls.

##### See also

*   [`iter_many_mut`](../../prelude/struct.QueryState.html#method.iter_many_mut "method bevy::prelude::QueryState::iter_many_mut") to get mutable query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1290-1294)

#### pub fn [iter\_many\_manual](#method.iter_many_manual)<'w, 's, EntityList>( &'s self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyIter](struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the read-only query items generated from an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

If `world` archetypes changed since [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") was last called, this will skip entities contained in new archetypes.

This can only be called for read-only queries.

##### See also

*   [`iter_many`](../../prelude/struct.QueryState.html#method.iter_many "method bevy::prelude::QueryState::iter_many") to update archetypes.
*   [`iter_manual`](../../prelude/struct.QueryState.html#method.iter_manual "method bevy::prelude::QueryState::iter_manual") to iterate over all query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1303-1307)

#### pub fn [iter\_many\_mut](#method.iter_many_mut)<'w, 's, EntityList>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyIter](struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'w, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an iterator over the query items generated from an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1320-1324)

#### pub fn [iter\_many\_unique](#method.iter_many_unique)<'w, 's, EntityList>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyUniqueIter](struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the unique read-only query items generated from an [`EntitySet`](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

##### See also

*   [`iter_many_unique_mut`](../../prelude/struct.QueryState.html#method.iter_many_unique_mut "method bevy::prelude::QueryState::iter_many_unique_mut") to get mutable query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1344-1348)

#### pub fn [iter\_many\_unique\_manual](#method.iter_many_unique_manual)<'w, 's, EntityList>( &'s self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyUniqueIter](struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the unique read-only query items generated from an [`EntitySet`](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

If `world` archetypes changed since [`Self::update_archetypes`](../../prelude/struct.QueryState.html#method.update_archetypes "method bevy::prelude::QueryState::update_archetypes") was last called, this will skip entities contained in new archetypes.

This can only be called for read-only queries.

##### See also

*   [`iter_many_unique`](../../prelude/struct.QueryState.html#method.iter_many "method bevy::prelude::QueryState::iter_many") to update archetypes.
*   [`iter_many`](../../prelude/struct.QueryState.html#method.iter_many "method bevy::prelude::QueryState::iter_many") to iterate over a non-unique entity list.
*   [`iter_manual`](../../prelude/struct.QueryState.html#method.iter_manual "method bevy::prelude::QueryState::iter_manual") to iterate over all query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1357-1363)

#### pub fn [iter\_many\_unique\_mut](#method.iter_many_unique_mut)<'w, 's, EntityList>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entities: EntityList, ) -> [QueryManyUniqueIter](struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'w, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an iterator over the unique query items generated from an [`EntitySet`](../entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1377-1380)

#### pub unsafe fn [iter\_unchecked](#method.iter_unchecked)<'w, 's>( &'s mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, ) -> [QueryIter](struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, D, F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1397-1402)

#### pub unsafe fn [iter\_combinations\_unchecked](#method.iter_combinations_unchecked)<'w, 's, const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &'s mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, ) -> [QueryCombinationIter](struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'w, 's, D, F, K> [ⓘ](#)

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over all possible combinations of `K` query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") without repetition. This can only be called for read-only queries.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1417-1420)

#### pub fn [par\_iter](#method.par_iter)<'w, 's>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryParIter](struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Returns a parallel iterator over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This can only be called for read-only queries, see [`par_iter_mut`](../../prelude/struct.QueryState.html#method.par_iter_mut "method bevy::prelude::QueryState::par_iter_mut") for write-queries.

Note that you must use the `for_each` method to iterate over the results, see [`par_iter_mut`](../../prelude/struct.QueryState.html#method.par_iter_mut "method bevy::prelude::QueryState::par_iter_mut") for an example.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1469-1471)

#### pub fn [par\_iter\_mut](#method.par_iter_mut)<'w, 's>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [QueryParIter](struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")<'w, 's, D, F>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a parallel iterator over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This can only be called for mutable queries, see [`par_iter`](../../prelude/struct.QueryState.html#method.par_iter "method bevy::prelude::QueryState::par_iter") for read-only-queries.

##### Examples

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;

#[derive(Component, PartialEq, Debug)]
struct A(usize);


let mut world = World::new();


let mut query_state = world.query::<&mut A>();

query_state.par_iter_mut(&mut world).for_each(|mut a| {
    a.0 += 5;
});




assert_eq!(match query_state.get_many_mut(&mut world, [invalid_entity]).unwrap_err() {QueryEntityError::QueryDoesNotMatch(entity, _) => entity, _ => panic!()}, invalid_entity);
```

##### Panics

The [`ComputeTaskPool`](../../tasks/struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool") is not initialized. If using this from a query that is being initialized and run from the ECS scheduler, this should never panic.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1479-1485)

#### pub fn [contiguous\_iter](#method.contiguous_iter)<'w, 's>( &'s mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[QueryContiguousIter](struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")<'w, 's, <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>, [QueryNotDenseError](struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")\>

where <D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"), F: [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

Returns a contiguous iterator over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") or [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") with [`QueryNotDenseError`](struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError") if the query is not dense hence not contiguously iterable.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1495-1501)

#### pub fn [contiguous\_iter\_mut](#method.contiguous_iter_mut)<'w, 's>( &'s mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[QueryContiguousIter](struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")<'w, 's, D, F>, [QueryNotDenseError](struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")\>

where D: [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"), F: [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

Returns a contiguous iterator over the query results for the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") or [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") with [`QueryNotDenseError`](struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError") if the query is not dense hence not contiguously iterable.

This can only be called for mutable queries, see [`Self::contiguous_iter`](../../prelude/struct.QueryState.html#method.contiguous_iter "method bevy::prelude::QueryState::contiguous_iter") for read-only-queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1747)

### impl<D, F> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1824-1827)

#### pub fn [single](#method.single)<'w>( &mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QuerySingleError](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

Returns a single immutable query result when there is exactly one entity matching the query.

This can only be called for read-only queries, see [`single_mut`](../../prelude/struct.QueryState.html#method.single_mut "method bevy::prelude::QueryState::single_mut") for write-queries.

If the number of query results is not exactly one, a [`QuerySingleError`](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Example

Sometimes, you might want to handle the error in a specific way, generally by spawning the missing entity.

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QuerySingleError;

#[derive(Component)]
struct A(usize);

fn my_system(query: Query<&A>, mut commands: Commands) {
    match query.single() {
        Ok(a) => (), // Do something with `a`
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                commands.spawn(A(0));
            }
            QuerySingleError::MultipleEntities(_) => panic!("Multiple entities found!"),
        },
    }
}
```

However in most cases, this error can simply be handled with a graceful early return. If this is an expected failure mode, you can do this using the `let else` pattern like so:

```rust
use bevy_ecs::prelude::*;

#[derive(Component)]
struct A(usize);

fn my_system(query: Query<&A>) {
  let Ok(a) = query.single() else {
    return;
  };

  // Do something with `a`
}
```

If this is unexpected though, you should probably use the `?` operator in combination with Bevy’s error handling apparatus.

```rust
use bevy_ecs::prelude::*;

#[derive(Component)]
struct A(usize);

fn my_system(query: Query<&A>) -> Result {
 let a = query.single()?;

 // Do something with `a`
 Ok(())
}
```

This allows you to globally control how errors are handled in your application, by setting up a custom error handler. See the [`bevy_ecs::error`](../error/index.html "mod bevy::ecs::error") module docs for more information! Commonly, you might want to panic on an error during development, but log the error and continue execution in production.

Simply unwrapping the [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") also works, but should generally be reserved for tests.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1841-1846)

#### pub fn [single\_mut](#method.single_mut)<'w>( &mut self, world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QuerySingleError](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a single mutable query result when there is exactly one entity matching the query.

If the number of query results is not exactly one, a [`QuerySingleError`](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Examples

Please see [`Query::single`](../../prelude/struct.Query.html#method.single "method bevy::prelude::Query::single") for advice on handling the error.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/window/persisting\_window\_settings.rs ([line 56](../../../src/persisting_window_settings/persisting_window_settings.rs.html#56))

```rust
49fn init_window_pos(app: &mut App) {
50    let world = app.world_mut();
51    let Some(window_settings) = world.get_resource::<WindowSettings>() else {
52        return;
53    };
54    let window_settings = window_settings.clone();
55
56    let Ok(mut window) = world.query::<&mut Window>().single_mut(world) else {
57        warn!("window not found");
58        return;
59    };
60
61    if let Some(position) = window_settings.position {
62        window.position = WindowPosition::new(position);
63    }
64
65    if let Some(size) = window_settings.size {
66        window.resolution = WindowResolution::new(size.x, size.y);
67    }
68
69    window.mode = if window_settings.fullscreen {
70        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
71    } else {
72        WindowMode::Windowed
73    };
74}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1861-1866)

#### pub unsafe fn [single\_unchecked](#method.single_unchecked)<'w>( &mut self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QuerySingleError](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a query result when there is exactly one entity matching the query.

If the number of query results is not exactly one, a [`QuerySingleError`](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1885-1892)

#### pub unsafe fn [single\_unchecked\_manual](#method.single_unchecked_manual)<'w>( &self, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, [QuerySingleError](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

where D: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a query result when there is exactly one entity matching the query, where the last change and the current change tick are given.

If the number of query results is not exactly one, a [`QuerySingleError`](enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Safety

This does not check for mutable query correctness. To be safe, make sure mutable queries have unique access to the components they query. This does not validate that `world.id()` matches `self.world_id`. Calling this on a `world` with a mismatched [`WorldId`](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId") is unsound.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#103)

### impl<D, F> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#104)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#40-41)

### impl<'a, D, F> [ExclusiveSystemParam](../system/trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for &'a mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#43)

#### type [State](../system/trait.ExclusiveSystemParam.html#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#44)

#### type [Item](../system/trait.ExclusiveSystemParam.html#associatedtype.Item)<'s> = &'s mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

The item type returned when constructing this system param. See [`SystemParam::Item`](../system/trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#46)

#### fn [init](../system/trait.ExclusiveSystemParam.html#tymethod.init)( world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), \_system\_meta: &mut [SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> <&'a mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F> as [ExclusiveSystemParam](../system/trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](../system/trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")

Creates a new instance of this param’s [`State`](../system/trait.ExclusiveSystemParam.html#associatedtype.State "associated type bevy::ecs::system::ExclusiveSystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#50-53)

#### fn [get\_param](../system/trait.ExclusiveSystemParam.html#tymethod.get_param)<'s>( state: &'s mut <&'a mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F> as [ExclusiveSystemParam](../system/trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](../system/trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"), \_system\_meta: &[SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&'a mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F> as [ExclusiveSystemParam](../system/trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[Item](../system/trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>, [SystemParamValidationError](../system/struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into an [`ExclusiveSystemParamFunction`](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1901)

### impl<D, F> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[QueryBuilder](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")<'\_, D, F>> for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#1902)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [QueryBuilder](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")<'\_, D, F>) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#116)

### impl<D, F> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#117)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Creates `Self` using data from the given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#489-490)

### impl<'w, 's, D, F> [SystemParamBuilder](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>> for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#492)

#### fn [build](../../prelude/trait.SystemParamBuilder.html#tymethod.build)(self, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Registers any [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") and creates a new instance of this param’s [`State`](../system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#139)

#### fn [build\_state](../../prelude/trait.SystemParamBuilder.html#method.build_state)(self, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [SystemState](../system/struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

Create a [`SystemState`](../system/struct.SystemState.html "struct bevy::ecs::system::SystemState") from a [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder"). To create a system, call [`SystemState::build_system`](../system/struct.SystemState.html#method.build_system "method bevy::ecs::system::SystemState::build_system") on the result.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#158-164)

#### fn [build\_system](../../prelude/trait.SystemParamBuilder.html#method.build_system)<Marker, In, Out, Func>( self, func: Func, ) -> [IntoBuilderSystem](../system/struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")<Marker, In, Out, Func, Self>

where Self: 'static, Func: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker, Param = P>,

Create a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") from a [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") directly. [Read more](../../prelude/trait.SystemParamBuilder.html#method.build_system)

## Auto Trait Implementations

### impl<D, F = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

### impl<D, F = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

### impl<D, F> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where <D as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"), <F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<D, F> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

### impl<D, F> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

### impl<D, F> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where <D as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<D, F> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where <D as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"), <F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"): [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","QueryCombinationIter<'w, 's, <D as QueryData>::ReadOnly, F, K>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, const K: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">K</a>\];</div>","QueryCombinationIter<'w, 's, D, F, K>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, const K: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">K</a>\];</div>","QueryIter<'w, 's, <D as QueryData>::ReadOnly, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryIter<'w, 's, D, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyIter<'w, 's, <D as QueryData>::ReadOnly, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../entity/trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyIter<'w, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../entity/trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyUniqueIter<'w, 's, <D as QueryData>::ReadOnly, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"../entity/trait.EntitySetIterator.html\\" title=\\"trait bevy::ecs::entity::EntitySetIterator\\">EntitySetIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyUniqueIter<'w, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"../entity/trait.EntitySetIterator.html\\" title=\\"trait bevy::ecs::entity::EntitySetIterator\\">EntitySetIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}