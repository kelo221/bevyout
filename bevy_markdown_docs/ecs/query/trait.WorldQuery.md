[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait WorldQuery 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#44)

```rust
pub unsafe trait WorldQuery {
    type Fetch<'w>: Clone;
    type State: Send + Sync;

    const IS_DENSE: bool;

    // Required methods
    fn shrink_fetch<'wlong, 'wshort>(
        fetch: Self::Fetch<'wlong>,
    ) -> Self::Fetch<'wshort>
       where 'wlong: 'wshort;
    unsafe fn init_fetch<'w, 's>(
        world: UnsafeWorldCell<'w>,
        state: &'s Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'w>;
    unsafe fn set_archetype<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        archetype: &'w Archetype,
        table: &'w Table,
    );
    unsafe fn set_table<'w, 's>(
        fetch: &mut Self::Fetch<'w>,
        state: &'s Self::State,
        table: &'w Table,
    );
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess);
    fn init_state(world: &mut World) -> Self::State;
    fn get_state(components: &Components) -> Option<Self::State>;
    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool;

    // Provided methods
    fn init_nested_access(
        _state: &Self::State,
        _system_name: Option<&str>,
        _component_access_set: &mut FilteredAccessSet,
        _world: UnsafeWorldCell<'_>,
    ) { ... }
    fn update_archetypes(_state: &mut Self::State, _world: UnsafeWorldCell<'_>) { ... }
}
```

Types that can be used as parameters in a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query"). Types that implement this should also implement either [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") or [`QueryFilter`](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter")

## Safety

Implementor must ensure that [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"), [`QueryData::provide_extra_access`](trait.QueryData.html#method.provide_extra_access "associated function bevy::ecs::query::QueryData::provide_extra_access"), [`matches_component_set`](trait.WorldQuery.html#tymethod.matches_component_set "associated function bevy::ecs::query::WorldQuery::matches_component_set"), [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch"), [`QueryFilter::filter_fetch`](trait.QueryFilter.html#tymethod.filter_fetch "associated function bevy::ecs::query::QueryFilter::filter_fetch") and [`init_fetch`](trait.WorldQuery.html#tymethod.init_fetch "associated function bevy::ecs::query::WorldQuery::init_fetch") obey the following:

*   For each component mutably accessed by [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch"), [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") or [`QueryData::provide_extra_access`](trait.QueryData.html#method.provide_extra_access "associated function bevy::ecs::query::QueryData::provide_extra_access") should add write access unless read or write access has already been added, in which case it should panic.
*   For each component readonly accessed by [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch") or [`QueryFilter::filter_fetch`](trait.QueryFilter.html#tymethod.filter_fetch "associated function bevy::ecs::query::QueryFilter::filter_fetch"), [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") or [`QueryData::provide_extra_access`](trait.QueryData.html#method.provide_extra_access "associated function bevy::ecs::query::QueryData::provide_extra_access") should add read access unless write access has already been added, in which case it should panic.
*   If `fetch` mutably accesses the same component twice, [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") should panic.
*   [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") may not add a `Without` filter for a component unless [`matches_component_set`](trait.WorldQuery.html#tymethod.matches_component_set "associated function bevy::ecs::query::WorldQuery::matches_component_set") always returns `false` when the component set contains that component.
*   [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") may not add a `With` filter for a component unless [`matches_component_set`](trait.WorldQuery.html#tymethod.matches_component_set "associated function bevy::ecs::query::WorldQuery::matches_component_set") always returns `false` when the component set doesn’t contain that component.
*   In cases where the query represents a disjunction (such as an `Or` filter) where each element is a valid [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"), the following rules must be obeyed:
    *   [`matches_component_set`](trait.WorldQuery.html#tymethod.matches_component_set "associated function bevy::ecs::query::WorldQuery::matches_component_set") must be a disjunction of the element’s implementations
    *   [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") must replace the filters with a disjunction of filters
    *   Each filter in that disjunction must be a conjunction of the corresponding element’s filter with the previous `access`
*   For each resource readonly accessed by [`init_fetch`](trait.WorldQuery.html#tymethod.init_fetch "associated function bevy::ecs::query::WorldQuery::init_fetch"), [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") should add read access.
*   Mutable resource access is not allowed.
*   Any access added during [`QueryData::provide_extra_access`](trait.QueryData.html#method.provide_extra_access "associated function bevy::ecs::query::QueryData::provide_extra_access") must be a subset of `available_access`, and must not conflict with any access in `access`.

When implementing [`update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"), note that `add_read` and `add_write` both also add a `With` filter, whereas `extend_access` does not change the filters.

## Required Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#88)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if (and only if) every table of every archetype matched by this fetch contains all of the matched components.

This is used to select a more efficient “table iterator” for “dense” queries. If this returns true, [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table") must be used before [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch") can be called for iterators. If this returns false, [`WorldQuery::set_archetype`](trait.WorldQuery.html#tymethod.set_archetype "associated function bevy::ecs::query::WorldQuery::set_archetype") must be used before [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch") can be called for iterators.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#46)

#### type [Fetch](#associatedtype.Fetch)<'w>: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")

Per archetype/table state retrieved by this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") to compute [`Self::Item`](trait.QueryData.html#associatedtype.Item "associated type bevy::ecs::query::QueryData::Item") for each entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#51)

#### type [State](#associatedtype.State): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

State used to construct a [`Self::Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch"). This will be cached inside [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState"), so it is best to move as much data / computation here as possible to reduce the cost of constructing [`Self::Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#54)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

This function manually implements subtyping for the query fetches.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#73-78)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, state: &'s Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

Creates a new instance of [`Self::Fetch`](trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch"), by combining data from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") with the cached [`Self::State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State"). Readonly accesses resources registered in [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access").

##### Safety

*   `state` must have been initialized (via [`WorldQuery::init_state`](trait.WorldQuery.html#tymethod.init_state "associated function bevy::ecs::query::WorldQuery::init_state")) using the same `world` passed in to this function.
*   `world` must have the **right** to access any access registered in [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") or [`WorldQuery::init_nested_access`](trait.WorldQuery.html#method.init_nested_access "associated function bevy::ecs::query::WorldQuery::init_nested_access").
*   [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") must not request conflicting access. If `Self` is `ReadOnlyQueryData` or `QueryFilter`, the access is read-only and can never conflict. Otherwise, [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access") must be called to ensure it does not panic.
*   [`WorldQuery::init_nested_access`](trait.WorldQuery.html#method.init_nested_access "associated function bevy::ecs::query::WorldQuery::init_nested_access") must not request conflicting access. If `Self` is [`ReadOnlyQueryData`](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") or [`QueryFilter`](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), the access is read-only and can never conflict. If `Self` is [`SingleEntityQueryData`](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), there is no external access and it cannot conflict. Otherwise, [`WorldQuery::init_nested_access`](trait.WorldQuery.html#method.init_nested_access "associated function bevy::ecs::query::WorldQuery::init_nested_access") must be called to ensure it does not panic.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#98-103)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w, 's>( fetch: &mut Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

Adjusts internal state to account for the next [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"). This will always be called on archetypes that match this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery").

##### Safety

*   `archetype` and `tables` must be from the same [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") that [`WorldQuery::init_state`](trait.WorldQuery.html#tymethod.init_state "associated function bevy::ecs::query::WorldQuery::init_state") was called on.
*   `table` must correspond to `archetype`.
*   `state` must be the [`State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") that `fetch` was initialized with.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#112-116)

#### unsafe fn [set\_table](#tymethod.set_table)<'w, 's>( fetch: &mut Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

Adjusts internal state to account for the next [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table"). This will always be called on tables that match this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery").

##### Safety

*   `table` must be from the same [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") that [`WorldQuery::init_state`](trait.WorldQuery.html#tymethod.init_state "associated function bevy::ecs::query::WorldQuery::init_state") was called on.
*   `state` must be the [`State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") that `fetch` was initialized with.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#123)

#### fn [update\_component\_access](#tymethod.update_component_access)(state: &Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"))

Adds any component accesses to the current entity used by this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") to `access`.

Used to check which queries are disjoint and can run in parallel

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#140)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

Creates and initializes a [`State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") for this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#144)

#### fn [get\_state](#tymethod.get_state)(components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

Attempts to initialize a [`State`](trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") for this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type using read-only access to [`Components`](../component/struct.Components.html "struct bevy::ecs::component::Components").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#151-154)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( state: &Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this query matches a set of components. Otherwise, returns `false`.

Used to check which [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")s can be skipped by the query (if none of the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s match). This is how archetypal query filters like `With` work.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#131-136)

#### fn [init\_nested\_access](#method.init_nested_access)( \_state: &Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, \_component\_access\_set: &mut [FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

Adds any component accesses to other entities used by this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery").

This method must panic if the access would conflict with any existing access in the [`FilteredAccessSet`](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet").

This is used for queries to request access to entities other than the current one, such as to read resources or to follow relations.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#158)

#### fn [update\_archetypes](#method.update_archetypes)(\_state: &mut Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>)

Called when the query state is updating its archetype cache. This can be used by nested queries to update their internal archetype caches.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, state: &'s <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w, 's>( fetch: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [set\_table](#tymethod.set_table)<'w, 's>( fetch: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [update\_component\_access](#tymethod.update_component_access)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [init\_nested\_access](#method.init_nested_access)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, \_component\_access\_set: &mut [FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [get\_state](#tymethod.get_state)(components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [update\_archetypes](#method.update_archetypes)( state: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2260)

### impl<'\_\_w, T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2291)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2261)

#### type [Fetch](#associatedtype.Fetch)<'w> = [WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2262)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2264)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2269-2274)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2299-2304)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w>( fetch: &mut [WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")<'w, T>, component\_id: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), \_archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2314-2318)

#### unsafe fn [set\_table](#tymethod.set_table)<'w>( fetch: &mut [WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")<'w, T>, \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2336)

#### fn [update\_component\_access](#tymethod.update_component_access)(\_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2345)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2349)

#### fn [get\_state](#tymethod.get_state)( components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2353-2356)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

### impl<F> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### type [Fetch](#associatedtype.Fetch)<'w> = (<F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### type [State](#associatedtype.State) = (<F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, state: &'s <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w, 's>( fetch: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### unsafe fn [set\_table](#tymethod.set_table)<'w, 's>( fetch: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, state: &'s <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [update\_component\_access](#tymethod.update_component_access)( state: &<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [init\_nested\_access](#method.init_nested_access)( state: &<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, \_component\_access\_set: &mut [FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [get\_state](#tymethod.get_state)(components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( state: &<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#264-271)

#### fn [update\_archetypes](#method.update_archetypes)( state: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1778)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1807)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1779)

#### type [Fetch](#associatedtype.Fetch)<'w> = [ReadFetch](struct.ReadFetch.html "struct bevy::ecs::query::ReadFetch")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1780)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1782)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1787-1792)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), \_last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), \_this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [ReadFetch](struct.ReadFetch.html "struct bevy::ecs::query::ReadFetch")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1815-1820)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w>( fetch: &mut [ReadFetch](struct.ReadFetch.html "struct bevy::ecs::query::ReadFetch")<'w, T>, component\_id: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), \_archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1830-1834)

#### unsafe fn [set\_table](#tymethod.set_table)<'w>( fetch: &mut [ReadFetch](struct.ReadFetch.html "struct bevy::ecs::query::ReadFetch")<'w, T>, \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1845)

#### fn [update\_component\_access](#tymethod.update_component_access)(\_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1854)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1858)

#### fn [get\_state](#tymethod.get_state)(components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1862-1865)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( \_: &[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3028)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3053)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = T::IS\_DENSE

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3029)

#### type [Fetch](#associatedtype.Fetch)<'w> = OptionFetch<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3030)

#### type [State](#associatedtype.State) = <T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3032)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3040-3045)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, state: &'s <T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> OptionFetch<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3056-3061)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w, 's>( fetch: &mut OptionFetch<'w, T>, state: &'s <T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3072-3076)

#### unsafe fn [set\_table](#tymethod.set_table)<'w, 's>( fetch: &mut OptionFetch<'w, T>, state: &'s <T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3086)

#### fn [update\_component\_access](#tymethod.update_component_access)( state: &<T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3101-3106)

#### fn [init\_nested\_access](#method.init_nested_access)( state: &<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, component\_access\_set: &mut [FilteredAccessSet](struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3110)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3114)

#### fn [get\_state](#tymethod.get_state)( components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3118-3121)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( \_state: &<T as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3125)

#### fn [update\_archetypes](#method.update_archetypes)( state: &mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3912)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3930)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3913)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3915)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3917)

#### fn [shrink\_fetch](#tymethod.shrink_fetch)<'wlong, 'wshort>( \_fetch: <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3920-3925)

#### unsafe fn [init\_fetch](#tymethod.init_fetch)<'w, 's>( \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, \_state: &'s <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), \_this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3932-3937)

#### unsafe fn [set\_archetype](#tymethod.set_archetype)<'w, 's>( \_fetch: &mut <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, \_state: &'s <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_archetype: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), \_table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3940-3944)

#### unsafe fn [set\_table](#tymethod.set_table)<'w, 's>( \_fetch: &mut <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, \_state: &'s <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_table: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3947)

#### fn [update\_component\_access](#tymethod.update_component_access)( \_state: &<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_access: &mut [FilteredAccess](struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3949)

#### fn [init\_state](#tymethod.init_state)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3951)

#### fn [get\_state](#tymethod.get_state)( \_components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3955-3958)

#### fn [matches\_component\_set](#tymethod.matches_component_set)( \_state: &<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1651)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1670)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1652)

#### type [Fetch](#associatedtype.Fetch)<'w> = (&'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities"), &'w [Archetypes](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1653)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#502)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#516)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#503)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#504)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#612)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#631)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#613)

#### type [Fetch](#associatedtype.Fetch)<'w> = &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#614)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1279)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1287)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1280)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1281)

#### type [State](#associatedtype.State) = [Access](struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1148)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1156)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1149)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1150)

#### type [State](#associatedtype.State) = [Access](struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#410)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#433)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&'static MainEntity as WorldQuery>::IS\_DENSE

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#411)

#### type [Fetch](#associatedtype.Fetch)<'w> = <&'static [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity") as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#412)

#### type [State](#associatedtype.State) = <&'static [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity") as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = NameOrEntityFetch<'\_\_w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### type [State](#associatedtype.State) = NameOrEntityState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = NodeQueryFetch<'\_\_w>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [State](#associatedtype.State) = NodeQueryState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = NodeQueryFetch<'\_\_w>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [State](#associatedtype.State) = NodeQueryState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = NodeQueryReadOnlyFetch<'\_\_w>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [State](#associatedtype.State) = NodeQueryState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = NodeQueryReadOnlyFetch<'\_\_w>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [State](#associatedtype.State) = NodeQueryState

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = PointerTraversalFetch<'\_\_w>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### type [State](#associatedtype.State) = PointerTraversalState

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#289)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#312)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&'static RenderEntity as WorldQuery>::IS\_DENSE

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#290)

#### type [Fetch](#associatedtype.Fetch)<'w> = <&'static [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity") as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#291)

#### type [State](#associatedtype.State) = <&'static [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity") as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#791)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#812)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#792)

#### type [Fetch](#associatedtype.Fetch)<'w> = SpawnDetailsFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#793)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1183)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Spawned](struct.Spawned.html "struct bevy::ecs::query::Spawned")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1205)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1184)

#### type [Fetch](#associatedtype.Fetch)<'w> = SpawnedFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1185)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### type [Fetch](#associatedtype.Fetch)<'\_\_w> = WindowTraversalFetch<'\_\_w>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### type [State](#associatedtype.State) = WindowTraversalState

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2501)

### impl<'\_\_w, T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

When `Mut<T>` is used in a query, it will be converted to `Ref<T>` when transformed into its read-only form, providing access to change detection methods.

By contrast `&mut T` will result in a `Mut<T>` item in mutable form to record mutations, but result in a bare `&T` in read-only form.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2521)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&mut T as WorldQuery>::IS\_DENSE

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2502)

#### type [Fetch](#associatedtype.Fetch)<'w> = [WriteFetch](struct.WriteFetch.html "struct bevy::ecs::query::WriteFetch")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2503)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1993)

### impl<'\_\_w, T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2024)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1994)

#### type [Fetch](#associatedtype.Fetch)<'w> = RefFetch<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1995)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1530-1532)

### impl<'a, 'b, B> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'a, 'b, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1554)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1534)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1535)

#### type [State](#associatedtype.State) = [Access](struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1407-1409)

### impl<'a, 'b, B> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'a, 'b, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1431)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1411)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1412)

#### type [State](#associatedtype.State) = [Access](struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1035)

### impl<'a> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1056)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1036)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1037)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#919)

### impl<'a> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#940)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#920)

#### type [Fetch](#associatedtype.Fetch)<'w> = EntityFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#921)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#155)

### impl<A> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [AssetChanged](../../prelude/struct.AssetChanged.html "struct bevy::prelude::AssetChanged")<A>

where A: [AsAssetId](../../asset/trait.AsAssetId.html "trait bevy::asset::AsAssetId"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#207)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&A>::IS\_DENSE

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#156)

#### type [Fetch](#associatedtype.Fetch)<'w> = AssetChangedFetch<'w, A>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#158)

#### type [State](#associatedtype.State) = AssetChangedState<A>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2865-2866)

### impl<D, F> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

where D: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2889)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2868)

#### type [Fetch](#associatedtype.Fetch)<'w> = NestedQueryFetch<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2869)

#### type [State](#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Fetch](#associatedtype.Fetch)<'w> = ((<F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [State](#associatedtype.State) = (<F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

### impl<F> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### type [Fetch](#associatedtype.Fetch)<'w> = (OrFetch<'w, F>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### type [State](#associatedtype.State) = (<F as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#758)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Added](../../prelude/struct.Added.html "struct bevy::prelude::Added")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#789)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#759)

#### type [Fetch](#associatedtype.Fetch)<'w> = AddedFetch<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#760)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#611)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Allow](../../prelude/struct.Allow.html "struct bevy::prelude::Allow")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#621)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#612)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#613)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#985)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Changed](../../prelude/struct.Changed.html "struct bevy::prelude::Changed")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1016)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#986)

#### type [Fetch](#associatedtype.Fetch)<'w> = ChangedFetch<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#987)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3274)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3292)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3275)

#### type [Fetch](#associatedtype.Fetch)<'w> = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3276)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#149)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [With](../../prelude/struct.With.html "struct bevy::prelude::With")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#164)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#150)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#151)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#250)

### impl<T> [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#265)

#### const [IS\_DENSE](#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#251)

#### type [Fetch](#associatedtype.Fetch)<'w> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#252)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")