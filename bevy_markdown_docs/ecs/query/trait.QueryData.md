[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait QueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#324)

```rust
pub unsafe trait QueryData: WorldQuery {
    type ReadOnly: ReadOnlyQueryData<State = Self::State>;
    type Item<'w, 's>;

    const IS_READ_ONLY: bool;
    const IS_ARCHETYPAL: bool;

    // Required methods
    fn shrink<'wlong, 'wshort, 's>(
        item: Self::Item<'wlong, 's>,
    ) -> Self::Item<'wshort, 's>
       where 'wlong: 'wshort;
    unsafe fn fetch<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entity: Entity,
        table_row: TableRow,
    ) -> Option<Self::Item<'w, 's>>;
    fn iter_access(
        state: &Self::State,
    ) -> impl Iterator<Item = EcsAccessType<'_>>;

    // Provided method
    fn provide_extra_access(
        _state: &mut Self::State,
        _access: &mut Access,
        _available_access: &Access,
    ) { ... }
}
```

Types that can be fetched from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") using a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query").

There are many types that natively implement this trait:

*   **Component references. (&T and &mut T)** Fetches a component by reference (immutably or mutably).
*   **`QueryData` tuples.** If every element of a tuple implements `QueryData`, then the tuple itself also implements the same trait. This enables a single `Query` to access multiple components. Due to the current lack of variadic generics in Rust, the trait has been implemented for tuples from 0 to 15 elements, but nesting of tuples allows infinite `WorldQuery`s.
*   **[`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").** Gets the identifier of the queried entity.
*   **[`EntityLocation`](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation").** Gets the location metadata of the queried entity.
*   **[`SpawnDetails`](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails").** Gets the tick the entity was spawned at.
*   **[`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef").** Read-only access to arbitrary components on the queried entity.
*   **[`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut").** Mutable access to arbitrary components on the queried entity.
*   **[`&Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype").** Read-only access to the archetype-level metadata of the queried entity.
*   **[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").** By default, a world query only tests entities that have the matching component types. Wrapping it into an `Option` will increase the query search space, and it will return `None` if an entity doesn’t satisfy the `WorldQuery`.
*   **[`AnyOf`](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf").** Equivalent to wrapping each world query inside it into an `Option`.
*   **[`Ref`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").** Similar to change detection filters but it is used as a query fetch parameter. It exposes methods to check for changes to the wrapped component.
*   **[`Mut`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut").** Mutable component access, with change detection data.
*   **[`Has`](../../prelude/struct.Has.html "struct bevy::prelude::Has").** Returns a bool indicating whether the entity has the specified component.

Implementing the trait manually can allow for a fundamentally new type of behavior.

## Trait derivation

Query design can be easily structured by deriving `QueryData` for custom types. Despite the added complexity, this approach has several advantages over using `QueryData` tuples. The most relevant improvements are:

*   Reusability across multiple systems.
*   There is no need to destructure a tuple since all fields are named.
*   Subqueries can be composed together to create a more complex query.
*   Methods can be implemented for the query items.
*   There is no hardcoded limit on the number of elements.

This trait can only be derived for structs, if each field also implements `QueryData`.

```rust
use bevy_ecs::query::QueryData;

#[derive(QueryData)]
struct MyQuery {
    entity: Entity,
    // It is required that all reference lifetimes are explicitly annotated, just like in any
    // struct. Each lifetime should be 'static.
    component_a: &'static ComponentA,
    component_b: &'static ComponentB,
}

fn my_system(query: Query<MyQuery>) {
    for q in &query {
        q.component_a;
    }
}
```

### Macro expansion

Expanding the macro will declare one to five additional structs, depending on whether or not the struct is marked as mutable or as contiguous. For a struct named `X`, the additional structs will be:

| Struct name | `mutable` only | `contiguous` target | Description |
| --- | --- | --- | --- |
| `XItem` | — | — | The type of the query item for `X` |
| `XReadOnlyItem` | ✓ | — | The type of the query item for `XReadOnly` |
| `XReadOnly` | ✓ | — | [`ReadOnly`](trait.QueryData.html#associatedtype.ReadOnly "associated type bevy::ecs::query::QueryData::ReadOnly") variant of `X` |
| `XContiguousItem` | — | `mutable` or `all` | The type of the contiguous query item for `X` |
| `XReadOnlyContiguousItem` | ✓ | `immutable` or `all` | The type of the contiguous query item for `XReadOnly` |

### Adding mutable references

Simply adding mutable references to a derived `QueryData` will result in a compilation error:

[ⓘ](# "This example deliberately fails to compile")

```rust
#[derive(QueryData)]
struct CustomQuery {
    component_a: &'static mut ComponentA,
}
```

To grant mutable access to components, the struct must be marked with the `#[query_data(mutable)]` attribute. This will also create three more structs that will be used for accessing the query immutably (see table above).

```rust
#[derive(QueryData)]
#[query_data(mutable)]
struct CustomQuery {
    component_a: &'static mut ComponentA,
}
```

### Supporting contiguous iteration

To create contiguous items additionally (to support contiguous iteration), the struct must be marked with the `#[query_data(contiguous(target))]` attribute, where the target may be `all`, `mutable` or `immutable` (see the table above).

For mutable queries it may be done like this:

```rust
#[derive(QueryData)]
/// - contiguous(all) will create contiguous items for both read and mutable versions
/// - contiguous(mutable) will only create a contiguous item for the mutable version
/// - contiguous(immutable) will only create a contiguous item for the read only version
#[query_data(mutable, contiguous(all))]
struct CustomQuery {
    component_a: &'static mut ComponentA,
}
```

For immutable queries `contiguous(immutable)` attribute will be **ignored**, meanwhile `contiguous(mutable)` and `contiguous(all)` will only generate a contiguous item for the (original) read only version.

To understand contiguous iteration refer to [`Query::contiguous_iter`](../../prelude/struct.Query.html#method.contiguous_iter "method bevy::prelude::Query::contiguous_iter")

### Adding methods to query items

It is possible to add methods to query items in order to write reusable logic about related components. This will often make systems more readable because low level logic is moved out from them. It is done by adding `impl` blocks with methods for the `-Item`, `-ReadOnlyItem`, `-ContiguousItem` or `ContiguousReadOnlyItem` generated structs.

```rust
#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Buff(f32);

#[derive(QueryData)]
#[query_data(mutable)]
struct HealthQuery {
    health: &'static mut Health,
    buff: Option<&'static mut Buff>,
}

// `HealthQueryItem` is only available when accessing the query with mutable methods.
impl<'w, 's> HealthQueryItem<'w, 's> {
    fn damage(&mut self, value: f32) {
        self.health.0 -= value;
    }

    fn total(&self) -> f32 {
        self.health.0 + self.buff.as_deref().map_or(0.0, |Buff(buff)| *buff)
    }
}

// `HealthQueryReadOnlyItem` is only available when accessing the query with immutable methods.
impl<'w, 's> HealthQueryReadOnlyItem<'w, 's> {
    fn total(&self) -> f32 {
        self.health.0 + self.buff.map_or(0.0, |Buff(buff)| *buff)
    }
}

fn my_system(mut health_query: Query<HealthQuery>) {
    // The item returned by the iterator is of type `HealthQueryReadOnlyItem`.
    for health in health_query.iter() {
        println!("Total: {}", health.total());
    }
    // The item returned by the iterator is of type `HealthQueryItem`.
    for mut health in &mut health_query {
        health.damage(1.0);
        println!("Total (mut): {}", health.total());
    }
}
```

### Deriving traits for query items

The `QueryData` derive macro does not automatically implement the traits of the struct to the query item types. Something similar can be done by using the `#[query_data(derive(...))]` attribute. This will apply the listed derivable traits to the query item structs.

```rust
#[derive(QueryData)]
#[query_data(mutable, derive(Debug), contiguous(all))]
struct CustomQuery {
    component_a: &'static ComponentA,
}

// This function statically checks that `T` implements `Debug`.
fn assert_debug<T: std::fmt::Debug>() {}

assert_debug::<CustomQueryItem>();
assert_debug::<CustomQueryReadOnlyItem>();
assert_debug::<CustomQueryContiguousItem>();
assert_debug::<CustomQueryReadOnlyContiguousItem>();
```

### Query composition

It is possible to use any `QueryData` as a field of another one. This means that a `QueryData` can also be used as a subquery, potentially in multiple places.

```rust
#[derive(QueryData)]
struct SubQuery {
    component_a: &'static ComponentA,
    component_b: &'static ComponentB,
}

#[derive(QueryData)]
struct MyQuery {
    subquery: SubQuery,
    component_c: &'static ComponentC,
}
```

## Generic Queries

When writing generic code, it is often necessary to use [`PhantomData`](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData") to constrain type parameters. Since `QueryData` is implemented for all `PhantomData<T>` types, this pattern can be used with this macro.

```rust
#[derive(QueryData)]
pub struct GenericQuery<T> {
    id: Entity,
    marker: PhantomData<T>,
}
```

## Safety

*   Component access of `Self::ReadOnly` must be a subset of `Self` and `Self::ReadOnly` must match exactly the same archetypes/tables as `Self`
*   `IS_READ_ONLY` must be `true` if and only if `Self: ReadOnlyQueryData`

## Required Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#326)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

True if this query is read-only and may not perform mutable access.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#335)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if (and only if) this query data relies strictly on archetypes to limit which entities are accessed by the Query.

This enables optimizations for [`QueryIter`](struct.QueryIter.html "struct bevy::ecs::query::QueryIter") that rely on knowing exactly how many elements are being iterated (such as `Iterator::collect()`).

If this is `true`, then [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch") must always return `Some`.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#338)

#### type [ReadOnly](#associatedtype.ReadOnly): [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")<State = Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

The read-only variant of this [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData"), which satisfies the [`ReadOnlyQueryData`](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") trait.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#343)

#### type [Item](#associatedtype.Item)<'w, 's>

The item returned by this [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") This will be the data retrieved by the query, and is visible to the end user when calling e.g. `Query<Self>::get`.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#346-348)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: Self::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> Self::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

This function manually implements subtyping for the query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#382-387)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( state: &'s Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

Fetch [`Self::Item`](trait.QueryData.html#associatedtype.Item "associated type bevy::ecs::query::QueryData::Item") for either the given `entity` in the current [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), or for the given `entity` in the current [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"). This must always be called after [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table") with a `table_row` in the range of the current [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table") or after [`WorldQuery::set_archetype`](trait.WorldQuery.html#tymethod.set_archetype "associated function bevy::ecs::query::WorldQuery::set_archetype") with an `entity` in the current archetype. Accesses components registered in [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access").

This method returns `None` if the entity does not match the query. If `Self` implements [`ArchetypeQueryData`](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), this must always return `Some`.

##### Safety

*   Must always be called _after_ [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table") or [`WorldQuery::set_archetype`](trait.WorldQuery.html#tymethod.set_archetype "associated function bevy::ecs::query::WorldQuery::set_archetype"). `entity` and `table_row` must be in the range of the current table and archetype.
*   There must not be simultaneous conflicting component access registered in `update_component_access`.
*   If `Self` does not impl `ReadOnlyQueryData`, then there must not be any other `Item`s alive for the current entity
*   If `Self` does not impl `IterQueryData`, then there must not be any other `Item`s alive for _any_ entity

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#392)

#### fn [iter\_access](#tymethod.iter_access)(state: &Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

Returns an iterator over the access needed by [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch"). Access conflicts are usually checked in [`WorldQuery::update_component_access`](trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"), but in certain cases this method can be useful to implement a way of checking for access conflicts in a non-allocating way.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#359-363)

#### fn [provide\_extra\_access](#method.provide_extra_access)( \_state: &mut Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_access: &mut [Access](struct.Access.html "struct bevy::ecs::query::Access"), \_available\_access: &[Access](struct.Access.html "struct bevy::ecs::query::Access"), )

Offers additional access above what we requested in `update_component_access`. Implementations may add additional access that is a subset of `available_access` and does not conflict with anything in `access`, and must update `access` to include that access.

This is used by [`WorldQuery`](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") types like [`FilteredEntityRef`](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef") and [`FilteredEntityMut`](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") to support dynamic access.

Called when constructing a [`QueryLens`](../system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") or calling [`QueryState::from_builder`](../../prelude/struct.QueryState.html#method.from_builder "associated function bevy::prelude::QueryState::from_builder")

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [ReadOnly](#associatedtype.ReadOnly) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [Item](#associatedtype.Item)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [provide\_extra\_access](#method.provide_extra_access)( state: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [Access](struct.Access.html "struct bevy::ecs::query::Access"), available\_access: &[Access](struct.Access.html "struct bevy::ecs::query::Access"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( state: &'s <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [iter\_access](#tymethod.iter_access)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2362)

### impl<'\_\_w, T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2363)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2364)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2365)

#### type [ReadOnly](#associatedtype.ReadOnly) = [&'\_\_w T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2366)

#### type [Item](#associatedtype.Item)<'w, 's> = [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2368-2370)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2375-2380)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( \_state: &'s <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2429)

#### fn [iter\_access](#tymethod.iter_access)( state: &<[&'\_\_w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [ReadOnly](#associatedtype.ReadOnly) = (<F as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [Item](#associatedtype.Item)<'w, 's> = (<F as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [provide\_extra\_access](#method.provide_extra_access)( state: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [Access](struct.Access.html "struct bevy::ecs::query::Access"), available\_access: &[Access](struct.Access.html "struct bevy::ecs::query::Access"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( state: &'s <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [iter\_access](#tymethod.iter_access)( state: &<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1871)

### impl<T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1872)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1873)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1874)

#### type [ReadOnly](#associatedtype.ReadOnly) = [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1875)

#### type [Item](#associatedtype.Item)<'w, 's> = [&'w T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1877-1879)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1884-1889)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( \_state: &'s <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1911)

#### fn [iter\_access](#tymethod.iter_access)( state: &<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3131)

### impl<T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3132)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = T::IS\_READ\_ONLY

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3135)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3136)

#### type [ReadOnly](#associatedtype.ReadOnly) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3137)

#### type [Item](#associatedtype.Item)<'w, 's> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3139-3141)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3146-3151)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( state: &'s <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3161)

#### fn [iter\_access](#tymethod.iter_access)( state: &<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3964)

### impl<T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3965)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3966)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3967)

#### type [ReadOnly](#associatedtype.ReadOnly) = [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3968)

#### type [Item](#associatedtype.Item)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3970-3972)

#### fn [shrink](#tymethod.shrink)<'wlong, 'wshort, 's>( \_item: <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3975-3980)

#### unsafe fn [fetch](#tymethod.fetch)<'w, 's>( \_state: &'s <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_fetch: &mut <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, \_entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), \_table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3984)

#### fn [iter\_access](#tymethod.iter_access)( \_state: &<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1706)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1707)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1708)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1709)

#### type [ReadOnly](#associatedtype.ReadOnly) = &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1710)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [ReadOnly](#associatedtype.ReadOnly) = [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Item](#associatedtype.Item)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#552)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#553)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#554)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#555)

#### type [ReadOnly](#associatedtype.ReadOnly) = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#557)

#### type [Item](#associatedtype.Item)<'w, 's> = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#667)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#668)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#669)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#670)

#### type [ReadOnly](#associatedtype.ReadOnly) = [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#671)

#### type [Item](#associatedtype.Item)<'w, 's> = [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#480)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#481)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#482)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&MainEntity as QueryData>::IS\_ARCHETYPAL

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#483)

#### type [ReadOnly](#associatedtype.ReadOnly) = [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#484)

#### type [Item](#associatedtype.Item)<'w, 's> = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [NameOrEntityItem](../name/struct.NameOrEntityItem.html "struct bevy::ecs::name::NameOrEntityItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [NodeQueryItem](../../ui/picking_backend/struct.NodeQueryItem.html "struct bevy::ui::picking_backend::NodeQueryItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [NodeQueryItem](../../ui/struct.NodeQueryItem.html "struct bevy::ui::NodeQueryItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [NodeQueryReadOnlyItem](../../ui/picking_backend/struct.NodeQueryReadOnlyItem.html "struct bevy::ui::picking_backend::NodeQueryReadOnlyItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [NodeQueryReadOnlyItem](../../ui/struct.NodeQueryReadOnlyItem.html "struct bevy::ui::NodeQueryReadOnlyItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### type [ReadOnly](#associatedtype.ReadOnly) = [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [PointerTraversalItem](../../prelude/struct.PointerTraversalItem.html "struct bevy::prelude::PointerTraversalItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#359)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#360)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#361)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = <&MainEntity as QueryData>::IS\_ARCHETYPAL

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#362)

#### type [ReadOnly](#associatedtype.ReadOnly) = [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#363)

#### type [Item](#associatedtype.Item)<'w, 's> = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#850)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#851)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#852)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#853)

#### type [ReadOnly](#associatedtype.ReadOnly) = [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#854)

#### type [Item](#associatedtype.Item)<'w, 's> = [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### type [ReadOnly](#associatedtype.ReadOnly) = [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

#### type [Item](#associatedtype.Item)<'\_\_w, '\_\_s> = [WindowTraversalItem](../../input_focus/struct.WindowTraversalItem.html "struct bevy::input_focus::WindowTraversalItem")<'\_\_w, '\_\_s>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2572)

### impl<'\_\_w, T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2573)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2574)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2575)

#### type [ReadOnly](#associatedtype.ReadOnly) = [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2576)

#### type [Item](#associatedtype.Item)<'w, 's> = [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2095)

### impl<'\_\_w, T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2096)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2097)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2098)

#### type [ReadOnly](#associatedtype.ReadOnly) = [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2099)

#### type [Item](#associatedtype.Item)<'w, 's> = [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1607-1609)

### impl<'a, 'b, B> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'a, 'b, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1611)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1612)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1613)

#### type [ReadOnly](#associatedtype.ReadOnly) = [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'a, 'b, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1614)

#### type [Item](#associatedtype.Item)<'w, 's> = [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'w, 's, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1483-1485)

### impl<'a, 'b, B> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'a, 'b, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1487)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1488)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1489)

#### type [ReadOnly](#associatedtype.ReadOnly) = [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'a, 'b, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1490)

#### type [Item](#associatedtype.Item)<'w, 's> = [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'w, 's, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1344)

### impl<'a, 'b> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1345)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1346)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1347)

#### type [ReadOnly](#associatedtype.ReadOnly) = [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'b>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1348)

#### type [Item](#associatedtype.Item)<'w, 's> = [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1213)

### impl<'a, 'b> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'b>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1214)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1215)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1216)

#### type [ReadOnly](#associatedtype.ReadOnly) = [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'b>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1217)

#### type [Item](#associatedtype.Item)<'w, 's> = [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1098)

### impl<'a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1099)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1100)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1101)

#### type [ReadOnly](#associatedtype.ReadOnly) = [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1102)

#### type [Item](#associatedtype.Item)<'w, 's> = [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#982)

### impl<'a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#983)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#984)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#985)

#### type [ReadOnly](#associatedtype.ReadOnly) = [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#986)

#### type [Item](#associatedtype.Item)<'w, 's> = [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2949-2950)

### impl<D, F> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

where D: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2952)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = D::IS\_READ\_ONLY

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2957)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2958)

#### type [ReadOnly](#associatedtype.ReadOnly) = [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2959)

#### type [Item](#associatedtype.Item)<'w, 's> = [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [ReadOnly](#associatedtype.ReadOnly) = [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<(<F as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"),)>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Item](#associatedtype.Item)<'w, 's> = ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<F as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3340)

### impl<T> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3341)

#### const [IS\_READ\_ONLY](#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3342)

#### const [IS\_ARCHETYPAL](#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3343)

#### type [ReadOnly](#associatedtype.ReadOnly) = [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3344)

#### type [Item](#associatedtype.Item)<'w, 's> = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)