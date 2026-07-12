[bevy](../../index.html)::[ecs](../index.html)

# Module entity\_disabling 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#35)

Disabled entities do not show up in queries unless the query explicitly mentions them.

Entities which are disabled in this way are not removed from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), and their relationships remain intact. In many cases, you may want to disable entire trees of entities at once, using [`EntityCommands::insert_recursive`](../../prelude/struct.EntityCommands.html#method.insert_recursive "method bevy::prelude::EntityCommands::insert_recursive").

While Bevy ships with a built-in [`Disabled`](struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled") component, you can also create your own disabling components, which will operate in the same way but can have distinct semantics.

```rust
use bevy_ecs::prelude::*;

// Our custom disabling component!
#[derive(Component, Clone)]
struct Prefab;

#[derive(Component)]
struct A;

let mut world = World::new();
world.register_disabling_component::<Prefab>();
world.spawn((A, Prefab));
world.spawn((A,));
world.spawn((A,));

let mut normal_query = world.query::<&A>();
assert_eq!(2, normal_query.iter(&world).count());

let mut prefab_query = world.query_filtered::<&A, With<Prefab>>();
assert_eq!(1, prefab_query.iter(&world).count());

let mut maybe_prefab_query = world.query::<(&A, Has<Prefab>)>();
assert_eq!(3, maybe_prefab_query.iter(&world).count());
```

### Default query filters

In Bevy, entity disabling is implemented through the construction of a global “default query filter” resource. Queries which do not explicitly mention the disabled component will not include entities with that component. If an entity has multiple disabling components, it will only be included in queries that mention all of them.

For example, `Query<&Position>` will not include entities with the [`Disabled`](struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled") component, even if they have a `Position` component, but `Query<&Position, With<Disabled>>` or `Query<(&Position, Has<Disabled>)>` will see them.

The [`Allow`](../../prelude/struct.Allow.html "struct bevy::prelude::Allow") query filter is designed to be used with default query filters, and ensures that the query will include entities both with and without the specified disabling component.

Entities with disabling components are still present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and can be accessed directly, using methods on [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") or [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands").

As default query filters are implemented through a resource, it’s possible to temporarily ignore any default filters by using [`World::resource_scope`](../../prelude/struct.World.html "struct bevy::prelude::World").

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::entity_disabling::{DefaultQueryFilters, Disabled};

let mut world = World::default();

#[derive(Component)]
struct CustomDisabled;

world.register_disabling_component::<CustomDisabled>();

world.spawn(Disabled);
world.spawn(CustomDisabled);

// resource_scope removes DefaultQueryFilters temporarily before re-inserting into the world.
world.resource_scope(|world: &mut World, _: Mut<DefaultQueryFilters>| {
    // within this scope, we can query like no components are disabled.
    assert_eq!(world.query::<&Disabled>().query(&world).count(), 1);
    assert_eq!(world.query::<&CustomDisabled>().query(&world).count(), 1);
    assert_eq!(world.query::<()>().query(&world).count(), world.entities().count_spawned() as usize);
})
```

#### Warnings

Currently, only queries for which the cache is built after enabling a default query filter will have entities with those components filtered. As a result, they should generally only be modified before the app starts.

Because filters are applied to all queries they can have performance implication for the entire [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), especially when they cause queries to mix sparse and table components. See [`Query` performance](../../prelude/struct.Query.html#performance "struct bevy::prelude::Query") for more info.

Custom disabling components can cause significant interoperability issues within the ecosystem, as users must be aware of each disabling component in use. Libraries should think carefully about whether they need to use a new disabling component, and clearly communicate their presence to their users to avoid the new for library compatibility flags.

## Structs

[DefaultQueryFilters](struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

Default query filters work by excluding entities with certain components from most queries.

[Disabled](struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

A marker component for disabled entities.