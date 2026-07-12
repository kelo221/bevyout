[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait QueryFilter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#84)

```rust
pub unsafe trait QueryFilter: WorldQuery {
    const IS_ARCHETYPAL: bool;

    // Required method
    unsafe fn filter_fetch(
        state: &Self::State,
        fetch: &mut Self::Fetch<'_>,
        entity: Entity,
        table_row: TableRow,
    ) -> bool;
}
```

Types that filter the results of a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

There are many types that natively implement this trait:

*   **Component filters.** [`With`](../../prelude/struct.With.html "struct bevy::prelude::With") and [`Without`](../../prelude/struct.Without.html "struct bevy::prelude::Without") filters can be applied to check if the queried entity does or does not contain a particular component.
*   **Change detection filters.** [`Added`](../../prelude/struct.Added.html "struct bevy::prelude::Added") and [`Changed`](../../prelude/struct.Changed.html "struct bevy::prelude::Changed") filters can be applied to detect component changes to an entity.
*   **Spawned filter.** [`Spawned`](struct.Spawned.html "struct bevy::ecs::query::Spawned") filter can be applied to check if the queried entity was spawned recently.
*   **`QueryFilter` tuples.** If every element of a tuple implements `QueryFilter`, then the tuple itself also implements the same trait. This enables a single `Query` to filter over multiple conditions. Due to the current lack of variadic generics in Rust, the trait has been implemented for tuples from 0 to 15 elements, but nesting of tuples allows infinite `QueryFilter`s.
*   **Filter disjunction operator.** By default, tuples compose query filters in such a way that all conditions must be satisfied to generate a query item for a given entity. Wrapping a tuple inside an [`Or`](../../prelude/struct.Or.html "struct bevy::prelude::Or") operator will relax the requirement to just one condition.

Implementing the trait manually can allow for a fundamentally new type of behavior.

Query design can be easily structured by deriving `QueryFilter` for custom types. Despite the added complexity, this approach has several advantages over using `QueryFilter` tuples. The most relevant improvements are:

*   Reusability across multiple systems.
*   Filters can be composed together to create a more complex filter.

This trait can only be derived for structs if each field also implements `QueryFilter`.

```rust
#[derive(QueryFilter)]
struct MyFilter<T: Component, P: Component> {
    // Field names are not relevant, since they are never manually accessed.
    with_a: With<ComponentA>,
    or_filter: Or<(With<ComponentC>, Added<ComponentB>)>,
    generic_tuple: (With<T>, Without<P>),
}

fn my_system(query: Query<Entity, MyFilter<ComponentD, ComponentE>>) {
    // ...
}
```

## Safety

The [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") implementation must not take any mutable access. This is the same safety requirement as [`ReadOnlyQueryData`](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData").

## Required Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#92)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if (and only if) this Filter relies strictly on archetypes to limit which components are accessed by the Query.

This enables optimizations for [`QueryIter`](struct.QueryIter.html "struct bevy::ecs::query::QueryIter") that rely on knowing exactly how many elements are being iterated (such as `Iterator::collect()`).

If this is `true`, then [`QueryFilter::filter_fetch`](trait.QueryFilter.html#tymethod.filter_fetch "associated function bevy::ecs::query::QueryFilter::filter_fetch") must always return true.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#107-112)

#### unsafe fn [filter\_fetch](#tymethod.filter_fetch)( state: &Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'\_>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the provided [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and [`TableRow`](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow") should be included in the query results. If false, the entity will be skipped.

Note that this is called after already restricting the matched [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table")s and [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")s to the ones that are compatible with the Filter’s access.

Implementors of this method will generally either have a trivial `true` body (required for archetypal filters), or access the necessary data within this function to make the final decision on filter inclusion.

##### Safety

Must always be called _after_ [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table") or [`WorldQuery::set_archetype`](trait.WorldQuery.html#tymethod.set_archetype "associated function bevy::ecs::query::WorldQuery::set_archetype"). `entity` and `table_row` must be in the range of the current table and archetype.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

### impl [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

#### unsafe fn [filter\_fetch](#tymethod.filter_fetch)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'\_>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

### impl<F> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#584-591)

#### unsafe fn [filter\_fetch](#tymethod.filter_fetch)( state: &<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'\_>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

### impl [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1234)

### impl [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Spawned](struct.Spawned.html "struct bevy::ecs::query::Spawned")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1235)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#291)

### impl<A> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [AssetChanged](../../prelude/struct.AssetChanged.html "struct bevy::prelude::AssetChanged")<A>

where A: [AsAssetId](../../asset/trait.AsAssetId.html "trait bevy::asset::AsAssetId"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/asset_changed.rs.html#292)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

### impl<F> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#592-599)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#852)

### impl<T> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Added](../../prelude/struct.Added.html "struct bevy::prelude::Added")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#853)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#649)

### impl<T> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Allow](../../prelude/struct.Allow.html "struct bevy::prelude::Allow")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#650)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1079)

### impl<T> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Changed](../../prelude/struct.Changed.html "struct bevy::prelude::Changed")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1080)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#205)

### impl<T> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [With](../../prelude/struct.With.html "struct bevy::prelude::With")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#206)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#306)

### impl<T> [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") for [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#307)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true