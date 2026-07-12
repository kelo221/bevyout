[bevy](../index.html)::[prelude](index.html)

# Struct Query 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#487)

```rust
pub struct Query<'world, 'state, D, F = ()>where
    D: QueryData,
    F: QueryFilter,{ /* private fields */ }
```

A [system parameter](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides selective access to the [`Component`](trait.Component.html "trait bevy::prelude::Component") data stored in a [`World`](struct.World.html "struct bevy::prelude::World").

Queries enable systems to access [entity identifiers](struct.Entity.html "struct bevy::prelude::Entity") and [components](trait.Component.html "trait bevy::prelude::Component") without requiring direct access to the [`World`](struct.World.html "struct bevy::prelude::World"). Its iterators and getter methods return _query items_, which are types containing data related to an entity.

`Query` is a generic data structure that accepts two type parameters:

*   **`D` (query data)**: The type of data fetched by the query, which will be returned as the query item. Only entities that match the requested data will generate an item. Must implement the [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") trait.
*   **`F` (query filter)**: An optional set of conditions that determine whether query items should be kept or discarded. This defaults to [`unit`](https://doc.rust-lang.org/nightly/std/primitive.unit.html "primitive unit"), which means no additional filters will be applied. Must implement the [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") trait.

## Similar parameters

`Query` has few sibling [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s, which perform additional validation:

*   [`Single`](struct.Single.html "struct bevy::prelude::Single") - Exactly one matching query item.
*   [`Option<Single>`](struct.Single.html "struct bevy::prelude::Single") - Zero or one matching query item.
*   [`Populated`](struct.Populated.html "struct bevy::prelude::Populated") - At least one matching query item.

These parameters will prevent systems from running if their requirements are not met.

## System parameter declaration

A query should always be declared as a system parameter. This section shows the most common idioms involving the declaration of `Query`.

### Component access

You can fetch an entity’s component by specifying a reference to that component in the query’s data parameter:

```rust
// A component can be accessed by a shared reference...
fn immutable_query(query: Query<&ComponentA>) {
    // ...
}

// ...or by a mutable reference.
fn mutable_query(query: Query<&mut ComponentA>) {
    // ...
}
```

Note that components need to be behind a reference (`&` or `&mut`), or the query will not compile:

[ⓘ](# "This example deliberately fails to compile")

```rust
// This needs to be `&ComponentA` or `&mut ComponentA` in order to compile.
fn invalid_query(query: Query<ComponentA>) {
    // ...
}
```

### Query filtering

Setting the query filter type parameter will ensure that each query item satisfies the given condition:

```rust
// `ComponentA` data will be accessed, but only for entities that also contain `ComponentB`.
fn filtered_query(query: Query<&ComponentA, With<ComponentB>>) {
    // ...
}
```

Note that the filter is `With<ComponentB>`, not `With<&ComponentB>`. Unlike query data, `With` does not require components to be behind a reference.

### `QueryData` or `QueryFilter` tuples

Using [`tuple`](https://doc.rust-lang.org/nightly/std/primitive.tuple.html "primitive tuple")s, each `Query` type parameter can contain multiple elements.

In the following example two components are accessed simultaneously, and the query items are filtered on two conditions:

```rust
fn complex_query(
    query: Query<(&mut ComponentA, &ComponentB), (With<ComponentC>, Without<ComponentD>)>
) {
    // ...
}
```

Note that this currently only works on tuples with 15 or fewer items. You may nest tuples to get around this limit:

```rust
fn nested_query(
    query: Query<(&ComponentA, &ComponentB, (&mut ComponentC, &mut ComponentD))>
) {
    // ...
}
```

### Entity identifier access

You can access [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"), the entity identifier, by including it in the query data parameter:

```rust
fn entity_id_query(query: Query<(Entity, &ComponentA)>) {
    // ...
}
```

Be aware that [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") is not a component, so it does not need to be behind a reference.

### Optional component access

A component can be made optional by wrapping it into an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). In the following example, a query item will still be generated even if the queried entity does not contain `ComponentB`. When this is the case, `Option<&ComponentB>`’s corresponding value will be `None`.

```rust
// Queried items must contain `ComponentA`. If they also contain `ComponentB`, its value will
// be fetched as well.
fn optional_component_query(query: Query<(&ComponentA, Option<&ComponentB>)>) {
    // ...
}
```

Optional components can hurt performance in some cases, so please read the [performance](#performance) section to learn more about them. Additionally, if you need to declare several optional components, you may be interested in using [`AnyOf`](struct.AnyOf.html "struct bevy::prelude::AnyOf").

### Disjoint queries

A system cannot contain two queries that break Rust’s mutability rules, or else it will panic when initialized. This can often be fixed with the [`Without`](struct.Without.html "struct bevy::prelude::Without") filter, which makes the queries disjoint.

In the following example, the two queries can mutably access the same `&mut Health` component if an entity has both the `Player` and `Enemy` components. Bevy will catch this and panic, however, instead of breaking Rust’s mutability rules:

[ⓘ](# "This example panics")

```rust
fn randomize_health(
    player_query: Query<&mut Health, With<Player>>,
    enemy_query: Query<&mut Health, With<Enemy>>,
) {
    // ...
}
```

Adding a [`Without`](struct.Without.html "struct bevy::prelude::Without") filter will disjoint the queries. In the following example, any entity that has both the `Player` and `Enemy` components will be excluded from _both_ queries:

```rust
fn randomize_health(
    player_query: Query<&mut Health, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&mut Health, (With<Enemy>, Without<Player>)>,
) {
    // ...
}
```

An alternative solution to this problem would be to wrap the conflicting queries in [`ParamSet`](struct.ParamSet.html "struct bevy::prelude::ParamSet").

### Whole Entity Access

[`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef") can be used in a query to gain read-only access to all components of an entity. This is useful when dynamically fetching components instead of baking them into the query type.

```rust
fn all_components_query(query: Query<(EntityRef, &ComponentA)>) {
    // ...
}
```

As [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef") can read any component on an entity, a query using it will conflict with _any_ mutable component access.

[ⓘ](# "This example panics")

```rust
// `EntityRef` provides read access to *all* components on an entity. When combined with
// `&mut ComponentA` in the same query, it creates a conflict because `EntityRef` could read
// `&ComponentA` while `&mut ComponentA` attempts to modify it - violating Rust's borrowing
// rules.
fn invalid_query(query: Query<(EntityRef, &mut ComponentA)>) {
    // ...
}
```

It is strongly advised to couple [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef") queries with the use of either [`With`](struct.With.html "struct bevy::prelude::With") / [`Without`](struct.Without.html "struct bevy::prelude::Without") filters or [`ParamSet`](struct.ParamSet.html "struct bevy::prelude::ParamSet")s. Not only does this improve the performance and parallelization of the system, but it enables systems to gain mutable access to other components:

```rust
// The first query only reads entities that have `ComponentA`, while the second query only
// modifies entities that *don't* have `ComponentA`. Because neither query will access the same
// entity, this system does not conflict.
fn disjoint_query(
    query_a: Query<EntityRef, With<ComponentA>>,
    query_b: Query<&mut ComponentB, Without<ComponentA>>,
) {
    // ...
}
```

The fundamental rule: [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef")’s ability to read all components means it can never coexist with mutable access. [`With`](struct.With.html "struct bevy::prelude::With") / [`Without`](struct.Without.html "struct bevy::prelude::Without") filters can guarantee this by keeping the queries on completely separate entities.

## Accessing query items

The following table summarizes the behavior of safe methods that can be used to get query items:

| Query methods | Effect |
| --- | --- |
| [`iter`](struct.Query.html#method.iter "method bevy::prelude::Query::iter")\[[`_mut`](struct.Query.html#method.iter_mut "method bevy::prelude::Query::iter_mut")\] | Returns an iterator over all query items. |
| [`iter[_mut]().for_each()`](#iteratorfor_each),  
[`par_iter`](struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter")\[[`_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut")\] | Runs a specified function for each query item. |
| [`iter_many`](struct.Query.html#method.iter_many "method bevy::prelude::Query::iter_many")\[[`_unique`](struct.Query.html#method.iter_many_unique "method bevy::prelude::Query::iter_many_unique")\]\[[`_mut`](struct.Query.html#method.iter_many_mut "method bevy::prelude::Query::iter_many_mut")\] | Iterates over query items that match a list of entities. |
| [`iter_combinations`](struct.Query.html#method.iter_combinations "method bevy::prelude::Query::iter_combinations")\[[`_mut`](struct.Query.html#method.iter_combinations_mut "method bevy::prelude::Query::iter_combinations_mut")\] | Iterates over all combinations of query items. |
| [`single`](struct.Query.html#method.single "method bevy::prelude::Query::single")\[[`_mut`](struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut")\] | Returns a single query item if only one exists. |
| [`get`](struct.Query.html#method.get "method bevy::prelude::Query::get")\[[`_mut`](struct.Query.html#method.get_mut "method bevy::prelude::Query::get_mut")\] | Returns the query item for a specified entity. |
| [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many")\[[`_unique`](struct.Query.html#method.get_many_unique "method bevy::prelude::Query::get_many_unique")\]\[[`_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut")\] | Returns all query items that match a list of entities. |

There are two methods for each type of query operation: immutable and mutable (ending with `_mut`). When using immutable methods, the query items returned are of type [`ROQueryItem`](../ecs/query/type.ROQueryItem.html "type bevy::ecs::query::ROQueryItem"), a read-only version of the query item. In this circumstance, every mutable reference in the query fetch type parameter is substituted by a shared reference.

## Performance

Creating a `Query` is a low-cost constant operation. Iterating it, on the other hand, fetches data from the world and generates items, which can have a significant computational cost.

Two systems cannot be executed in parallel if both access the same component type where at least one of the accesses is mutable. Because of this, it is recommended for queries to only fetch mutable access to components when necessary, since immutable access can be parallelized.

Query filters ([`With`](struct.With.html "struct bevy::prelude::With") / [`Without`](struct.Without.html "struct bevy::prelude::Without")) can improve performance because they narrow the kinds of entities that can be fetched. Systems that access fewer kinds of entities are more likely to be parallelized by the scheduler.

On the other hand, be careful using optional components (`Option<&ComponentA>`) and [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef") because they broaden the amount of entities kinds that can be accessed. This is especially true of a query that _only_ fetches optional components or [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef"), as the query would iterate over all entities in the world.

There are two types of [component storage types](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"): [`Table`](../ecs/storage/struct.Table.html "struct bevy::ecs::storage::Table") and [`SparseSet`](../ecs/storage/struct.SparseSet.html "struct bevy::ecs::storage::SparseSet"). [`Table`](../ecs/storage/struct.Table.html "struct bevy::ecs::storage::Table") offers fast iteration speeds, but slower insertion and removal speeds. [`SparseSet`](../ecs/storage/struct.SparseSet.html "struct bevy::ecs::storage::SparseSet") is the opposite: it offers fast component insertion and removal speeds, but slower iteration speeds.

The following table compares the computational complexity of the various methods and operations, where:

*   **n** is the number of entities that match the query.
*   **r** is the number of elements in a combination.
*   **k** is the number of involved entities in the operation.
*   **a** is the number of archetypes in the world.
*   **C** is the [binomial coefficient](https://en.wikipedia.org/wiki/Binomial_coefficient), used to count combinations. nCr is read as “_n_ choose _r_” and is equivalent to the number of distinct unordered subsets of _r_ elements that can be taken from a set of _n_ elements.

| Query operation | Computational complexity |
| --- | --- |
| [`iter`](struct.Query.html#method.iter "method bevy::prelude::Query::iter")\[[`_mut`](struct.Query.html#method.iter_mut "method bevy::prelude::Query::iter_mut")\] | O(n) |
| [`iter[_mut]().for_each()`](#iteratorfor_each),  
[`par_iter`](struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter")\[[`_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut")\] | O(n) |
| [`iter_many`](struct.Query.html#method.iter_many "method bevy::prelude::Query::iter_many")\[[`_mut`](struct.Query.html#method.iter_many_mut "method bevy::prelude::Query::iter_many_mut")\] | O(k) |
| [`iter_combinations`](struct.Query.html#method.iter_combinations "method bevy::prelude::Query::iter_combinations")\[[`_mut`](struct.Query.html#method.iter_combinations_mut "method bevy::prelude::Query::iter_combinations_mut")\] | O(nCr) |
| [`single`](struct.Query.html#method.single "method bevy::prelude::Query::single")\[[`_mut`](struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut")\] | O(a) |
| [`get`](struct.Query.html#method.get "method bevy::prelude::Query::get")\[[`_mut`](struct.Query.html#method.get_mut "method bevy::prelude::Query::get_mut")\] | O(1) |
| [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") | O(k) |
| [`get_many_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut") | O(k2) |
| Archetype-based filtering ([`With`](struct.With.html "struct bevy::prelude::With"), [`Without`](struct.Without.html "struct bevy::prelude::Without"), [`Or`](struct.Or.html "struct bevy::prelude::Or")) | O(a) |
| Change detection filtering ([`Added`](struct.Added.html "struct bevy::prelude::Added"), [`Changed`](struct.Changed.html "struct bevy::prelude::Changed"), [`Spawned`](../ecs/query/struct.Spawned.html "struct bevy::ecs::query::Spawned")) | O(a + n) |

## `Iterator::for_each`

The `for_each` methods appear to be generally faster than `for`\-loops when run on worlds with high archetype fragmentation, and may enable additional optimizations like [autovectorization](https://en.wikipedia.org/wiki/Automatic_vectorization). It is strongly advised to only use [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") if it tangibly improves performance. _Always_ profile or benchmark before and after the change!

```rust
fn system(query: Query<&ComponentA>) {
    // This may result in better performance...
    query.iter().for_each(|component| {
        // ...
    });

    // ...than this. Always benchmark to validate the difference!
    for component in query.iter() {
        // ...
    }
}
```

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#12)

### impl<'w, 's, D, F> [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#15-17)

#### pub fn [related](#method.related)<R>(&'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](struct.Entity.html "struct bevy::prelude::Entity")\>

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

If the given `entity` contains the `R` [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") component, returns the target entity of that relationship.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#24-29)

#### pub fn [relationship\_sources](#method.relationship_sources)<S>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](struct.Entity.html "struct bevy::prelude::Entity")\> + 'w

where S: [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

If the given `entity` contains the `S` [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component, returns the source entities stored on that component.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#43-45)

#### pub fn [root\_ancestor](#method.root_ancestor)<R>(&'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

Recursively walks up the tree defined by the given `R` [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") until there are no more related entities, returning the “root entity” of the relationship hierarchy.

##### Warning

For relationship graphs that contain loops, this could loop infinitely. If your relationship is not a tree (like Bevy’s hierarchy), be sure to stop if you encounter a duplicate entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#60-66)

#### pub fn [iter\_leaves](#method.iter_leaves)<S>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](struct.Entity.html "struct bevy::prelude::Entity")\> + use<'w, 's, S, D, F>

where S: [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, <<S as [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")\>::[Collection](trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection") as [RelationshipSourceCollection](../ecs/relationship/trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](../ecs/relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'w>: [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Iterates all “leaf entities” as defined by the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") hierarchy.

##### Warning

For relationship graphs that contain loops, this could loop infinitely. If your relationship is not a tree (like Bevy’s hierarchy), be sure to stop if you encounter a duplicate entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#78-83)

#### pub fn [iter\_siblings](#method.iter_siblings)<R>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](struct.Entity.html "struct bevy::prelude::Entity")\> + 'w

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'w R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'w <R as [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship")\>::[RelationshipTarget](../ecs/relationship/trait.Relationship.html#associatedtype.RelationshipTarget "type bevy::ecs::relationship::Relationship::RelationshipTarget")\>)>,

Iterates all sibling entities that also have the `R` [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") with the same target entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#101-106)

#### pub fn [iter\_descendants](#method.iter_descendants)<S>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [DescendantIter](../ecs/relationship/struct.DescendantIter.html "struct bevy::ecs::relationship::DescendantIter")<'w, 's, D, F, S> [ⓘ](#)

where S: [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

Iterates all descendant entities as defined by the given `entity`’s [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") and their recursive [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget").

##### Warning

For relationship graphs that contain loops, this could loop infinitely. If your relationship is not a tree (like Bevy’s hierarchy), be sure to stop if you encounter a duplicate entity.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/gltf/update\_gltf\_scene.rs ([line 65](../../src/update_gltf_scene/update_gltf_scene.rs.html#65))

```rust
57fn move_scene_entities(
58    time: Res<Time>,
59    moved_scene: Query<Entity, With<MovedScene>>,
60    children: Query<&Children>,
61    mut transforms: Query<&mut Transform>,
62) {
63    for moved_scene_entity in &moved_scene {
64        let mut offset = 0.;
65        for entity in children.iter_descendants(moved_scene_entity) {
66            if let Ok(mut transform) = transforms.get_mut(entity) {
67                transform.translation = Vec3::new(
68                    offset * ops::sin(time.elapsed_secs()) / 20.,
69                    0.,
70                    ops::cos(time.elapsed_secs()) / 20.,
71                );
72                offset += 0.5;
73            }
74        }
75    }
76}
```

Hide additional examples

examples/animation/morph\_targets.rs ([line 66](../../src/morph_targets/morph_targets.rs.html#66))

```rust
58fn play_animation_when_ready(
59    scene_ready: On<WorldInstanceReady>,
60    mut commands: Commands,
61    children: Query<&Children>,
62    animations_to_play: Query<&AnimationToPlay>,
63    mut players: Query<&mut AnimationPlayer>,
64) {
65    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
66        for child in children.iter_descendants(scene_ready.entity) {
67            if let Ok(mut player) = players.get_mut(child) {
68                player.play(animation_to_play.index).repeat();
69
70                commands
71                    .entity(child)
72                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
73            }
74        }
75    }
76}
```

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 105](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#105))

```rust
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
```

examples/stress\_tests/many\_foxes.rs ([line 259](../../src/many_foxes/many_foxes.rs.html#259))

```rust
251fn setup_scene_once_loaded(
252    scene_ready: On<WorldInstanceReady>,
253    animations: Res<Animations>,
254    foxes: Res<Foxes>,
255    mut commands: Commands,
256    children: Query<&Children>,
257    mut players: Query<&mut AnimationPlayer>,
258) {
259    for child in children.iter_descendants(scene_ready.entity) {
260        if let Ok(mut player) = players.get_mut(child) {
261            let playing_animation = player.play(animations.node_indices[0]).repeat();
262            if !foxes.sync {
263                playing_animation.seek_to(scene_ready.entity.index_u32() as f32 / 10.0);
264            }
265            commands.entity(child).insert((
266                AnimationGraphHandle(animations.graph.clone()),
267                AnimationTransitions::default(),
268            ));
269        }
270    }
271}
```

examples/testbed/3d.rs ([line 351](../../src/testbed_3d/3d.rs.html#351))

```rust
344    fn pause_animation_frame(
345        scene_ready: On<WorldInstanceReady>,
346        children: Query<&Children>,
347        mut commands: Commands,
348        animation: Res<Animation>,
349        mut players: Query<(Entity, &mut AnimationPlayer)>,
350    ) {
351        for child in children.iter_descendants(scene_ready.entity) {
352            if let Ok((entity, mut player)) = players.get_mut(child) {
353                let mut transitions = AnimationTransitions::new();
354                transitions
355                    .play(&mut player, animation.animation, Duration::ZERO)
356                    .seek_to(0.5)
357                    .pause();
358
359                commands
360                    .entity(entity)
361                    .insert(AnimationGraphHandle(animation.graph.clone()))
362                    .insert(transitions);
363            }
364        }
365    }
366}
367
368mod gizmos {
369    use bevy::{color::palettes::css::*, prelude::*};
370
371    pub fn setup(mut commands: Commands) {
372        commands.spawn((
373            Camera3d::default(),
374            Transform::from_xyz(-1.0, 2.5, 6.5).looking_at(Vec3::ZERO, Vec3::Y),
375            DespawnOnExit(super::Scene::Gizmos),
376        ));
377    }
378
379    pub fn draw_gizmos(mut gizmos: Gizmos) {
380        gizmos.cube(
381            Transform::from_translation(Vec3::X * -1.75).with_scale(Vec3::splat(1.25)),
382            RED,
383        );
384        gizmos
385            .sphere(Isometry3d::from_translation(Vec3::X * -3.5), 0.75, GREEN)
386            .resolution(30_000 / 3);
387
388        gizmos.text(
389            Isometry3d::from_translation(Vec3::Y * 1.5),
390            "text gizmo",
391            0.3,
392            Vec2 { x: 0., y: 0. },
393            Color::WHITE,
394        );
395
396        // 3d grids with all variations of outer edges on or off
397        for i in 0..8 {
398            let x = 1.5 * (i % 4) as f32;
399            let y = 1.0 * (0.5 - (i / 4) as f32);
400            let mut grid = gizmos.grid_3d(
401                Isometry3d::from_translation(Vec3::new(x, y, 0.0)),
402                UVec3::new(5, 4, 3),
403                Vec3::splat(0.175),
404                Color::WHITE,
405            );
406            if i & 1 > 0 {
407                grid = grid.outer_edges_x();
408            }
409            if i & 2 > 0 {
410                grid = grid.outer_edges_y();
411            }
412            if i & 4 > 0 {
413                grid.outer_edges_z();
414            }
415        }
416    }
417}
418
419mod gltf_coordinate_conversion {
420    use bevy::{
421        color::palettes::basic::*,
422        gltf::{convert_coordinates::GltfConvertCoordinates, GltfLoaderSettings},
423        prelude::*,
424        world_serialization::WorldInstanceReady,
425    };
426
427    const CURRENT_SCENE: super::Scene = super::Scene::GltfCoordinateConversion;
428
429    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
430        commands.spawn((
431            Camera3d::default(),
432            Transform::from_xyz(-4.0, 4.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
433            DespawnOnExit(CURRENT_SCENE),
434        ));
435
436        commands.spawn((
437            DirectionalLight {
438                color: BLUE.into(),
439                ..default()
440            },
441            Transform::IDENTITY.looking_to(Dir3::Z, Dir3::Y),
442            DespawnOnExit(CURRENT_SCENE),
443        ));
444
445        commands.spawn((
446            DirectionalLight {
447                color: RED.into(),
448                ..default()
449            },
450            Transform::IDENTITY.looking_to(Dir3::X, Dir3::Y),
451            DespawnOnExit(CURRENT_SCENE),
452        ));
453
454        commands.spawn((
455            DirectionalLight {
456                color: GREEN.into(),
457                ..default()
458            },
459            Transform::IDENTITY.looking_to(Dir3::NEG_Y, Dir3::X),
460            DespawnOnExit(CURRENT_SCENE),
461        ));
462
463        commands
464            .spawn((
465                WorldAssetRoot(
466                    asset_server
467                        .load_builder()
468                        .with_settings(|s: &mut GltfLoaderSettings| {
469                            s.convert_coordinates = Some(GltfConvertCoordinates {
470                                rotate_scene_entity: true,
471                                rotate_meshes: true,
472                            });
473                        })
474                        .load(GltfAssetLabel::Scene(0).from_asset("models/Faces/faces.glb")),
475                ),
476                DespawnOnExit(CURRENT_SCENE),
477            ))
478            .observe(show_aabbs);
479    }
480
481    pub fn show_aabbs(
482        scene_ready: On<WorldInstanceReady>,
483        mut commands: Commands,
484        children: Query<&Children>,
485        meshes: Query<(), With<Mesh3d>>,
486    ) {
487        for child in children
488            .iter_descendants(scene_ready.entity)
489            .filter(|&e| meshes.contains(e))
490        {
491            commands.entity(child).insert(ShowAabbGizmo {
492                color: Some(BLACK.into()),
493            });
494        }
495    }
```

examples/stress\_tests/many\_morph\_targets.rs ([line 377](../../src/many_morph_targets/many_morph_targets.rs.html#377))

```rust
366fn play_animation(
367    trigger: On<WorldInstanceReady>,
368    mut commands: Commands,
369    args: Res<Args>,
370    children: Query<&Children>,
371    animations_to_play: Query<&AnimationToPlay>,
372    mut players: Query<&mut AnimationPlayer>,
373) {
374    if args.weights == ArgWeights::Animated
375        && let Ok(animation_to_play) = animations_to_play.get(trigger.entity)
376    {
377        for child in children.iter_descendants(trigger.entity) {
378            if let Ok(mut player) = players.get_mut(child) {
379                commands
380                    .entity(child)
381                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
382
383                player
384                    .play(animation_to_play.index)
385                    .repeat()
386                    .set_speed(animation_to_play.speed);
387            }
388        }
389    }
390}
391
392fn set_weights(
393    trigger: On<WorldInstanceReady>,
394    args: Res<Args>,
395    children: Query<&Children>,
396    mut weight_components: Query<&mut MorphWeights>,
397) {
398    if let Some(weight_value) = match args.weights {
399        ArgWeights::One => Some(1.0),
400        ArgWeights::Zero => Some(0.0),
401        ArgWeights::Tiny => Some(0.00001),
402        _ => None,
403    } {
404        for child in children.iter_descendants(trigger.entity) {
405            if let Ok(mut weight_component) = weight_components.get_mut(child) {
406                weight_component.weights_mut().fill(weight_value);
407            }
408        }
409    }
410}
```

Additional examples can be found in:  

*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#68)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#289)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#474)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#276)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#81)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#72)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#387)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#118-124)

#### pub fn [iter\_descendants\_depth\_first](#method.iter_descendants_depth_first)<S>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [DescendantDepthFirstIter](../ecs/relationship/struct.DescendantDepthFirstIter.html "struct bevy::ecs::relationship::DescendantDepthFirstIter")<'w, 's, D, F, S> [ⓘ](#)

where S: [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, <<S as [RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")\>::[Collection](trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection") as [RelationshipSourceCollection](../ecs/relationship/trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](../ecs/relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'w>: [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Iterates all descendant entities as defined by the given `entity`’s [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") and their recursive [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") in depth-first order.

##### Warning

For relationship graphs that contain loops, this could loop infinitely. If your relationship is not a tree (like Bevy’s hierarchy), be sure to stop if you encounter a duplicate entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_query.rs.html#135-140)

#### pub fn [iter\_ancestors](#method.iter_ancestors)<R>( &'w self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [AncestorIter](../ecs/relationship/struct.AncestorIter.html "struct bevy::ecs::relationship::AncestorIter")<'w, 's, D, F, R> [ⓘ](#)

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")<Item<'w, 's> = [&'w R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

Iterates all ancestors of the given `entity` as defined by the `R` [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship").

##### Warning

For relationship graphs that contain loops, this could loop infinitely. If your relationship is not a tree (like Bevy’s hierarchy), be sure to stop if you encounter a duplicate entity.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ecs/relationships.rs ([line 168](../../src/relationships/relationships.rs.html#168))

```rust
152    fn check_for_cycles(
153        // We want to check every entity for cycles
154        query_to_check: Query<Entity, With<Targeting>>,
155        // Fetch the names for easier debugging.
156        name_query: Query<&Name>,
157        // The targeting_query allows us to traverse the relationship graph.
158        targeting_query: Query<&Targeting>,
159    ) -> Result<(), TargetingCycle> {
160        for initial_entity in query_to_check.iter() {
161            let mut visited = EntityHashSet::new();
162            let mut targeting_name = name_query.get(initial_entity).unwrap().clone();
163            println!("Checking for cycles starting at {targeting_name}",);
164
165            // There's all sorts of methods like this; check the `Query` docs for more!
166            // This would also be easy to do by just manually checking the `Targeting` component,
167            // and calling `query.get(targeted_entity)` on the entity that it targets in a loop.
168            for targeting in targeting_query.iter_ancestors(initial_entity) {
169                let target_name = name_query.get(targeting).unwrap();
170                println!("{targeting_name} is targeting {target_name}",);
171                targeting_name = target_name.clone();
172
173                if !visited.insert(targeting) {
174                    return Err(TargetingCycle {
175                        initial_entity,
176                        visited,
177                    });
178                }
179            }
180        }
181
182        // If we've checked all the entities and haven't found a cycle, we're good!
183        Ok(())
184    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#515)

### impl<'w, 's, D, F> [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#547)

#### pub fn [as\_readonly](#method.as_readonly)(&self) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Returns another `Query` from this that fetches the read-only version of the query items.

For example, `Query<(&mut D1, &D2, &mut D3), With<F>>` will become `Query<(&D1, &D2, &D3), With<F>>`. This can be useful when working around the borrow checker, or reusing functionality between systems via functions that accept query types.

##### See also

[`into_readonly`](struct.Query.html#method.into_readonly "method bevy::prelude::Query::into_readonly") for a version that consumes the `Query` to return one with the full `'world` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#577)

#### pub fn [into\_readonly](#method.into_readonly)(self) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Returns another `Query` from this that fetches the read-only version of the query items.

For example, `Query<(&mut D1, &D2, &mut D3), With<F>>` will become `Query<(&D1, &D2, &D3), With<F>>`. This can be useful when working around the borrow checker, or reusing functionality between systems via functions that accept query types.

##### See also

[`as_readonly`](struct.Query.html#method.as_readonly "method bevy::prelude::Query::as_readonly") for a version that borrows the `Query` instead of consuming it.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#609)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F>

Returns a new `Query` reborrowing the access from this one. The current query will be unusable while the new one exists.

##### Example

For example this allows to call other methods or other systems that require an owned `Query` without completely giving up ownership of it.

```rust
fn helper_system(query: Query<&ComponentA>) { /* ... */}

fn system(mut query: Query<&ComponentA>) {
    helper_system(query.reborrow());
    // Can still use query here:
    for component in &query {
        // ...
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#626)

#### pub unsafe fn [reborrow\_unsafe](#method.reborrow_unsafe)(&self) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F>

Returns a new `Query` reborrowing the access from this one. The current query will still be usable while the new one exists, but must not be used in a way that violates aliasing.

##### Safety

This function makes it possible to violate Rust’s aliasing guarantees. You must make sure this call does not result in a mutable or shared reference to a component with a mutable reference.

##### See also

*   [`reborrow`](struct.Query.html#method.reborrow "method bevy::prelude::Query::reborrow") for the safe versions.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#678)

#### pub fn [iter](#method.iter)(&self) -> [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the read-only query items.

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

##### Example

Here, the `report_names_system` iterates over the `Player` component of every entity that contains it:

```rust
fn report_names_system(query: Query<&Player>) {
    for player in &query {
        println!("Say hello to {}!", player.name);
    }
}
```

##### See also

[`iter_mut`](struct.Query.html#method.iter_mut "method bevy::prelude::Query::iter_mut") for mutable query items.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ecs/system\_param.rs ([line 31](../../src/system_param/system_param.rs.html#31))

```rust
30    fn count(&mut self) {
31        self.count.0 = self.players.iter().len();
32    }
```

Hide additional examples

tests/window/desktop\_request\_redraw.rs ([line 104](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#104))

```rust
103fn redraw(mut commands: Commands, query: Query<Entity, With<AnimationActive>>) {
104    if query.iter().next().is_some() {
105        commands.write_message(RequestRedraw);
106    }
107}
```

examples/usage/context\_menu.rs ([line 76](../../src/context_menu/context_menu.rs.html#76))

```rust
71fn on_trigger_close_menus(
72    _event: On<CloseContextMenus>,
73    mut commands: Commands,
74    menus: Query<Entity, With<ContextMenu>>,
75) {
76    for e in menus.iter() {
77        commands.entity(e).despawn();
78    }
79}
```

examples/app/headless\_renderer.rs ([line 315](../../src/headless_renderer/headless_renderer.rs.html#315))

```rust
313fn image_copy_extract(mut commands: Commands, image_copiers: Extract<Query<&ImageCopier>>) {
314    commands.insert_resource(ImageCopiers(
315        image_copiers.iter().cloned().collect::<Vec<ImageCopier>>(),
316    ));
317}
318
319// Copies image content from render target to buffer
320fn image_copy_driver(
321    render_context: RenderContext,
322    image_copiers: Res<ImageCopiers>,
323    render_queue: Res<RenderQueue>,
324    gpu_images: Res<RenderAssets<bevy::render::texture::GpuImage>>,
325) {
326    for image_copier in image_copiers.iter() {
327        if !image_copier.enabled() {
328            continue;
329        }
330
331        let src_image = gpu_images.get(&image_copier.src_image).unwrap();
332
333        let mut encoder = render_context
334            .render_device()
335            .create_command_encoder(&CommandEncoderDescriptor::default());
336
337        let block_dimensions = src_image.texture_descriptor.format.block_dimensions();
338        let block_size = src_image
339            .texture_descriptor
340            .format
341            .block_copy_size(None)
342            .unwrap();
343
344        // Calculating correct size of image row because
345        // copy_texture_to_buffer can copy image only by rows aligned wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
346        // That's why image in buffer can be little bit wider
347        // This should be taken into account at copy from buffer stage
348        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
349            (src_image.texture_descriptor.size.width as usize / block_dimensions.0 as usize)
350                * block_size as usize,
351        );
352
353        encoder.copy_texture_to_buffer(
354            src_image.texture.as_image_copy(),
355            TexelCopyBufferInfo {
356                buffer: &image_copier.buffer,
357                layout: TexelCopyBufferLayout {
358                    offset: 0,
359                    bytes_per_row: Some(
360                        std::num::NonZero::<u32>::new(padded_bytes_per_row as u32)
361                            .unwrap()
362                            .into(),
363                    ),
364                    rows_per_image: None,
365                },
366            },
367            src_image.texture_descriptor.size,
368        );
369
370        render_queue.submit(std::iter::once(encoder.finish()));
371    }
372}
373
374/// runs in render world after Render stage to send image from buffer via channel (receiver is in main world)
375fn receive_image_from_buffer(
376    image_copiers: Res<ImageCopiers>,
377    render_device: Res<RenderDevice>,
378    sender: Res<RenderWorldSender>,
379) {
380    for image_copier in image_copiers.0.iter() {
381        if !image_copier.enabled() {
382            continue;
383        }
384
385        // Finally time to get our data back from the gpu.
386        // First we get a buffer slice which represents a chunk of the buffer (which we
387        // can't access yet).
388        // We want the whole thing so use unbounded range.
389        let buffer_slice = image_copier.buffer.slice(..);
390
391        // Now things get complicated. WebGPU, for safety reasons, only allows either the GPU
392        // or CPU to access a buffer's contents at a time. We need to "map" the buffer which means
393        // flipping ownership of the buffer over to the CPU and making access legal. We do this
394        // with `BufferSlice::map_async`.
395        //
396        // The problem is that map_async is not an async function so we can't await it. What
397        // we need to do instead is pass in a closure that will be executed when the slice is
398        // either mapped or the mapping has failed.
399        //
400        // The problem with this is that we don't have a reliable way to wait in the main
401        // code for the buffer to be mapped and even worse, calling get_mapped_range or
402        // get_mapped_range_mut prematurely will cause a panic, not return an error.
403        //
404        // Using channels solves this as awaiting the receiving of a message from
405        // the passed closure will force the outside code to wait. It also doesn't hurt
406        // if the closure finishes before the outside code catches up as the message is
407        // buffered and receiving will just pick that up.
408        //
409        // It may also be worth noting that although on native, the usage of asynchronous
410        // channels is wholly unnecessary, for the sake of portability to Wasm
411        // we'll use async channels that work on both native and Wasm.
412
413        let (s, r) = crossbeam_channel::bounded(1);
414
415        // Maps the buffer so it can be read on the cpu
416        buffer_slice.map_async(MapMode::Read, move |r| match r {
417            // This will execute once the gpu is ready, so after the call to poll()
418            Ok(r) => s.send(r).expect("Failed to send map update"),
419            Err(err) => panic!("Failed to map buffer {err}"),
420        });
421
422        // In order for the mapping to be completed, one of three things must happen.
423        // One of those can be calling `Device::poll`. This isn't necessary on the web as devices
424        // are polled automatically but natively, we need to make sure this happens manually.
425        // `Maintain::Wait` will cause the thread to wait on native but not on WebGpu.
426
427        // This blocks until the gpu is done executing everything
428        render_device
429            .poll(PollType::wait_indefinitely())
430            .expect("Failed to poll device for map async");
431
432        // This blocks until the buffer is mapped
433        r.recv().expect("Failed to receive the map_async message");
434
435        // This could fail on app exit, if Main world clears resources (including receiver) while Render world still renders
436        let _ = sender.send(buffer_slice.get_mapped_range().to_vec());
437
438        // We need to make sure all `BufferView`'s are dropped before we do what we're about
439        // to do.
440        // Unmap so that we can copy to the staging buffer in the next iteration.
441        image_copier.buffer.unmap();
442    }
443}
444
445/// CPU-side image for saving
446#[derive(Component, Deref, DerefMut)]
447struct ImageToSave(Handle<Image>);
448
449// Takes from channel image content sent from render world and saves it to disk
450fn update(
451    images_to_save: Query<&ImageToSave>,
452    receiver: Res<MainWorldReceiver>,
453    mut images: ResMut<Assets<Image>>,
454    mut scene_controller: ResMut<SceneController>,
455    mut app_exit_writer: MessageWriter<AppExit>,
456    mut file_number: Local<u32>,
457) {
458    if let SceneState::Render(n) = scene_controller.state {
459        if n < 1 {
460            // We don't want to block the main world on this,
461            // so we use try_recv which attempts to receive without blocking
462            let mut image_data = Vec::new();
463            while let Ok(data) = receiver.try_recv() {
464                // image generation could be faster than saving to fs,
465                // that's why use only last of them
466                image_data = data;
467            }
468            if !image_data.is_empty() {
469                for image in images_to_save.iter() {
470                    // Fill correct data from channel to image
471                    let mut img_bytes = images.get_mut(image.id()).unwrap();
472
473                    // We need to ensure that this works regardless of the image dimensions
474                    // If the image became wider when copying from the texture to the buffer,
475                    // then the data is reduced to its original size when copying from the buffer to the image.
476                    let row_bytes = img_bytes.width() as usize
477                        * img_bytes.texture_descriptor.format.pixel_size().unwrap();
478                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
479                    if row_bytes == aligned_row_bytes {
480                        img_bytes.data.as_mut().unwrap().clone_from(&image_data);
481                    } else {
482                        // shrink data to original image size
483                        img_bytes.data = Some(
484                            image_data
485                                .chunks(aligned_row_bytes)
486                                .take(img_bytes.height() as usize)
487                                .flat_map(|row| &row[..row_bytes.min(row.len())])
488                                .cloned()
489                                .collect(),
490                        );
491                    }
492
493                    // Create RGBA Image Buffer
494                    let img = match img_bytes.clone().try_into_dynamic() {
495                        Ok(img) => img.to_rgba8(),
496                        Err(e) => panic!("Failed to create image buffer {e:?}"),
497                    };
498
499                    // Prepare directory for images, test_images in bevy folder is used here for example
500                    // You should choose the path depending on your needs
501                    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_images");
502                    info!("Saving image to: {images_dir:?}");
503                    std::fs::create_dir_all(&images_dir).unwrap();
504
505                    // Choose filename starting from 000.png
506                    let image_path = images_dir.join(format!("{:03}.png", file_number.deref()));
507                    *file_number.deref_mut() += 1;
508
509                    // Finally saving image to file, this heavy blocking operation is kept here
510                    // for example simplicity, but in real app you should move it to a separate task
511                    if let Err(e) = img.save(image_path) {
512                        panic!("Failed to save image: {e}");
513                    };
514                }
515                if scene_controller.single_image {
516                    app_exit_writer.write(AppExit::Success);
517                }
518            }
519        } else {
520            // clears channel for skipped frames
521            while receiver.try_recv().is_ok() {}
522            scene_controller.state = SceneState::Render(n - 1);
523        }
524    }
525}
```

examples/stress\_tests/many\_animated\_sprites.rs ([line 139](../../src/many_animated_sprites/many_animated_sprites.rs.html#139))

```rust
135fn print_sprite_count(time: Res<Time>, mut timer: Local<PrintingTimer>, sprites: Query<&Sprite>) {
136    timer.tick(time.delta());
137
138    if timer.just_finished() {
139        info!("Sprites: {}", sprites.iter().count());
140    }
141}
```

examples/stress\_tests/many\_sprites.rs ([line 123](../../src/many_sprites/many_sprites.rs.html#123))

```rust
119fn print_sprite_count(time: Res<Time>, mut timer: Local<PrintingTimer>, sprites: Query<&Sprite>) {
120    timer.tick(time.delta());
121
122    if timer.just_finished() {
123        info!("Sprites: {}", sprites.iter().count());
124    }
125}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#146)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#145)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#129)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#76)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#127)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#72)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#45)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#573)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#273)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#176)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#240)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#52)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#36)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#195)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#167)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#94)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#115)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#155)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#433)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#121)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#323)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#695)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#330)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#67)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#188)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#78)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#122)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#164)
*   [examples/ecs/callbacks.rs](../../src/callbacks/callbacks.rs.html#30)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#42)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#174)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#103)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#426)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#144)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#71)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#191)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#116)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#367)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#430)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#288)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#66)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#231)
*   [examples/ecs/relationships.rs](../../src/relationships/relationships.rs.html#85)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#26)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#468)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#275)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#421)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#73)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#49)
*   [examples/stress\_tests/many\_components.rs](../../src/many_components/many_components.rs.html#40)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#332)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#415)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#555)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#140)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#26)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#495)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#243)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#819)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#305)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#752)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#716)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'\_, 's, D, F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items.

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

If the [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") does not implement [`IterQueryData`](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), then it is not sound to yield multiple items concurrently and the resulting [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter") will not implement [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"). To iterate over the items in that case, use the [`QueryIter::fetch_next()`](../ecs/query/struct.QueryIter.html#method.fetch_next "method bevy::ecs::query::QueryIter::fetch_next") method, which ensures only one item is alive at a time.

##### Example

Here, the `gravity_system` updates the `Velocity` component of every entity that contains it:

```rust
fn gravity_system(mut query: Query<&mut Velocity>) {
    const DELTA: f32 = 1.0 / 60.0;
    for mut velocity in &mut query {
        velocity.y -= 9.8 * DELTA;
    }
}
```

##### See also

[`iter`](struct.Query.html#method.iter "method bevy::prelude::Query::iter") for read-only query items.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 305](../../src/irradiance_volumes/irradiance_volumes.rs.html#305))

```rust
304fn update_text(mut text_query: Query<&mut Text>, app_status: Res<AppStatus>) {
305    for mut text in text_query.iter_mut() {
306        *text = app_status.create_text();
307    }
308}
309
310impl AppStatus {
311    // Constructs the help text at the bottom of the screen based on the
312    // application status.
313    fn create_text(&self) -> Text {
314        let irradiance_volume_help_text = if self.irradiance_volume_present {
315            DISABLE_IRRADIANCE_VOLUME_HELP_TEXT
316        } else {
317            ENABLE_IRRADIANCE_VOLUME_HELP_TEXT
318        };
319
320        let voxels_help_text = if self.voxels_visible {
321            HIDE_VOXELS_HELP_TEXT
322        } else {
323            SHOW_VOXELS_HELP_TEXT
324        };
325
326        let rotation_help_text = if self.rotating {
327            STOP_ROTATION_HELP_TEXT
328        } else {
329            START_ROTATION_HELP_TEXT
330        };
331
332        let switch_mesh_help_text = match self.model {
333            ExampleModel::Sphere => SWITCH_TO_FOX_HELP_TEXT,
334            ExampleModel::Fox => SWITCH_TO_SPHERE_HELP_TEXT,
335        };
336
337        format!(
338            "{CLICK_TO_MOVE_HELP_TEXT}\n\
339            {voxels_help_text}\n\
340            {irradiance_volume_help_text}\n\
341            {rotation_help_text}\n\
342            {switch_mesh_help_text}"
343        )
344        .into()
345    }
346}
347
348// Rotates the camera a bit every frame.
349fn rotate_camera(
350    mut camera_query: Query<&mut Transform, With<Camera3d>>,
351    time: Res<Time>,
352    app_status: Res<AppStatus>,
353) {
354    if !app_status.rotating {
355        return;
356    }
357
358    for mut transform in camera_query.iter_mut() {
359        transform.translation = Vec2::from_angle(ROTATION_SPEED * time.delta_secs())
360            .rotate(transform.translation.xz())
361            .extend(transform.translation.y)
362            .xzy();
363        transform.look_at(Vec3::ZERO, Vec3::Y);
364    }
365}
366
367// Toggles between the unskinned sphere model and the skinned fox model if the
368// user requests it.
369fn change_main_object(
370    keyboard: Res<ButtonInput<KeyCode>>,
371    mut app_status: ResMut<AppStatus>,
372    mut sphere_query: Query<
373        &mut Visibility,
374        (With<MainObject>, With<Mesh3d>, Without<WorldAssetRoot>),
375    >,
376    mut fox_query: Query<&mut Visibility, (With<MainObject>, With<WorldAssetRoot>)>,
377) {
378    if !keyboard.just_pressed(KeyCode::Tab) {
379        return;
380    }
381    let Some(mut sphere_visibility) = sphere_query.iter_mut().next() else {
382        return;
383    };
384    let Some(mut fox_visibility) = fox_query.iter_mut().next() else {
385        return;
386    };
387
388    match app_status.model {
389        ExampleModel::Sphere => {
390            *sphere_visibility = Visibility::Hidden;
391            *fox_visibility = Visibility::Visible;
392            app_status.model = ExampleModel::Fox;
393        }
394        ExampleModel::Fox => {
395            *sphere_visibility = Visibility::Visible;
396            *fox_visibility = Visibility::Hidden;
397            app_status.model = ExampleModel::Sphere;
398        }
399    }
400}
401
402impl Default for AppStatus {
403    fn default() -> Self {
404        Self {
405            irradiance_volume_present: true,
406            rotating: true,
407            model: ExampleModel::Sphere,
408            voxels_visible: false,
409        }
410    }
411}
412
413// Turns on and off the irradiance volume as requested by the user.
414fn toggle_irradiance_volumes(
415    mut commands: Commands,
416    keyboard: Res<ButtonInput<KeyCode>>,
417    light_probe_query: Query<Entity, With<LightProbe>>,
418    mut app_status: ResMut<AppStatus>,
419    assets: Res<ExampleAssets>,
420    mut ambient_light: ResMut<GlobalAmbientLight>,
421) {
422    if !keyboard.just_pressed(KeyCode::Space) {
423        return;
424    };
425
426    let Some(light_probe) = light_probe_query.iter().next() else {
427        return;
428    };
429
430    if app_status.irradiance_volume_present {
431        commands.entity(light_probe).remove::<IrradianceVolume>();
432        ambient_light.brightness = AMBIENT_LIGHT_BRIGHTNESS * IRRADIANCE_VOLUME_INTENSITY;
433        app_status.irradiance_volume_present = false;
434    } else {
435        commands.entity(light_probe).insert(IrradianceVolume {
436            voxels: assets.irradiance_volume.clone(),
437            intensity: IRRADIANCE_VOLUME_INTENSITY,
438            ..default()
439        });
440        ambient_light.brightness = 0.0;
441        app_status.irradiance_volume_present = true;
442    }
443}
444
445fn toggle_rotation(keyboard: Res<ButtonInput<KeyCode>>, mut app_status: ResMut<AppStatus>) {
446    if keyboard.just_pressed(KeyCode::Enter) {
447        app_status.rotating = !app_status.rotating;
448    }
449}
450
451// Handles clicks on the plane that reposition the object.
452fn handle_mouse_clicks(
453    buttons: Res<ButtonInput<MouseButton>>,
454    windows: Query<&Window, With<PrimaryWindow>>,
455    cameras: Query<(&Camera, &GlobalTransform)>,
456    mut main_objects: Query<&mut Transform, With<MainObject>>,
457) {
458    if !buttons.pressed(MouseButton::Left) {
459        return;
460    }
461    let Some(mouse_position) = windows.iter().next().and_then(Window::cursor_position) else {
462        return;
463    };
464    let Some((camera, camera_transform)) = cameras.iter().next() else {
465        return;
466    };
467
468    // Figure out where the user clicked on the plane.
469    let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position) else {
470        return;
471    };
472    let Some(plane_intersection) =
473        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
474    else {
475        return;
476    };
477    // Move all the main objects.
478    for mut transform in main_objects.iter_mut() {
479        transform.translation = vec3(
480            plane_intersection.x,
481            transform.translation.y,
482            plane_intersection.z,
483        );
484    }
485}
486
487impl FromWorld for ExampleAssets {
488    fn from_world(world: &mut World) -> Self {
489        let fox_animation =
490            world.load_asset(GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb"));
491        let (fox_animation_graph, fox_animation_node) =
492            AnimationGraph::from_clip(fox_animation.clone());
493
494        ExampleAssets {
495            main_sphere: world.add_asset(Sphere::default().mesh().uv(32, 18)),
496            fox: world.load_asset(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb")),
497            main_sphere_material: world.add_asset(Color::from(SILVER)),
498            main_scene: world.load_asset(
499                GltfAssetLabel::Scene(0)
500                    .from_asset("models/IrradianceVolumeExample/IrradianceVolumeExample.glb"),
501            ),
502            irradiance_volume: world.load_asset("irradiance_volumes/Example.vxgi.ktx2"),
503            fox_animation_graph: world.add_asset(fox_animation_graph),
504            fox_animation_node,
505            voxel_cube: world.add_asset(Cuboid::default()),
506            // Just use a specular map for the skybox since it's not too blurry.
507            // In reality you wouldn't do this--you'd use a real skybox texture--but
508            // reusing the textures like this saves space in the Bevy repository.
509            skybox: world.load_asset("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
510        }
511    }
512}
513
514// Plays the animation on the fox.
515fn play_animations(
516    mut commands: Commands,
517    assets: Res<ExampleAssets>,
518    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
519) {
520    for (entity, mut player) in players.iter_mut() {
521        commands
522            .entity(entity)
523            .insert(AnimationGraphHandle(assets.fox_animation_graph.clone()));
524        player.play(assets.fox_animation_node).repeat();
525    }
526}
527
528fn create_cubes(
529    image_assets: Res<Assets<Image>>,
530    mut commands: Commands,
531    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
532    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
533    voxel_cubes: Query<Entity, With<VoxelCube>>,
534    example_assets: Res<ExampleAssets>,
535    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
536) {
537    // If voxel cubes have already been spawned, don't do anything.
538    if !voxel_cubes.is_empty() {
539        return;
540    }
541
542    let Some(voxel_cube_parent) = voxel_cube_parents.iter().next() else {
543        return;
544    };
545
546    for (irradiance_volume, global_transform) in irradiance_volumes.iter() {
547        let Some(image) = image_assets.get(&irradiance_volume.voxels) else {
548            continue;
549        };
550
551        let resolution = image.texture_descriptor.size;
552
553        let voxel_cube_material = voxel_visualization_material_assets.add(ExtendedMaterial {
554            base: StandardMaterial::from(Color::from(RED)),
555            extension: VoxelVisualizationExtension {
556                irradiance_volume_info: VoxelVisualizationIrradianceVolumeInfo {
557                    world_from_voxel: VOXEL_FROM_WORLD.inverse(),
558                    voxel_from_world: VOXEL_FROM_WORLD,
559                    resolution: uvec3(
560                        resolution.width,
561                        resolution.height,
562                        resolution.depth_or_array_layers,
563                    ),
564                    intensity: IRRADIANCE_VOLUME_INTENSITY,
565                },
566            },
567        });
568
569        let scale = vec3(
570            1.0 / resolution.width as f32,
571            1.0 / resolution.height as f32,
572            1.0 / resolution.depth_or_array_layers as f32,
573        );
574
575        // Spawn a cube for each voxel.
576        for z in 0..resolution.depth_or_array_layers {
577            for y in 0..resolution.height {
578                for x in 0..resolution.width {
579                    let uvw = (uvec3(x, y, z).as_vec3() + 0.5) * scale - 0.5;
580                    let pos = global_transform.transform_point(uvw);
581                    let voxel_cube = commands
582                        .spawn((
583                            Mesh3d(example_assets.voxel_cube.clone()),
584                            MeshMaterial3d(voxel_cube_material.clone()),
585                            Transform::from_scale(Vec3::splat(VOXEL_CUBE_SCALE))
586                                .with_translation(pos),
587                        ))
588                        .insert(VoxelCube)
589                        .insert(NotShadowCaster)
590                        .id();
591
592                    commands.entity(voxel_cube_parent).add_child(voxel_cube);
593                }
594            }
595        }
596    }
597}
598
599// Draws a gizmo showing the bounds of the irradiance volume.
600fn draw_gizmo(
601    mut gizmos: Gizmos,
602    irradiance_volume_query: Query<&GlobalTransform, With<IrradianceVolume>>,
603    app_status: Res<AppStatus>,
604) {
605    if app_status.voxels_visible {
606        for transform in irradiance_volume_query.iter() {
607            gizmos.cube(*transform, GIZMO_COLOR);
608        }
609    }
610}
611
612// Handles a request from the user to toggle the voxel visibility on and off.
613fn toggle_voxel_visibility(
614    keyboard: Res<ButtonInput<KeyCode>>,
615    mut app_status: ResMut<AppStatus>,
616    mut voxel_cube_parent_query: Query<&mut Visibility, With<VoxelCubeParent>>,
617) {
618    if !keyboard.just_pressed(KeyCode::Backspace) {
619        return;
620    }
621
622    app_status.voxels_visible = !app_status.voxels_visible;
623
624    for mut visibility in voxel_cube_parent_query.iter_mut() {
625        *visibility = if app_status.voxels_visible {
626            Visibility::Visible
627        } else {
628            Visibility::Hidden
629        };
630    }
631}
```

Hide additional examples

examples/3d/reflection\_probes.rs ([line 281](../../src/reflection_probes/reflection_probes.rs.html#281))

```rust
280fn update_text(mut text_query: Query<&mut Text>, app_status: Res<AppStatus>) {
281    for mut text in text_query.iter_mut() {
282        *text = app_status.create_text();
283    }
284}
285
286impl TryFrom<u32> for ReflectionMode {
287    type Error = ();
288
289    fn try_from(value: u32) -> Result<Self, Self::Error> {
290        match value {
291            0 => Ok(ReflectionMode::EnvironmentMap),
292            1 => Ok(ReflectionMode::ReflectionProbe),
293            2 => Ok(ReflectionMode::GeneratedEnvironmentMap),
294            _ => Err(()),
295        }
296    }
297}
298
299impl Display for ReflectionMode {
300    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
301        let text = match *self {
302            ReflectionMode::EnvironmentMap => "Environment map",
303            ReflectionMode::ReflectionProbe => "Reflection probe",
304            ReflectionMode::GeneratedEnvironmentMap => "Generated environment map",
305        };
306        formatter.write_str(text)
307    }
308}
309
310impl AppStatus {
311    // Constructs the help text at the bottom of the screen based on the
312    // application status.
313    fn create_text(&self) -> Text {
314        let rotation_help_text = if self.rotating {
315            STOP_ROTATION_HELP_TEXT
316        } else {
317            START_ROTATION_HELP_TEXT
318        };
319
320        format!(
321            "{}\n{}\nRoughness: {:.2}\n{}\nUp/Down arrows to change roughness",
322            self.reflection_mode,
323            rotation_help_text,
324            self.sphere_roughness,
325            REFLECTION_MODE_HELP_TEXT
326        )
327        .into()
328    }
329}
330
331// Creates the world environment map light, used as a fallback if no reflection
332// probe is applicable to a mesh.
333fn create_camera_environment_map_light(cubemaps: &Cubemaps) -> EnvironmentMapLight {
334    EnvironmentMapLight {
335        diffuse_map: cubemaps.diffuse_environment_map.clone(),
336        specular_map: cubemaps.specular_environment_map.clone(),
337        intensity: ENV_MAP_INTENSITY,
338        ..default()
339    }
340}
341
342// Rotates the camera a bit every frame.
343fn rotate_camera(
344    time: Res<Time>,
345    mut camera_query: Query<&mut Transform, With<Camera3d>>,
346    app_status: Res<AppStatus>,
347) {
348    if !app_status.rotating {
349        return;
350    }
351
352    for mut transform in camera_query.iter_mut() {
353        transform.translation = Vec2::from_angle(time.delta_secs() * PI / 5.0)
354            .rotate(transform.translation.xz())
355            .extend(transform.translation.y)
356            .xzy();
357        transform.look_at(Vec3::ZERO, Vec3::Y);
358    }
359}
```

examples/math/bounding\_2d.rs ([line 40](../../src/bounding_2d/bounding_2d.rs.html#40))

```rust
39fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
40    for mut transform in query.iter_mut() {
41        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 5.);
42    }
43}
44
45#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
46enum Test {
47    AabbSweep,
48    CircleSweep,
49    #[default]
50    RayCast,
51    AabbCast,
52    CircleCast,
53}
54
55fn update_test_state(
56    keycode: Res<ButtonInput<KeyCode>>,
57    cur_state: Res<State<Test>>,
58    mut state: ResMut<NextState<Test>>,
59) {
60    if !keycode.just_pressed(KeyCode::Space) {
61        return;
62    }
63
64    use Test::*;
65    let next = match **cur_state {
66        AabbSweep => CircleSweep,
67        CircleSweep => RayCast,
68        RayCast => AabbCast,
69        AabbCast => CircleCast,
70        CircleCast => AabbSweep,
71    };
72    state.set(next);
73}
74
75fn update_text(mut text: Single<&mut Text>, cur_state: Res<State<Test>>) {
76    if !cur_state.is_changed() {
77        return;
78    }
79
80    text.clear();
81
82    text.push_str("Intersection test:\n");
83    use Test::*;
84    for &test in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
85        let s = if **cur_state == test { "*" } else { " " };
86        text.push_str(&format!(" {s} {test:?} {s}\n"));
87    }
88    text.push_str("\nPress space to cycle");
89}
90
91#[derive(Component)]
92enum Shape {
93    Rectangle(Rectangle),
94    Circle(Circle),
95    Triangle(Triangle2d),
96    Line(Segment2d),
97    Capsule(Capsule2d),
98    Polygon(RegularPolygon),
99}
100
101fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
102    let color = GRAY;
103    for (shape, transform) in query.iter() {
104        let translation = transform.translation.xy();
105        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
106        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
107        match shape {
108            Shape::Rectangle(r) => {
109                gizmos.primitive_2d(r, isometry, color);
110            }
111            Shape::Circle(c) => {
112                gizmos.primitive_2d(c, isometry, color);
113            }
114            Shape::Triangle(t) => {
115                gizmos.primitive_2d(t, isometry, color);
116            }
117            Shape::Line(l) => {
118                gizmos.primitive_2d(l, isometry, color);
119            }
120            Shape::Capsule(c) => {
121                gizmos.primitive_2d(c, isometry, color);
122            }
123            Shape::Polygon(p) => {
124                gizmos.primitive_2d(p, isometry, color);
125            }
126        }
127    }
128}
129
130#[derive(Component)]
131enum DesiredVolume {
132    Aabb,
133    Circle,
134}
135
136#[derive(Component, Debug)]
137enum CurrentVolume {
138    Aabb(Aabb2d),
139    Circle(BoundingCircle),
140}
141
142fn update_volumes(
143    mut commands: Commands,
144    query: Query<
145        (Entity, &DesiredVolume, &Shape, &Transform),
146        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>,
147    >,
148) {
149    for (entity, desired_volume, shape, transform) in query.iter() {
150        let translation = transform.translation.xy();
151        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
152        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
153        match desired_volume {
154            DesiredVolume::Aabb => {
155                let aabb = match shape {
156                    Shape::Rectangle(r) => r.aabb_2d(isometry),
157                    Shape::Circle(c) => c.aabb_2d(isometry),
158                    Shape::Triangle(t) => t.aabb_2d(isometry),
159                    Shape::Line(l) => l.aabb_2d(isometry),
160                    Shape::Capsule(c) => c.aabb_2d(isometry),
161                    Shape::Polygon(p) => p.aabb_2d(isometry),
162                };
163                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
164            }
165            DesiredVolume::Circle => {
166                let circle = match shape {
167                    Shape::Rectangle(r) => r.bounding_circle(isometry),
168                    Shape::Circle(c) => c.bounding_circle(isometry),
169                    Shape::Triangle(t) => t.bounding_circle(isometry),
170                    Shape::Line(l) => l.bounding_circle(isometry),
171                    Shape::Capsule(c) => c.bounding_circle(isometry),
172                    Shape::Polygon(p) => p.bounding_circle(isometry),
173                };
174                commands
175                    .entity(entity)
176                    .insert(CurrentVolume::Circle(circle));
177            }
178        }
179    }
180}
181
182fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
183    for (volume, intersects) in query.iter() {
184        let color = if **intersects { AQUA } else { ORANGE_RED };
185        match volume {
186            CurrentVolume::Aabb(a) => {
187                gizmos.rect_2d(a.center(), a.half_size() * 2., color);
188            }
189            CurrentVolume::Circle(c) => {
190                gizmos.circle_2d(c.center(), c.radius(), color);
191            }
192        }
193    }
194}
195
196#[derive(Component, Deref, DerefMut, Default)]
197struct Intersects(bool);
198
199const OFFSET_X: f32 = 125.;
200const OFFSET_Y: f32 = 75.;
201
202fn setup(mut commands: Commands) {
203    commands.spawn(Camera2d);
204
205    commands.spawn((
206        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
207        Shape::Circle(Circle::new(45.)),
208        DesiredVolume::Aabb,
209        Intersects::default(),
210    ));
211
212    commands.spawn((
213        Transform::from_xyz(0., OFFSET_Y, 0.),
214        Shape::Rectangle(Rectangle::new(80., 80.)),
215        Spin,
216        DesiredVolume::Circle,
217        Intersects::default(),
218    ));
219
220    commands.spawn((
221        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.),
222        Shape::Triangle(Triangle2d::new(
223            Vec2::new(-40., -40.),
224            Vec2::new(-20., 40.),
225            Vec2::new(40., 50.),
226        )),
227        Spin,
228        DesiredVolume::Aabb,
229        Intersects::default(),
230    ));
231
232    commands.spawn((
233        Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.),
234        Shape::Line(Segment2d::from_direction_and_length(
235            Dir2::from_xy(1., 0.3).unwrap(),
236            90.,
237        )),
238        Spin,
239        DesiredVolume::Circle,
240        Intersects::default(),
241    ));
242
243    commands.spawn((
244        Transform::from_xyz(0., -OFFSET_Y, 0.),
245        Shape::Capsule(Capsule2d::new(25., 50.)),
246        Spin,
247        DesiredVolume::Aabb,
248        Intersects::default(),
249    ));
250
251    commands.spawn((
252        Transform::from_xyz(OFFSET_X, -OFFSET_Y, 0.),
253        Shape::Polygon(RegularPolygon::new(50., 6)),
254        Spin,
255        DesiredVolume::Circle,
256        Intersects::default(),
257    ));
258
259    commands.spawn((
260        Text::default(),
261        Node {
262            position_type: PositionType::Absolute,
263            top: px(12),
264            left: px(12),
265            ..default()
266        },
267    ));
268}
269
270fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
271    for r in [1., 2., 3.] {
272        gizmos.circle_2d(position, r, color);
273    }
274}
275
276fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
277    gizmos.line_2d(
278        ray.ray.origin,
279        ray.ray.origin + *ray.ray.direction * ray.max,
280        WHITE,
281    );
282    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
283}
284
285fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
286    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
287    let dist = 150. + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.;
288
289    let aabb_ray = Ray2d {
290        origin: ray * 250.,
291        direction: Dir2::new_unchecked(-ray),
292    };
293    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);
294
295    draw_ray(gizmos, &ray_cast);
296    ray_cast
297}
298
299fn ray_cast_system(
300    mut gizmos: Gizmos,
301    time: Res<Time>,
302    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
303) {
304    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
305
306    for (volume, mut intersects) in volumes.iter_mut() {
307        let toi = match volume {
308            CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
309            CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
310        };
311        **intersects = toi.is_some();
312        if let Some(toi) = toi {
313            draw_filled_circle(
314                &mut gizmos,
315                ray_cast.ray.origin + *ray_cast.ray.direction * toi,
316                LIME,
317            );
318        }
319    }
320}
321
322fn aabb_cast_system(
323    mut gizmos: Gizmos,
324    time: Res<Time>,
325    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
326) {
327    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
328    let aabb_cast = AabbCast2d {
329        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
330        ray: ray_cast,
331    };
332
333    for (volume, mut intersects) in volumes.iter_mut() {
334        let toi = match *volume {
335            CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
336            CurrentVolume::Circle(_) => None,
337        };
338
339        **intersects = toi.is_some();
340        if let Some(toi) = toi {
341            gizmos.rect_2d(
342                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
343                aabb_cast.aabb.half_size() * 2.,
344                LIME,
345            );
346        }
347    }
348}
349
350fn bounding_circle_cast_system(
351    mut gizmos: Gizmos,
352    time: Res<Time>,
353    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
354) {
355    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
356    let circle_cast = BoundingCircleCast {
357        circle: BoundingCircle::new(Vec2::ZERO, 15.),
358        ray: ray_cast,
359    };
360
361    for (volume, mut intersects) in volumes.iter_mut() {
362        let toi = match *volume {
363            CurrentVolume::Aabb(_) => None,
364            CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
365        };
366
367        **intersects = toi.is_some();
368        if let Some(toi) = toi {
369            gizmos.circle_2d(
370                circle_cast.ray.ray.origin + *circle_cast.ray.ray.direction * toi,
371                circle_cast.circle.radius(),
372                LIME,
373            );
374        }
375    }
376}
377
378fn get_intersection_position(time: &Time) -> Vec2 {
379    let x = ops::cos(0.8 * time.elapsed_secs()) * 250.;
380    let y = ops::sin(0.4 * time.elapsed_secs()) * 100.;
381    Vec2::new(x, y)
382}
383
384fn aabb_intersection_system(
385    mut gizmos: Gizmos,
386    time: Res<Time>,
387    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
388) {
389    let center = get_intersection_position(&time);
390    let aabb = Aabb2d::new(center, Vec2::splat(50.));
391    gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);
392
393    for (volume, mut intersects) in volumes.iter_mut() {
394        let hit = match volume {
395            CurrentVolume::Aabb(a) => aabb.intersects(a),
396            CurrentVolume::Circle(c) => aabb.intersects(c),
397        };
398
399        **intersects = hit;
400    }
401}
402
403fn circle_intersection_system(
404    mut gizmos: Gizmos,
405    time: Res<Time>,
406    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
407) {
408    let center = get_intersection_position(&time);
409    let circle = BoundingCircle::new(center, 50.);
410    gizmos.circle_2d(center, circle.radius(), YELLOW);
411
412    for (volume, mut intersects) in volumes.iter_mut() {
413        let hit = match volume {
414            CurrentVolume::Aabb(a) => circle.intersects(a),
415            CurrentVolume::Circle(c) => circle.intersects(c),
416        };
417
418        **intersects = hit;
419    }
420}
```

examples/3d/scrolling\_fog.rs ([line 123](../../src/scrolling_fog/scrolling_fog.rs.html#123))

```rust
122fn scroll_fog(time: Res<Time>, mut query: Query<&mut FogVolume>) {
123    for mut fog_volume in query.iter_mut() {
124        fog_volume.density_texture_offset += Vec3::new(0.0, 0.0, 0.04) * time.delta_secs();
125    }
126}
```

examples/stress\_tests/many\_cameras\_lights.rs ([line 100](../../src/many_cameras_lights/many_cameras_lights.rs.html#100))

```rust
99fn rotate_cameras(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
100    for mut transform in query.iter_mut() {
101        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs()));
102    }
103}
```

examples/shader/shader\_prepass.rs ([line 174](../../src/shader_prepass/shader_prepass.rs.html#174))

```rust
173fn rotate(mut q: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
174    for mut t in q.iter_mut() {
175        let rot = (ops::sin(time.elapsed_secs()) * 0.5 + 0.5) * std::f32::consts::PI * 2.0;
176        t.rotation = Quat::from_rotation_z(rot);
177    }
178}
```

Additional examples can be found in:  

*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#83)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#298)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#277)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#224)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#90)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#627)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#155)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#136)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#78)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#75)
*   [examples/ecs/contiguous\_query.rs](../../src/contiguous_query/contiguous_query.rs.html#40)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#102)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#156)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#313)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#451)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#140)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#187)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#257)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#160)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#43)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#246)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#264)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#161)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#452)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#98)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#248)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#100)
*   [examples/3d/../helpers/widgets.rs](../../src/clustered_decal_maps/helpers/widgets.rs.html#173)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#514)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#383)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#121)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#106)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#127)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#54)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#138)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#113)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#115)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#83)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#389)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#79)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#89)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#289)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#169)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#281)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#94)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#352)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#133)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#115)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#191)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#266)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#409)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#431)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#501)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#482)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#413)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#51)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#448)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#197)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#423)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#304)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#106)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#350)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#269)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#751)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#195)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#222)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#751)

#### pub fn [iter\_inner](#method.iter_inner)(self) -> [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, D, F> [ⓘ](#)

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items, with the actual “inner” world lifetime.

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

If the [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") does not implement [`IterQueryData`](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), then it is not sound to yield multiple items concurrently and the resulting [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter") will not implement [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"). To iterate over the items in that case, use the [`QueryIter::fetch_next()`](../ecs/query/struct.QueryIter.html#method.fetch_next "method bevy::ecs::query::QueryIter::fetch_next") method, which ensures only one item is alive at a time.

##### Example

Here, the `report_names_system` iterates over the `Player` component of every entity that contains it:

```rust
fn report_names_system(query: Query<&Player>) {
    for player in &query {
        println!("Say hello to {}!", player.name);
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#783-785)

#### pub fn [iter\_combinations](#method.iter_combinations)<const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [QueryCombinationIter](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, K> [ⓘ](#)

Returns a [`QueryCombinationIter`](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter") over all combinations of `K` read-only query items without repetition.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

##### Example

```rust
fn some_system(query: Query<&ComponentA>) {
    for [a1, a2] in query.iter_combinations() {
        // ...
    }
}
```

##### See also

*   [`iter_combinations_mut`](struct.Query.html#method.iter_combinations_mut "method bevy::prelude::Query::iter_combinations_mut") for mutable query item combinations.
*   [`iter_combinations_inner`](struct.Query.html#method.iter_combinations_inner "method bevy::prelude::Query::iter_combinations_inner") for mutable query item combinations with the full `'world` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#813-815)

#### pub fn [iter\_combinations\_mut](#method.iter_combinations_mut)<const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> [QueryCombinationIter](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'\_, 's, D, F, K> [ⓘ](#)

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a [`QueryCombinationIter`](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter") over all combinations of `K` query items without repetition.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

##### Example

```rust
fn some_system(mut query: Query<&mut ComponentA>) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = combinations.fetch_next() {
        // mutably access components data
    }
}
```

##### See also

*   [`iter_combinations`](struct.Query.html#method.iter_combinations "method bevy::prelude::Query::iter_combinations") for read-only query item combinations.
*   [`iter_combinations_inner`](struct.Query.html#method.iter_combinations_inner "method bevy::prelude::Query::iter_combinations_inner") for mutable query item combinations with the full `'world` lifetime.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/ecs/iter\_combinations.rs ([line 123](../../src/iter_combinations/iter_combinations.rs.html#123))

```rust
122fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
123    let mut iter = query.iter_combinations_mut();
124    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
125        iter.fetch_next()
126    {
127        let delta = transform2.translation() - transform1.translation();
128        let distance_sq: f32 = delta.length_squared();
129
130        let f = GRAVITY_CONSTANT / distance_sq;
131        let force_unit_mass = delta * f;
132        acc1.0 += force_unit_mass * *m2;
133        acc2.0 -= force_unit_mass * *m1;
134    }
135}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#845-847)

#### pub fn [iter\_combinations\_inner](#method.iter_combinations_inner)<const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( self, ) -> [QueryCombinationIter](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'w, 's, D, F, K> [ⓘ](#)

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a [`QueryCombinationIter`](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter") over all combinations of `K` query items without repetition. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

##### Example

```rust
fn some_system(query: Query<&mut ComponentA>) {
    let mut combinations = query.iter_combinations_inner();
    while let Some([mut a1, mut a2]) = combinations.fetch_next() {
        // mutably access components data
    }
}
```

##### See also

*   [`iter_combinations`](struct.Query.html#method.iter_combinations "method bevy::prelude::Query::iter_combinations") for read-only query item combinations.
*   [`iter_combinations_mut`](struct.Query.html#method.iter_combinations_mut "method bevy::prelude::Query::iter_combinations_mut") for mutable query item combinations.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#891-894)

#### pub fn [iter\_many](#method.iter_many)<EntityList>( &self, entities: EntityList, ) -> [QueryManyIter](../ecs/query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the read-only query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities, and may not be unique if the input doesn’t guarantee uniqueness. Entities that don’t match the query are skipped.

##### Example

```rust
// A component containing an entity list.
#[derive(Component)]
struct Friends {
    list: Vec<Entity>,
}

fn system(
    friends_query: Query<&Friends>,
    counter_query: Query<&Counter>,
) {
    for friends in &friends_query {
        for counter in counter_query.iter_many(&friends.list) {
            println!("Friend's counter: {}", counter.value);
        }
    }
}
```

##### See also

*   [`iter_many_mut`](struct.Query.html#method.iter_many_mut "method bevy::prelude::Query::iter_many_mut") to get mutable query items.
*   [`iter_many_inner`](struct.Query.html#method.iter_many_inner "method bevy::prelude::Query::iter_many_inner") to get mutable query items with the full `'world` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#936-939)

#### pub fn [iter\_many\_mut](#method.iter_many_mut)<EntityList>( &mut self, entities: EntityList, ) -> [QueryManyIter](../ecs/query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'\_, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an iterator over the query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities, and may not be unique if the input doesn’t guarantee uniqueness. Entities that don’t match the query are skipped.

##### Examples

```rust
#[derive(Component)]
struct Counter {
    value: i32
}

#[derive(Component)]
struct Friends {
    list: Vec<Entity>,
}

fn system(
    friends_query: Query<&Friends>,
    mut counter_query: Query<&mut Counter>,
) {
    for friends in &friends_query {
        let mut iter = counter_query.iter_many_mut(&friends.list);
        while let Some(mut counter) = iter.fetch_next() {
            println!("Friend's counter: {}", counter.value);
            counter.value += 1;
        }
    }
}
```

##### See also

*   [`iter_many`](struct.Query.html#method.iter_many "method bevy::prelude::Query::iter_many") to get read-only query items.
*   [`iter_many_inner`](struct.Query.html#method.iter_many_inner "method bevy::prelude::Query::iter_many_inner") to get mutable query items with the full `'world` lifetime.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/animation/animation\_graph.rs ([line 433](../../src/animation_graph/animation_graph.rs.html#433))

```rust
424fn update_ui(
425    mut text_query: Query<&mut Text>,
426    mut background_query: Query<&mut Node, Without<Text>>,
427    container_query: Query<(&Children, &ClipNode)>,
428    animation_weights_query: Query<&ExampleAnimationWeights, Changed<ExampleAnimationWeights>>,
429) {
430    for animation_weights in animation_weights_query.iter() {
431        for (children, clip_node) in &container_query {
432            // Draw the green background color to visually indicate the weight.
433            let mut bg_iter = background_query.iter_many_mut(children);
434            if let Some(mut node) = bg_iter.fetch_next() {
435                // All nodes are the same width, so `NODE_RECTS[0]` is as good as any other.
436                node.width = px(NODE_RECTS[0].width * animation_weights.weights[clip_node.index]);
437            }
438
439            // Update the node labels with the current weights.
440            let mut text_iter = text_query.iter_many_mut(children);
441            if let Some(mut text) = text_iter.fetch_next() {
442                **text = format!(
443                    "{}\n{:.2}",
444                    clip_node.text, animation_weights.weights[clip_node.index]
445                );
446            }
447        }
448    }
449}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#954-957)

#### pub fn [iter\_many\_inner](#method.iter_many_inner)<EntityList>( self, entities: EntityList, ) -> [QueryManyIter](../ecs/query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'w, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an iterator over the query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

Items are returned in the order of the list of entities, and may not be unique if the input doesn’t guarantee uniqueness. Entities that don’t match the query are skipped.

##### See also

*   [`iter_many`](struct.Query.html#method.iter_many "method bevy::prelude::Query::iter_many") to get read-only query items.
*   [`iter_many_mut`](struct.Query.html#method.iter_many_mut "method bevy::prelude::Query::iter_many_mut") to get mutable query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1019-1022)

#### pub fn [iter\_many\_unique](#method.iter_many_unique)<EntityList>( &self, entities: EntityList, ) -> [QueryManyUniqueIter](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the unique read-only query items generated from an [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

##### Example

```rust
// `Friends` ensures that it only lists unique entities.
#[derive(Component)]
struct Friends {
    unique_list: Vec<Entity>,
}

impl<'a> IntoIterator for &'a Friends {

    type Item = &'a Entity;
    type IntoIter = UniqueEntityIter<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: `Friends` ensures that it unique_list contains only unique entities.
       unsafe { UniqueEntityIter::from_iter_unchecked(self.unique_list.iter()) }
    }
}

fn system(
    friends_query: Query<&Friends>,
    counter_query: Query<&Counter>,
) {
    for friends in &friends_query {
        for counter in counter_query.iter_many_unique(friends) {
            println!("Friend's counter: {:?}", counter.value);
        }
    }
}
```

##### See also

*   [`iter_many_unique_mut`](struct.Query.html#method.iter_many_unique_mut "method bevy::prelude::Query::iter_many_unique_mut") to get mutable query items.
*   [`iter_many_unique_inner`](struct.Query.html#method.iter_many_unique_inner "method bevy::prelude::Query::iter_many_unique_inner") to get with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1074-1079)

#### pub fn [iter\_many\_unique\_mut](#method.iter_many_unique_mut)<EntityList>( &mut self, entities: EntityList, ) -> [QueryManyUniqueIter](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'\_, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an iterator over the unique query items generated from an [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

##### Examples

```rust
#[derive(Component)]
struct Counter {
    value: i32
}

// `Friends` ensures that it only lists unique entities.
#[derive(Component)]
struct Friends {
    unique_list: Vec<Entity>,
}

impl<'a> IntoIterator for &'a Friends {
    type Item = &'a Entity;
    type IntoIter = UniqueEntityIter<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: `Friends` ensures that it unique_list contains only unique entities.
        unsafe { UniqueEntityIter::from_iter_unchecked(self.unique_list.iter()) }
    }
}

fn system(
    friends_query: Query<&Friends>,
    mut counter_query: Query<&mut Counter>,
) {
    for friends in &friends_query {
        for mut counter in counter_query.iter_many_unique_mut(friends) {
            println!("Friend's counter: {:?}", counter.value);
            counter.value += 1;
        }
    }
}
```

##### See also

*   [`iter_many_unique`](struct.Query.html#method.iter_many_unique "method bevy::prelude::Query::iter_many_unique") to get read-only query items.
*   [`iter_many_unique_inner`](struct.Query.html#method.iter_many_unique_inner "method bevy::prelude::Query::iter_many_unique_inner") to get with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1132-1137)

#### pub fn [iter\_many\_unique\_inner](#method.iter_many_unique_inner)<EntityList>( self, entities: EntityList, ) -> [QueryManyUniqueIter](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'w, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an iterator over the unique query items generated from an [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

##### Examples

```rust
#[derive(Component)]
struct Counter {
    value: i32
}

// `Friends` ensures that it only lists unique entities.
#[derive(Component)]
struct Friends {
    unique_list: Vec<Entity>,
}

impl<'a> IntoIterator for &'a Friends {
    type Item = &'a Entity;
    type IntoIter = UniqueEntityIter<slice::Iter<'a, Entity>>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: `Friends` ensures that it unique_list contains only unique entities.
        unsafe { UniqueEntityIter::from_iter_unchecked(self.unique_list.iter()) }
    }
}

fn system(
    friends_query: Query<&Friends>,
    mut counter_query: Query<&mut Counter>,
) {
    let friends = friends_query.single().unwrap();
    for mut counter in counter_query.iter_many_unique_inner(friends) {
        println!("Friend's counter: {:?}", counter.value);
        counter.value += 1;
    }
}
```

##### See also

*   [`iter_many_unique`](struct.Query.html#method.iter_many_unique "method bevy::prelude::Query::iter_many_unique") to get read-only query items.
*   [`iter_many_unique_mut`](struct.Query.html#method.iter_many_unique_mut "method bevy::prelude::Query::iter_many_unique_mut") to get mutable query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1172-1174)

#### pub unsafe fn [iter\_unsafe](#method.iter_unsafe)(&self) -> [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'\_, 's, D, F> [ⓘ](#)

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items.

This iterator is always guaranteed to return results from each matching entity once and only once. Iteration order is not guaranteed.

If the [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") does not implement [`IterQueryData`](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), then it is not sound to yield multiple items concurrently and the resulting [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter") will not implement [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"). To iterate over the items in that case, use the [`QueryIter::fetch_next()`](../ecs/query/struct.QueryIter.html#method.fetch_next "method bevy::ecs::query::QueryIter::fetch_next") method, which ensures only one item is alive at a time.

##### Safety

This function makes it possible to violate Rust’s aliasing guarantees. You must make sure this call does not result in multiple mutable references to the same component.

##### See also

*   [`iter`](struct.Query.html#method.iter "method bevy::prelude::Query::iter") and [`iter_mut`](struct.Query.html#method.iter_mut "method bevy::prelude::Query::iter_mut") for the safe versions.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1194-1198)

#### pub unsafe fn [iter\_combinations\_unsafe](#method.iter_combinations_unsafe)<const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, ) -> [QueryCombinationIter](../ecs/query/struct.QueryCombinationIter.html "struct bevy::ecs::query::QueryCombinationIter")<'\_, 's, D, F, K> [ⓘ](#)

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Iterates over all possible combinations of `K` query items without repetition.

This iterator is always guaranteed to return results from each unique pair of matching entities. Iteration order is not guaranteed.

##### Safety

This allows aliased mutability. You must make sure this call does not result in multiple mutable references to the same component.

##### See also

*   [`iter_combinations`](struct.Query.html#method.iter_combinations "method bevy::prelude::Query::iter_combinations") and [`iter_combinations_mut`](struct.Query.html#method.iter_combinations_mut "method bevy::prelude::Query::iter_combinations_mut") for the safe versions.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1218-1221)

#### pub unsafe fn [iter\_many\_unsafe](#method.iter_many_unsafe)<EntityList>( &self, entities: EntityList, ) -> [QueryManyIter](../ecs/query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'\_, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities, and may not be unique if the input doesnn’t guarantee uniqueness. Entities that don’t match the query are skipped.

##### Safety

This allows aliased mutability and does not check for entity uniqueness. You must make sure this call does not result in multiple mutable references to the same component. Particular care must be taken when collecting the data (rather than iterating over it one item at a time) such as via [`Iterator::collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect").

##### See also

*   [`iter_many_mut`](struct.Query.html#method.iter_many_mut "method bevy::prelude::Query::iter_many_mut") to safely access the query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1240-1245)

#### pub unsafe fn [iter\_many\_unique\_unsafe](#method.iter_many_unique_unsafe)<EntityList>( &self, entities: EntityList, ) -> [QueryManyUniqueIter](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'\_, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over the unique query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list.

Items are returned in the order of the list of entities. Entities that don’t match the query are skipped.

##### Safety

This allows aliased mutability. You must make sure this call does not result in multiple mutable references to the same component.

##### See also

*   [`iter_many_unique`](struct.Query.html#method.iter_many_unique "method bevy::prelude::Query::iter_many_unique") to get read-only query items.
*   [`iter_many_unique_mut`](struct.Query.html#method.iter_many_unique_mut "method bevy::prelude::Query::iter_many_unique_mut") to get mutable query items.
*   [`iter_many_unique_inner`](struct.Query.html#method.iter_many_unique_inner "method bevy::prelude::Query::iter_many_unique_inner") to get with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1267)

#### pub fn [par\_iter](#method.par_iter)(&self) -> [QueryParIter](../ecs/query/struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Returns a parallel iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World").

This parallel iterator is always guaranteed to return results from each matching entity once and only once. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter").

This can only be called for read-only queries, see [`par_iter_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut") for write-queries.

Note that you must use the `for_each` method to iterate over the results, see [`par_iter_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut") for an example.

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([line 293](../../src/manual_material/manual_material.rs.html#293))

```rust
270fn check_entities_needing_specialization(
271    needs_specialization: Query<
272        Entity,
273        (
274            Or<(
275                Changed<Mesh3d>,
276                AssetChanged<Mesh3d>,
277                Changed<ImageMaterial3d>,
278                AssetChanged<ImageMaterial3d>,
279            )>,
280            With<ImageMaterial3d>,
281        ),
282    >,
283    mut par_local: Local<Parallel<Vec<Entity>>>,
284    mut entities_needing_specialization: ResMut<EntitiesNeedingSpecialization<ImageMaterial>>,
285    mut removed_mesh_3d_components: RemovedComponents<Mesh3d>,
286    mut removed_mesh_material_3d_components: RemovedComponents<ImageMaterial3d>,
287) {
288    entities_needing_specialization.changed.clear();
289    entities_needing_specialization.removed.clear();
290
291    // Gather all entities that need their specializations regenerated.
292    needs_specialization
293        .par_iter()
294        .for_each(|entity| par_local.borrow_local_mut().push(entity));
295    par_local.drain_into(&mut entities_needing_specialization.changed);
296
297    // All entities that removed their `Mesh3d` or `ImageMaterial3d` components
298    // need to have their specializations removed as well.
299    for entity in removed_mesh_3d_components
300        .read()
301        .chain(removed_mesh_material_3d_components.read())
302    {
303        entities_needing_specialization.removed.push(entity);
304    }
305}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1302-1304)

#### pub fn [par\_iter\_mut](#method.par_iter_mut)(&mut self) -> [QueryParIter](../ecs/query/struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")<'\_, 's, D, F>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a parallel iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World").

This parallel iterator is always guaranteed to return results from each matching entity once and only once. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter").

This can only be called for mutable queries, see [`par_iter`](struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter") for read-only-queries.

##### Example

Here, the `gravity_system` updates the `Velocity` component of every entity that contains it:

```rust
fn gravity_system(mut query: Query<&mut Velocity>) {
    const DELTA: f32 = 1.0 / 60.0;
    query.par_iter_mut().for_each(|mut velocity| {
        velocity.y -= 9.8 * DELTA;
    });
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/stress\_tests/bevymark\_3d.rs ([line 451](../../src/bevymark_3d/bevymark_3d.rs.html#451))

```rust
450fn collision_system(mut cube_query: Query<(&mut Cube, &Transform)>) {
451    cube_query.par_iter_mut().for_each(|(mut cube, transform)| {
452        handle_collision(&transform.translation, &mut cube.velocity);
453    });
454}
```

Hide additional examples

examples/stress\_tests/many\_cubes.rs ([line 602](../../src/many_cubes/many_cubes.rs.html#602))

```rust
598fn rotate_cubes(
599    mut query: Query<&mut Transform, (With<Mesh3d>, Without<NotShadowCaster>)>,
600    time: Res<Time>,
601) {
602    query.par_iter_mut().for_each(|mut transform| {
603        transform.rotate_y(10.0 * time.delta_secs());
604    });
605}
```

examples/ecs/parallel\_query.rs ([line 38](../../src/parallel_query/parallel_query.rs.html#38))

```rust
28fn move_system(mut sprites: Query<(&mut Transform, &Velocity)>) {
29    // Compute the new location of each sprite in parallel on the
30    // ComputeTaskPool
31    //
32    // This example is only for demonstrative purposes. Using a
33    // ParallelIterator for an inexpensive operation like addition on only 128
34    // elements will not typically be faster than just using a normal Iterator.
35    // See the ParallelIterator documentation for more information on when
36    // to use or not use ParallelIterator over a normal Iterator.
37    sprites
38        .par_iter_mut()
39        .for_each(|(mut transform, velocity)| {
40            transform.translation += velocity.extend(0.0);
41        });
42}
43
44// Bounce sprites outside the window
45fn bounce_system(window: Query<&Window>, mut sprites: Query<(&Transform, &mut Velocity)>) {
46    let Ok(window) = window.single() else {
47        return;
48    };
49    let width = window.width();
50    let height = window.height();
51    let left = width / -2.0;
52    let right = width / 2.0;
53    let bottom = height / -2.0;
54    let top = height / 2.0;
55    // The default batch size can also be overridden.
56    // In this case a batch size of 32 is chosen to limit the overhead of
57    // ParallelIterator, since negating a vector is very inexpensive.
58    sprites
59        .par_iter_mut()
60        .batching_strategy(BatchingStrategy::fixed(32))
61        .for_each(|(transform, mut v)| {
62            if !(left < transform.translation.x
63                && transform.translation.x < right
64                && bottom < transform.translation.y
65                && transform.translation.y < top)
66            {
67                // For simplicity, just reverse the velocity; don't use realistic bounces
68                v.0 = -v.0;
69            }
70        });
71}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1336-1338)

#### pub fn [par\_iter\_inner](#method.par_iter_inner)(self) -> [QueryParIter](../ecs/query/struct.QueryParIter.html "struct bevy::ecs::query::QueryParIter")<'w, 's, D, F>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a parallel iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

This parallel iterator is always guaranteed to return results from each matching entity once and only once. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryIter`](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter").

##### Example

Here, the `gravity_system` updates the `Velocity` component of every entity that contains it:

```rust
fn gravity_system(query: Query<&mut Velocity>) {
    const DELTA: f32 = 1.0 / 60.0;
    query.par_iter_inner().for_each(|mut velocity| {
        velocity.y -= 9.8 * DELTA;
    });
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1365-1368)

#### pub fn [par\_iter\_many](#method.par_iter_many)<EntityList>( &self, entities: EntityList, ) -> [QueryParManyIter](../ecs/query/struct.QueryParManyIter.html "struct bevy::ecs::query::QueryParManyIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>

where EntityList: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

Returns a parallel iterator over the read-only query items generated from an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") list.

Entities that don’t match the query are skipped. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryManyIter`](../ecs/query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter").

This can only be called for read-only queries. To avoid potential aliasing, there is no `par_iter_many_mut` equivalent. See [`par_iter_many_unique_mut`](struct.Query.html#method.par_iter_many_unique_mut "method bevy::prelude::Query::par_iter_many_unique_mut") for an alternative using [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Note that you must use the `for_each` method to iterate over the results, see [`par_iter_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut") for an example.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1394-1397)

#### pub fn [par\_iter\_many\_unique](#method.par_iter_many_unique)<EntityList>( &self, entities: EntityList, ) -> [QueryParManyUniqueIter](../ecs/query/struct.QueryParManyUniqueIter.html "struct bevy::ecs::query::QueryParManyUniqueIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

Returns a parallel iterator over the unique read-only query items generated from an [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Entities that don’t match the query are skipped. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryManyUniqueIter`](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter").

This can only be called for read-only queries, see [`par_iter_many_unique_mut`](struct.Query.html#method.par_iter_many_unique_mut "method bevy::prelude::Query::par_iter_many_unique_mut") for write-queries.

Note that you must use the `for_each` method to iterate over the results, see [`par_iter_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut") for an example.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1423-1428)

#### pub fn [par\_iter\_many\_unique\_mut](#method.par_iter_many_unique_mut)<EntityList>( &mut self, entities: EntityList, ) -> [QueryParManyUniqueIter](../ecs/query/struct.QueryParManyUniqueIter.html "struct bevy::ecs::query::QueryParManyUniqueIter")<'\_, 's, D, F, <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>

where EntityList: [EntitySet](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet"), <EntityList as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a parallel iterator over the unique query items generated from an [`EntitySet`](../ecs/entity/trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

Entities that don’t match the query are skipped. Iteration order and thread assignment is not guaranteed.

If the `multithreaded` feature is disabled, iterating with this operates identically to [`Iterator::for_each`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each "method core::iter::traits::iterator::Iterator::for_each") on [`QueryManyUniqueIter`](../ecs/query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter").

This can only be called for mutable queries, see [`par_iter_many_unique`](struct.Query.html#method.par_iter_many_unique "method bevy::prelude::Query::par_iter_many_unique") for read-only-queries.

Note that you must use the `for_each` method to iterate over the results, see [`par_iter_mut`](struct.Query.html#method.par_iter_mut "method bevy::prelude::Query::par_iter_mut") for an example.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1472-1477)

#### pub fn [contiguous\_iter](#method.contiguous_iter)( &self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[QueryContiguousIter](../ecs/query/struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")<'\_, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>, [QueryNotDenseError](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")\>

where <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): [ContiguousQueryData](../ecs/query/trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"), F: [ArchetypeFilter](../ecs/query/trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

Returns a contiguous iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World") or [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") with [`QueryNotDenseError`](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError") if the query is not dense hence not contiguously iterable.

Contiguous iteration enables getting slices of contiguously lying components (which lie in the same table), which for example may be used for simd-operations, which may accelerate an algorithm.

##### Example

The following system despawns all entities which health is negative.

```rust
fn despawn_all_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entities, health) in query.contiguous_iter().unwrap() {
        // For each entity there is one component, hence it always holds true
        assert!(entities.len() == health.len());
        for (entity, health) in entities.iter().zip(health.iter()) {
            if health.0 < 0.0 {
                commands.entity(*entity).despawn();
            }
        }
    }
}
```

A mutable version: [`Self::contiguous_iter_mut`](struct.Query.html#method.contiguous_iter_mut "method bevy::prelude::Query::contiguous_iter_mut")

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/ecs/custom\_query\_param.rs ([line 217](../../src/custom_query_param/custom_query_param.rs.html#217))

```rust
215fn print_components_contiguous_iter(query: Query<CustomContiguousQuery<ComponentC, ComponentD>>) {
216    println!("Print components (contiguous_iter):");
217    for e in query.contiguous_iter().unwrap() {
218        let e: CustomContiguousQueryContiguousItem<'_, '_, _, _> = e;
219        println!("Entity: {:?}", e.entity);
220        println!("A: {:?}", e.a);
221        println!("B: {:?}", e.b);
222        println!(
223            "Generic: {:?} {:?}",
224            e.generic.generic.0, e.generic.generic.1
225        );
226    }
227}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1515-1520)

#### pub fn [contiguous\_iter\_mut](#method.contiguous_iter_mut)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[QueryContiguousIter](../ecs/query/struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")<'\_, 's, D, F>, [QueryNotDenseError](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")\>

where D: [ContiguousQueryData](../ecs/query/trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"), F: [ArchetypeFilter](../ecs/query/trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

Returns a mutable contiguous iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World") or [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") with [`QueryNotDenseError`](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError") if the query is not dense hence not contiguously iterable.

Contiguous iteration enables getting slices of contiguously lying components (which lie in the same table), which for example may be used for simd-operations, which may accelerate an algorithm.

##### Example

The following system applies a “health decay” effect on all entities, which reduces their health by some fraction.

```rust
fn apply_health_decay(mut query: Query<(&mut Health, &HealthDecay)>) {
    for (mut health, decay) in query.contiguous_iter_mut().unwrap() {
        // all data slices returned by component queries are the same size
        assert!(health.len() == decay.len());
        // we could have used health.bypass_change_detection() to do less work.
        for (health, decay) in health.iter_mut().zip(decay) {
            health.0 *= decay.0;
        }
    }
}
```

An immutable version: [`Self::contiguous_iter`](struct.Query.html#method.contiguous_iter "method bevy::prelude::Query::contiguous_iter")

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/ecs/contiguous\_query.rs ([line 28](../../src/contiguous_query/contiguous_query.rs.html#28))

```rust
26fn apply_health_decay(mut query: Query<(&mut Health, &HealthDecay)>) {
27    // contiguous_iter_mut() would return None if query couldn't be iterated contiguously
28    for (mut health, decay) in query.contiguous_iter_mut().unwrap() {
29        // all data slices returned by component queries are the same size
30        assert!(health.len() == decay.len());
31        // we could also bypass change detection via bypass_change_detection() because we do not
32        // use it anyways.
33        for (health, decay) in health.iter_mut().zip(decay) {
34            health.0 *= decay.0;
35        }
36    }
37}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1529-1534)

#### pub fn [contiguous\_iter\_inner](#method.contiguous_iter_inner)( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[QueryContiguousIter](../ecs/query/struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter")<'w, 's, D, F>, [QueryNotDenseError](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError")\>

where D: [ContiguousQueryData](../ecs/query/trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"), F: [ArchetypeFilter](../ecs/query/trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

Returns a contiguous iterator over the query results for the given [`World`](struct.World.html "struct bevy::prelude::World") or [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") with [`QueryNotDenseError`](../ecs/query/struct.QueryNotDenseError.html "struct bevy::ecs::query::QueryNotDenseError") if the query is not dense hence not contiguously iterable. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1577)

#### pub fn [get](#method.get)( &self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>, [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the read-only query item for the given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

This is always guaranteed to run in `O(1)` time.

##### Example

Here, `get` is used to retrieve the exact query item of the entity specified by the `SelectedCharacter` resource.

```rust
fn print_selected_character_name_system(
       query: Query<&Character>,
       selection: Res<SelectedCharacter>
)
{
    if let Ok(selected_character) = query.get(selection.entity) {
        println!("{}", selected_character.name);
    }
}
```

##### See also

*   [`get_mut`](struct.Query.html#method.get_mut "method bevy::prelude::Query::get_mut") to get a mutable query item.

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/ecs/observer\_propagation.rs ([line 80](../../src/observer_propagation/observer_propagation.rs.html#80))

```rust
79fn attack_hits(attack: On<Attack>, name: Query<&Name>) {
80    if let Ok(name) = name.get(attack.entity) {
81        info!("Attack hit {}", name);
82    }
83}
84
85/// A callback placed on [`Armor`], checking if it absorbed all the [`Attack`] damage.
86fn block_attack(mut attack: On<Attack>, armor: Query<(&Armor, &Name)>) {
87    let (armor, name) = armor.get(attack.entity).unwrap();
88    let damage = attack.damage.saturating_sub(**armor);
89    if damage > 0 {
90        info!("🩸 {} damage passed through {}", damage, name);
91        // The attack isn't stopped by the armor. We reduce the damage of the attack, and allow
92        // it to continue on to the goblin.
93        attack.damage = damage;
94    } else {
95        info!("🛡️  {} damage blocked by {}", attack.damage, name);
96        // Armor stopped the attack, the event stops here.
97        attack.propagate(false);
98        info!("(propagation halted early)\n");
99    }
100}
```

Hide additional examples

examples/ui/navigation/directional\_navigation.rs ([line 374](../../src/directional_navigation/directional_navigation.rs.html#374))

```rust
367fn update_focus_display(
368    input_focus: Res<InputFocus>,
369    button_query: Query<&Name, With<Button>>,
370    mut display_query: Query<&mut Text, With<FocusDisplay>>,
371) {
372    if let Ok(mut text) = display_query.single_mut() {
373        if let Some(focused_entity) = input_focus.get() {
374            if let Ok(name) = button_query.get(focused_entity) {
375                **text = format!("Focused: {}", name);
376            } else {
377                **text = "Focused: Unknown".to_string();
378            }
379        } else {
380            **text = "Focused: None".to_string();
381        }
382    }
383}
```

examples/usage/context\_menu.rs ([line 46](../../src/context_menu/context_menu.rs.html#46))

```rust
40fn text_color_on_hover<T: Debug + Clone + Reflect>(
41    color: Color,
42) -> impl FnMut(On<Pointer<T>>, Query<&mut TextColor>, Query<&Children>) {
43    move |mut event: On<Pointer<T>>,
44          mut text_color: Query<&mut TextColor>,
45          children: Query<&Children>| {
46        let Ok(children) = children.get(event.original_event_target()) else {
47            return;
48        };
49        event.propagate(false);
50
51        // find the text among children and change its color
52        for child in children.iter() {
53            if let Ok(mut col) = text_color.get_mut(child) {
54                col.0 = color;
55            }
56        }
57    }
58}
59
60fn setup(mut commands: Commands) {
61    commands.spawn(Camera2d);
62
63    commands.spawn(background_and_button()).observe(
64        // any click bubbling up here should lead to closing any open menu
65        |_: On<Pointer<Press>>, mut commands: Commands| {
66            commands.trigger(CloseContextMenus);
67        },
68    );
69}
70
71fn on_trigger_close_menus(
72    _event: On<CloseContextMenus>,
73    mut commands: Commands,
74    menus: Query<Entity, With<ContextMenu>>,
75) {
76    for e in menus.iter() {
77        commands.entity(e).despawn();
78    }
79}
80
81fn on_trigger_menu(event: On<OpenContextMenu>, mut commands: Commands) {
82    commands.trigger(CloseContextMenus);
83
84    let pos = event.pos;
85
86    debug!("open context menu at: {pos}");
87
88    commands
89        .spawn((
90            Name::new("context menu"),
91            ContextMenu,
92            Node {
93                position_type: PositionType::Absolute,
94                left: px(pos.x),
95                top: px(pos.y),
96                flex_direction: FlexDirection::Column,
97                border_radius: BorderRadius::all(px(4)),
98                ..default()
99            },
100            BorderColor::all(Color::BLACK),
101            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
102            children![
103                context_item("fuchsia", basic::FUCHSIA),
104                context_item("gray", basic::GRAY),
105                context_item("maroon", basic::MAROON),
106                context_item("purple", basic::PURPLE),
107                context_item("teal", basic::TEAL),
108            ],
109        ))
110        .observe(
111            |event: On<Pointer<Press>>,
112             menu_items: Query<&ContextMenuItem>,
113             mut clear_col: ResMut<ClearColor>,
114             mut commands: Commands| {
115                let target = event.original_event_target();
116
117                if let Ok(item) = menu_items.get(target) {
118                    clear_col.0 = item.0.into();
119                    commands.trigger(CloseContextMenus);
120                }
121            },
122        );
123}
```

examples/animation/morph\_targets.rs ([line 65](../../src/morph_targets/morph_targets.rs.html#65))

```rust
58fn play_animation_when_ready(
59    scene_ready: On<WorldInstanceReady>,
60    mut commands: Commands,
61    children: Query<&Children>,
62    animations_to_play: Query<&AnimationToPlay>,
63    mut players: Query<&mut AnimationPlayer>,
64) {
65    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
66        for child in children.iter_descendants(scene_ready.entity) {
67            if let Ok(mut player) = players.get_mut(child) {
68                player.play(animation_to_play.index).repeat();
69
70                commands
71                    .entity(child)
72                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
73            }
74        }
75    }
76}
```

examples/ui/text/multiple\_text\_inputs.rs ([line 244](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#244))

```rust
233fn update_row_border_colors(
234    input_focus: Res<InputFocus>,
235    input_rows: Query<&TextInputRow, With<EditableText>>,
236    mut row_borders: Query<(&TextInputRow, &mut BorderColor, Has<EditableText>)>,
237) {
238    if !input_focus.is_changed() {
239        return;
240    }
241
242    let focused_row = input_focus
243        .get()
244        .and_then(|focused_entity| input_rows.get(focused_entity).ok())
245        .map(|row| row.0);
246
247    for (row, mut border_color, is_input) in &mut row_borders {
248        let mut color = if is_input {
249            SLATE_300.into()
250        } else {
251            Color::WHITE
252        };
253        if Some(row.0) != focused_row {
254            color = color.darker(0.75);
255        }
256        border_color.set_all(color);
257    }
258}
```

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 104](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#104))

```rust
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
```

Additional examples can be found in:  

*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#119)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#375)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#473)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#168)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#285)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#56)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#52)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#337)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#403)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#216)
*   [examples/ecs/relationships.rs](../../src/relationships/relationships.rs.html#86)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#117)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#76)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#200)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#180)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#28)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#278)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#66)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#737)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#289)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#321)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#389)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#579)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#844)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#94)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1625-1628)

#### pub fn [get\_many](#method.get_many)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, entities: \[[Entity](struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the read-only query items for the given array of [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

The returned query items are in the same order as the input. In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead. The elements of the array do not need to be unique, unlike `get_many_mut`.

##### Examples

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();
let entity_vec: Vec<Entity> = (0..3).map(|i| world.spawn(A(i)).id()).collect();
let entities: [Entity; 3] = entity_vec.try_into().unwrap();

world.spawn(A(73));

let mut query_state = world.query::<&A>();
let query = query_state.query(&world);

let component_values = query.get_many(entities).unwrap();

assert_eq!(component_values, [&A(0), &A(1), &A(2)]);

let wrong_entity = Entity::from_raw_u32(365).unwrap();

assert_eq!(
    match query.get_many([wrong_entity]).unwrap_err() {
        QueryEntityError::NotSpawned(error) => error.entity(),
        _ => panic!(),
    },
    wrong_entity
);
```

##### See also

*   [`get_many_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut") to get mutable query items.
*   [`get_many_unique`](struct.Query.html#method.get_many_unique "method bevy::prelude::Query::get_many_unique") to only handle unique inputs.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1676-1679)

#### pub fn [get\_many\_unique](#method.get_many_unique)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &self, entities: [UniqueEntityEquivalentArray](../ecs/entity/struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<[Entity](struct.Entity.html "struct bevy::prelude::Entity"), N>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the read-only query items for the given [`UniqueEntityArray`](../ecs/entity/type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray").

The returned query items are in the same order as the input. In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

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
let query = query_state.query(&world);

let component_values = query.get_many_unique(entity_set).unwrap();

assert_eq!(component_values, [&A(0), &A(1), &A(2)]);

let wrong_entity = Entity::from_raw_u32(365).unwrap();

assert_eq!(
    match query.get_many_unique(UniqueEntityArray::from([wrong_entity])).unwrap_err() {
        QueryEntityError::NotSpawned(error) => error.entity(),
        _ => panic!(),
    },
    wrong_entity
);
```

##### See also

*   [`get_many_unique_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut") to get mutable query items.
*   [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") to handle inputs with duplicates.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1713)

#### pub fn [get\_mut](#method.get_mut)( &mut self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>, [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the query item for the given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

This is always guaranteed to run in `O(1)` time.

##### Example

Here, `get_mut` is used to retrieve the exact query item of the entity specified by the `PoisonedCharacter` resource.

```rust
fn poison_system(mut query: Query<&mut Health>, poisoned: Res<PoisonedCharacter>) {
    if let Ok(mut health) = query.get_mut(poisoned.character_id) {
        health.0 -= 1;
    }
}
```

##### See also

*   [`get`](struct.Query.html#method.get "method bevy::prelude::Query::get") to get a read-only query item.

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([line 262](../../src/asset_saving/asset_saving.rs.html#262))

```rust
258fn on_enter_selectable(
259    event: On<Pointer<Enter>>,
260    mut border: Query<&mut BorderColor, (With<SelectableColor>, Without<Selected>)>,
261) {
262    let Ok(mut border) = border.get_mut(event.entity) else {
263        return;
264    };
265
266    *border = BorderColor::all(HIGHLIGHT_COLOR);
267}
268
269fn on_leave_selectable(
270    event: On<Pointer<Leave>>,
271    mut border: Query<&mut BorderColor, (With<SelectableColor>, Without<Selected>)>,
272) {
273    let Ok(mut border) = border.get_mut(event.entity) else {
274        return;
275    };
276
277    *border = BorderColor::all(NORMAL_COLOR);
278}
279
280fn on_press_selectable(
281    event: On<Pointer<Press>>,
282    mut borders: Query<(Entity, &mut BorderColor, &BackgroundColor), With<SelectableColor>>,
283    mut draw_color: ResMut<DrawColor>,
284    mut commands: Commands,
285) {
286    if !borders.contains(event.entity) {
287        return;
288    }
289    for (entity, mut border, _) in borders.iter_mut() {
290        commands.entity(entity).remove::<Selected>();
291        *border = BorderColor::all(NORMAL_COLOR);
292    }
293    let (_, mut border, background_color) = borders.get_mut(event.entity).unwrap();
294    *border = BorderColor::all(SELECTED_COLOR);
295    commands.entity(event.entity).insert(Selected);
296
297    draw_color.0 = background_color.0;
298}
```

Hide additional examples

examples/picking/sprite\_picking.rs ([line 157](../../src/sprite_picking/sprite_picking.rs.html#157))

```rust
153fn recolor_on<E: EntityEvent + Debug + Clone + Reflect>(
154    color: Color,
155) -> impl Fn(On<E>, Query<&mut Sprite>) {
156    move |ev, mut sprites| {
157        let Ok(mut sprite) = sprites.get_mut(ev.event_target()) else {
158            return;
159        };
160        sprite.color = color;
161    }
162}
```

examples/ecs/removal\_detection.rs ([line 53](../../src/removal_detection/removal_detection.rs.html#53))

```rust
51fn react_on_removal(remove: On<Remove, MyComponent>, mut query: Query<&mut Sprite>) {
52    // The `Remove` event was automatically triggered for the `Entity` that had its `MyComponent` removed.
53    if let Ok(mut sprite) = query.get_mut(remove.entity) {
54        sprite.color = Color::srgb(0.5, 1., 1.);
55    }
56}
```

examples/ui/widgets/viewport\_node.rs ([line 92](../../src/viewport_node/viewport_node.rs.html#92))

```rust
90fn on_drag_viewport(drag: On<Pointer<Drag>>, mut node_query: Query<&mut Node>) {
91    if matches!(drag.button, PointerButton::Secondary) {
92        let mut node = node_query.get_mut(drag.entity).unwrap();
93
94        if let (Val::Px(top), Val::Px(left)) = (node.top, node.left) {
95            node.left = px(left + drag.delta.x);
96            node.top = px(top + drag.delta.y);
97        };
98    }
99}
100
101fn on_drag_cuboid(drag: On<Pointer<Drag>>, mut transform_query: Query<&mut Transform>) {
102    if matches!(drag.button, PointerButton::Primary) {
103        let mut transform = transform_query.get_mut(drag.entity).unwrap();
104        transform.rotate_y(drag.delta.x * 0.02);
105        transform.rotate_x(drag.delta.y * 0.02);
106    }
107}
```

examples/ui/navigation/directional\_navigation.rs ([line 88](../../src/directional_navigation/directional_navigation.rs.html#88))

```rust
83fn universal_button_click_behavior(
84    mut click: On<Pointer<Click>>,
85    mut button_query: Query<(&mut BackgroundColor, &mut ResetTimer)>,
86) {
87    let button_entity = click.entity;
88    if let Ok((mut color, mut reset_timer)) = button_query.get_mut(button_entity) {
89        color.0 = PRESSED_BUTTON.into();
90        reset_timer.0 = Timer::from_seconds(0.3, TimerMode::Once);
91        click.propagate(false);
92    }
93}
```

examples/asset/asset\_saving\_with\_subassets.rs ([line 362](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#362))

```rust
358fn drag_box(event: On<Pointer<Drag>>, mut boxes: Query<&mut Transform, With<Box>>) {
359    if event.button != PointerButton::Primary {
360        return;
361    }
362    let Ok(mut transform) = boxes.get_mut(event.entity) else {
363        return;
364    };
365
366    // This is wrong in general (e.g., doesn't consider scale), but it's close enough for our
367    // purposes.
368    transform.translation += Vec3::new(event.delta.x, -event.delta.y, 0.0);
369}
```

Additional examples can be found in:  

*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#126)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#153)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#110)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#107)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#167)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#53)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#172)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#66)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#67)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#317)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#106)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#260)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#26)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#352)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#112)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#378)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#292)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#69)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#26)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#194)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#411)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#211)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#66)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#319)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#185)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#69)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#462)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#277)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#459)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#65)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#29)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#42)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#82)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#51)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#243)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#170)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#169)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#70)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#293)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#84)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#79)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#102)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#106)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#61)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#753)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1728)

#### pub fn [get\_inner](#method.get_inner)( self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>, [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the query item for the given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

This is always guaranteed to run in `O(1)` time.

##### See also

*   [`get_mut`](struct.Query.html#method.get_mut "method bevy::prelude::Query::get_mut") to get the item using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1859-1864)

#### pub fn [get\_many\_mut](#method.get_many_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, entities: \[[Entity](struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query items for the given array of [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

The returned query items are in the same order as the input. In case of a nonexisting entity, duplicate entities or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### Examples

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::query::QueryEntityError;

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();

let entities: Vec<Entity> = (0..3).map(|i| world.spawn(A(i)).id()).collect();
let entities: [Entity; 3] = entities.try_into().unwrap();

world.spawn(A(73));
let wrong_entity = Entity::from_raw_u32(57).unwrap();
let invalid_entity = world.spawn_empty().id();


let mut query_state = world.query::<&mut A>();
let mut query = query_state.query_mut(&mut world);

let mut mutable_component_values = query.get_many_mut(entities).unwrap();

for mut a in &mut mutable_component_values {
    a.0 += 5;
}

let component_values = query.get_many(entities).unwrap();

assert_eq!(component_values, [&A(5), &A(6), &A(7)]);

assert_eq!(
    match query
        .get_many_mut([wrong_entity])
        .unwrap_err()
    {
        QueryEntityError::NotSpawned(error) => error.entity(),
        _ => panic!(),
    },
    wrong_entity
);
assert_eq!(
    match query
        .get_many_mut([invalid_entity])
        .unwrap_err()
    {
        QueryEntityError::QueryDoesNotMatch(entity, _) => entity,
        _ => panic!(),
    },
    invalid_entity
);
assert_eq!(
    query
        .get_many_mut([entities[0], entities[0]])
        .unwrap_err(),
    QueryEntityError::AliasedMutability(entities[0])
);
```

##### See also

*   [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") to get read-only query items without checking for duplicate entities.

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/ui/ui\_drag\_and\_drop.rs ([line 91](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#91))

```rust
16fn setup(mut commands: Commands) {
17    commands.spawn(Camera2d);
18    commands
19        .spawn((Node {
20            display: Display::Grid,
21            align_self: AlignSelf::Center,
22            justify_self: JustifySelf::Center,
23            ..Default::default()
24        }, Pickable::IGNORE, BackgroundColor(Color::srgb(0.4, 0.4, 0.4))))
25        .with_children(|parent| {
26            let tile_colors = [
27                Color::srgb(0.2, 0.2, 0.8),
28                Color::srgb(0.8, 0.2, 0.2)
29            ];
30            for column in 0..COLUMNS {
31                for row in 0..ROWS {
32                    let i = column + row * COLUMNS;
33                    let tile_color = tile_colors[((row % 2) + column) as usize % tile_colors.len()];
34                    let tile_border_color = tile_color.darker(0.025);
35                    parent
36                        .spawn((
37                            Node {
38                                width: px(TILE_SIZE),
39                                height: px(TILE_SIZE),
40                                border: px(4.).all(),
41                                grid_row: GridPlacement::start(row + 1),
42                                grid_column: GridPlacement::start(column + 1),
43                                align_items: AlignItems::Center,
44                                justify_content: JustifyContent::Center,
45                                ..Default::default()
46                            },
47                            BorderColor::all(tile_border_color),
48                            BackgroundColor(tile_color),
49                            Outline {
50                                width: px(2.),
51                                offset: Val::ZERO,
52                                color: Color::NONE,
53                            },
54                            Pickable {
55                                should_block_lower: false,
56                                is_hoverable: true,
57                            },
58                            GlobalZIndex::default()
59                        ))
60                        .observe(move |on_over: On<Pointer<Over>>, mut query: Query<(&mut BackgroundColor, &mut BorderColor)>| {
61                            if let Ok((mut background_color, mut border_color)) = query.get_mut(on_over.event_target()) {
62                                background_color.0 = tile_color.lighter(0.1);
63                                border_color.set_all(tile_border_color.lighter(0.1));
64                            }
65                        })
66                        .observe(move |on_out: On<Pointer<Out>>, mut query: Query<(&mut BackgroundColor, &mut BorderColor)>| {
67                            if let Ok((mut background_color, mut border_color)) = query.get_mut(on_out.event_target()) {
68                                background_color.0 = tile_color;
69                                border_color.set_all(tile_border_color);
70                            }
71                        })
72                        .observe(|on_drag_start: On<Pointer<DragStart>>, mut query: Query<(&mut Outline, &mut GlobalZIndex)>| {
73                            if let Ok((mut outline, mut global_zindex, )) = query.get_mut(on_drag_start.event_target()) {
74                                outline.color = Color::WHITE;
75                                global_zindex.0 = 1;
76                            }
77                        })
78                        .observe(|on_drag: On<Pointer<Drag>>, mut query: Query<&mut UiTransform>| {
79                            if let Ok(mut transform) = query.get_mut(on_drag.event_target()) {
80                                transform.translation = Val2::px(on_drag.distance.x, on_drag.distance.y);
81                            }
82                        })
83                        .observe(move |on_drag_end: On<Pointer<DragEnd>>, mut query: Query<(&mut UiTransform, &mut Outline, &mut GlobalZIndex)>| {
84                            if let Ok((mut transform, mut outline, mut global_zindex)) = query.get_mut(on_drag_end.event_target()) {
85                                transform.translation = Val2::ZERO;
86                                outline.color = Color::NONE;
87                                global_zindex.0 = 0;
88                            }
89                        })
90                        .observe(|on_drag_drop: On<Pointer<DragDrop>>, mut query: Query<&mut Node>| {
91                            if let Ok([mut a, mut b]) = query.get_many_mut([on_drag_drop.event_target(), on_drag_drop.dropped]) {
92                                core::mem::swap(&mut a.grid_row, &mut b.grid_row);
93                                core::mem::swap(&mut a.grid_column, &mut b.grid_column);
94                            }
95                        })
96                        .with_child((Text::new(format!("{i}")), Pickable::IGNORE));
97                }
98            }
99        });
100}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1930-1935)

#### pub fn [get\_many\_unique\_mut](#method.get_many_unique_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, entities: [UniqueEntityEquivalentArray](../ecs/entity/struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<[Entity](struct.Entity.html "struct bevy::prelude::Entity"), N>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query items for the given [`UniqueEntityArray`](../ecs/entity/type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray").

The returned query items are in the same order as the input. In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### Examples

```rust
use bevy_ecs::{prelude::*, query::QueryEntityError, entity::{EntitySetIterator, UniqueEntityArray, UniqueEntityVec}};

#[derive(Component, PartialEq, Debug)]
struct A(usize);

let mut world = World::new();

let entity_set: UniqueEntityVec = world.spawn_batch((0..3).map(A)).collect_set();
let entity_set: UniqueEntityArray<3> = entity_set.try_into().unwrap();

world.spawn(A(73));
let wrong_entity = Entity::from_raw_u32(57).unwrap();
let invalid_entity = world.spawn_empty().id();


let mut query_state = world.query::<&mut A>();
let mut query = query_state.query_mut(&mut world);

let mut mutable_component_values = query.get_many_unique_mut(entity_set).unwrap();

for mut a in &mut mutable_component_values {
    a.0 += 5;
}

let component_values = query.get_many_unique(entity_set).unwrap();

assert_eq!(component_values, [&A(5), &A(6), &A(7)]);

assert_eq!(
    match query
        .get_many_unique_mut(UniqueEntityArray::from([wrong_entity]))
        .unwrap_err()
    {
        QueryEntityError::NotSpawned(error) => error.entity(),
        _ => panic!(),
    },
    wrong_entity
);
assert_eq!(
    match query
        .get_many_unique_mut(UniqueEntityArray::from([invalid_entity]))
        .unwrap_err()
    {
        QueryEntityError::QueryDoesNotMatch(entity, _) => entity,
        _ => panic!(),
    },
    invalid_entity
);
```

##### See also

*   [`get_many_unique`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") to get read-only query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1952-1957)

#### pub fn [get\_many\_mut\_inner](#method.get_many_mut_inner)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( self, entities: \[[Entity](struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query items for the given array of [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

The returned query items are in the same order as the input. In case of a nonexisting entity, duplicate entities or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### See also

*   [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") to get read-only query items without checking for duplicate entities.
*   [`get_many_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut") to get items using a mutable reference.
*   [`get_many_inner`](struct.Query.html#method.get_many_mut_inner "method bevy::prelude::Query::get_many_mut_inner") to get read-only query items with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#1983-1988)

#### pub fn [get\_many\_inner](#method.get_many_inner)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( self, entities: \[[Entity](struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [ReadOnlyQueryData](../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"),

Returns the query items for the given array of [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

The returned query items are in the same order as the input. In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### See also

*   [`get_many`](struct.Query.html#method.get_many "method bevy::prelude::Query::get_many") to get read-only query items without checking for duplicate entities.
*   [`get_many_mut`](struct.Query.html#method.get_many_mut "method bevy::prelude::Query::get_many_mut") to get items using a mutable reference.
*   [`get_many_mut_inner`](struct.Query.html#method.get_many_mut_inner "method bevy::prelude::Query::get_many_mut_inner") to get mutable query items with the actual “inner” world lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2005-2010)

#### pub fn [get\_many\_unique\_inner](#method.get_many_unique_inner)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( self, entities: [UniqueEntityEquivalentArray](../ecs/entity/struct.UniqueEntityEquivalentArray.html "struct bevy::ecs::entity::UniqueEntityEquivalentArray")<[Entity](struct.Entity.html "struct bevy::prelude::Entity"), N>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns the query items for the given [`UniqueEntityArray`](../ecs/entity/type.UniqueEntityArray.html "type bevy::ecs::entity::UniqueEntityArray"). This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

The returned query items are in the same order as the input. In case of a nonexisting entity, duplicate entities or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

##### See also

*   [`get_many_unique`](struct.Query.html#method.get_many_unique "method bevy::prelude::Query::get_many_unique") to get read-only query items without checking for duplicate entities.
*   [`get_many_unique_mut`](struct.Query.html#method.get_many_unique_mut "method bevy::prelude::Query::get_many_unique_mut") to get items using a mutable reference.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2058-2061)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)( &self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>, [QueryEntityError](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError")\>

Returns the query item for the given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

In case of a nonexisting entity or mismatched component, a [`QueryEntityError`](../ecs/query/enum.QueryEntityError.html "enum bevy::ecs::query::QueryEntityError") is returned instead.

This is always guaranteed to run in `O(1)` time.

##### Safety

This function makes it possible to violate Rust’s aliasing guarantees. You must make sure this call does not result in multiple mutable references to the same component.

##### See also

*   [`get_mut`](struct.Query.html#method.get_mut "method bevy::prelude::Query::get_mut") for the safe version.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2097)

#### pub fn [single](#method.single)( &self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>, [QuerySingleError](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

Returns a single read-only query item when there is exactly one entity matching the query.

If the number of query items is not exactly one, a [`QuerySingleError`](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Example

```rust
fn player_scoring_system(query: Query<&PlayerScore>) {
    match query.single() {
        Ok(PlayerScore(score)) => {
            println!("Score: {}", score);
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
        }
    }
}
```

##### See also

*   [`single_mut`](struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut") to get the mutable query item.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/audio/audio\_control.rs ([line 62](../../src/audio_control/audio_control.rs.html#62))

```rust
61fn update_speed(music_controller: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
62    let Ok(sink) = music_controller.single() else {
63        return;
64    };
65    if sink.is_paused() {
66        return;
67    }
68
69    sink.set_speed((ops::sin(time.elapsed_secs() / 5.0) + 1.0).max(0.1));
70}
71
72fn pause(
73    keyboard_input: Res<ButtonInput<KeyCode>>,
74    music_controller: Query<&AudioSink, With<MyMusic>>,
75) {
76    let Ok(sink) = music_controller.single() else {
77        return;
78    };
79
80    if keyboard_input.just_pressed(KeyCode::Space) {
81        sink.toggle_playback();
82    }
83}
```

Hide additional examples

examples/camera/pan\_camera\_controller.rs ([line 31](../../src/pan_camera_controller/pan_camera_controller.rs.html#31))

```rust
23fn spawn_text(mut commands: Commands, camera: Query<&PanCamera>) {
24    commands.spawn((
25        Node {
26            position_type: PositionType::Absolute,
27            top: px(-16),
28            left: px(12),
29            ..default()
30        },
31        children![Text::new(format!("{}", camera.single().unwrap()))],
32    ));
33}
```

examples/ui/text/text\_background\_colors.rs ([line 93](../../src/text_background_colors/text_background_colors.rs.html#93))

```rust
87fn cycle_text_background_colors(
88    time: Res<Time>,
89    children_query: Query<&Children, With<Text>>,
90    mut text_background_colors_query: Query<&mut TextBackgroundColor>,
91) {
92    let n = time.elapsed_secs() as usize;
93    let children = children_query.single().unwrap();
94
95    for (i, child) in children.iter().enumerate() {
96        text_background_colors_query.get_mut(child).unwrap().0 = PALETTE[(i + n) % PALETTE.len()];
97    }
98}
```

examples/3d/light\_textures.rs ([line 358](../../src/light_textures/light_textures.rs.html#358))

```rust
357fn draw_gizmos(mut gizmos: Gizmos, spotlight: Query<(&GlobalTransform, &SpotLight, &Visibility)>) {
358    if let Ok((global_transform, spotlight, visibility)) = spotlight.single()
359        && visibility != Visibility::Hidden
360    {
361        gizmos.primitive_3d(
362            &Cone::new(7.0 * spotlight.outer_angle, 7.0),
363            Isometry3d {
364                rotation: global_transform.rotation() * Quat::from_rotation_x(FRAC_PI_2),
365                translation: global_transform.translation_vec3a() * 0.5,
366            },
367            YELLOW,
368        );
369    }
370}
```

examples/ecs/observers.rs ([line 196](../../src/observers/observers.rs.html#196))

```rust
190fn handle_click(
191    mouse_button_input: Res<ButtonInput<MouseButton>>,
192    camera: Single<(&Camera, &GlobalTransform)>,
193    windows: Query<&Window>,
194    mut commands: Commands,
195) {
196    let Ok(windows) = windows.single() else {
197        return;
198    };
199
200    let (camera, camera_transform) = *camera;
201    if let Some(pos) = windows
202        .cursor_position()
203        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
204        .map(|ray| ray.origin.truncate())
205        && mouse_button_input.just_pressed(MouseButton::Left)
206    {
207        commands.trigger(ExplodeMines { pos, radius: 1.0 });
208    }
209}
```

examples/window/persisting\_window\_settings.rs ([line 97](../../src/persisting_window_settings/persisting_window_settings.rs.html#97))

```rust
90fn update_window_settings(
91    mut move_events: MessageReader<WindowMoved>,
92    mut resize_events: MessageReader<WindowResized>,
93    windows: Query<&mut Window>,
94    window_settings: ResMut<WindowSettings>,
95    mut commands: Commands,
96) {
97    let Ok(window) = windows.single() else {
98        return;
99    };
100
101    let mut window_changed = false;
102    for _ in move_events.read() {
103        window_changed = true;
104    }
105
106    for _ in resize_events.read() {
107        window_changed = true;
108    }
109
110    if window_changed && store_window_settings(window_settings, window) {
111        commands.queue(SaveSettingsDeferred(Duration::from_secs_f32(0.5)));
112    }
113}
```

Additional examples can be found in:  

*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#220)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#103)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#40)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#46)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#79)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#115)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#184)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#334)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#463)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#260)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#69)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#189)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#109)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#285)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2126-2128)

#### pub fn [single\_mut](#method.single_mut)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 's>, [QuerySingleError](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a single query item when there is exactly one entity matching the query.

If the number of query items is not exactly one, a [`QuerySingleError`](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Example

```rust
fn regenerate_player_health_system(mut query: Query<&mut Health, With<Player>>) {
    let mut health = query.single_mut().expect("Error: Could not find a single player.");
    health.0 += 1;
}
```

##### See also

*   [`single`](struct.Query.html#method.single "method bevy::prelude::Query::single") to get the read-only query item.

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

tests/window/desktop\_request\_redraw.rs ([line 98](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#98))

```rust
97fn update(time: Res<Time>, mut query: Query<&mut Transform, With<AnimationActive>>) {
98    if let Ok(mut transform) = query.single_mut() {
99        transform.rotate_x(time.delta_secs().min(1.0 / 60.0));
100    }
101}
```

Hide additional examples

examples/3d/meshlet.rs ([line 123](../../src/meshlet/meshlet.rs.html#123))

```rust
122fn bunny_wiggler(mut bunny: Query<&mut Transform, With<BunnyWiggler>>, time: Res<Time>) {
123    bunny.single_mut().as_deref_mut().unwrap().translation.z +=
124        ops::cos(time.elapsed_secs() * 10.0) * 0.003;
125}
```

examples/audio/audio\_control.rs ([line 89](../../src/audio_control/audio_control.rs.html#89))

```rust
85fn mute(
86    keyboard_input: Res<ButtonInput<KeyCode>>,
87    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
88) {
89    let Ok(mut sink) = music_controller.single_mut() else {
90        return;
91    };
92
93    if keyboard_input.just_pressed(KeyCode::KeyM) {
94        sink.toggle_mute();
95    }
96}
97
98fn volume(
99    keyboard_input: Res<ButtonInput<KeyCode>>,
100    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
101) {
102    let Ok(mut sink) = music_controller.single_mut() else {
103        return;
104    };
105
106    if keyboard_input.just_pressed(KeyCode::Equal) {
107        let current_volume = sink.volume();
108        sink.set_volume(current_volume.increase_by_percentage(10.0));
109    } else if keyboard_input.just_pressed(KeyCode::Minus) {
110        let current_volume = sink.volume();
111        sink.set_volume(current_volume.increase_by_percentage(-10.0));
112    }
113}
```

examples/stress\_tests/many\_text2d.rs ([line 160](../../src/many_text2d/many_text2d.rs.html#160))

```rust
159fn move_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera>>) {
160    let Ok(mut camera_transform) = camera_query.single_mut() else {
161        return;
162    };
163    camera_transform.rotate_z(time.delta_secs() * 0.5);
164    *camera_transform =
165        *camera_transform * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
166}
```

examples/asset/multi\_asset\_sync.rs ([line 264](../../src/multi_asset_sync/multi_asset_sync.rs.html#264))

```rust
253fn get_async_loading_state(
254    state: Res<AsyncLoadingState>,
255    mut next_loading_state: ResMut<NextState<LoadingState>>,
256    mut text: Query<&mut Text, With<LoadingText>>,
257) {
258    // Load the value written by the `Future`.
259    let is_loaded = state.0.load(Ordering::Acquire);
260
261    // If loaded, change the state.
262    if is_loaded {
263        next_loading_state.set(LoadingState::Loaded);
264        if let Ok(mut text) = text.single_mut() {
265            "Loaded!".clone_into(&mut **text);
266        }
267    }
268}
```

examples/ui/navigation/directional\_navigation.rs ([line 372](../../src/directional_navigation/directional_navigation.rs.html#372))

```rust
367fn update_focus_display(
368    input_focus: Res<InputFocus>,
369    button_query: Query<&Name, With<Button>>,
370    mut display_query: Query<&mut Text, With<FocusDisplay>>,
371) {
372    if let Ok(mut text) = display_query.single_mut() {
373        if let Some(focused_entity) = input_focus.get() {
374            if let Ok(name) = button_query.get(focused_entity) {
375                **text = format!("Focused: {}", name);
376            } else {
377                **text = "Focused: Unknown".to_string();
378            }
379        } else {
380            **text = "Focused: None".to_string();
381        }
382    }
383}
384
385fn update_key_display(
386    keyboard_input: Res<ButtonInput<KeyCode>>,
387    gamepad_input: Query<&Gamepad>,
388    mut display_query: Query<&mut Text, With<KeyDisplay>>,
389) {
390    if let Ok(mut text) = display_query.single_mut() {
391        // Check for keyboard inputs
392        for action in DirectionalNavigationAction::variants() {
393            if keyboard_input.just_pressed(action.keycode()) {
394                let key_name = match action {
395                    DirectionalNavigationAction::Up => "Up Arrow",
396                    DirectionalNavigationAction::Down => "Down Arrow",
397                    DirectionalNavigationAction::Left => "Left Arrow",
398                    DirectionalNavigationAction::Right => "Right Arrow",
399                    DirectionalNavigationAction::Select => "Enter",
400                };
401                **text = format!("Last Key: {}", key_name);
402                return;
403            }
404        }
405
406        // Check for gamepad inputs
407        for gamepad in gamepad_input.iter() {
408            for action in DirectionalNavigationAction::variants() {
409                if gamepad.just_pressed(action.gamepad_button()) {
410                    let button_name = match action {
411                        DirectionalNavigationAction::Up => "D-Pad Up",
412                        DirectionalNavigationAction::Down => "D-Pad Down",
413                        DirectionalNavigationAction::Left => "D-Pad Left",
414                        DirectionalNavigationAction::Right => "D-Pad Right",
415                        DirectionalNavigationAction::Select => "A Button",
416                    };
417                    **text = format!("Last Key: {}", button_name);
418                    return;
419                }
420            }
421        }
422    }
423}
```

Additional examples can be found in:  

*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#765)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#71)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#120)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#148)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#150)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#296)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#662)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#48)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#199)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#184)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#206)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2160-2162)

#### pub fn [single\_inner](#method.single_inner)( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>, [QuerySingleError](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError")\>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

Returns a single query item when there is exactly one entity matching the query. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

If the number of query items is not exactly one, a [`QuerySingleError`](../ecs/query/enum.QuerySingleError.html "enum bevy::ecs::query::QuerySingleError") is returned instead.

##### Example

```rust
fn regenerate_player_health_system(query: Query<&mut Health, With<Player>>) {
    let mut health = query.single_inner().expect("Error: Could not find a single player.");
    health.0 += 1;
}
```

##### See also

*   [`single`](struct.Query.html#method.single "method bevy::prelude::Query::single") to get the read-only query item.
*   [`single_mut`](struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut") to get the mutable query item.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2207)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if there are no query items.

This is equivalent to `self.iter().next().is_none()`, and thus the worst case runtime will be `O(n)` where `n` is the number of _potential_ matches. This can be notably expensive for queries that rely on non-archetypal filters such as [`Added`](struct.Added.html "struct bevy::prelude::Added"), [`Changed`](struct.Changed.html "struct bevy::prelude::Changed") or [`Spawned`](../ecs/query/struct.Spawned.html "struct bevy::ecs::query::Spawned") which must individually check each query result for a match.

##### Example

Here, the score is increased only if an entity with a `Player` component is present in the world:

```rust
fn update_score_system(query: Query<(), With<Player>>, mut score: ResMut<Score>) {
    if !query.is_empty() {
        score.0 += 1;
    }
}
```

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 629](../../src/mirror/mirror.rs.html#629))

```rust
617fn play_fox_animation(
618    mut commands: Commands,
619    mut animation_players_query: Query<
620        (Entity, &mut AnimationPlayer),
621        Without<AnimationGraphHandle>,
622    >,
623    asset_server: Res<AssetServer>,
624    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
625) {
626    // Only pick up animation players that don't already have an animation graph
627    // handle.
628    // This ensures that we only start playing the animation once.
629    if animation_players_query.is_empty() {
630        return;
631    }
632
633    let fox_animation = asset_server.load(GltfAssetLabel::Animation(0).from_asset(FOX_ASSET_PATH));
634    let (fox_animation_graph, fox_animation_node) =
635        AnimationGraph::from_clip(fox_animation.clone());
636    let fox_animation_graph = animation_graphs.add(fox_animation_graph);
637
638    for (entity, mut animation_player) in animation_players_query.iter_mut() {
639        commands
640            .entity(entity)
641            .insert(AnimationGraphHandle(fox_animation_graph.clone()));
642        animation_player.play(fox_animation_node).repeat();
643    }
644}
```

Hide additional examples

examples/3d/irradiance\_volumes.rs ([line 538](../../src/irradiance_volumes/irradiance_volumes.rs.html#538))

```rust
528fn create_cubes(
529    image_assets: Res<Assets<Image>>,
530    mut commands: Commands,
531    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
532    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
533    voxel_cubes: Query<Entity, With<VoxelCube>>,
534    example_assets: Res<ExampleAssets>,
535    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
536) {
537    // If voxel cubes have already been spawned, don't do anything.
538    if !voxel_cubes.is_empty() {
539        return;
540    }
541
542    let Some(voxel_cube_parent) = voxel_cube_parents.iter().next() else {
543        return;
544    };
545
546    for (irradiance_volume, global_transform) in irradiance_volumes.iter() {
547        let Some(image) = image_assets.get(&irradiance_volume.voxels) else {
548            continue;
549        };
550
551        let resolution = image.texture_descriptor.size;
552
553        let voxel_cube_material = voxel_visualization_material_assets.add(ExtendedMaterial {
554            base: StandardMaterial::from(Color::from(RED)),
555            extension: VoxelVisualizationExtension {
556                irradiance_volume_info: VoxelVisualizationIrradianceVolumeInfo {
557                    world_from_voxel: VOXEL_FROM_WORLD.inverse(),
558                    voxel_from_world: VOXEL_FROM_WORLD,
559                    resolution: uvec3(
560                        resolution.width,
561                        resolution.height,
562                        resolution.depth_or_array_layers,
563                    ),
564                    intensity: IRRADIANCE_VOLUME_INTENSITY,
565                },
566            },
567        });
568
569        let scale = vec3(
570            1.0 / resolution.width as f32,
571            1.0 / resolution.height as f32,
572            1.0 / resolution.depth_or_array_layers as f32,
573        );
574
575        // Spawn a cube for each voxel.
576        for z in 0..resolution.depth_or_array_layers {
577            for y in 0..resolution.height {
578                for x in 0..resolution.width {
579                    let uvw = (uvec3(x, y, z).as_vec3() + 0.5) * scale - 0.5;
580                    let pos = global_transform.transform_point(uvw);
581                    let voxel_cube = commands
582                        .spawn((
583                            Mesh3d(example_assets.voxel_cube.clone()),
584                            MeshMaterial3d(voxel_cube_material.clone()),
585                            Transform::from_scale(Vec3::splat(VOXEL_CUBE_SCALE))
586                                .with_translation(pos),
587                        ))
588                        .insert(VoxelCube)
589                        .insert(NotShadowCaster)
590                        .id();
591
592                    commands.entity(voxel_cube_parent).add_child(voxel_cube);
593                }
594            }
595        }
596    }
597}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2242)

#### pub fn [contains](#method.contains)(&self, entity: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") matches the query.

This is always guaranteed to run in `O(1)` time.

##### Example

```rust
fn targeting_system(in_range_query: Query<&InRange>, target: Res<Target>) {
    if in_range_query.contains(target.entity) {
        println!("Bam!")
    }
}
```

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/testbed/3d.rs ([line 489](../../src/testbed_3d/3d.rs.html#489))

```rust
481    pub fn show_aabbs(
482        scene_ready: On<WorldInstanceReady>,
483        mut commands: Commands,
484        children: Query<&Children>,
485        meshes: Query<(), With<Mesh3d>>,
486    ) {
487        for child in children
488            .iter_descendants(scene_ready.entity)
489            .filter(|&e| meshes.contains(e))
490        {
491            commands.entity(child).insert(ShowAabbGizmo {
492                color: Some(BLACK.into()),
493            });
494        }
495    }
```

Hide additional examples

examples/ui/widgets/viewport\_node.rs ([line 118](../../src/viewport_node/viewport_node.rs.html#118))

```rust
109fn draw_mesh_intersections(
110    pointers: Query<&PointerInteraction>,
111    untargetable: Query<Entity, Without<Shape>>,
112    mut gizmos: Gizmos,
113) {
114    for (point, normal) in pointers
115        .iter()
116        .flat_map(|interaction| interaction.iter())
117        .filter_map(|(entity, hit)| {
118            if !untargetable.contains(*entity) {
119                hit.position.zip(hit.normal)
120            } else {
121                None
122            }
123        })
124    {
125        gizmos.arrow(point, point + normal.normalize() * 0.5, Color::WHITE);
126    }
127}
```

examples/ecs/entity\_disabling.rs ([line 46](../../src/entity_disabling/entity_disabling.rs.html#46))

```rust
38fn disable_entities_on_click(
39    click: On<Pointer<Click>>,
40    valid_query: Query<&DisableOnClick>,
41    mut commands: Commands,
42) {
43    // Windows and text are entities and can be clicked!
44    // We definitely don't want to disable the window itself,
45    // because that would cause the app to close!
46    if valid_query.contains(click.entity) {
47        // Just add the `Disabled` component to the entity to disable it.
48        // Note that the `Disabled` component is *only* added to the entity,
49        // its children are not affected.
50        commands.entity(click.entity).insert(Disabled);
51    }
52}
```

examples/asset/asset\_saving.rs ([line 286](../../src/asset_saving/asset_saving.rs.html#286))

```rust
280fn on_press_selectable(
281    event: On<Pointer<Press>>,
282    mut borders: Query<(Entity, &mut BorderColor, &BackgroundColor), With<SelectableColor>>,
283    mut draw_color: ResMut<DrawColor>,
284    mut commands: Commands,
285) {
286    if !borders.contains(event.entity) {
287        return;
288    }
289    for (entity, mut border, _) in borders.iter_mut() {
290        commands.entity(entity).remove::<Selected>();
291        *border = BorderColor::all(NORMAL_COLOR);
292    }
293    let (_, mut border, background_color) = borders.get_mut(event.entity).unwrap();
294    *border = BorderColor::all(SELECTED_COLOR);
295    commands.entity(event.entity).insert(Selected);
296
297    draw_color.0 = background_color.0;
298}
```

examples/asset/asset\_saving\_with\_subassets.rs ([line 274](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#274))

```rust
265fn spawn_box(
266    event: On<Pointer<Press>>,
267    window: Query<(), With<Window>>,
268    camera: Single<(&Camera, &GlobalTransform)>,
269    mut commands: Commands,
270) {
271    if event.button != PointerButton::Primary {
272        return;
273    }
274    if !window.contains(event.entity) {
275        return;
276    }
277
278    let (camera, camera_transform) = camera.into_inner();
279    let Ok(click_point) =
280        camera.viewport_to_world_2d(camera_transform, event.pointer_location.position)
281    else {
282        return;
283    };
284    commands.spawn((
285        Sprite::from_color(tailwind::RED_500, Vec2::new(100.0, 100.0)),
286        Transform::from_translation(click_point.extend(0.0)),
287        Pickable::default(),
288        Box,
289    ));
290}
291
292/// A component to rotate the hue of a sprite every frame.
293#[derive(Component)]
294struct RotateHue;
295
296/// Rotates the hue of each [`Sprite`] tagged with [`RotateHue`].
297fn rotate_hue(time: Res<Time>, mut sprites: Query<&mut Sprite, With<RotateHue>>) {
298    for mut sprite in sprites.iter_mut() {
299        // Make a full rotation every 2 seconds.
300        sprite.color = sprite.color.rotate_hue(time.delta_secs() * 180.0);
301    }
302}
303
304/// Starts rotating the hue of a box that has been right-clicked.
305fn start_rotate_box_hue(
306    event: On<Pointer<Press>>,
307    boxes: Query<(), With<Box>>,
308    mut commands: Commands,
309) {
310    if event.button != PointerButton::Secondary {
311        return;
312    }
313    if !boxes.contains(event.entity) {
314        return;
315    }
316    commands.entity(event.entity).insert(RotateHue);
317}
318
319/// Stops rotating the box hue if it's right-click is released.
320fn end_rotate_box_hue_on_release(
321    event: On<Pointer<Release>>,
322    boxes: Query<(), (With<Box>, With<RotateHue>)>,
323    mut commands: Commands,
324) {
325    if event.button != PointerButton::Secondary {
326        return;
327    }
328    if !boxes.contains(event.entity) {
329        return;
330    }
331    commands.entity(event.entity).remove::<RotateHue>();
332}
333
334/// Stops rotating the box hue if the cursor moves off the entity.
335fn end_rotate_box_hue_on_out(
336    event: On<Pointer<Out>>,
337    boxes: Query<(), (With<Box>, With<RotateHue>)>,
338    mut commands: Commands,
339) {
340    if !boxes.contains(event.entity) {
341        return;
342    }
343    commands.entity(event.entity).remove::<RotateHue>();
344}
345
346/// Blocks propagation of pointer press events on left-clicked boxes.
347fn stop_propagate_on_clicked_box(mut event: On<Pointer<Press>>, boxes: Query<(), With<Box>>) {
348    if event.button != PointerButton::Primary {
349        return;
350    }
351    if !boxes.contains(event.entity) {
352        return;
353    }
354    event.propagate(false);
355}
```

examples/ui/render\_ui\_to\_texture.rs ([line 200](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#200))

```rust
173fn drive_diegetic_pointer(
174    mut cursor_last: Local<Vec2>,
175    mut raycast: MeshRayCast,
176    rays: Res<RayMap>,
177    cubes: Query<&Mesh3d, With<Cube>>,
178    ui_camera: Query<&RenderTarget, With<Camera2d>>,
179    primary_window: Query<Entity, With<PrimaryWindow>>,
180    windows: Query<(Entity, &Window)>,
181    images: Res<Assets<Image>>,
182    manual_texture_views: Res<ManualTextureViews>,
183    mut window_events: MessageReader<WindowEvent>,
184    mut pointer_inputs: MessageWriter<PointerInput>,
185) -> Result {
186    // Get the size of the texture, so we can convert from dimensionless UV coordinates that span
187    // from 0 to 1, to pixel coordinates.
188    let target = ui_camera
189        .single()?
190        .normalize(primary_window.single().ok())
191        .unwrap();
192    let target_info = target
193        .get_render_target_info(windows, &images, &manual_texture_views)
194        .unwrap();
195    let size = target_info.physical_size.as_vec2();
196
197    // Find raycast hits and update the virtual pointer.
198    let raycast_settings = MeshRayCastSettings {
199        visibility: RayCastVisibility::VisibleInView,
200        filter: &|entity| cubes.contains(entity),
201        early_exit_test: &|_| false,
202    };
203    for (_id, ray) in rays.iter() {
204        for (_cube, hit) in raycast.cast_ray(*ray, &raycast_settings) {
205            let position = size * hit.uv.unwrap();
206            if position != *cursor_last {
207                pointer_inputs.write(PointerInput::new(
208                    CUBE_POINTER_ID,
209                    Location {
210                        target: target.clone(),
211                        position,
212                    },
213                    PointerAction::Move {
214                        delta: position - *cursor_last,
215                    },
216                ));
217                *cursor_last = position;
218            }
219        }
220    }
221
222    // Pipe pointer button presses to the virtual pointer on the UI texture.
223    for window_event in window_events.read() {
224        if let WindowEvent::MouseButtonInput(input) = window_event {
225            let button = match input.button {
226                MouseButton::Left => PointerButton::Primary,
227                MouseButton::Right => PointerButton::Secondary,
228                MouseButton::Middle => PointerButton::Middle,
229                _ => continue,
230            };
231            let action = match input.state {
232                ButtonState::Pressed => PointerAction::Press(button),
233                ButtonState::Released => PointerAction::Release(button),
234            };
235            pointer_inputs.write(PointerInput::new(
236                CUBE_POINTER_ID,
237                Location {
238                    target: target.clone(),
239                    position: *cursor_last,
240                },
241                action,
242            ));
243        }
244    }
245
246    Ok(())
247}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2274)

#### pub fn [count](#method.count)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Counts the number of entities that match the query.

This is equivalent to `self.iter().count()` but may be more efficient in some cases.

If [`D::IS_ARCHETYPAL`](../ecs/query/trait.QueryData.html#associatedconstant.IS_ARCHETYPAL "associated constant bevy::ecs::query::QueryData::IS_ARCHETYPAL") && [`F::IS_ARCHETYPAL`](../ecs/query/trait.QueryFilter.html#associatedconstant.IS_ARCHETYPAL "associated constant bevy::ecs::query::QueryFilter::IS_ARCHETYPAL") is `true`, this will do work proportional to the number of matched archetypes or tables, but will not iterate each entity. If it is `false`, it will have to do work for each entity.

##### Example

```rust
fn targeting_system(in_range_query: Query<&InRange>) {
    let count = in_range_query.count();
    println!("{count} targets in range!");
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2465)

#### pub fn [transmute\_lens](#method.transmute_lens)<NewD>(&mut self) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'\_, NewD>

where NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") that can be used to construct a new [`Query`](struct.Query.html "struct bevy::prelude::Query") giving more restrictive access to the entities matched by the current query.

A transmute is valid only if `NewD` has a subset of the read, write, and required access of the current query. A precise description of the access required by each parameter type is given in the table below, but typical uses are to:

*   Remove components, e.g. `Query<(&A, &B)>` to `Query<&A>`.
*   Retrieve an existing component with reduced or equal access, e.g. `Query<&mut A>` to `Query<&A>` or `Query<&T>` to `Query<Ref<T>>`.
*   Add parameters with no new access, for example adding an `Entity` parameter.

Note that since filter terms are dropped, non-archetypal filters like [`Added`](struct.Added.html "struct bevy::prelude::Added"), [`Changed`](struct.Changed.html "struct bevy::prelude::Changed") and [`Spawned`](../ecs/query/struct.Spawned.html "struct bevy::ecs::query::Spawned") will not be respected. To maintain or change filter terms see [`Self::transmute_lens_filtered`](struct.Query.html#method.transmute_lens_filtered "method bevy::prelude::Query::transmute_lens_filtered").

| `QueryData` parameter type | Access required |
| --- | --- |
| [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"), [`EntityLocation`](../ecs/entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation"), [`SpawnDetails`](../ecs/query/struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails"), [`&Archetype`](../ecs/archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), [`Has<T>`](struct.Has.html "struct bevy::prelude::Has"), [`PhantomData<T>`](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData") | No access |
| [`EntityMut`](struct.EntityMut.html "struct bevy::prelude::EntityMut") | Read and write access to all components, but no required access |
| [`EntityRef`](struct.EntityRef.html "struct bevy::prelude::EntityRef") | Read access to all components, but no required access |
| `&T`, [`Ref<T>`](struct.Ref.html "struct bevy::prelude::Ref") | Read and required access to `T` |
| `&mut T`, [`Mut<T>`](struct.Mut.html "struct bevy::prelude::Mut") | Read, write and required access to `T` |
| [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"), [`AnyOf<(D, ...)>`](struct.AnyOf.html "struct bevy::prelude::AnyOf") | Read and write access to `T`, but no required access |
| Tuples of query data and  
`#[derive(QueryData)]` structs | The union of the access of their subqueries |
| [`FilteredEntityRef`](../ecs/world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef"), [`FilteredEntityMut`](../ecs/world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") | Determined by the [`QueryBuilder`](struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder") used to construct them. Any query can be transmuted to them, and they will receive the access of the source query. When combined with other `QueryData`, they will receive any access of the source query that does not conflict with the other data |

`transmute_lens` drops filter terms, but [`Self::transmute_lens_filtered`](struct.Query.html#method.transmute_lens_filtered "method bevy::prelude::Query::transmute_lens_filtered") supports returning a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") with a new filter type - the access required by filter parameters are as follows.

| `QueryFilter` parameter type | Access required |
| --- | --- |
| [`Added<T>`](struct.Added.html "struct bevy::prelude::Added"), [`Changed<T>`](struct.Changed.html "struct bevy::prelude::Changed") | Read and required access to `T` |
| [`With<T>`](struct.With.html "struct bevy::prelude::With"), [`Without<T>`](struct.Without.html "struct bevy::prelude::Without") | No access |
| [`Or<(T, ...)>`](struct.Or.html "struct bevy::prelude::Or") | Read access of the subqueries, but no required access |
| Tuples of query filters and `#[derive(QueryFilter)]` structs | The union of the access of their subqueries |

###### Panics

This will panic if the access required by `NewD` is not a subset of that required by the original fetch `D`.

###### Example

```rust
fn reusable_function(lens: &mut QueryLens<&A>) {
    assert_eq!(lens.query().single().unwrap().0, 10);
}

// We can use the function in a system that takes the exact query.
fn system_1(mut query: Query<&A>) {
    reusable_function(&mut query.as_query_lens());
}

// We can also use it with a query that does not match exactly
// by transmuting it.
fn system_2(mut query: Query<(&mut A, &B)>) {
    let mut lens = query.transmute_lens::<&A>();
    reusable_function(&mut lens);
}
```

###### Examples of valid transmutes

```rust
// `&mut T` and `Mut<T>` access the same data and can be transmuted to each other,
// `&T` and `Ref<T>` access the same data and can be transmuted to each other,
// and mutable versions can be transmuted to read-only versions
assert_valid_transmute::<&mut T, &T>();
assert_valid_transmute::<&mut T, Mut<T>>();
assert_valid_transmute::<Mut<T>, &mut T>();
assert_valid_transmute::<&T, Ref<T>>();
assert_valid_transmute::<Ref<T>, &T>();

// The structure can be rearranged, or subqueries dropped
assert_valid_transmute::<(&T, &U), &T>();
assert_valid_transmute::<((&T, &U), &V), (&T, (&U, &V))>();
assert_valid_transmute::<Option<(&T, &U)>, (Option<&T>, Option<&U>)>();

// Queries with no access can be freely added
assert_valid_transmute::<
    &T,
    (&T, Entity, EntityLocation, &Archetype, Has<U>, PhantomData<T>),
>();

// Required access can be transmuted to optional,
// and optional access can be transmuted to other optional access
assert_valid_transmute::<&T, Option<&T>>();
assert_valid_transmute::<AnyOf<(&mut T, &mut U)>, Option<&T>>();
// Note that removing subqueries from `AnyOf` will result
// in an `AnyOf` where all subqueries can yield `None`!
assert_valid_transmute::<AnyOf<(&T, &U, &V)>, AnyOf<(&T, &U)>>();
assert_valid_transmute::<EntityMut, Option<&mut T>>();

// Anything can be transmuted to `FilteredEntityRef` or `FilteredEntityMut`
// This will create a `FilteredEntityMut` that only has read access to `T`
assert_valid_transmute::<&T, FilteredEntityMut>();
// This will create a `FilteredEntityMut` that has no access to `T`,
// read access to `U`, and write access to `V`.
assert_valid_transmute::<(&mut T, &mut U, &mut V), (&mut T, &U, FilteredEntityMut)>();

// `Added<T>` and `Changed<T>` filters have the same access as `&T` data
// Remember that they are only evaluated on the transmuted query, not the original query!
assert_valid_transmute_filtered::<Entity, Changed<T>, &T, ()>();
assert_valid_transmute_filtered::<&mut T, (), &T, Added<T>>();
// Nested inside of an `Or` filter, they have the same access as `Option<&T>`.
assert_valid_transmute_filtered::<Option<&T>, (), Entity, Or<(Changed<T>, With<U>)>>();
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2521)

#### pub fn [transmute\_lens\_inner](#method.transmute_lens_inner)<NewD>(self) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, NewD>

where NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") that can be used to construct a new `Query` giving more restrictive access to the entities matched by the current query.

This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

See [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for a description of allowed transmutes.

###### Panics

This will panic if `NewD` is not a subset of the original fetch `D`

###### Example

```rust
fn reusable_function(mut lens: QueryLens<&A>) {
    assert_eq!(lens.query().single().unwrap().0, 10);
}

// We can use the function in a system that takes the exact query.
fn system_1(query: Query<&A>) {
    reusable_function(query.into_query_lens());
}

// We can also use it with a query that does not match exactly
// by transmuting it.
fn system_2(query: Query<(&mut A, &B)>) {
    let mut lens = query.transmute_lens_inner::<&A>();
    reusable_function(lens);
}
```

##### See also

*   [`transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") to convert to a lens using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2535-2537)

#### pub fn [transmute\_lens\_filtered](#method.transmute_lens_filtered)<NewD, NewF>( &mut self, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'\_, NewD, NewF>

where NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Equivalent to [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") but also includes a [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") type.

See [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for a description of allowed transmutes.

Note that the lens will iterate the same tables and archetypes as the original query. This means that additional archetypal query terms like [`With`](struct.With.html "struct bevy::prelude::With") and [`Without`](struct.Without.html "struct bevy::prelude::Without") will not necessarily be respected and non-archetypal terms like [`Added`](struct.Added.html "struct bevy::prelude::Added"), [`Changed`](struct.Changed.html "struct bevy::prelude::Changed") and [`Spawned`](../ecs/query/struct.Spawned.html "struct bevy::ecs::query::Spawned") will only be respected if they are in the type signature.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2556-2558)

#### pub fn [transmute\_lens\_filtered\_inner](#method.transmute_lens_filtered_inner)<NewD, NewF>( self, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, NewD, NewF>

where NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Equivalent to [`Self::transmute_lens_inner`](struct.Query.html#method.transmute_lens_inner "method bevy::prelude::Query::transmute_lens_inner") but also includes a [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") type. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

See [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for a description of allowed transmutes.

Note that the lens will iterate the same tables and archetypes as the original query. This means that additional archetypal query terms like [`With`](struct.With.html "struct bevy::prelude::With") and [`Without`](struct.Without.html "struct bevy::prelude::Without") will not necessarily be respected and non-archetypal terms like [`Added`](struct.Added.html "struct bevy::prelude::Added"), [`Changed`](struct.Changed.html "struct bevy::prelude::Changed") and [`Spawned`](../ecs/query/struct.Spawned.html "struct bevy::ecs::query::Spawned") will only be respected if they are in the type signature.

##### See also

*   [`transmute_lens_filtered`](struct.Query.html#method.transmute_lens_filtered "method bevy::prelude::Query::transmute_lens_filtered") to convert to a lens using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2569-2571)

#### pub fn [as\_query\_lens](#method.as_query_lens)(&mut self) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'\_, D>

where D: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Gets a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") with the same accesses as the existing query

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2581-2583)

#### pub fn [into\_query\_lens](#method.into_query_lens)(self) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, D>

where D: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Gets a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") with the same accesses as the existing query

##### See also

*   [`as_query_lens`](struct.Query.html#method.as_query_lens "method bevy::prelude::Query::as_query_lens") to convert to a lens using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2642-2645)

#### pub fn [join](#method.join)<'a, OtherD, NewD>( &'a mut self, other: &'a mut [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, OtherD>, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'a, NewD>

where OtherD: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") that can be used to get a query with the combined fetch.

For example, this can take a `Query<&A>` and a `Query<&B>` and return a `Query<(&A, &B)>`. The returned query will only return items with both `A` and `B`. Note that since filters are dropped, non-archetypal filters like `Added`, `Changed` and `Spawned` will not be respected. To maintain or change filter terms see `Self::join_filtered`.

###### Example

```rust
fn system(
    mut transforms: Query<&Transform>,
    mut players: Query<&Player>,
    mut enemies: Query<&Enemy>
) {
    let mut players_transforms: QueryLens<(&Transform, &Player)> = transforms.join(&mut players);
    for (transform, player) in &players_transforms.query() {
        // do something with a and b
    }

    let mut enemies_transforms: QueryLens<(&Transform, &Enemy)> = transforms.join(&mut enemies);
    for (transform, enemy) in &enemies_transforms.query() {
        // do something with a and b
    }
}
```

###### Panics

This will panic if `NewD` is not a subset of the union of the original fetch `Q` and `OtherD`.

###### Allowed Transmutes

Like `transmute_lens` the query terms can be changed with some restrictions. See [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for more details.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2669-2672)

#### pub fn [join\_inner](#method.join_inner)<OtherD, NewD>( self, other: [Query](struct.Query.html "struct bevy::prelude::Query")<'w, '\_, OtherD>, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, NewD>

where OtherD: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns a [`QueryLens`](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens") that can be used to get a query with the combined fetch. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

For example, this can take a `Query<&A>` and a `Query<&B>` and return a `Query<(&A, &B)>`. The returned query will only return items with both `A` and `B`. Note that since filters are dropped, non-archetypal filters like `Added`, `Changed` and `Spawned` will not be respected. To maintain or change filter terms see `Self::join_filtered`.

###### Panics

This will panic if `NewD` is not a subset of the union of the original fetch `Q` and `OtherD`.

###### Allowed Transmutes

Like `transmute_lens` the query terms can be changed with some restrictions. See [`Self::transmute_lens`](struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") for more details.

##### See also

*   [`join`](struct.Query.html#method.join "method bevy::prelude::Query::join") to join using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2683-2692)

#### pub fn [join\_filtered](#method.join_filtered)<'a, OtherD, OtherF, NewD, NewF>( &'a mut self, other: &'a mut [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, OtherD, OtherF>, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'a, NewD, NewF>

where OtherD: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), OtherF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Equivalent to [`Self::join`](struct.Query.html#method.join "method bevy::prelude::Query::join") but also includes a [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") type.

Note that the lens with iterate a subset of the original queries’ tables and archetypes. This means that additional archetypal query terms like `With` and `Without` will not necessarily be respected and non-archetypal terms like `Added`, `Changed` and `Spawned` will only be respected if they are in the type signature.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2708-2716)

#### pub fn [join\_filtered\_inner](#method.join_filtered_inner)<OtherD, OtherF, NewD, NewF>( self, other: [Query](struct.Query.html "struct bevy::prelude::Query")<'w, '\_, OtherD, OtherF>, ) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, NewD, NewF>

where OtherD: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), OtherF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), NewD: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), NewF: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Equivalent to [`Self::join_inner`](struct.Query.html#method.join_inner "method bevy::prelude::Query::join_inner") but also includes a [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") type. This consumes the [`Query`](struct.Query.html "struct bevy::prelude::Query") to return results with the actual “inner” world lifetime.

Note that the lens with iterate a subset of the original queries’ tables and archetypes. This means that additional archetypal query terms like `With` and `Without` will not necessarily be respected and non-archetypal terms like `Added`, `Changed` and `Spawned` will only be respected if they are in the type signature.

##### See also

*   [`join_filtered`](struct.Query.html#method.join_filtered "method bevy::prelude::Query::join_filtered") to join using a mutable borrow of the [`Query`](struct.Query.html "struct bevy::prelude::Query").

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#495)

### impl<D, F> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

where D: [ReadOnlyQueryData](../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#496)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#501)

### impl<D, F> [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

where D: [ReadOnlyQueryData](../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#503)

### impl<D, F> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#504)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2800-2801)

### impl<'w, 'q, Q, F> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'q mut [Query](struct.Query.html "struct bevy::prelude::Query")<'w, '\_, Q, F>> for [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'q, Q, F>

where Q: [SingleEntityQueryData](../ecs/query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2803)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &'q mut [Query](struct.Query.html "struct bevy::prelude::Query")<'w, '\_, Q, F>) -> [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'q, Q, F>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2792-2793)

### impl<'w, 's, Q, F> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'s mut [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, Q, F>> for [Query](struct.Query.html "struct bevy::prelude::Query")<'s, 's, Q, F>

where Q: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2795)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &'s mut [QueryLens](../ecs/system/struct.QueryLens.html "struct bevy::ecs::system::QueryLens")<'w, Q, F>) -> [Query](struct.Query.html "struct bevy::prelude::Query")<'s, 's, Q, F>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2729)

### impl<'w, 's, D, F> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2730)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2731)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, D, F>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2733)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <[Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2738)

### impl<'w, 's, D, F> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'w [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2739)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <<D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2740)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"), F>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2742)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'w [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2747)

### impl<'w, 's, D, F> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'w mut [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F>

where D: [IterQueryData](../ecs/query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2748)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <D as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2749)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [QueryIter](../ecs/query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, D, F>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/query.rs.html#2751)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'w mut [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#297-298)

### impl<'w, 's, D, F> [ReadOnlySystemParam](../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

where D: [ReadOnlyQueryData](../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#304)

### impl<D, F> [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#305)

#### type [State](../ecs/system/trait.SystemParam.html#associatedtype.State) = [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#306)

#### type [Item](../ecs/system/trait.SystemParam.html#associatedtype.Item)<'w, 's> = [Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

The item type returned when constructing this system param. The value of this associated type should be `Self`, instantiated with new lifetimes. [Read more](../ecs/system/trait.SystemParam.html#associatedtype.Item)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#308)

#### fn [init\_state](../ecs/system/trait.SystemParam.html#tymethod.init_state)(world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> <[Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F> as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Creates a new instance of this param’s [`State`](../ecs/system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#315-320)

#### fn [init\_access](../ecs/system/trait.SystemParam.html#tymethod.init_access)( state: &<[Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F> as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](../ecs/system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../ecs/query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](struct.World.html "struct bevy::prelude::World"), )

Registers any [`World`](struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"). [Read more](../ecs/system/trait.SystemParam.html#tymethod.init_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#325-330)

#### unsafe fn [get\_param](../ecs/system/trait.SystemParam.html#tymethod.get_param)<'w, 's>( state: &'s mut <[Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F> as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](../ecs/system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, change\_tick: [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Query](struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F> as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>, [SystemParamValidationError](../ecs/system/struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into a [`SystemParamFunction`](trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction"). [Read more](../ecs/system/trait.SystemParam.html#tymethod.get_param)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#250)

#### fn [apply](../ecs/system/trait.SystemParam.html#method.apply)(state: &mut Self::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](../ecs/system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](struct.World.html "struct bevy::prelude::World"))

Applies any deferred mutations stored in this [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")’s state. This is used to apply [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") during [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#258)

#### fn [queue](../ecs/system/trait.SystemParam.html#method.queue)( state: &mut Self::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](../ecs/system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#489-490)

### impl<'w, 's, D, F> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>> for [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#492)

#### fn [build](trait.SystemParamBuilder.html#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Registers any [`World`](struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") and creates a new instance of this param’s [`State`](../ecs/system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#139)

#### fn [build\_state](trait.SystemParamBuilder.html#method.build_state)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> [SystemState](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

Create a [`SystemState`](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder"). To create a system, call [`SystemState::build_system`](../ecs/system/struct.SystemState.html#method.build_system "method bevy::ecs::system::SystemState::build_system") on the result.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#158-164)

#### fn [build\_system](trait.SystemParamBuilder.html#method.build_system)<Marker, In, Out, Func>( self, func: Func, ) -> [IntoBuilderSystem](../ecs/system/struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")<Marker, In, Out, Func, Self>

where Self: 'static, Func: [SystemParamFunction](trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker, Param = P>,

Create a [`System`](trait.System.html "trait bevy::prelude::System") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") directly. [Read more](trait.SystemParamBuilder.html#method.build_system)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#563-569)

### impl<'w, 's, D, F, T> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>> for [QueryParamBuilder](../ecs/system/struct.QueryParamBuilder.html "struct bevy::ecs::system::QueryParamBuilder")<T>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static, T: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [QueryBuilder](struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")<'\_, D, F>),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#571)

#### fn [build](trait.SystemParamBuilder.html#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

Registers any [`World`](struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") and creates a new instance of this param’s [`State`](../ecs/system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#139)

#### fn [build\_state](trait.SystemParamBuilder.html#method.build_state)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> [SystemState](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

Create a [`SystemState`](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder"). To create a system, call [`SystemState::build_system`](../ecs/system/struct.SystemState.html#method.build_system "method bevy::ecs::system::SystemState::build_system") on the result.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#158-164)

#### fn [build\_system](trait.SystemParamBuilder.html#method.build_system)<Marker, In, Out, Func>( self, func: Func, ) -> [IntoBuilderSystem](../ecs/system/struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")<Marker, In, Out, Func, Self>

where Self: 'static, Func: [SystemParamFunction](trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker, Param = P>,

Create a [`System`](trait.System.html "trait bevy::prelude::System") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") directly. [Read more](trait.SystemParamBuilder.html#method.build_system)

## Auto Trait Implementations

### impl<'world, 'state, D, F = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

### impl<'world, 'state, D, F> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Query](struct.Query.html "struct bevy::prelude::Query")<'world, 'state, D, F>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

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

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

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

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

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

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"AncestorIter<'w, 's, D, F, R>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/relationship/struct.AncestorIter.html\\" title=\\"struct bevy::ecs::relationship::AncestorIter\\">AncestorIter</a>&lt;'w, 's, D, F, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/relationship/struct.AncestorIter.html\\" title=\\"struct bevy::ecs::relationship::AncestorIter\\">AncestorIter</a>&lt;'w, 's, D, F, R&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n R: <a class=\\"trait\\" href=\\"../ecs/relationship/trait.Relationship.html\\" title=\\"trait bevy::ecs::relationship::Relationship\\">Relationship</a>,\\n &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.ReadOnly\\" title=\\"type bevy::ecs::query::QueryData::ReadOnly\\">ReadOnly</a>: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&lt;Item&lt;'w, 's&gt; = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'w R</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","DescendantDepthFirstIter<'w, 's, D, F, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/relationship/struct.DescendantDepthFirstIter.html\\" title=\\"struct bevy::ecs::relationship::DescendantDepthFirstIter\\">DescendantDepthFirstIter</a>&lt;'w, 's, D, F, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, S&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/relationship/struct.DescendantDepthFirstIter.html\\" title=\\"struct bevy::ecs::relationship::DescendantDepthFirstIter\\">DescendantDepthFirstIter</a>&lt;'w, 's, D, F, S&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n S: <a class=\\"trait\\" href=\\"trait.RelationshipTarget.html\\" title=\\"trait bevy::prelude::RelationshipTarget\\">RelationshipTarget</a>,\\n &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.ReadOnly\\" title=\\"type bevy::ecs::query::QueryData::ReadOnly\\">ReadOnly</a>: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&lt;Item&lt;'w, 's&gt; = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'w S</a>&gt;,\\n &lt;&lt;S as <a class=\\"trait\\" href=\\"trait.RelationshipTarget.html\\" title=\\"trait bevy::prelude::RelationshipTarget\\">RelationshipTarget</a>&gt;::<a class=\\"associatedtype\\" href=\\"trait.RelationshipTarget.html#associatedtype.Collection\\" title=\\"type bevy::prelude::RelationshipTarget::Collection\\">Collection</a> as <a class=\\"trait\\" href=\\"../ecs/relationship/trait.RelationshipSourceCollection.html\\" title=\\"trait bevy::ecs::relationship::RelationshipSourceCollection\\">RelationshipSourceCollection</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter\\" title=\\"type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter\\">SourceIter</a>&lt;'w&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/double\_ended/trait.DoubleEndedIterator.html\\" title=\\"trait core::iter::traits::double\_ended::DoubleEndedIterator\\">DoubleEndedIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","DescendantIter<'w, 's, D, F, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/relationship/struct.DescendantIter.html\\" title=\\"struct bevy::ecs::relationship::DescendantIter\\">DescendantIter</a>&lt;'w, 's, D, F, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, S&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/relationship/struct.DescendantIter.html\\" title=\\"struct bevy::ecs::relationship::DescendantIter\\">DescendantIter</a>&lt;'w, 's, D, F, S&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n S: <a class=\\"trait\\" href=\\"trait.RelationshipTarget.html\\" title=\\"trait bevy::prelude::RelationshipTarget\\">RelationshipTarget</a>,\\n &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.ReadOnly\\" title=\\"type bevy::ecs::query::QueryData::ReadOnly\\">ReadOnly</a>: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&lt;Item&lt;'w, 's&gt; = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'w S</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","QueryCombinationIter<'\_, 's, <D as QueryData>::ReadOnly, F, K>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, const K: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">K</a>\];</div>","QueryCombinationIter<'\_, 's, D, F, K>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, const K: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">K</a>\];</div>","QueryCombinationIter<'w, 's, D, F, K>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, const K: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryCombinationIter.html\\" title=\\"struct bevy::ecs::query::QueryCombinationIter\\">QueryCombinationIter</a>&lt;'w, 's, D, F, K&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">K</a>\];</div>","QueryIter<'\_, 's, <D as QueryData>::ReadOnly, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryIter<'\_, 's, D, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryIter<'w, 's, D, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryIter.html\\" title=\\"struct bevy::ecs::query::QueryIter\\">QueryIter</a>&lt;'w, 's, D, F&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyIter<'\_, 's, <D as QueryData>::ReadOnly, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyIter<'\_, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyIter<'w, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyIter.html\\" title=\\"struct bevy::ecs::query::QueryManyIter\\">QueryManyIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.ReadOnlyQueryData.html\\" title=\\"trait bevy::ecs::query::ReadOnlyQueryData\\">ReadOnlyQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntityEquivalent.html\\" title=\\"trait bevy::ecs::entity::EntityEquivalent\\">EntityEquivalent</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyUniqueIter<'\_, 's, <D as QueryData>::ReadOnly, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntitySetIterator.html\\" title=\\"trait bevy::ecs::entity::EntitySetIterator\\">EntitySetIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyUniqueIter<'\_, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntitySetIterator.html\\" title=\\"trait bevy::ecs::entity::EntitySetIterator\\">EntitySetIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","QueryManyUniqueIter<'w, 's, D, F, <EntityList as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'w, 's, D, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/query/struct.QueryManyUniqueIter.html\\" title=\\"struct bevy::ecs::query::QueryManyUniqueIter\\">QueryManyUniqueIter</a>&lt;'w, 's, D, F, I&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"../ecs/query/trait.IterQueryData.html\\" title=\\"trait bevy::ecs::query::IterQueryData\\">IterQueryData</a>,\\n F: <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryFilter.html\\" title=\\"trait bevy::ecs::query::QueryFilter\\">QueryFilter</a>,\\n I: <a class=\\"trait\\" href=\\"../ecs/entity/trait.EntitySetIterator.html\\" title=\\"trait bevy::ecs::entity::EntitySetIterator\\">EntitySetIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;D as <a class=\\"trait\\" href=\\"../ecs/query/trait.QueryData.html\\" title=\\"trait bevy::ecs::query::QueryData\\">QueryData</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/query/trait.QueryData.html#associatedtype.Item\\" title=\\"type bevy::ecs::query::QueryData::Item\\">Item</a>&lt;'w, 's&gt;;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}